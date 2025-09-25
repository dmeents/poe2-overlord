use crate::infrastructure::system::os_detection::detect_os;
use std::path::PathBuf;

/// Provides platform-specific paths for Path of Exile 2 client log files
/// 
/// Handles the different default installation locations across Windows, macOS, and Linux.
/// Supports both Steam and standalone installations where applicable.
pub struct PoeClientLogPaths;

impl PoeClientLogPaths {
    /// Returns the default log file path for the specified operating system
    /// 
    /// Provides platform-specific paths based on common POE2 installation locations:
    /// - Windows: Program Files (x86) installation
    /// - macOS: Application Support directory
    /// - Linux: Steam Flatpak installation path
    /// - Unknown: Fallback to current directory
    pub fn get_path_for_os(os: &crate::infrastructure::system::os_detection::OperatingSystem) -> PathBuf {
        match os {
            crate::infrastructure::system::os_detection::OperatingSystem::Windows => {
                // Standard Windows installation path
                PathBuf::from("C:\\Program Files (x86)")
                    .join("Grinding Gear Games")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::infrastructure::system::os_detection::OperatingSystem::MacOs => {
                // macOS Application Support directory
                let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/default".to_string());
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("Path of Exile 2")
                    .join("logs")
                    .join("Client.txt")
            }
            crate::infrastructure::system::os_detection::OperatingSystem::Linux => {
                // Linux Steam Flatpak installation path
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
                // Fallback for unknown OS
                PathBuf::from("Client.txt")
            }
        }
    }

    /// Returns the default log file path for the current operating system
    /// 
    /// Automatically detects the OS and returns the appropriate path.
    pub fn get_default_path() -> PathBuf {
        Self::get_path_for_os(&detect_os())
    }

    /// Returns the default log file path as a string
    /// 
    /// Convenience method that returns the path as a String for easier
    /// use in contexts that require string types.
    pub fn get_default_path_string() -> String {
        Self::get_default_path().to_string_lossy().to_string()
    }
}
