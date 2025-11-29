use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata},
    traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use crate::infrastructure::persistence::{AppPaths, FileService};
use async_trait::async_trait;
use log::{debug, info, warn};
use serde_json;
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

    fn load_embedded_zones() -> AppResult<ZoneConfiguration> {
        let content = include_str!("../../../config/zones.json");

        #[derive(serde::Deserialize)]
        struct OldZoneConfig {
            acts: Vec<OldActDefinition>,
        }

        #[derive(serde::Deserialize)]
        struct OldActDefinition {
            #[allow(dead_code)]
            act_name: String,
            act_number: u32,
            zones: Vec<OldZoneMapping>,
        }

        #[derive(serde::Deserialize)]
        struct OldZoneMapping {
            zone_name: String,
            is_town: bool,
        }

        let old_config: OldZoneConfig = serde_json::from_str(content)?;
        let mut new_config = ZoneConfiguration::new();

        for act in old_config.acts {
            for zone in act.zones {
                let mut zone_metadata = ZoneMetadata::new(zone.zone_name);
                zone_metadata.act = act.act_number;
                zone_metadata.is_town = zone.is_town;
                new_config.add_zone(zone_metadata);
            }
        }

        info!(
            "Converted {} zones from embedded configuration",
            new_config.zones.len()
        );
        Ok(new_config)
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        if FileService::exists(&self.file_path) {
            debug!("Loading zone configuration from: {:?}", self.file_path);
            let config: ZoneConfiguration = FileService::read_json(&self.file_path).await?;
            info!(
                "Loaded {} zones from configuration file",
                config.zones.len()
            );
            Ok(config)
        } else {
            warn!("Zone configuration file not found, creating from embedded data");
            let config = Self::load_embedded_zones()?;

            if let Err(e) = self.save_configuration(&config).await {
                warn!("Failed to save converted zone configuration: {}", e);
            }

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
