use crate::domain::zone_configuration::{
    models::ZoneConfiguration, traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use crate::infrastructure::file_management::{AppPaths, FileService};
use async_trait::async_trait;
use log::{debug, info};
use std::path::PathBuf;

pub struct ZoneConfigurationRepositoryImpl {
    file_path: PathBuf,
}

impl ZoneConfigurationRepositoryImpl {
    pub async fn new() -> AppResult<Self> {
        let data_dir = AppPaths::ensure_data_dir().await?;
        let file_path = data_dir.join("zones.json");

        debug!("Zone configuration will be stored at: {:?}", file_path);

        Ok(Self { file_path })
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        if FileService::exists(&self.file_path).await? {
            debug!("Loading zone configuration from: {:?}", self.file_path);
            let config: ZoneConfiguration = FileService::read_json(&self.file_path).await?;
            info!(
                "Loaded {} zones from configuration file",
                config.zones.len()
            );
            Ok(config)
        } else {
            info!("Zone configuration file not found, creating new empty configuration");
            let config = ZoneConfiguration::new();
            Ok(config)
        }
    }

    async fn save_configuration(&self, config: &ZoneConfiguration) -> AppResult<()> {
        debug!("Saving zone configuration to: {:?}", self.file_path);
        FileService::write_json(&self.file_path, config).await?;
        info!("Saved {} zones to configuration file", config.zones.len());
        Ok(())
    }

    async fn get_configuration_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
