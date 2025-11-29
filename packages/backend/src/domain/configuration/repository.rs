use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::traits::ConfigurationRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{AppPaths, FileService};
use async_trait::async_trait;
use log::debug;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::RwLock;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Clone)]
pub struct ConfigurationRepositoryImpl {
    config: Arc<RwLock<AppConfig>>,
    file_path: PathBuf,
    data_loaded: Arc<AtomicBool>,
}

impl ConfigurationRepositoryImpl {
    pub async fn new() -> AppResult<Self> {
        let config_dir = AppPaths::ensure_config_dir().await?;
        let file_path = config_dir.join(CONFIG_FILE_NAME);

        Ok(Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            file_path,
            data_loaded: Arc::new(AtomicBool::new(false)),
        })
    }

    async fn ensure_data_loaded(&self) -> AppResult<()> {
        if !self.data_loaded.load(Ordering::Relaxed) {
            if let Err(e) = self.load().await {
                debug!("Failed to load configuration, using defaults: {}", e);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ConfigurationRepository for ConfigurationRepositoryImpl {
    async fn save(&self, config: &AppConfig) -> AppResult<()> {
        FileService::write_json(&self.file_path, config).await
    }

    async fn load(&self) -> AppResult<AppConfig> {
        let config: AppConfig = FileService::read_json_optional(&self.file_path)
            .await?
            .unwrap_or_default();

        {
            let mut current_config = self.config.write().await;
            *current_config = config.clone();
        }

        self.data_loaded.store(true, Ordering::Relaxed);
        debug!("Configuration loaded successfully");
        Ok(config)
    }

    async fn exists(&self) -> AppResult<bool> {
        Ok(FileService::exists(&self.file_path))
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
        let mut config = self.config.write().await;
        config.poe_client_log_path = path;
        drop(config);
        self.save(&self.get_in_memory_config().await?).await
    }

    async fn set_log_level(&self, level: String) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        self.ensure_valid_log_level(&level).await?;

        let mut config = self.config.write().await;
        config.log_level = level;
        drop(config);
        self.save(&self.get_in_memory_config().await?).await
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let default_config = AppConfig::default();
        self.update_in_memory_config(default_config.clone()).await?;
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
        let valid_log_levels = ["trace", "debug", "info", "warn", "warning", "error"];
        if !valid_log_levels.contains(&level.to_lowercase().as_str()) {
            return Err(AppError::validation_error(
                "validate_log_level",
                &format!(
                    "Invalid log level '{}'. Valid levels are: {}",
                    level,
                    valid_log_levels.join(", ")
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
