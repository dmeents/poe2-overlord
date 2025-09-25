use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main application configuration structure
/// 
/// This struct contains all user-configurable settings for the POE2 Overlord application.
/// It includes settings for log file paths, logging levels, and other application preferences.
/// 
/// # Serialization
/// 
/// The struct is serialized to/from JSON for persistent storage in the configuration file.
/// All fields support both serialization and deserialization for frontend communication.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// Path to the Path of Exile client log file
    /// 
    /// This path is used by the application to monitor POE client events and activities.
    /// Can be set to a custom path or use the system default location.
    pub poe_client_log_path: String,
    
    /// Application logging level
    /// 
    /// Controls the verbosity of application logs. Valid values are:
    /// "trace", "debug", "info", "warn", "warning", "error"
    pub log_level: String,
}

impl AppConfig {
    /// Create a new AppConfig instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new AppConfig instance with specific values
    /// 
    /// # Arguments
    /// 
    /// * `poe_client_log_path` - Path to the POE client log file
    /// * `log_level` - Logging level for the application
    pub fn with_values(poe_client_log_path: String, log_level: String) -> Self {
        Self {
            poe_client_log_path,
            log_level,
        }
    }

    /// Validate the current configuration
    /// 
    /// Performs comprehensive validation of all configuration fields including:
    /// - Log level must be one of the supported values
    /// - POE client log path must not be empty
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if validation passes
    /// * `Err(String)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        let valid_log_levels = ["trace", "debug", "info", "warn", "warning", "error"];
        if !valid_log_levels.contains(&self.log_level.to_lowercase().as_str()) {
            return Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                self.log_level,
                valid_log_levels.join(", ")
            ));
        }

        if self.poe_client_log_path.trim().is_empty() {
            return Err("POE client log path cannot be empty".to_string());
        }

        Ok(())
    }

    /// Get the system default POE client log file path
    /// 
    /// This method retrieves the platform-specific default location where
    /// Path of Exile typically stores its client log files.
    pub fn get_default_poe_client_log_path() -> String {
        crate::infrastructure::system::paths::PoeClientLogPaths::get_default_path_string()
    }

    /// Check if the current POE client log path is using the system default
    pub fn is_using_default_poe_path(&self) -> bool {
        self.poe_client_log_path == Self::get_default_poe_client_log_path()
    }

    /// Reset the POE client log path to the system default location
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

/// Event emitted when configuration changes occur
/// 
/// This event is broadcast throughout the application when any configuration
/// setting is modified, allowing other components to react to configuration changes.
/// 
/// # Usage
/// 
/// Components can subscribe to these events to update their behavior when
/// configuration settings change, such as updating log levels or file watchers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChangedEvent {
    /// The new configuration state after the change
    pub new_config: AppConfig,
    
    /// The previous configuration state before the change
    pub previous_config: AppConfig,
    
    /// UTC timestamp when the configuration change occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ConfigurationChangedEvent {
    /// Create a new configuration changed event with the current timestamp
    /// 
    /// # Arguments
    /// 
    /// * `new_config` - The new configuration state
    /// * `previous_config` - The previous configuration state
    pub fn new(new_config: AppConfig, previous_config: AppConfig) -> Self {
        Self {
            new_config,
            previous_config,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Result of configuration validation operations
/// 
/// Contains validation status and any error messages encountered during validation.
/// Used to provide detailed feedback about configuration validity to users and other components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValidationResult {
    /// Whether the configuration passed validation
    pub is_valid: bool,
    
    /// List of validation error messages (empty if validation passed)
    pub errors: Vec<String>,
}

impl ConfigurationValidationResult {
    /// Create a validation result indicating success
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    /// Create a validation result indicating failure with specific errors
    /// 
    /// # Arguments
    /// 
    /// * `errors` - List of validation error messages
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }

    /// Add a validation error to this result
    /// 
    /// Automatically sets `is_valid` to false and appends the error message.
    /// 
    /// # Arguments
    /// 
    /// * `error` - The error message to add
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

/// Information about the configuration file on disk
/// 
/// Provides metadata about the configuration file including existence,
/// size, and modification time. Used for monitoring file changes and
/// providing diagnostic information to users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationFileInfo {
    /// Path to the configuration file
    pub path: PathBuf,
    
    /// Whether the configuration file exists on disk
    pub exists: bool,
    
    /// Size of the configuration file in bytes (None if file doesn't exist or can't be read)
    pub size: Option<u64>,
    
    /// Last modification time of the configuration file (None if unavailable)
    pub last_modified: Option<std::time::SystemTime>,
}

impl ConfigurationFileInfo {
    /// Create a new ConfigurationFileInfo by inspecting the given file path
    /// 
    /// This method reads file metadata from the filesystem to populate all fields.
    /// If the file doesn't exist or metadata can't be read, appropriate None values
    /// are set for size and last_modified fields.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to the configuration file to inspect
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
