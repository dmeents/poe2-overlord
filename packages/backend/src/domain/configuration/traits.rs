use crate::domain::configuration::models::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait for configuration service operations
#[async_trait]
pub trait ConfigurationService: Send + Sync {
    /// Get the current configuration
    async fn get_config(&self) -> AppResult<AppConfig>;

    /// Update the entire configuration
    async fn update_config(&self, new_config: AppConfig) -> AppResult<()>;

    /// Update the POE client log path
    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()>;

    /// Update the log level
    async fn set_log_level(&self, level: String) -> AppResult<()>;

    /// Reset configuration to defaults
    async fn reset_to_defaults(&self) -> AppResult<()>;

    /// Load configuration from storage
    async fn load_config(&self) -> AppResult<()>;

    /// Save current configuration to storage
    async fn save_config(&self) -> AppResult<()>;

    /// Validate configuration
    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;

    /// Get configuration file information
    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo>;

    /// Get the POE client log path (with fallback to default)
    async fn get_poe_client_log_path(&self) -> AppResult<String>;

    /// Get log level
    async fn get_log_level(&self) -> AppResult<String>;

    /// Get the OS-specific default POE client log path
    fn get_default_poe_client_log_path(&self) -> String;

    /// Reset the POE client log path to the OS-specific default
    async fn reset_poe_client_log_path_to_default(&self) -> AppResult<()>;

    /// Subscribe to configuration change events
    fn subscribe_to_config_changes(
        &self,
    ) -> tokio::sync::broadcast::Receiver<ConfigurationChangedEvent>;
}
