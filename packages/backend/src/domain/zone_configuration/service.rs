use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata},
    traits::{ZoneConfigurationRepository, ZoneConfigurationService},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Implementation of ZoneConfigurationService with caching for performance
/// Uses a cached lookup map for fast O(1) zone-to-act resolution
#[allow(clippy::type_complexity)]
pub struct ZoneConfigurationServiceImpl {
    repository: Arc<dyn ZoneConfigurationRepository>,
    zone_lookup: Arc<RwLock<Option<HashMap<String, (u32, bool)>>>>,
    zone_config: Arc<RwLock<Option<ZoneConfiguration>>>,
}

impl ZoneConfigurationServiceImpl {
    /// Creates a new zone configuration service with the provided repository
    pub fn new(repository: Arc<dyn ZoneConfigurationRepository>) -> Self {
        Self {
            repository,
            zone_lookup: Arc::new(RwLock::new(None)),
            zone_config: Arc::new(RwLock::new(None)),
        }
    }

    /// Ensures the lookup cache is populated
    async fn ensure_cache_loaded(&self) -> AppResult<()> {
        // Check if cache is already loaded
        {
            let config_guard = self.zone_config.read().await;
            if config_guard.is_some() {
                return Ok(());
            }
        }

        // Cache not loaded, acquire write lock and load
        let mut config_guard = self.zone_config.write().await;
        let mut lookup_guard = self.zone_lookup.write().await;

        // Double-check in case another thread loaded while we were waiting
        if config_guard.is_some() {
            return Ok(());
        }

        debug!("Zone configuration not cached, loading from repository...");
        let config = self.repository.load_configuration().await?;

        debug!("Zone lookup cache not initialized, building lookup map...");
        let mut lookup = HashMap::new();

        for (zone_name, zone_metadata) in &config.zones {
            lookup.insert(
                zone_name.clone(),
                (zone_metadata.act, zone_metadata.is_town),
            );
        }

        debug!("Built zone lookup cache with {} zones", lookup.len());

        *config_guard = Some(config);
        *lookup_guard = Some(lookup);

        Ok(())
    }

    /// Updates the cached lookup map from the current configuration
    async fn update_lookup_cache(&self) -> AppResult<()> {
        // Reload the configuration and rebuild the cache
        debug!("Reloading zone configuration from repository...");
        let config = self.repository.load_configuration().await?;
        let mut lookup = HashMap::new();

        for (zone_name, zone_metadata) in &config.zones {
            lookup.insert(
                zone_name.clone(),
                (zone_metadata.act, zone_metadata.is_town),
            );
        }

        debug!("Updating cache with {} zones", lookup.len());

        // Update both caches with new data
        let mut config_guard = self.zone_config.write().await;
        let mut lookup_guard = self.zone_lookup.write().await;

        *config_guard = Some(config);
        *lookup_guard = Some(lookup);

        debug!("Cache successfully updated");
        Ok(())
    }
}

#[async_trait]
impl ZoneConfigurationService for ZoneConfigurationServiceImpl {
    /// Gets the act number for a specific zone by zone name using cached lookup
    /// Returns None for unknown zones
    async fn get_act_for_zone(&self, zone_name: &str) -> Option<u32> {
        if self.ensure_cache_loaded().await.is_err() {
            debug!(
                "Failed to load zone configuration cache for zone_name: {}",
                zone_name
            );
            return None;
        }

        let cache = self.zone_lookup.read().await;
        let cache_ref = cache.as_ref()?;
        let result = cache_ref.get(zone_name).map(|(act, _)| *act);

        debug!(
            "Zone lookup for zone_name '{}': {:?} (cache size: {})",
            zone_name,
            result,
            cache_ref.len()
        );
        result
    }

    /// Checks if a zone is a town by zone name using cached lookup
    /// Returns false for unknown zones (they are not explicitly marked as towns)
    async fn is_town_zone(&self, zone_name: &str) -> bool {
        if self.ensure_cache_loaded().await.is_err() {
            return false;
        }

        let cache = self.zone_lookup.read().await;
        cache
            .as_ref()
            .and_then(|c| c.get(zone_name).map(|(_, is_town)| *is_town))
            .unwrap_or(false)
    }

    /// Gets zone metadata by zone name
    async fn get_zone_metadata(&self, zone_name: &str) -> Option<ZoneMetadata> {
        if self.ensure_cache_loaded().await.is_err() {
            debug!(
                "Failed to load zone configuration cache for zone: {}",
                zone_name
            );
            return None;
        }

        let config = self.zone_config.read().await;
        config.as_ref()?.get_zone_by_name(zone_name).cloned()
    }

    /// Gets all zones for a specific act
    async fn get_act_zones(&self, act: u32) -> Vec<ZoneMetadata> {
        if self.ensure_cache_loaded().await.is_err() {
            debug!("Failed to load zone configuration cache for act: {}", act);
            return Vec::new();
        }

        let config = self.zone_config.read().await;
        config
            .as_ref()
            .map(|c| c.get_act_zones(act).into_iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Adds or updates a zone
    async fn add_zone(&self, metadata: ZoneMetadata) -> AppResult<()> {
        let zone_name = metadata.zone_name.clone();
        let mut config = self.repository.load_configuration().await?;
        config.add_zone(metadata);
        self.repository.save_configuration(&config).await?;
        self.update_lookup_cache().await?;
        info!("Added zone: {}", zone_name);
        Ok(())
    }

    /// Updates an existing zone
    async fn update_zone(&self, metadata: ZoneMetadata) -> AppResult<()> {
        let zone_name = metadata.zone_name.clone();
        let mut config = self.repository.load_configuration().await?;
        config.add_zone(metadata); // add_zone handles both add and update
        self.repository.save_configuration(&config).await?;
        self.update_lookup_cache().await?;
        info!("Updated zone: {}", zone_name);
        Ok(())
    }

    /// Loads the current zone configuration
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        self.repository.load_configuration().await
    }

    /// Reloads the zone configuration and updates the cache
    async fn reload_configuration(&self) -> AppResult<()> {
        self.update_lookup_cache().await
    }
}
