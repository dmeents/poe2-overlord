use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata}, traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use crate::infrastructure::persistence::DirectoryManager;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use serde_json;
use std::path::PathBuf;

/// Implementation of ZoneConfigurationRepository for file-based data
/// Loads and saves zone configuration from/to the data directory
pub struct ZoneConfigurationRepositoryImpl {
    file_path: PathBuf,
}

impl ZoneConfigurationRepositoryImpl {
    /// Creates a new repository with file-based zone configuration
    pub fn new() -> AppResult<Self> {
        let data_dir = DirectoryManager::ensure_data_directory()?;
        let file_path = data_dir.join("zones.json");
        
        debug!("Zone configuration will be stored at: {:?}", file_path);
        
        Ok(Self { file_path })
    }

    /// Loads embedded zones.json as fallback seed data
    fn load_embedded_zones() -> AppResult<ZoneConfiguration> {
        let content = include_str!("../../../config/zones.json");
        
        // Parse the old format and convert to new format
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
        
        // Convert old format to new format
        for act in old_config.acts {
            for zone in act.zones {
                let mut zone_metadata = ZoneMetadata::new(zone.zone_name);
                zone_metadata.act = act.act_number;
                zone_metadata.is_town = zone.is_town;
                new_config.add_zone(zone_metadata);
            }
        }
        
        info!("Converted {} zones from embedded configuration", new_config.zones.len());
        Ok(new_config)
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    /// Loads zone configuration from file or creates from embedded data
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        if self.file_path.exists() {
            debug!("Loading zone configuration from: {:?}", self.file_path);
            let content = tokio::fs::read_to_string(&self.file_path).await.map_err(|e| {
                error!("Failed to read zone configuration file: {}", e);
                crate::errors::AppError::file_system_error("read_zones_file", &e.to_string())
            })?;
            
            let config: ZoneConfiguration = serde_json::from_str(&content).map_err(|e| {
                error!("Failed to parse zone configuration: {}", e);
                crate::errors::AppError::serialization_error("parse_zones_config", &e.to_string())
            })?;
            
            info!("Loaded {} zones from configuration file", config.zones.len());
            Ok(config)
        } else {
            warn!("Zone configuration file not found, creating from embedded data");
            let config = Self::load_embedded_zones()?;
            
            // Save the converted configuration to file
            if let Err(e) = self.save_configuration(&config).await {
                warn!("Failed to save converted zone configuration: {}", e);
            }
            
            Ok(config)
        }
    }

    /// Saves zone configuration to file
    async fn save_configuration(&self, config: &ZoneConfiguration) -> AppResult<()> {
        debug!("Saving zone configuration to: {:?}", self.file_path);
        
        // Ensure directory exists
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                error!("Failed to create zones directory: {}", e);
                crate::errors::AppError::file_system_error("create_zones_dir", &e.to_string())
            })?;
        }
        
        // Write to temporary file first, then atomically rename
        let temp_path = self.file_path.with_extension("tmp");
        let content = serde_json::to_string_pretty(config).map_err(|e| {
            error!("Failed to serialize zone configuration: {}", e);
            crate::errors::AppError::serialization_error("serialize_zones_config", &e.to_string())
        })?;
        
        tokio::fs::write(&temp_path, content).await.map_err(|e| {
            error!("Failed to write zone configuration: {}", e);
            crate::errors::AppError::file_system_error("write_zones_file", &e.to_string())
        })?;
        
        tokio::fs::rename(&temp_path, &self.file_path).await.map_err(|e| {
            error!("Failed to rename zone configuration file: {}", e);
            crate::errors::AppError::file_system_error("rename_zones_file", &e.to_string())
        })?;
        
        info!("Saved {} zones to configuration file", config.zones.len());
        Ok(())
    }

    /// Gets the path to the configuration file
    async fn get_configuration_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
