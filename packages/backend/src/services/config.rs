use crate::models::AppConfig;
use log::{info, warn};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;

/// Configuration service that manages application settings
pub struct ConfigService {
    pub config: Mutex<AppConfig>,
    pub config_path: PathBuf,
}

impl ConfigService {
    /// Create a new configuration service
    pub fn new(_app_handle: &AppHandle) -> Self {
        // Use standard config directory for the current user
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord");

        // Ensure config directory exists
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                warn!("Failed to create config directory: {}", e);
            }
        }

        let config_path = config_dir.join("config.json");

        let service = Self {
            config: Mutex::new(AppConfig::default()),
            config_path,
        };

        // Load existing configuration or create default
        if let Err(e) = service.load_config() {
            warn!("Failed to load config, using defaults: {}", e);
        }

        service
    }

    /// Load configuration from file
    pub fn load_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            info!("No config file found, creating default configuration");
            self.save_config()?;
            return Ok(());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: AppConfig = serde_json::from_str(&content)?;

        {
            let mut current_config = self.config.lock().unwrap();
            *current_config = config;
        }

        info!(
            "Configuration loaded successfully from {:?}",
            self.config_path
        );
        Ok(())
    }

    /// Save current configuration to file
    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.lock().unwrap();
        let content = serde_json::to_string_pretty(&*config)?;
        fs::write(&self.config_path, content)?;

        info!("Configuration saved successfully to {:?}", self.config_path);
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    /// Update configuration
    pub fn update_config(&self, new_config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut current_config = self.config.lock().unwrap();
            *current_config = new_config;
        }

        self.save_config()?;
        Ok(())
    }

    /// Update specific configuration field
    pub fn update_field<F>(&self, updater: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut AppConfig),
    {
        {
            let mut config = self.config.lock().unwrap();
            updater(&mut *config);
        }

        self.save_config()?;
        Ok(())
    }

    /// Get the POE client log path
    pub fn get_poe_client_log_path(&self) -> String {
        let config = self.config.lock().unwrap();
        let path = &config.poe_client_log_path;
        
        // If the path is empty, return the OS-specific default
        if path.is_empty() {
            crate::utils::PoeClientLogPaths::get_default_path_string()
        } else {
            path.clone()
        }
    }

    /// Set the POE client log path
    pub fn set_poe_client_log_path(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        self.update_field(|config| {
            config.poe_client_log_path = path;
        })
    }

    /// Get log level
    pub fn get_log_level(&self) -> String {
        self.config.lock().unwrap().log_level.clone()
    }

    /// Set log level
    pub fn set_log_level(&self, level: String) -> Result<(), Box<dyn std::error::Error>> {
        self.update_field(|config| {
            config.log_level = level;
        })
    }

    /// Get the OS-specific default POE client log path
    pub fn get_default_poe_client_log_path(&self) -> String {
        crate::utils::PoeClientLogPaths::get_default_path_string()
    }

    /// Reset the POE client log path to the OS-specific default
    pub fn reset_poe_client_log_path_to_default(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default_path = self.get_default_poe_client_log_path();
        self.set_poe_client_log_path(default_path)
    }
}
