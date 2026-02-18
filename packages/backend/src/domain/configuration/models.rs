use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZoneRefreshInterval {
    FiveMinutes,
    OneHour,
    TwelveHours,
    TwentyFourHours,
    ThreeDays,
    #[default]
    SevenDays,
}

impl ZoneRefreshInterval {
    pub fn to_seconds(&self) -> i64 {
        match self {
            ZoneRefreshInterval::FiveMinutes => 5 * 60,
            ZoneRefreshInterval::OneHour => 60 * 60,
            ZoneRefreshInterval::TwelveHours => 12 * 60 * 60,
            ZoneRefreshInterval::TwentyFourHours => 24 * 60 * 60,
            ZoneRefreshInterval::ThreeDays => 3 * 24 * 60 * 60,
            ZoneRefreshInterval::SevenDays => 7 * 24 * 60 * 60,
        }
    }

    pub fn all_options() -> Vec<ZoneRefreshInterval> {
        vec![
            ZoneRefreshInterval::FiveMinutes,
            ZoneRefreshInterval::OneHour,
            ZoneRefreshInterval::TwelveHours,
            ZoneRefreshInterval::TwentyFourHours,
            ZoneRefreshInterval::ThreeDays,
            ZoneRefreshInterval::SevenDays,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            ZoneRefreshInterval::FiveMinutes => "5 Minutes",
            ZoneRefreshInterval::OneHour => "1 Hour",
            ZoneRefreshInterval::TwelveHours => "12 Hours",
            ZoneRefreshInterval::TwentyFourHours => "24 Hours",
            ZoneRefreshInterval::ThreeDays => "3 Days",
            ZoneRefreshInterval::SevenDays => "7 Days",
        }
    }
}

impl std::fmt::Display for ZoneRefreshInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRefreshIntervalOption {
    pub value: String,
    pub label: String,
    pub seconds: i64,
}

/// Default config version for new configs
fn default_config_version() -> u32 {
    AppConfig::CURRENT_VERSION
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// Configuration schema version for migration compatibility
    #[serde(default = "default_config_version")]
    pub config_version: u32,
    pub poe_client_log_path: String,
    pub log_level: String,
    pub zone_refresh_interval: ZoneRefreshInterval,
}

impl AppConfig {
    /// Current configuration schema version
    /// Increment this when making breaking changes that require migration
    pub const CURRENT_VERSION: u32 = 1;

    /// Valid log levels for configuration
    pub const VALID_LOG_LEVELS: &'static [&'static str] =
        &["trace", "debug", "info", "warn", "error"];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(poe_client_log_path: String, log_level: String) -> Self {
        Self {
            config_version: Self::CURRENT_VERSION,
            poe_client_log_path,
            log_level,
            zone_refresh_interval: ZoneRefreshInterval::default(),
        }
    }

    /// Check if a log level string is valid (case-insensitive)
    pub fn is_valid_log_level(level: &str) -> bool {
        Self::VALID_LOG_LEVELS.contains(&level.to_lowercase().as_str())
    }

    /// Check if config needs migration to current version
    pub fn needs_migration(&self) -> bool {
        self.config_version < Self::CURRENT_VERSION
    }

    /// Normalize and validate log level (converts to lowercase)
    pub fn normalize_log_level(level: &str) -> Result<String, String> {
        let normalized = level.to_lowercase();
        if Self::is_valid_log_level(&normalized) {
            Ok(normalized)
        } else {
            Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                level,
                Self::VALID_LOG_LEVELS.join(", ")
            ))
        }
    }

    /// Validate configuration including security checks for paths
    pub fn validate(&self) -> Result<(), String> {
        if !Self::is_valid_log_level(&self.log_level) {
            return Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                self.log_level,
                Self::VALID_LOG_LEVELS.join(", ")
            ));
        }

        if self.poe_client_log_path.trim().is_empty() {
            return Err("POE client log path cannot be empty".to_string());
        }

        // Security validation for POE path
        use crate::infrastructure::PathValidator;
        let validator = PathValidator::new_for_poe_logs();
        validator
            .validate_path(&self.poe_client_log_path)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Basic validation without security checks (for internal use during migration)
    pub fn validate_basic(&self) -> Result<(), String> {
        if !Self::is_valid_log_level(&self.log_level) {
            return Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                self.log_level,
                Self::VALID_LOG_LEVELS.join(", ")
            ));
        }

        if self.poe_client_log_path.trim().is_empty() {
            return Err("POE client log path cannot be empty".to_string());
        }

        Ok(())
    }

    pub fn get_default_poe_client_log_path() -> String {
        use std::env;

        match env::consts::OS {
            "windows" => {
                "C:\\Program Files (x86)\\Grinding Gear Games\\Path of Exile 2\\logs\\Client.txt"
                    .to_string()
            }
            "macos" => {
                let home = env::var("HOME").unwrap_or_else(|_| "/Users/default".to_string());
                format!(
                    "{}/Library/Application Support/Path of Exile 2/logs/Client.txt",
                    home
                )
            }
            "linux" => {
                let home = env::var("HOME").unwrap_or_else(|_| "/home/default".to_string());
                format!(
                    "{}/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Path of Exile 2/logs/Client.txt",
                    home
                )
            }
            _ => "Client.txt".to_string(),
        }
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
            config_version: Self::CURRENT_VERSION,
            poe_client_log_path: Self::get_default_poe_client_log_path(),
            log_level: "info".to_string(),
            zone_refresh_interval: ZoneRefreshInterval::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChangedEvent {
    pub new_config: AppConfig,
    pub previous_config: AppConfig,
    pub timestamp: String,
}

impl ConfigurationChangedEvent {
    pub fn new(new_config: AppConfig, previous_config: AppConfig) -> Self {
        Self {
            new_config,
            previous_config,
            timestamp: chrono::Utc::now().to_rfc3339(),
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
