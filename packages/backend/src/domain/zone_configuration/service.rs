use crate::domain::zone_configuration::{
    models::ZoneConfiguration,
    traits::{ZoneConfigurationRepository, ZoneConfigurationService},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementation of ZoneConfigurationService with caching for performance
/// Uses a cached lookup map for fast O(1) zone-to-act resolution
pub struct ZoneConfigurationServiceImpl {
    repository: Arc<dyn ZoneConfigurationRepository>,
    zone_lookup: Arc<tokio::sync::RwLock<HashMap<String, (String, bool)>>>,
}

impl ZoneConfigurationServiceImpl {
    /// Creates a new zone configuration service with the provided repository
    pub fn new(repository: Arc<dyn ZoneConfigurationRepository>) -> Self {
        Self {
            repository,
            zone_lookup: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Updates the cached lookup map from the current configuration
    async fn update_lookup_cache(&self) -> AppResult<()> {
        let config = self.repository.load_configuration().await?;
        let lookup = config.create_zone_lookup();
        debug!("Loaded zone configuration with {} zones", lookup.len());
        let mut cache = self.zone_lookup.write().await;
        *cache = lookup;
        Ok(())
    }

    /// Ensures the lookup cache is populated
    async fn ensure_cache_loaded(&self) -> AppResult<()> {
        let cache = self.zone_lookup.read().await;
        if cache.is_empty() {
            drop(cache);
            self.update_lookup_cache().await?;
        }
        Ok(())
    }
}

#[async_trait]
impl ZoneConfigurationService for ZoneConfigurationServiceImpl {
    /// Gets the act name for a specific zone using cached lookup
    /// Returns "Endgame" as fallback for unknown zones
    async fn get_act_for_zone(&self, zone_name: &str) -> Option<String> {
        if let Err(_) = self.ensure_cache_loaded().await {
            debug!(
                "Failed to load zone configuration cache, returning Endgame for zone: {}",
                zone_name
            );
            return Some("Endgame".to_string());
        }

        let cache = self.zone_lookup.read().await;
        let result = cache
            .get(zone_name)
            .map(|(act, _)| act.clone())
            .or_else(|| Some("Endgame".to_string()));

        debug!(
            "Zone lookup for '{}': {:?} (cache size: {})",
            zone_name,
            result,
            cache.len()
        );
        result
    }

    /// Checks if a zone is a town using cached lookup
    /// Returns false for unknown zones (they are not explicitly marked as towns)
    async fn is_town_zone(&self, zone_name: &str) -> bool {
        if let Err(_) = self.ensure_cache_loaded().await {
            return false;
        }

        let cache = self.zone_lookup.read().await;
        cache
            .get(zone_name)
            .map(|(_, is_town)| *is_town)
            .unwrap_or(false)
    }

    /// Gets all zones for a specific act by loading the full configuration
    async fn get_act_zones(
        &self,
        act_name: &str,
    ) -> Option<Vec<crate::domain::zone_configuration::models::ZoneMapping>> {
        let config = self.repository.load_configuration().await.ok()?;
        config.get_act_zones(act_name)
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
