use crate::infrastructure::system::os_detection::detect_os;
use std::path::PathBuf;

pub struct PoeClientLogPaths;

impl PoeClientLogPaths {
    pub fn get_path_for_os(os: &crate::infrastructure::system::os_detection::OperatingSystem) -> PathBuf {
        match os {
            crate::infrastructure::system::os_detection::OperatingSystem::Windows => {
                PathBuf::from("C:\\Program Files (x86)")
                    .join("Grinding Gear Games")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::infrastructure::system::os_detection::OperatingSystem::MacOs => {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/default".to_string());
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::infrastructure::system::os_detection::OperatingSystem::Linux => {
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
            crate::infrastructure::system::os_detection::OperatingSystem::Unknown => {
                PathBuf::from("Client.txt")
            }
        }
    }

    pub fn get_default_path() -> PathBuf {
        Self::get_path_for_os(&detect_os())
    }

    pub fn get_default_path_string() -> String {
        Self::get_default_path().to_string_lossy().to_string()
    }
}
