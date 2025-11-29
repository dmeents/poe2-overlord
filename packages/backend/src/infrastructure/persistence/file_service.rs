use crate::errors::{AppError, AppResult};
use log::{debug, error, warn};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use tokio::fs;

/// Provides atomic JSON file I/O operations for data persistence
///
/// All write operations use atomic write-temp-rename pattern to prevent
/// data corruption during system crashes or interruptions.
pub struct FileService;

impl FileService {
    /// Read and deserialize JSON from a file
    ///
    /// Returns an error if the file doesn't exist or contains invalid JSON.
    ///
    /// # Example
    /// ```ignore
    /// let config: AppConfig = FileService::read_json("config.json").await?;
    /// ```
    pub async fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> AppResult<T> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(AppError::file_system_error(
                "read_json",
                &format!("File not found: {:?}", path),
            ));
        }

        let content = Self::read_file(path).await?;

        serde_json::from_str(&content).map_err(|e| {
            error!("Failed to deserialize JSON from {:?}: {}", path, e);
            AppError::internal_error(
                "json_deserialize",
                &format!("Failed to deserialize JSON from {:?}: {}", path, e),
            )
        })
    }

    /// Read and deserialize JSON from a file, returning None if file doesn't exist
    ///
    /// Returns an error only if the file exists but contains invalid JSON.
    ///
    /// # Example
    /// ```ignore
    /// let config: Option<AppConfig> = FileService::read_json_optional("config.json").await?;
    /// let config = config.unwrap_or_default();
    /// ```
    pub async fn read_json_optional<T: DeserializeOwned>(
        path: impl AsRef<Path>,
    ) -> AppResult<Option<T>> {
        let path = path.as_ref();

        if !path.exists() {
            debug!("File does not exist, returning None: {:?}", path);
            return Ok(None);
        }

        match Self::read_json(path).await {
            Ok(data) => Ok(Some(data)),
            Err(e) => {
                error!("Failed to load JSON from {:?}: {}", path, e);
                Err(e)
            }
        }
    }

    /// Serialize and atomically write JSON to a file
    ///
    /// Uses the write-temp-rename pattern to ensure atomic writes.
    /// The file is either completely written or left unchanged.
    ///
    /// # Example
    /// ```ignore
    /// FileService::write_json("config.json", &config).await?;
    /// ```
    pub async fn write_json<T: Serialize>(path: impl AsRef<Path>, data: &T) -> AppResult<()> {
        let path = path.as_ref();

        // Serialize to pretty-printed JSON
        let json_content = serde_json::to_string_pretty(data).map_err(|e| {
            error!("Failed to serialize data: {}", e);
            AppError::internal_error(
                "json_serialize",
                &format!("Failed to serialize data: {}", e),
            )
        })?;

        Self::write_atomic(path, &json_content).await
    }

    /// Delete a file if it exists
    ///
    /// Returns Ok(()) even if the file doesn't exist.
    ///
    /// # Example
    /// ```ignore
    /// FileService::delete("old_config.json").await?;
    /// ```
    pub async fn delete(path: impl AsRef<Path>) -> AppResult<()> {
        let path = path.as_ref();

        if !path.exists() {
            debug!("File does not exist, nothing to delete: {:?}", path);
            return Ok(());
        }

        fs::remove_file(path).await.map_err(|e| {
            error!("Failed to delete file {:?}: {}", path, e);
            AppError::file_system_error(
                "delete_file",
                &format!("Failed to delete file {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully deleted file: {:?}", path);
        Ok(())
    }

    /// Check if a file exists
    ///
    /// # Example
    /// ```ignore
    /// if FileService::exists("config.json") {
    ///     // File exists
    /// }
    /// ```
    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    /// Ensure the parent directory exists for a file path
    ///
    /// Creates all necessary parent directories.
    ///
    /// # Example
    /// ```ignore
    /// FileService::ensure_parent_dir("/path/to/config.json")?;
    /// // Now /path/to/ directory exists
    /// ```
    pub async fn ensure_parent_dir(path: impl AsRef<Path>) -> AppResult<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    error!("Failed to create parent directory {:?}: {}", parent, e);
                    AppError::file_system_error(
                        "ensure_parent_dir",
                        &format!("Failed to create parent directory {:?}: {}", parent, e),
                    )
                })?;
                debug!("Created parent directory: {:?}", parent);
            }
        }

        Ok(())
    }

    // ============================================================================
    // Private helper methods
    // ============================================================================

    /// Read file content as string
    async fn read_file(path: &Path) -> AppResult<String> {
        fs::read_to_string(path).await.map_err(|e| {
            error!("Failed to read file {:?}: {}", path, e);
            AppError::file_system_error(
                "read_file",
                &format!("Failed to read file {:?}: {}", path, e),
            )
        })
    }

    /// Write content to a file atomically using write-temp-rename pattern
    ///
    /// Creates a temporary file, writes the content, then renames it to the final location.
    /// This ensures the original file is either completely replaced or left unchanged.
    async fn write_atomic(path: &Path, content: &str) -> AppResult<()> {
        let temp_path = Self::get_temp_path(path);

        // Ensure parent directory exists
        Self::ensure_parent_dir(path).await?;

        // Write to temporary file first
        fs::write(&temp_path, content).await.map_err(|e| {
            error!("Failed to write to temporary file {:?}: {}", temp_path, e);
            AppError::file_system_error(
                "write_temp_file",
                &format!("Failed to write to temporary file {:?}: {}", temp_path, e),
            )
        })?;

        // Atomically move temp file to final location
        fs::rename(&temp_path, path).await.map_err(|e| {
            error!(
                "Failed to rename temp file {:?} to {:?}: {}",
                temp_path, path, e
            );

            // Attempt cleanup of temp file on failure
            if let Err(cleanup_err) = std::fs::remove_file(&temp_path) {
                warn!(
                    "Failed to clean up temp file {:?}: {}",
                    temp_path, cleanup_err
                );
            }

            AppError::file_system_error(
                "rename_temp_file",
                &format!(
                    "Failed to rename temp file {:?} to {:?}: {}",
                    temp_path, path, e
                ),
            )
        })?;

        debug!("Successfully wrote file atomically: {:?}", path);
        Ok(())
    }

    /// Generate a temporary file path by adding .tmp extension
    fn get_temp_path(path: &Path) -> std::path::PathBuf {
        let mut temp_path = path.to_path_buf();
        let mut extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        if !extension.is_empty() {
            extension.push_str(".tmp");
        } else {
            extension = "tmp".to_string();
        }

        temp_path.set_extension(extension);
        temp_path
    }
}
