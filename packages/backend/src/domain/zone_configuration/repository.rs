use crate::domain::zone_configuration::{
    models::ZoneConfiguration, traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use std::path::PathBuf;

/// Implementation of ZoneConfigurationRepository for embedded data
/// Loads zone configuration from embedded JSON data
pub struct ZoneConfigurationRepositoryImpl;

impl ZoneConfigurationRepositoryImpl {
    /// Creates a new repository with embedded zone configuration
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    /// Loads zone configuration from embedded JSON data
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        let content = include_str!("../../../config/zones.json");
        let config: ZoneConfiguration = serde_json::from_str(content)?;
        Ok(config)
    }

    /// Saves zone configuration (not supported for embedded data)
    async fn save_configuration(&self, _config: &ZoneConfiguration) -> AppResult<()> {
        Err(crate::errors::AppError::internal_error(
            "save_configuration",
            "Zone configuration is embedded and cannot be modified at runtime",
        ))
    }

    /// Gets the path to the configuration file (not applicable for embedded data)
    async fn get_configuration_path(&self) -> PathBuf {
        PathBuf::from("embedded:zones.json")
    }
}
