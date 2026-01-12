use crate::domain::configuration::models::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::repository::ConfigurationRepositoryImpl;
use crate::domain::configuration::traits::{ConfigurationRepository, ConfigurationService};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::events::{AppEvent, EventBus, EventType};
use async_trait::async_trait;
use log::{debug, info, warn};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct ConfigurationServiceImpl {
    repository: Arc<ConfigurationRepositoryImpl>,
    event_bus: Arc<EventBus>,
}

impl ConfigurationServiceImpl {
    pub async fn new() -> AppResult<Self> {
        let repository = Arc::new(ConfigurationRepositoryImpl::new().await?);
        let event_bus = Arc::new(EventBus::new());

        let service = Self {
            repository,
            event_bus,
        };

        if let Err(e) = service.load_config().await {
            warn!("Failed to load config, using defaults: {}", e);
            if let Err(save_err) = service.save_config().await {
                warn!("Failed to save default config: {}", save_err);
            }
        }

        Ok(service)
    }

    async fn publish_config_change(&self, new_config: AppConfig, previous_config: AppConfig) {
        let event = AppEvent::ConfigurationChanged(ConfigurationChangedEvent::new(
            new_config,
            previous_config,
        ));
        if let Err(e) = self.event_bus.publish(event).await {
            warn!("Failed to publish configuration change event: {}", e);
        }
    }
}

#[async_trait]
impl ConfigurationService for ConfigurationServiceImpl {
    async fn get_config(&self) -> AppResult<AppConfig> {
        self.repository.get_in_memory_config().await
    }

    async fn update_config(&self, new_config: AppConfig) -> AppResult<()> {
        let validation_result = self.repository.validate_config(&new_config).await?;
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "validate_config",
                &format!(
                    "Configuration validation failed: {}",
                    validation_result.errors.join(", ")
                ),
            ));
        }

        let previous_config = self.get_config().await?;

        self.repository
            .update_in_memory_config(new_config.clone())
            .await?;
        self.repository.save(&new_config).await?;

        self.publish_config_change(new_config, previous_config)
            .await;

        info!("Configuration updated successfully");
        Ok(())
    }

    async fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        let previous_config = self.get_config().await?;
        let mut new_config = previous_config.clone();

        new_config.poe_client_log_path = path;

        let validation_result = self.repository.validate_config(&new_config).await?;
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "validate_config",
                &format!(
                    "Configuration validation failed: {}",
                    validation_result.errors.join(", ")
                ),
            ));
        }

        self.repository
            .update_in_memory_config(new_config.clone())
            .await?;
        self.repository.save(&new_config).await?;

        self.publish_config_change(new_config, previous_config)
            .await;

        debug!("POE client log path updated successfully");
        Ok(())
    }

    async fn set_log_level(&self, level: String) -> AppResult<()> {
        let previous_config = self.get_config().await?;
        let mut new_config = previous_config.clone();

        new_config.log_level = level;

        let validation_result = self.repository.validate_config(&new_config).await?;
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "validate_config",
                &format!(
                    "Configuration validation failed: {}",
                    validation_result.errors.join(", ")
                ),
            ));
        }

        self.repository
            .update_in_memory_config(new_config.clone())
            .await?;
        self.repository.save(&new_config).await?;

        self.publish_config_change(new_config, previous_config)
            .await;

        debug!("Log level updated successfully");
        Ok(())
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        let default_config = AppConfig::default();
        self.update_config(default_config).await
    }

    async fn flush(&self) -> AppResult<()> {
        self.repository.flush().await
    }

    async fn load_config(&self) -> AppResult<()> {
        self.repository.load().await?;
        info!("Configuration loaded successfully");
        Ok(())
    }

    async fn save_config(&self) -> AppResult<()> {
        let config = self.repository.get_in_memory_config().await?;
        self.repository.save(&config).await
    }

    async fn validate_config(
        &self,
        config: &AppConfig,
    ) -> AppResult<ConfigurationValidationResult> {
        self.repository.validate_config(config).await
    }

    async fn get_file_info(&self) -> AppResult<ConfigurationFileInfo> {
        self.repository.get_file_info().await
    }

    async fn get_poe_client_log_path(&self) -> AppResult<String> {
        self.repository.get_poe_client_log_path().await
    }

    async fn get_log_level(&self) -> AppResult<String> {
        self.repository.get_log_level().await
    }

    async fn get_default_poe_client_log_path(&self) -> String {
        self.repository.get_default_poe_client_log_path().await
    }

    async fn reset_poe_client_log_path_to_default(&self) -> AppResult<()> {
        let default_path = AppConfig::get_default_poe_client_log_path();
        self.set_poe_client_log_path(default_path).await
    }

    async fn subscribe_to_config_changes(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(EventType::Configuration).await
    }

    async fn get_zone_refresh_interval(
        &self,
    ) -> AppResult<crate::domain::configuration::models::ZoneRefreshInterval> {
        let config = self.repository.get_in_memory_config().await?;
        Ok(config.zone_refresh_interval)
    }

    async fn set_zone_refresh_interval(
        &self,
        interval: crate::domain::configuration::models::ZoneRefreshInterval,
    ) -> AppResult<()> {
        let previous_config = self.get_config().await?;
        let mut new_config = previous_config.clone();

        new_config.zone_refresh_interval = interval;

        let validation_result = self.repository.validate_config(&new_config).await?;
        if !validation_result.is_valid {
            return Err(AppError::validation_error(
                "validate_config",
                &format!(
                    "Configuration validation failed: {}",
                    validation_result.errors.join(", ")
                ),
            ));
        }

        self.repository
            .update_in_memory_config(new_config.clone())
            .await?;
        self.repository.save(&new_config).await?;

        self.publish_config_change(new_config, previous_config)
            .await;

        debug!("Zone refresh interval updated to: {}", interval);
        Ok(())
    }
}

// NOTE: Default trait removed intentionally - ConfigurationServiceImpl::new() is async
// and must be called explicitly. Using block_on in Default would risk deadlocks.
// All initialization should go through ServiceRegistry or explicit new().await calls.
