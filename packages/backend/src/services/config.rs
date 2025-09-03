use crate::errors::{AppError, AppResult};
use crate::models::AppConfig;
use log::{debug, error, info, warn};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use tauri::AppHandle;

/// Configuration constants
const CONFIG_DIR_NAME: &str = "poe2-overlord";
const CONFIG_FILE_NAME: &str = "config.json";
const TEMP_FILE_EXTENSION: &str = "tmp";

/// Configuration service that manages application settings
#[derive(Clone)]
pub struct ConfigService {
    pub config: Arc<RwLock<AppConfig>>,
    pub config_path: PathBuf,
}

impl ConfigService {
    /// Create a new configuration service
    pub fn new(_app_handle: &AppHandle) -> Self {
        // Use standard config directory for the current user
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(CONFIG_DIR_NAME);

        // Ensure config directory exists
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                warn!("Failed to create config directory: {}", e);
            }
        }

        let config_path = config_dir.join(CONFIG_FILE_NAME);

        let service = Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            config_path,
        };

        // Load existing configuration or create default
        if let Err(e) = service.load_config() {
            warn!("Failed to load config, using defaults: {}", e);
            // Try to save the default config to ensure we have a working file
            if let Err(save_err) = service.save_config() {
                error!("Failed to save default config: {}", save_err);
            }
        }

        service
    }

    /// Load configuration from file
    pub fn load_config(&self) -> AppResult<()> {
        if !self.config_path.exists() {
            info!("No config file found, creating default configuration");
            self.save_config()?;
            return Ok(());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig = serde_json::from_str(&content).map_err(|e| {
            error!("Failed to parse config file JSON: {}", e);
            error!("Config file content: {}", content);
            // If JSON parsing fails, create a new config file with defaults
            AppError::Serialization(format!("Failed to parse config file: {}", e))
        })?;

        {
            let mut current_config = self.config.write().unwrap();
            *current_config = config;
        }

        info!(
            "Configuration loaded successfully from {:?}",
            self.config_path
        );
        Ok(())
    }

    /// Save current configuration to file
    pub fn save_config(&self) -> AppResult<()> {
        let config = self.config.read().unwrap();
        let content = serde_json::to_string_pretty(&*config)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize config: {}", e)))?;

        // Write to a temporary file first, then rename to ensure atomic write
        let temp_path = self.config_path.with_extension(TEMP_FILE_EXTENSION);
        fs::write(&temp_path, content)
            .map_err(|e| AppError::FileSystem(format!("Failed to write temp file: {}", e)))?;

        fs::rename(&temp_path, &self.config_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to rename temp file: {}", e)))?;

        debug!("Configuration saved successfully to {:?}", self.config_path);
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }

    /// Update configuration
    pub fn update_config(&self, new_config: AppConfig) -> AppResult<()> {
        {
            let mut current_config = self.config.write().unwrap();
            *current_config = new_config;
        }

        self.save_config()?;
        Ok(())
    }

    /// Update specific configuration field
    pub fn update_field<F>(&self, updater: F) -> AppResult<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        {
            let mut config = self.config.write().unwrap();
            updater(&mut *config);
        }

        self.save_config()?;
        Ok(())
    }

    /// Get the POE client log path
    pub fn get_poe_client_log_path(&self) -> String {
        let config = self.config.read().unwrap();
        let path = &config.poe_client_log_path;

        // If the path is empty, return the OS-specific default
        if path.is_empty() {
            crate::utils::PoeClientLogPaths::get_default_path_string()
        } else {
            path.clone()
        }
    }

    /// Set the POE client log path
    pub fn set_poe_client_log_path(&self, path: String) -> AppResult<()> {
        self.update_field(|config| {
            config.poe_client_log_path = path;
        })
    }

    /// Get log level
    pub fn get_log_level(&self) -> String {
        self.config.read().unwrap().log_level.clone()
    }

    /// Set log level
    pub fn set_log_level(&self, level: String) -> AppResult<()> {
        self.update_field(|config| {
            config.log_level = level;
        })
    }

    /// Get the OS-specific default POE client log path
    pub fn get_default_poe_client_log_path(&self) -> String {
        crate::utils::PoeClientLogPaths::get_default_path_string()
    }

    /// Reset the POE client log path to the OS-specific default
    pub fn reset_poe_client_log_path_to_default(&self) -> AppResult<()> {
        let default_path = self.get_default_poe_client_log_path();
        self.set_poe_client_log_path(default_path)
    }
}
