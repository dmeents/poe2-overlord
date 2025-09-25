use crate::domain::configuration::models::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::traits::ConfigurationService;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio::task;

/// Configuration constants
const CONFIG_DIR_NAME: &str = "poe2-overlord";
const CONFIG_FILE_NAME: &str = "config.json";
const TEMP_FILE_EXTENSION: &str = "tmp";

/// Configuration service implementation
pub struct ConfigurationServiceImpl {
    config_path: PathBuf,
    config: Arc<RwLock<AppConfig>>,
    event_sender: broadcast::Sender<ConfigurationChangedEvent>,
}

impl ConfigurationServiceImpl {
    /// Create a new configuration service
    pub fn new() -> AppResult<Self> {
        // Use standard config directory for the current user
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(CONFIG_DIR_NAME);

        // Ensure config directory exists
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                warn!("Failed to create config directory: {}", e);
                return Err(AppError::file_system_error("create_config_directory", &e.to_string()));
            }
        }

        let config_path = config_dir.join(CONFIG_FILE_NAME);
        let (event_sender, _) = broadcast::channel(16);
        
        let service = Self {
            config_path,
            config: Arc::new(RwLock::new(AppConfig::default())),
            event_sender,
        };

        // Load existing configuration or create default
        if let Err(e) = tauri::async_runtime::block_on(service.load_config()) {
            warn!("Failed to load config, using defaults: {}", e);
            // Try to save the default config to ensure we have a working file
            if let Err(save_err) = tauri::async_runtime::block_on(service.save_config()) {
                warn!("Failed to save default config: {}", save_err);
            }
        }

        Ok(service)
    }

    /// Broadcast configuration change event
    fn broadcast_config_change(&self, new_config: AppConfig, previous_config: AppConfig) {
        let event = ConfigurationChangedEvent::new(new_config, previous_config);
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to broadcast configuration change event: {}", e);
        }
    }

    /// Validate configuration and return detailed results
    fn validate_config_internal(&self, config: &AppConfig) -> ConfigurationValidationResult {
        match config.validate() {
            Ok(()) => ConfigurationValidationResult::valid(),
            Err(error) => ConfigurationValidationResult::invalid(vec![error]),
        }
    }

    /// Write content to a temporary file, then rename atomically
    async fn atomic_write(&self, content: &str) -> AppResult<()> {
        let temp_path = self.config_path.with_extension(TEMP_FILE_EXTENSION);
        
        // Write to temporary file
        task::spawn_blocking({
            let temp_path = temp_path.clone();
            let content = content.to_string();
            move || fs::write(&temp_path, content)
        })
        .await
        .map_err(|e| AppError::file_system_error("spawn_write_task", &e.to_string()))?
        .map_err(|e| AppError::file_system_error("write_temp_file", &e.to_string()))?;

        // Rename to final location
        task::spawn_blocking({
            let temp_path = temp_path.clone();
            let config_path = self.config_path.clone();
            move || fs::rename(&temp_path, &config_path)
        })
        .await
        .map_err(|e| AppError::file_system_error("spawn_rename_task", &e.to_string()))?
        .map_err(|e| AppError::file_system_error("rename_temp_file", &e.to_string()))?;

        debug!("Configuration saved successfully to {:?}", self.config_path);
        Ok(())
    }

    /// Read content from file
    async fn read_file(&self) -> AppResult<String> {
        task::spawn_blocking({
            let config_path = self.config_path.clone();
            move || fs::read_to_string(&config_path)
        })
        .await
        .map_err(|e| AppError::file_system_error("spawn_read_task", &e.to_string()))?
        .map_err(|e| AppError::file_system_error("read_config_file", &e.to_string()))
    }
}

#[async_trait]
impl ConfigurationService for ConfigurationServiceImpl {
    async fn get_config(&self) -> AppResult<AppConfig> {
        let config = self.config.read().await;
        Ok(config.clone())
    }

