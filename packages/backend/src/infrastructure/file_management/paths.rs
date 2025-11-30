use crate::errors::{AppError, AppResult};
use log::debug;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Expands a path that may contain a tilde (~) to represent the home directory
///
/// This function handles the shell convention of using `~` to represent the user's
/// home directory. Since Rust's standard library doesn't automatically expand `~`,
/// this function provides that functionality.
///
/// # Arguments
///
/// * `path` - A path that may start with `~` or `~/`
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
/// let expanded = expand_tilde("~/Documents/file.txt");
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

pub struct AppPaths;

impl AppPaths {
    const APP_NAME: &'static str = "poe2-overlord";

    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub async fn ensure_dir(path: impl AsRef<std::path::Path>) -> AppResult<()> {
        let path = path.as_ref();

        match fs::metadata(path).await {
            Ok(metadata) if metadata.is_dir() => {
                debug!("Directory already exists: {:?}", path);
                Ok(())
            }
            Ok(_) => Err(AppError::file_system_error(
                "ensure_dir",
                &format!("Path exists but is not a directory: {:?}", path),
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir_all(path).await.map_err(|e| {
                    AppError::file_system_error(
                        "ensure_dir",
                        &format!("Failed to create directory {:?}: {}", path, e),
                    )
                })?;
                debug!("Created directory: {:?}", path);
                Ok(())
            }
            Err(e) => Err(AppError::file_system_error(
                "ensure_dir",
                &format!("Failed to check directory {:?}: {}", path, e),
            )),
        }
    }

    pub async fn ensure_config_dir() -> AppResult<PathBuf> {
        let config_dir = Self::config_dir();
        Self::ensure_dir(&config_dir).await?;
        Ok(config_dir)
    }

    pub async fn ensure_data_dir() -> AppResult<PathBuf> {
        let data_dir = Self::data_dir();
        Self::ensure_dir(&data_dir).await?;
        Ok(data_dir)
    }

    pub async fn ensure_cache_dir() -> AppResult<PathBuf> {
        let cache_dir = Self::cache_dir();
        Self::ensure_dir(&cache_dir).await?;
        Ok(cache_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde_with_path() {
        let path = "~/test/file.txt";
        let expanded = expand_tilde(path);

        // Should not contain literal tilde
        assert!(!expanded.to_string_lossy().starts_with("~"));

        // Should end with the same relative path
        assert!(expanded.to_string_lossy().ends_with("test/file.txt"));
    }

    #[test]
    fn test_expand_tilde_alone() {
        let path = "~";
        let expanded = expand_tilde(path);

        // Should not be just "~"
        assert_ne!(expanded.to_string_lossy(), "~");
    }

    #[test]
    fn test_expand_tilde_no_slash() {
        let path = "~test/file.txt";
        let expanded = expand_tilde(path);

        // Should remain unchanged (not a home directory reference)
        assert_eq!(expanded.to_string_lossy(), "~test/file.txt");
    }

    #[test]
    fn test_expand_tilde_absolute_path() {
        let path = "/absolute/path/file.txt";
        let expanded = expand_tilde(path);

        // Should remain unchanged
        assert_eq!(expanded.to_string_lossy(), "/absolute/path/file.txt");
    }
}
