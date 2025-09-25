use crate::domain::configuration::models::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::errors::AppResult;
use async_trait::async_trait;

/// High-level configuration service interface
/// 
/// This trait defines the primary business logic interface for configuration management.
/// It provides high-level operations that coordinate between the repository layer and
/// external clients, handling validation, events, and complex operations.
/// 
/// # Responsibilities
/// 
/// - Configuration CRUD operations with validation
/// - Event broadcasting for configuration changes  
/// - Coordination between repository and business logic
/// - File I/O operations with error handling
/// - Configuration change notifications
/// 
/// # Implementation Notes
/// 
/// Implementations should ensure thread safety and handle all error cases gracefully.
/// All configuration changes should trigger appropriate events for dependent components.
#[async_trait]
pub trait ConfigurationService: Send + Sync {
    /// Retrieve the current configuration
    async fn get_config(&self) -> AppResult<AppConfig>;

    /// Update the entire configuration with validation and event broadcasting
    async fn update_config(&self, new_config: AppConfig) -> AppResult<()>;

    /// Set the Path of Exile client log file path
    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()>;

    /// Set the application logging level
    async fn set_log_level(&self, level: String) -> AppResult<()>;

    /// Reset all configuration settings to their default values
    async fn reset_to_defaults(&self) -> AppResult<()>;

    /// Load configuration from persistent storage
    async fn load_config(&self) -> AppResult<()>;

    /// Save current configuration to persistent storage
    async fn save_config(&self) -> AppResult<()>;

    /// Validate a configuration object for correctness
    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;

    /// Get metadata information about the configuration file
    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo>;

    /// Get the current POE client log path (with fallback to default)
    async fn get_poe_client_log_path(&self) -> AppResult<String>;

    /// Get the current application log level
    async fn get_log_level(&self) -> AppResult<String>;

    /// Get the system default POE client log path (synchronous)
    fn get_default_poe_client_log_path(&self) -> String;

    /// Reset POE client log path to the system default
    async fn reset_poe_client_log_path_to_default(&self) -> AppResult<()>;

    /// Subscribe to configuration change events
    /// 
    /// Returns a broadcast receiver that will receive all future configuration
    /// change events until it is dropped.
    fn subscribe_to_config_changes(
        &self,
    ) -> tokio::sync::broadcast::Receiver<ConfigurationChangedEvent>;
}

/// Low-level configuration repository interface
/// 
/// This trait defines the data access layer for configuration management.
/// It handles direct interaction with persistence storage, in-memory caching,
/// and basic validation operations.
/// 
/// # Responsibilities
/// 
/// - Persistent storage operations (save, load, delete)
/// - In-memory configuration caching
/// - Basic validation and integrity checks
/// - File system operations and metadata
/// - Low-level configuration field access
/// 
/// # Design Philosophy
/// 
/// The repository should be a thin layer over the persistence mechanism,
/// providing thread-safe access to configuration data without complex
/// business logic. Business rules and event handling should be implemented
/// in the service layer.
#[async_trait]
pub trait ConfigurationRepository: Send + Sync {
    /// Save configuration to persistent storage
    async fn save(&self, config: &AppConfig) -> AppResult<()>;
    
    /// Load configuration from persistent storage
    async fn load(&self) -> AppResult<AppConfig>;
    
    /// Check if configuration file exists in persistent storage
    async fn exists(&self) -> AppResult<bool>;
    
    /// Delete configuration file from persistent storage
    async fn delete(&self) -> AppResult<()>;

    /// Get the current in-memory configuration (fast access)
    async fn get_in_memory_config(&self) -> AppResult<AppConfig>;
    
    /// Update the in-memory configuration cache
    async fn update_in_memory_config(&self, config: AppConfig) -> AppResult<()>;

    /// Get POE client log path from in-memory config
    async fn get_poe_client_log_path(&self) -> AppResult<String>;
    
    /// Get log level from in-memory config
    async fn get_log_level(&self) -> AppResult<String>;
    
    /// Get configuration file metadata information
    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo>;

    /// Set POE client log path and persist to storage
    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()>;
    
    /// Set log level with validation and persist to storage
    async fn set_log_level(&self, level: String) -> AppResult<()>;
    
    /// Reset configuration to defaults and persist
    async fn reset_to_defaults(&self) -> AppResult<()>;

    /// Validate a configuration object
    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;
        
    /// Validate that a log level string is acceptable
    async fn ensure_valid_log_level(&self, level: &str) -> AppResult<()>;
    
    /// Validate that a POE client path is acceptable
    async fn ensure_valid_poe_path(&self, path: &str) -> AppResult<()>;

    /// Get the system default POE client log path
    async fn get_default_poe_client_log_path(&self) -> String;
    
    /// Check if current POE path is using the system default
    async fn is_using_default_poe_path(&self) -> AppResult<bool>;
}
