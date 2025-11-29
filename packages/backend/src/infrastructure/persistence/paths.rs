use crate::errors::{AppError, AppResult};
use log::debug;
use std::path::PathBuf;
use tokio::fs;

/// Provides XDG-compliant directory path helpers for application data
///
/// This struct provides standardized paths following the XDG Base Directory
/// specification for configuration, data, and cache directories.
pub struct AppPaths;

impl AppPaths {
    /// Application name used for directory paths
    const APP_NAME: &'static str = "poe2-overlord";

    /// Get the config directory path (~/.config/poe2-overlord)
    ///
    /// Returns the XDG config directory. Falls back to current directory if
    /// XDG directory cannot be determined.
    ///
    /// # Example
    /// ```ignore
    /// let config_dir = AppPaths::config_dir();
    /// let config_path = config_dir.join("config.json");
    /// ```
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    /// Get the data directory path (~/.local/share/poe2-overlord)
    ///
    /// Returns the XDG data directory. Falls back to current directory if
    /// XDG directory cannot be determined.
    ///
    /// # Example
    /// ```ignore
    /// let data_dir = AppPaths::data_dir();
    /// let character_path = data_dir.join("character_123.json");
    /// ```
    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    /// Get the cache directory path (~/.cache/poe2-overlord)
    ///
    /// Returns the XDG cache directory. Falls back to current directory if
    /// XDG directory cannot be determined.
    ///
    /// # Example
    /// ```ignore
    /// let cache_dir = AppPaths::cache_dir();
    /// let cached_data_path = cache_dir.join("temp.json");
    /// ```
    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    /// Ensure a directory exists, creating it if necessary
    ///
    /// Creates all necessary parent directories. Returns Ok(()) if the
    /// directory already exists. Returns an error if the path exists but
    /// is not a directory.
    ///
    /// # Example
    /// ```ignore
    /// AppPaths::ensure_dir("/path/to/directory").await?;
    /// ```
    pub async fn ensure_dir(path: impl AsRef<std::path::Path>) -> AppResult<()> {
        let path = path.as_ref();

        if path.exists() {
            if path.is_dir() {
                debug!("Directory already exists: {:?}", path);
                return Ok(());
            } else {
                return Err(AppError::file_system_error(
                    "ensure_dir",
                    &format!("Path exists but is not a directory: {:?}", path),
                ));
            }
        }

        fs::create_dir_all(path).await.map_err(|e| {
            AppError::file_system_error(
                "ensure_dir",
                &format!("Failed to create directory {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully created directory: {:?}", path);
        Ok(())
    }

    /// Get the config directory and ensure it exists
    ///
    /// Combines `config_dir()` and `ensure_dir()` for convenience.
    ///
    /// # Example
    /// ```ignore
    /// let config_dir = AppPaths::ensure_config_dir().await?;
    /// let config_path = config_dir.join("config.json");
    /// ```
    pub async fn ensure_config_dir() -> AppResult<PathBuf> {
        let config_dir = Self::config_dir();
        Self::ensure_dir(&config_dir).await?;
        Ok(config_dir)
    }

    /// Get the data directory and ensure it exists
    ///
    /// Combines `data_dir()` and `ensure_dir()` for convenience.
    ///
    /// # Example
    /// ```ignore
    /// let data_dir = AppPaths::ensure_data_dir().await?;
    /// let character_path = data_dir.join("character_123.json");
    /// ```
    pub async fn ensure_data_dir() -> AppResult<PathBuf> {
        let data_dir = Self::data_dir();
        Self::ensure_dir(&data_dir).await?;
        Ok(data_dir)
    }

    /// Get the cache directory and ensure it exists
    ///
    /// Combines `cache_dir()` and `ensure_dir()` for convenience.
    ///
    /// # Example
    /// ```ignore
    /// let cache_dir = AppPaths::ensure_cache_dir().await?;
    /// let temp_path = cache_dir.join("temp.json");
    /// ```
    pub async fn ensure_cache_dir() -> AppResult<PathBuf> {
        let cache_dir = Self::cache_dir();
        Self::ensure_dir(&cache_dir).await?;
        Ok(cache_dir)
    }
}
