use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayState {
    pub visible: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub always_on_top: bool,
}

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
