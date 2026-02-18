use crate::domain::configuration::models::{AppConfig, ConfigurationChangedEvent};
use crate::domain::configuration::traits::{ConfigurationRepository, ConfigurationService};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::events::{AppEvent, EventBus};
use async_trait::async_trait;
use log::{debug, info, warn};
use std::sync::Arc;

pub struct ConfigurationServiceImpl {
    repository: Arc<dyn ConfigurationRepository + Send + Sync>,
    event_bus: Arc<EventBus>,
}

impl ConfigurationServiceImpl {
    /// Create a new configuration service with dependency injection
    pub fn new(
        repository: Arc<dyn ConfigurationRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            repository,
            event_bus,
        }
    }


    /// Load configuration from repository (internal use)
    async fn load_config(&self) -> AppResult<()> {
        self.repository.load().await?;
        info!("Configuration loaded successfully");
        Ok(())
    }

    /// Save current configuration to repository (internal use)
    async fn save_config(&self) -> AppResult<()> {
        let config = self.repository.get_in_memory_config().await?;
        self.repository.save(&config).await
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

    async fn update_config(&self, mut new_config: AppConfig) -> AppResult<()> {
        // Normalize log level to lowercase
        new_config.log_level = AppConfig::normalize_log_level(&new_config.log_level)
            .map_err(|e| AppError::validation_error("normalize_log_level", &e))?;

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

        // save() handles in-memory update and schedules debounced disk write
        self.repository.save(&new_config).await?;

        self.publish_config_change(new_config, previous_config)
            .await;

        info!("Configuration updated successfully");
        Ok(())
    }

    async fn reset_to_defaults(&self) -> AppResult<()> {
        let default_config = AppConfig::default();
        self.update_config(default_config).await
    }

    async fn flush(&self) -> AppResult<()> {
        self.repository.flush().await
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

        // Normalize log level to lowercase
        new_config.log_level = AppConfig::normalize_log_level(&new_config.log_level)
            .map_err(|e| AppError::validation_error("normalize_log_level", &e))?;

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

        // save() handles in-memory update and schedules debounced disk write
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
