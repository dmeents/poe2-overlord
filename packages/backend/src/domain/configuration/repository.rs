use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::traits::ConfigurationRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::file_management::{AppPaths, FileService};
use crate::infrastructure::PathValidator;
use async_trait::async_trait;
use log::{debug, warn};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::{mpsc, Notify, RwLock};
use tokio::time::{timeout, Duration};

const CONFIG_FILE_NAME: &str = "config.json";
/// Debounce duration for disk writes - 500ms after last save request
const DEBOUNCE_DURATION_MS: u64 = 500;

#[derive(Clone)]
pub struct ConfigurationRepositoryImpl {
    config: Arc<RwLock<AppConfig>>,
    file_path: PathBuf,
    data_loaded: Arc<AtomicBool>,
    /// Sender to signal the debounce task that a save is requested
    save_signal: mpsc::UnboundedSender<()>,
    /// Notify to signal immediate flush
    flush_notify: Arc<Notify>,
}

impl ConfigurationRepositoryImpl {
    pub async fn new() -> AppResult<Self> {
        let config_dir = AppPaths::ensure_config_dir().await?;
        let file_path = config_dir.join(CONFIG_FILE_NAME);

        let config = Arc::new(RwLock::new(AppConfig::default()));
        let (save_signal, save_rx) = mpsc::unbounded_channel();
        let flush_notify = Arc::new(Notify::new());

        // Spawn debounce background task
        let config_clone = config.clone();
        let file_path_clone = file_path.clone();
        let flush_notify_clone = flush_notify.clone();

        tokio::spawn(async move {
            Self::debounce_write_task(save_rx, config_clone, file_path_clone, flush_notify_clone)
                .await;
        });

        Ok(Self {
            config,
            file_path,
            data_loaded: Arc::new(AtomicBool::new(false)),
            save_signal,
            flush_notify,
        })
    }

    /// Background task that debounces disk writes.
    /// Waits for DEBOUNCE_DURATION_MS after the last save signal before writing.
    /// Can be interrupted by a flush signal for immediate writes.
    async fn debounce_write_task(
        mut save_rx: mpsc::UnboundedReceiver<()>,
        config: Arc<RwLock<AppConfig>>,
        file_path: PathBuf,
        flush_notify: Arc<Notify>,
    ) {
        loop {
            // Wait for a save request
            if save_rx.recv().await.is_none() {
                // Channel closed, exit task
                break;
            }

            // Start debounce period - wait for either:
            // 1. Debounce timeout to elapse (no more save signals)
            // 2. Flush signal for immediate write
            loop {
                tokio::select! {
                    // Wait for debounce duration or flush signal
                    result = timeout(
                        Duration::from_millis(DEBOUNCE_DURATION_MS),
                        save_rx.recv()
                    ) => {
                        match result {
                            Ok(Some(())) => {
                                // Another save came in, restart debounce timer
                                continue;
                            }
                            Ok(None) => {
                                // Channel closed, perform final write and exit
                                Self::perform_disk_write(&config, &file_path).await;
                                return;
                            }
                            Err(_) => {
                                // Timeout elapsed, no more saves - write to disk
                                break;
                            }
                        }
                    }
                    _ = flush_notify.notified() => {
                        // Flush requested, write immediately
                        break;
                    }
                }
            }

            // Perform the actual disk write
            Self::perform_disk_write(&config, &file_path).await;
        }
    }

    /// Perform the actual disk write operation
    async fn perform_disk_write(config: &Arc<RwLock<AppConfig>>, file_path: &PathBuf) {
        let config_to_save = {
            let config_guard = config.read().await;
            config_guard.clone()
        };

        match FileService::write_json(file_path, &config_to_save).await {
            Ok(_) => debug!("Debounced config write completed successfully"),
            Err(e) => warn!("Failed to write debounced config: {}", e),
        }
    }

    /// Force immediate flush of pending writes to disk
    pub async fn flush(&self) -> AppResult<()> {
        // Signal the debounce task to write immediately
        self.flush_notify.notify_one();
        // Give the task time to process the flush
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    /// Lazy-loads configuration on first access to avoid startup overhead
    async fn ensure_data_loaded(&self) -> AppResult<()> {
        if !self.data_loaded.load(Ordering::Relaxed) {
            if let Err(e) = self.load().await {
                debug!("Failed to load configuration, using defaults: {}", e);
                // Set flag to prevent repeated load attempts and log spam
                self.data_loaded.store(true, Ordering::Relaxed);
                // Ensure default config is set in memory
                let default_config = AppConfig::default();
                let mut config = self.config.write().await;
                *config = default_config;
            }
        }
        Ok(())
    }

    /// Migrate configuration with invalid/insecure POE path to use default path
    async fn migrate_invalid_path(&self, mut config: AppConfig) -> AppConfig {
        let validator = PathValidator::new_for_poe_logs();

        match validator.validate_path(&config.poe_client_log_path) {
            Ok(validated_path) => {
                // Path is valid, update to canonical form
                config.poe_client_log_path = validated_path.to_string_lossy().to_string();
                debug!("POE path validated and canonicalized");
            }
            Err(e) => {
                // Path is invalid/insecure, reset to default
                warn!(
                    "Existing POE path '{}' failed validation: {}. Resetting to default.",
                    config.poe_client_log_path, e
                );
                config.poe_client_log_path = AppConfig::get_default_poe_client_log_path();
            }
        }

        config.config_version = AppConfig::CURRENT_VERSION;
        config
    }
}

#[async_trait]
impl ConfigurationRepository for ConfigurationRepositoryImpl {
    async fn save(&self, config: &AppConfig) -> AppResult<()> {
        // Update in-memory config with incremented version
        let config_to_save = config.with_incremented_version();
        {
            let mut current_config = self.config.write().await;
            *current_config = config_to_save;
        }

        // Signal the debounce task to schedule a disk write
        // The actual write will happen after DEBOUNCE_DURATION_MS of no more saves
        self.save_signal.send(()).map_err(|_| {
            AppError::internal_error(
                "debounce_save",
                "Failed to send save signal to background task",
            )
        })?;

        Ok(())
    }

