use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// Path to the POE client.txt file
    pub poe_client_log_path: String,
    /// Log level for the application
    pub log_level: String,
}

impl AppConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new configuration with custom values
    pub fn with_values(poe_client_log_path: String, log_level: String) -> Self {
        Self {
            poe_client_log_path,
            log_level,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate log level
        let valid_log_levels = ["trace", "debug", "info", "warn", "warning", "error"];
        if !valid_log_levels.contains(&self.log_level.to_lowercase().as_str()) {
            return Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                self.log_level,
                valid_log_levels.join(", ")
            ));
        }

        // Validate POE client log path (should not be empty)
        if self.poe_client_log_path.trim().is_empty() {
            return Err("POE client log path cannot be empty".to_string());
        }

        Ok(())
    }

    /// Get the OS-specific default POE client log path
    pub fn get_default_poe_client_log_path() -> String {
        crate::infrastructure::system::paths::PoeClientLogPaths::get_default_path_string()
    }

    /// Check if the current POE client log path is the default
    pub fn is_using_default_poe_path(&self) -> bool {
        self.poe_client_log_path == Self::get_default_poe_client_log_path()
    }

    /// Reset POE client log path to default
    pub fn reset_poe_path_to_default(&mut self) {
        self.poe_client_log_path = Self::get_default_poe_client_log_path();
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            poe_client_log_path: Self::get_default_poe_client_log_path(),
            log_level: "info".to_string(),
        }
    }
}

/// Configuration change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChangedEvent {
    /// The new configuration
    pub new_config: AppConfig,
    /// The previous configuration
    pub previous_config: AppConfig,
    /// Timestamp of the change
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ConfigurationChangedEvent {
    /// Create a new configuration change event
    pub fn new(new_config: AppConfig, previous_config: AppConfig) -> Self {
        Self {
            new_config,
            previous_config,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Configuration validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValidationResult {
    /// Whether the configuration is valid
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
}

impl ConfigurationValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    /// Create an invalid result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }

    /// Add an error to the result
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

/// Configuration file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationFileInfo {
    /// Path to the configuration file
    pub path: PathBuf,
    /// Whether the file exists
    pub exists: bool,
    /// File size in bytes (if exists)
    pub size: Option<u64>,
    /// Last modified time (if exists)
    pub last_modified: Option<std::time::SystemTime>,
}

impl ConfigurationFileInfo {
    /// Create file info for a given path
    pub fn new(path: PathBuf) -> Self {
        let exists = path.exists();
        let (size, last_modified) = if exists {
            if let Ok(metadata) = std::fs::metadata(&path) {
                (
                    Some(metadata.len()),
                    metadata.modified().ok(),
                )
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        Self {
            path,
            exists,
            size,
            last_modified,
        }
    }
}
