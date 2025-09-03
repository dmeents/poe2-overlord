use crate::utils::os_detection::detect_os;
use std::path::PathBuf;

/// Default POE client log paths for different operating systems
pub struct PoeClientLogPaths;

impl PoeClientLogPaths {
    /// Get the default POE client log path for a specific operating system
    pub fn get_path_for_os(os: &crate::utils::os_detection::OperatingSystem) -> PathBuf {
        match os {
            crate::utils::os_detection::OperatingSystem::Windows => {
                // Windows: C:\Program Files (x86)\Grinding Gear Games\Path of Exile 2\logs\Client.txt
                PathBuf::from("C:\\Program Files (x86)")
                    .join("Grinding Gear Games")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::utils::os_detection::OperatingSystem::MacOs => {
                // macOS: ~/Library/Application Support/Path of Exile 2/logs/Client.txt
                let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/default".to_string());
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::utils::os_detection::OperatingSystem::Linux => {
                // Linux: ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/Path of Exile 2/logs/Client.txt
                let home = std::env::var("HOME").unwrap_or_else(|_| "/home/default".to_string());
                PathBuf::from(home)
                    .join(".var")
                    .join("app")
                    .join("com.valvesoftware.Steam")
                    .join(".local")
                    .join("share")
                    .join("Steam")
                    .join("steamapps")
                    .join("common")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::utils::os_detection::OperatingSystem::Unknown => {
                // Fallback to a generic path
                PathBuf::from("Client.txt")
            }
        }
    }

    /// Get the default POE client log path for the current operating system
    pub fn get_default_path() -> PathBuf {
        Self::get_path_for_os(&detect_os())
    }

    /// Get the default POE client log path as a string for the current OS
    pub fn get_default_path_string() -> String {
        Self::get_default_path().to_string_lossy().to_string()
    }
}
