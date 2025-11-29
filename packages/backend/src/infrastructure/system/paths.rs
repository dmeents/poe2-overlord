use crate::infrastructure::system::os_detection::detect_os;
use std::path::{Path, PathBuf};

/// Provides platform-specific paths for Path of Exile 2 client log files
///
/// Handles the different default installation locations across Windows, macOS, and Linux.
/// Supports both Steam and standalone installations where applicable.
pub struct PoeClientLogPaths;

impl PoeClientLogPaths {
    /// Expands a path that may contain a tilde (~) to represent the home directory
    ///
    /// This function handles the shell convention of using `~` to represent the user's
    /// home directory. Since Rust's standard library doesn't automatically expand `~`,
    /// this function provides that functionality.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that may start with `~` or `~/`
    ///
    /// # Returns
    ///
    /// A `PathBuf` with the tilde expanded to the actual home directory path.
    /// If the path doesn't start with `~`, it's returned as-is.
    /// If the home directory cannot be determined, the path is returned unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// let expanded = PoeClientLogPaths::expand_tilde("~/Documents/file.txt");
    /// // On Linux with user "alice", this would return "/home/alice/Documents/file.txt"
    /// ```
    pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        let path_str = path.to_string_lossy();

        if path_str.starts_with("~/") {
            // Replace ~ with home directory
            if let Some(home) = dirs::home_dir() {
                return home.join(&path_str[2..]);
            }
        } else if path_str == "~" {
            // Just ~ by itself
            if let Some(home) = dirs::home_dir() {
                return home;
            }
        }

        // Return the path as-is if it doesn't start with ~ or if home dir couldn't be determined
        path.to_path_buf()
    }

    /// Returns the default log file path for the specified operating system
    ///
    /// Provides platform-specific paths based on common POE2 installation locations:
    /// - Windows: Program Files (x86) installation
    /// - macOS: Application Support directory
    /// - Linux: Steam Flatpak installation path
    /// - Unknown: Fallback to current directory
    pub fn get_path_for_os(
        os: &crate::infrastructure::system::os_detection::OperatingSystem,
    ) -> PathBuf {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde_with_path() {
        let path = "~/test/file.txt";
        let expanded = PoeClientLogPaths::expand_tilde(path);

        // Should not contain literal tilde
        assert!(!expanded.to_string_lossy().starts_with("~"));

        // Should end with the same relative path
        assert!(expanded.to_string_lossy().ends_with("test/file.txt"));
    }

    #[test]
    fn test_expand_tilde_alone() {
        let path = "~";
        let expanded = PoeClientLogPaths::expand_tilde(path);

        // Should not be just "~"
        assert_ne!(expanded.to_string_lossy(), "~");
    }

    #[test]
    fn test_expand_tilde_no_slash() {
        let path = "~test/file.txt";
        let expanded = PoeClientLogPaths::expand_tilde(path);

        // Should remain unchanged (not a home directory reference)
        assert_eq!(expanded.to_string_lossy(), "~test/file.txt");
    }

    #[test]
    fn test_expand_tilde_absolute_path() {
        let path = "/absolute/path/file.txt";
        let expanded = PoeClientLogPaths::expand_tilde(path);

        // Should remain unchanged
        assert_eq!(expanded.to_string_lossy(), "/absolute/path/file.txt");
    }
}
