use crate::domain::zone_configuration::models::{ZoneConfiguration, ZoneMetadata};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Service trait for zone configuration functionality
/// Provides methods for querying zone-to-act mappings and town status
#[async_trait]
pub trait ZoneConfigurationService: Send + Sync {
    /// Gets the act number for a specific zone
    /// Returns None if the zone is not found in the configuration
    async fn get_act_for_zone(&self, zone_name: &str) -> Option<u32>;

    /// Checks if a zone is a town
    /// Returns false if the zone is not found in the configuration
    async fn is_town_zone(&self, zone_name: &str) -> bool;

    /// Gets zone metadata by name
    async fn get_zone_metadata(&self, zone_name: &str) -> Option<ZoneMetadata>;

    /// Gets all zones for a specific act
    async fn get_act_zones(&self, act: u32) -> Vec<ZoneMetadata>;

    /// Adds or updates a zone
    async fn add_zone(&self, metadata: ZoneMetadata) -> AppResult<()>;

    /// Updates an existing zone
    async fn update_zone(&self, metadata: ZoneMetadata) -> AppResult<()>;

    /// Loads the current zone configuration
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration>;

    /// Reloads the zone configuration from the source
    async fn reload_configuration(&self) -> AppResult<()>;
}

/// Repository trait for zone configuration persistence
/// Handles loading and saving zone configuration data
#[async_trait]
pub trait ZoneConfigurationRepository: Send + Sync {
    /// Loads zone configuration from the persistent storage
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration>;

    /// Saves zone configuration to persistent storage
    async fn save_configuration(&self, config: &ZoneConfiguration) -> AppResult<()>;

    /// Inserts or updates a single zone (UPSERT operation)
    /// Preserves first_discovered on conflict, updates all other fields
    async fn upsert_zone(&self, metadata: &ZoneMetadata) -> AppResult<()>;
}
