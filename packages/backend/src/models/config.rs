use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Path to the POE client.txt file
    pub poe_client_log_path: String,
    /// Log level for the application
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            poe_client_log_path: crate::utils::PoeClientLogPaths::get_default_path_string(),
            log_level: "info".to_string(),
        }
    }
}
