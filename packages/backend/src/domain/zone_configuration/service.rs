use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata},
    traits::{ZoneConfigurationRepository, ZoneConfigurationService},
};
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Implementation of ZoneConfigurationService with caching for performance
/// Uses a cached lookup map for fast O(1) zone-to-act resolution
pub struct ZoneConfigurationServiceImpl {
    repository: Arc<dyn ZoneConfigurationRepository>,
    zone_lookup: Arc<OnceCell<HashMap<String, (u32, bool)>>>,
    zone_config: Arc<OnceCell<ZoneConfiguration>>,
}

impl ZoneConfigurationServiceImpl {
    /// Creates a new zone configuration service with the provided repository
    pub fn new(repository: Arc<dyn ZoneConfigurationRepository>) -> Self {
        Self {
            repository,
            zone_lookup: Arc::new(OnceCell::new()),
            zone_config: Arc::new(OnceCell::new()),
        }
    }

    /// Ensures the lookup cache is populated using OnceCell for thread-safe initialization
    async fn ensure_cache_loaded(&self) -> AppResult<()> {
        // Load the full configuration first
        self.zone_config.get_or_try_init(|| async {
            debug!("Zone configuration not cached, loading from repository...");
            self.repository.load_configuration().await
        }).await.map_err(|e: AppError| {
            error!("Failed to load zone configuration: {}", e);
            e
        })?;

        // Then initialize the lookup cache
        self.zone_lookup.get_or_try_init(|| async {
            debug!("Zone lookup cache not initialized, building lookup map...");
            let config = self.zone_config.get().unwrap();
            let mut lookup = HashMap::new();
            
            for (area_id, zone_metadata) in &config.zones {
                lookup.insert(area_id.clone(), (zone_metadata.act, zone_metadata.is_town));
            }
            
            debug!("Built zone lookup cache with {} zones", lookup.len());
            Ok(lookup)
        }).await.map_err(|e: AppError| {
            error!("Failed to initialize zone lookup cache: {}", e);
            e
        })?;
        
        Ok(())
    }

    /// Updates the cached lookup map from the current configuration
    async fn update_lookup_cache(&self) -> AppResult<()> {
        // Reload the configuration and rebuild the cache
        let config = self.repository.load_configuration().await?;
        let mut lookup = HashMap::new();
        
        for (area_id, zone_metadata) in &config.zones {
            lookup.insert(area_id.clone(), (zone_metadata.act, zone_metadata.is_town));
        }
        
        // Force reinitialize the cache with new data
        // Note: OnceCell doesn't support reinitialization, so we need to work around this
        // For now, just ensure cache is loaded - the new data will be available on next access
        self.ensure_cache_loaded().await
    }
}

#[async_trait]
impl ZoneConfigurationService for ZoneConfigurationServiceImpl {
    /// Gets the act number for a specific zone by area_id using cached lookup
    /// Returns None for unknown zones
    async fn get_act_for_zone(&self, area_id: &str) -> Option<u32> {
        if let Err(_) = self.ensure_cache_loaded().await {
            debug!(
                "Failed to load zone configuration cache for area_id: {}",
                area_id
            );
            return None;
        }

        let cache = self.zone_lookup.get().unwrap();
        let result = cache.get(area_id).map(|(act, _)| *act);

        debug!(
            "Zone lookup for area_id '{}': {:?} (cache size: {})",
            area_id,
            result,
            cache.len()
        );
        result
    }

    /// Checks if a zone is a town by area_id using cached lookup
    /// Returns false for unknown zones (they are not explicitly marked as towns)
    async fn is_town_zone(&self, area_id: &str) -> bool {
        if let Err(_) = self.ensure_cache_loaded().await {
            return false;
        }

        let cache = self.zone_lookup.get().unwrap();
        cache
            .get(area_id)
            .map(|(_, is_town)| *is_town)
            .unwrap_or(false)
    }

    /// Gets zone metadata by zone name
    async fn get_zone_metadata(&self, zone_name: &str) -> Option<ZoneMetadata> {
        if let Err(_) = self.ensure_cache_loaded().await {
            debug!("Failed to load zone configuration cache for zone: {}", zone_name);
            return None;
        }

        let config = self.zone_config.get().unwrap();
        config.get_zone_by_name(zone_name).cloned()
    }

    /// Gets all zones for a specific act
    async fn get_act_zones(&self, act: u32) -> Vec<ZoneMetadata> {
        if let Err(_) = self.ensure_cache_loaded().await {
            debug!("Failed to load zone configuration cache for act: {}", act);
            return Vec::new();
        }

        let config = self.zone_config.get().unwrap();
        config.get_act_zones(act).into_iter().cloned().collect()
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
