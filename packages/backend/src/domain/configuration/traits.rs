use crate::domain::configuration::models::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ConfigurationService: Send + Sync {
    async fn get_config(&self) -> AppResult<AppConfig>;

    async fn update_config(&self, new_config: AppConfig) -> AppResult<()>;

    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()>;

    async fn set_log_level(&self, level: String) -> AppResult<()>;

    async fn reset_to_defaults(&self) -> AppResult<()>;

    async fn load_config(&self) -> AppResult<()>;

    async fn save_config(&self) -> AppResult<()>;

    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;

    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo>;

    async fn get_poe_client_log_path(&self) -> AppResult<String>;

    async fn get_log_level(&self) -> AppResult<String>;

    fn get_default_poe_client_log_path(&self) -> String;

    async fn reset_poe_client_log_path_to_default(&self) -> AppResult<()>;

    fn subscribe_to_config_changes(
        &self,
    ) -> tokio::sync::broadcast::Receiver<ConfigurationChangedEvent>;
}

#[async_trait]
pub trait ConfigurationRepository: Send + Sync {
    async fn save(&self, config: &AppConfig) -> AppResult<()>;
    async fn load(&self) -> AppResult<AppConfig>;
    async fn exists(&self) -> AppResult<bool>;
    async fn delete(&self) -> AppResult<()>;

    async fn get_in_memory_config(&self) -> AppResult<AppConfig>;
    async fn update_in_memory_config(&self, config: AppConfig) -> AppResult<()>;

    async fn get_poe_client_log_path(&self) -> AppResult<String>;
    async fn get_log_level(&self) -> AppResult<String>;
    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo>;

    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()>;
    async fn set_log_level(&self, level: String) -> AppResult<()>;
    async fn reset_to_defaults(&self) -> AppResult<()>;

    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;
    async fn ensure_valid_log_level(&self, level: &str) -> AppResult<()>;
    async fn ensure_valid_poe_path(&self, path: &str) -> AppResult<()>;

    async fn get_default_poe_client_log_path(&self) -> String;
    async fn is_using_default_poe_path(&self) -> AppResult<bool>;
}
