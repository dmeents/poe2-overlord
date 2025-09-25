use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::traits::ConfigurationRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{
    PersistenceRepository, PersistenceRepositoryImpl,
};
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration repository constants
const CONFIG_FILE_NAME: &str = "config.json";

/// Configuration repository implementation that handles all configuration data operations
#[derive(Clone)]
pub struct ConfigurationRepositoryImpl {
    /// Configuration data with thread-safe access
    config: Arc<RwLock<AppConfig>>,
    /// Persistence repository for configuration data
    persistence: PersistenceRepositoryImpl<AppConfig>,
}

impl ConfigurationRepositoryImpl {
    /// Create a new configuration repository
    pub fn new() -> AppResult<Self> {
        // Create persistence repository in config directory
        let persistence = PersistenceRepositoryImpl::<AppConfig>::new_in_config_dir(CONFIG_FILE_NAME)?;

        let repository = Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            persistence,
        };

        // Load existing configuration
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load()) {
            debug!("Failed to load configuration, using defaults: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ConfigurationRepository for ConfigurationRepositoryImpl {
    // Persistence operations
    async fn save(&self, config: &AppConfig) -> AppResult<()> {
        self.persistence.save(config).await
    }

    async fn load(&self) -> AppResult<AppConfig> {
        let config = self.persistence.load().await?;
        
        {
            let mut current_config = self.config.write().await;
            *current_config = config.clone();
        }

        debug!("Configuration loaded successfully");
        Ok(config)
    }

    async fn exists(&self) -> AppResult<bool> {
        self.persistence.exists().await
    }

    async fn delete(&self) -> AppResult<()> {
        self.persistence.delete().await?;

        // Reset to default configuration
        {
            let mut config = self.config.write().await;
            *config = AppConfig::default();
        }

        debug!("Configuration file deleted and reset to defaults");
        Ok(())
    }

    // Data management
    async fn get_in_memory_config(&self) -> AppResult<AppConfig> {
        let config = self.config.read().await;
        Ok(config.clone())
    }

    async fn update_in_memory_config(&self, config: AppConfig) -> AppResult<()> {
        {
            let mut current_config = self.config.write().await;
            *current_config = config;
        }
        Ok(())
    }

    // Query operations
    async fn get_poe_client_log_path(&self) -> AppResult<String> {
        let config = self.config.read().await;
        Ok(config.poe_client_log_path.clone())
    }

    async fn get_log_level(&self) -> AppResult<String> {
        let config = self.config.read().await;
        Ok(config.log_level.clone())
    }

    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo> {
        Ok(ConfigurationFileInfo::new(self.persistence.get_file_path().clone()))
    }

    // Data manipulation
    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        let mut config = self.config.write().await;
        config.poe_client_log_path = path;
        drop(config);
        self.save(&self.get_in_memory_config().await?).await
    }

    async fn set_log_level(&self, level: String) -> AppResult<()> {
        // Validate log level first
        self.ensure_valid_log_level(&level).await?;
        
        let mut config = self.config.write().await;
        config.log_level = level;
        drop(config);
        self.save(&self.get_in_memory_config().await?).await
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        let default_config = AppConfig::default();
        self.update_in_memory_config(default_config.clone()).await?;
        self.save(&default_config).await
    }

    // Business rules and validation
    async fn validate_config(&self, config: &AppConfig) -> AppResult<ConfigurationValidationResult> {
        match config.validate() {
            Ok(()) => Ok(ConfigurationValidationResult::valid()),
            Err(error) => Ok(ConfigurationValidationResult::invalid(vec![error])),
        }
    }

    async fn ensure_valid_log_level(&self, level: &str) -> AppResult<()> {
        let valid_log_levels = ["trace", "debug", "info", "warn", "warning", "error"];
        if !valid_log_levels.contains(&level.to_lowercase().as_str()) {
            return Err(AppError::validation_error(
                "log_level",
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
                "poe_client_log_path",
                "POE client log path cannot be empty",
            ));
        }
        Ok(())
    }

    // Utility operations
    async fn get_default_poe_client_log_path(&self) -> String {
        AppConfig::get_default_poe_client_log_path()
    }

    async fn is_using_default_poe_path(&self) -> AppResult<bool> {
        let config = self.config.read().await;
        Ok(config.is_using_default_poe_path())
    }
}
