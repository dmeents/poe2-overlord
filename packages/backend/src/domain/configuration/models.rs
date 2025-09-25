use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub poe_client_log_path: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(poe_client_log_path: String, log_level: String) -> Self {
        Self {
            poe_client_log_path,
            log_level,
        }
    }

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

    pub fn get_default_poe_client_log_path() -> String {
        crate::infrastructure::system::paths::PoeClientLogPaths::get_default_path_string()
    }

    pub fn is_using_default_poe_path(&self) -> bool {
        self.poe_client_log_path == Self::get_default_poe_client_log_path()
    }

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChangedEvent {
    pub new_config: AppConfig,
    pub previous_config: AppConfig,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ConfigurationChangedEvent {
    pub fn new(new_config: AppConfig, previous_config: AppConfig) -> Self {
        Self {
            new_config,
            previous_config,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl ConfigurationValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationFileInfo {
    pub path: PathBuf,
    pub exists: bool,
    pub size: Option<u64>,
    pub last_modified: Option<std::time::SystemTime>,
}

impl ConfigurationFileInfo {
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
