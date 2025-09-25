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

/// Configuration file name used for persistent storage
const CONFIG_FILE_NAME: &str = "config.json";

/// Concrete implementation of the ConfigurationRepository trait
/// 
/// This implementation provides thread-safe configuration management with both
/// persistent storage and in-memory caching for optimal performance.
/// 
/// # Architecture
/// 
/// - Uses `PersistenceRepositoryImpl` for file I/O operations
/// - Maintains in-memory cache with `RwLock` for thread-safe access
/// - Automatically loads configuration on initialization
/// - Provides atomic operations for configuration updates
/// 
/// # Thread Safety
/// 
/// All operations are thread-safe through the use of `RwLock` for in-memory
/// configuration access and the underlying persistence layer's safety guarantees.
#[derive(Clone)]
pub struct ConfigurationRepositoryImpl {
    /// Thread-safe in-memory configuration cache for fast access
    config: Arc<RwLock<AppConfig>>,
    
    /// Persistence layer for configuration file I/O operations
    persistence: PersistenceRepositoryImpl<AppConfig>,
}

impl ConfigurationRepositoryImpl {
    /// Create a new configuration repository instance
    /// 
    /// Initializes the repository with the persistence layer and attempts to load
    /// existing configuration. If loading fails, the repository will use default
    /// configuration values and log the error.
    /// 
    /// # Returns
    /// 
    /// * `Ok(ConfigurationRepositoryImpl)` on successful initialization
    /// * `Err(AppError)` if the persistence layer cannot be initialized
    /// 
    /// # Behavior
    /// 
    /// - Creates persistence layer in the system config directory
    /// - Initializes in-memory cache with default values
    /// - Attempts to load existing configuration from disk
    /// - Falls back to defaults if loading fails (with debug logging)
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<AppConfig>::new_in_config_dir(CONFIG_FILE_NAME)?;

        let repository = Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            persistence,
        };

        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load()) {
            debug!("Failed to load configuration, using defaults: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ConfigurationRepository for ConfigurationRepositoryImpl {
    /// Save configuration to persistent storage
    /// 
    /// Delegates to the underlying persistence layer to write configuration
    /// data to the file system. Does not update in-memory cache.
    async fn save(&self, config: &AppConfig) -> AppResult<()> {
        self.persistence.save(config).await
    }

    /// Load configuration from persistent storage and update in-memory cache
    /// 
    /// Reads configuration from the file system and updates the in-memory cache
    /// with the loaded values. This ensures consistency between disk and memory.
    /// 
    /// # Returns
    /// 
    /// The loaded configuration data
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

        {
            let mut config = self.config.write().await;
            *config = AppConfig::default();
        }

        debug!("Configuration file deleted and reset to defaults");
        Ok(())
    }

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

    /// Set POE client log path and persist to storage
    /// 
    /// Updates the in-memory configuration and immediately saves to persistent
    /// storage to ensure consistency.
    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        let mut config = self.config.write().await;
        config.poe_client_log_path = path;
        drop(config);
        self.save(&self.get_in_memory_config().await?).await
    }

    /// Set log level with validation and persist to storage
    /// 
    /// Validates the log level before updating the configuration.
    /// Updates both in-memory cache and persistent storage atomically.
    async fn set_log_level(&self, level: String) -> AppResult<()> {
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

    async fn validate_config(&self, config: &AppConfig) -> AppResult<ConfigurationValidationResult> {
        match config.validate() {
            Ok(()) => Ok(ConfigurationValidationResult::valid()),
            Err(error) => Ok(ConfigurationValidationResult::invalid(vec![error])),
        }
    }

    /// Validate that a log level string is acceptable
    /// 
    /// Checks the provided log level against the list of supported levels.
    /// This validation is case-insensitive.
    /// 
    /// # Arguments
    /// 
    /// * `level` - The log level string to validate
    /// 
    /// # Supported Levels
    /// 
    /// trace, debug, info, warn, warning, error
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

    /// Validate that a POE client path is acceptable
    /// 
    /// Ensures the path is not empty or whitespace-only. Additional validation
    /// like file existence checking should be done at a higher level.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The file path string to validate
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
        let config = self.config.read().await;
        Ok(config.is_using_default_poe_path())
    }
}
