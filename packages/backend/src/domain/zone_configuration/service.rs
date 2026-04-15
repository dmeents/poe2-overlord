use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata},
    traits::{ZoneConfigurationRepository, ZoneConfigurationService},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::info;
use std::sync::Arc;

/// Implementation of `ZoneConfigurationService`.
/// Delegates all operations directly to the repository (`SQLite` with indexes).
pub struct ZoneConfigurationServiceImpl {
    repository: Arc<dyn ZoneConfigurationRepository>,
}

impl ZoneConfigurationServiceImpl {
    pub fn new(repository: Arc<dyn ZoneConfigurationRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ZoneConfigurationService for ZoneConfigurationServiceImpl {
    async fn get_act_for_zone(&self, zone_name: &str) -> Option<u32> {
        self.repository
            .get_act_and_town(zone_name)
            .await
            .ok()
            .flatten()
            .map(|(act, _)| act)
    }

    async fn is_town_zone(&self, zone_name: &str) -> bool {
        self.repository
            .get_act_and_town(zone_name)
            .await
            .ok()
            .flatten()
            .is_some_and(|(_, is_town)| is_town)
    }

    async fn get_zone_metadata(&self, zone_name: &str) -> Option<ZoneMetadata> {
        self.repository
            .get_zone_by_name(zone_name)
            .await
            .ok()
            .flatten()
    }

    async fn get_act_zones(&self, act: u32) -> Vec<ZoneMetadata> {
        self.repository
            .get_zones_by_act(act)
            .await
            .unwrap_or_default()
    }

    async fn add_zone(&self, metadata: ZoneMetadata) -> AppResult<()> {
        let zone_name = metadata.zone_name.clone();
        self.repository.upsert_zone(&metadata).await?;
        info!("Added zone: {zone_name}");
        Ok(())
    }

    async fn update_zone(&self, metadata: ZoneMetadata) -> AppResult<()> {
        let zone_name = metadata.zone_name.clone();
        self.repository.upsert_zone(&metadata).await?;
        info!("Updated zone: {zone_name}");
        Ok(())
    }

    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        self.repository.load_configuration().await
    }

    async fn reload_configuration(&self) -> AppResult<()> {
        // No-op: SQLite queries bypass any in-memory cache, always reflect current DB state.
        Ok(())
    }
}
