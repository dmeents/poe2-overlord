use crate::domain::zone_configuration::{
    models::ZoneConfiguration, traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use std::path::PathBuf;

/// Implementation of ZoneConfigurationRepository for file-based storage
/// Loads and saves zone configuration from JSON files
pub struct ZoneConfigurationRepositoryImpl {
    config_path: PathBuf,
}

impl ZoneConfigurationRepositoryImpl {
    /// Creates a new repository with the specified configuration file path
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    /// Loads zone configuration from the JSON file
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        let content = tokio::fs::read_to_string(&self.config_path).await?;
        let config: ZoneConfiguration = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Saves zone configuration to the JSON file
    async fn save_configuration(&self, config: &ZoneConfiguration) -> AppResult<()> {
        let content = serde_json::to_string_pretty(config)?;
        tokio::fs::write(&self.config_path, content).await?;
        Ok(())
    }

    /// Gets the path to the configuration file
    async fn get_configuration_path(&self) -> PathBuf {
        self.config_path.clone()
    }
}