    async fn flush(&self) -> AppResult<()> {
        // Delegate to the impl method
        ConfigurationRepositoryImpl::flush(self).await
    }

    async fn load(&self) -> AppResult<AppConfig> {
        let mut config: AppConfig = FileService::read_json_optional(&self.file_path)
            .await?
            .unwrap_or_default();

        // Check if migration is needed (old config without version field defaults to CURRENT_VERSION via serde)
        // We detect old configs by checking if the path validation fails (they didn't have validation before)
        let needs_path_migration = {
            let validator = PathValidator::new_for_poe_logs();
            validator
                .validate_path(&config.poe_client_log_path)
                .is_err()
        };

        if needs_path_migration {
            debug!("Configuration needs path migration - validating and potentially resetting POE path");
            config = self.migrate_invalid_path(config).await;

            // Save migrated config
            if let Err(e) = self.save(&config).await {
                warn!("Failed to save migrated config: {}", e);
            }
        }

        {
            let mut current_config = self.config.write().await;
            *current_config = config.clone();
        }

        self.data_loaded.store(true, Ordering::Relaxed);
        debug!("Configuration loaded successfully");
        Ok(config)
    }

    async fn exists(&self) -> AppResult<bool> {
        Ok(FileService::exists(&self.file_path).await?)
    }

    async fn delete(&self) -> AppResult<()> {
        FileService::delete(&self.file_path).await?;

        {
            let mut config = self.config.write().await;
            *config = AppConfig::default();
        }

        debug!("Configuration file deleted and reset to defaults");
        Ok(())
    }

    async fn get_in_memory_config(&self) -> AppResult<AppConfig> {
        self.ensure_data_loaded().await?;
        let config = self.config.read().await;
        Ok(config.clone())
    }

    async fn update_in_memory_config(&self, config: AppConfig) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        {
            let mut current_config = self.config.write().await;
            *current_config = config;
        }
        Ok(())
    }

    async fn get_poe_client_log_path(&self) -> AppResult<String> {
        self.ensure_data_loaded().await?;
        let config = self.config.read().await;
        Ok(config.poe_client_log_path.clone())
    }

    async fn get_log_level(&self) -> AppResult<String> {
        self.ensure_data_loaded().await?;
        let config = self.config.read().await;
        Ok(config.log_level.clone())
    }

    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo> {
        self.ensure_data_loaded().await?;
        Ok(ConfigurationFileInfo::new(self.file_path.clone()))
    }

    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        self.ensure_data_loaded().await?;

        // Read current config and prepare updated config
        let config_to_save = {
            let config = self.config.read().await;
            let mut updated = config.clone();
            updated.poe_client_log_path = path;
            updated
        };

        // Save handles in-memory update and schedules debounced disk write
        self.save(&config_to_save).await
    }

    async fn set_log_level(&self, level: String) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        self.ensure_valid_log_level(&level).await?;

        // Read current config and prepare updated config
        let config_to_save = {
            let config = self.config.read().await;
            let mut updated = config.clone();
            updated.log_level = level;
            updated
        };

        // Save handles in-memory update and schedules debounced disk write
        self.save(&config_to_save).await
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        self.ensure_data_loaded().await?;

        // Read current version for the version check
        let current_version = {
            let config = self.config.read().await;
            config.version
        };

        // Create default config but preserve version for the check
        let mut default_config = AppConfig::default();
        default_config.version = current_version;

        // Save handles in-memory update and schedules debounced disk write
        self.save(&default_config).await
    }

    async fn validate_config(
        &self,
        config: &AppConfig,
    ) -> AppResult<ConfigurationValidationResult> {
        match config.validate() {
            Ok(()) => Ok(ConfigurationValidationResult::valid()),
            Err(error) => Ok(ConfigurationValidationResult::invalid(vec![error])),
        }
    }

    async fn ensure_valid_log_level(&self, level: &str) -> AppResult<()> {
        if !AppConfig::is_valid_log_level(level) {
            return Err(AppError::validation_error(
                "validate_log_level",
                &format!(
                    "Invalid log level '{}'. Valid levels are: {}",
                    level,
                    AppConfig::VALID_LOG_LEVELS.join(", ")
                ),
            ));
        }
        Ok(())
    }

    async fn ensure_valid_poe_path(&self, path: &str) -> AppResult<()> {
        if path.trim().is_empty() {
            return Err(AppError::validation_error(
                "validate_poe_path",
                "POE client log path cannot be empty",
            ));
        }

        // Security validation
        let validator = PathValidator::new_for_poe_logs();
        validator.validate_path(path)?;

        Ok(())
    }

    async fn get_default_poe_client_log_path(&self) -> String {
        AppConfig::get_default_poe_client_log_path()
    }

    async fn is_using_default_poe_path(&self) -> AppResult<bool> {
        self.ensure_data_loaded().await?;
        let config = self.config.read().await;
        Ok(config.is_using_default_poe_path())
    }
}
