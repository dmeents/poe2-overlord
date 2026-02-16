use crate::domain::configuration::models::{AppConfig, ConfigurationValidationResult};
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ConfigurationService: Send + Sync {
    async fn get_config(&self) -> AppResult<AppConfig>;

    async fn update_config(&self, new_config: AppConfig) -> AppResult<()>;

    /// Force immediate flush of pending configuration writes to disk.
    /// Call this on app shutdown to ensure all changes are persisted.
    async fn flush(&self) -> AppResult<()>;

    async fn reset_to_defaults(&self) -> AppResult<()>;

    async fn get_zone_refresh_interval(
        &self,
    ) -> AppResult<crate::domain::configuration::models::ZoneRefreshInterval>;

    async fn set_zone_refresh_interval(
        &self,
        interval: crate::domain::configuration::models::ZoneRefreshInterval,
    ) -> AppResult<()>;
}

#[async_trait]
pub trait ConfigurationRepository: Send + Sync {
    async fn save(&self, config: &AppConfig) -> AppResult<()>;

    /// Force immediate flush of pending writes to disk, bypassing debounce.
    /// Call this on app shutdown or when immediate persistence is required.
    async fn flush(&self) -> AppResult<()>;

    async fn load(&self) -> AppResult<AppConfig>;

    async fn get_in_memory_config(&self) -> AppResult<AppConfig>;

    async fn validate_config(&self, config: &AppConfig)
        -> AppResult<ConfigurationValidationResult>;
}