    async fn update_config(&self, new_config: AppConfig) -> AppResult<()> {
        // Validate the new configuration
        let validation_result = self.validate_config_internal(&new_config);
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "configuration",
                &format!("Configuration validation failed: {}", validation_result.errors.join(", "))
            ));
        }

        // Get current config for event
        let previous_config = self.get_config().await?;

        // Update the configuration
        {
            let mut config = self.config.write().await;
            *config = new_config.clone();
        }

        // Save to storage
        self.save_config().await?;

        // Broadcast change event
        self.broadcast_config_change(new_config, previous_config);

        info!("Configuration updated successfully");
        Ok(())
    }

    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        let previous_config = self.get_config().await?;
        let mut new_config = previous_config.clone();
        
        new_config.poe_client_log_path = path;

        // Validate the updated configuration
        let validation_result = self.validate_config_internal(&new_config);
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "configuration",
                &format!("Configuration validation failed: {}", validation_result.errors.join(", "))
            ));
        }

        // Update the configuration
        {
            let mut config = self.config.write().await;
            *config = new_config.clone();
        }

        // Save to storage
        self.save_config().await?;

        // Broadcast change event
        self.broadcast_config_change(new_config, previous_config);

        debug!("POE client log path updated successfully");
        Ok(())
    }

    async fn set_log_level(&self, level: String) -> AppResult<()> {
        let previous_config = self.get_config().await?;
        let mut new_config = previous_config.clone();
        
        new_config.log_level = level;

        // Validate the updated configuration
        let validation_result = self.validate_config_internal(&new_config);
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "configuration",
                &format!("Configuration validation failed: {}", validation_result.errors.join(", "))
            ));
        }

        // Update the configuration
        {
            let mut config = self.config.write().await;
            *config = new_config.clone();
        }

        // Save to storage
        self.save_config().await?;

        // Broadcast change event
        self.broadcast_config_change(new_config, previous_config);

        debug!("Log level updated successfully");
        Ok(())
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        let default_config = AppConfig::default();
        self.update_config(default_config).await
    }

    async fn load_config(&self) -> AppResult<()> {
        if !self.config_path.exists() {
            info!("No config file found, creating default configuration");
            self.save_config().await?;
            return Ok(());
        }

        let content = self.read_file().await?;

        let config: AppConfig = serde_json::from_str(&content).map_err(|e| {
            error!("Failed to parse config file JSON: {}", e);
            error!("Config file content: {}", content);
            AppError::serialization_error("parse_config_file", &e.to_string())
        })?;

        {
            let mut current_config = self.config.write().await;
            *current_config = config;
        }

        info!(
            "Configuration loaded successfully from {:?}",
            self.config_path
        );
        Ok(())
    }

    async fn save_config(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let content = serde_json::to_string_pretty(&*config)
            .map_err(|e| AppError::serialization_error("serialize_config", &e.to_string()))?;

        self.atomic_write(&content).await
    }

    async fn validate_config(&self, config: &AppConfig) -> AppResult<ConfigurationValidationResult> {
        Ok(self.validate_config_internal(config))
    }

    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo> {
        Ok(ConfigurationFileInfo::new(self.config_path.clone()))
    }

    async fn get_poe_client_log_path(&self) -> AppResult<String> {
        let config = self.config.read().await;
        let path = &config.poe_client_log_path;

        // If the path is empty, return the OS-specific default
        if path.is_empty() {
            Ok(AppConfig::get_default_poe_client_log_path())
        } else {
            Ok(path.clone())
        }
    }

    async fn get_log_level(&self) -> AppResult<String> {
        let config = self.config.read().await;
        Ok(config.log_level.clone())
    }

    fn get_default_poe_client_log_path(&self) -> String {
        AppConfig::get_default_poe_client_log_path()
    }

    async fn reset_poe_client_log_path_to_default(&self) -> AppResult<()> {
        let default_path = self.get_default_poe_client_log_path();
        self.set_poe_client_log_path(default_path).await
    }

    fn subscribe_to_config_changes(&self) -> broadcast::Receiver<ConfigurationChangedEvent> {
        self.event_sender.subscribe()
    }
}

impl Default for ConfigurationServiceImpl {
    fn default() -> Self {
        Self::new().expect("Failed to create default configuration service")
    }
}
