use crate::domain::zone_configuration::models::{ZoneConfiguration, ZoneMapping};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Service trait for zone configuration functionality
/// Provides methods for querying zone-to-act mappings and town status
#[async_trait]
pub trait ZoneConfigurationService: Send + Sync {
    /// Gets the act name for a specific zone
    /// Returns None if the zone is not found in the configuration
    async fn get_act_for_zone(&self, zone_name: &str) -> Option<String>;

    /// Checks if a zone is a town
    /// Returns false if the zone is not found in the configuration
    async fn is_town_zone(&self, zone_name: &str) -> bool;

    /// Gets all zones for a specific act
    /// Returns None if the act is not found in the configuration
    async fn get_act_zones(&self, act_name: &str) -> Option<Vec<ZoneMapping>>;

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

    /// Gets the path to the configuration file
    async fn get_configuration_path(&self) -> std::path::PathBuf;
}
