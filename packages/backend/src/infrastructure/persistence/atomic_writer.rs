use crate::errors::{AppError, AppResult};
use log::{debug, error, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Provides atomic file writing operations to prevent data corruption
/// 
/// Implements the write-temp-rename pattern to ensure files are either
/// completely written or not modified at all, preventing partial writes
/// that could corrupt data during system crashes or interruptions.
pub struct AtomicWriter;

impl AtomicWriter {
    /// Writes content to a file atomically using a temporary file
    /// 
    /// Creates a temporary file, writes the content, then renames it to the final location.
    /// This ensures the original file is either completely replaced or left unchanged.
    pub fn write_atomic<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
        let path = path.as_ref();
        let temp_path = Self::get_temp_path(path);

        // Write to temporary file first
        fs::write(&temp_path, content).map_err(|e| {
            error!("Failed to write to temporary file {:?}: {}", temp_path, e);
            AppError::file_system_error(
                "write_temp_file",
                &format!("Failed to write to temporary file {:?}: {}", temp_path, e),
            )
        })?;

        // Atomically move temp file to final location
        fs::rename(&temp_path, path).map_err(|e| {
            error!("Failed to move temp file {:?} to final location {:?}: {}", temp_path, path, e);
            
            // Attempt cleanup of temp file on failure
            if let Err(cleanup_err) = fs::remove_file(&temp_path) {
                warn!("Failed to clean up temp file {:?}: {}", temp_path, cleanup_err);
            }
            
            AppError::file_system_error(
                "rename_temp_file",
                &format!("Failed to move temp file {:?} to final location {:?}: {}", temp_path, path, e),
            )
        })?;

        debug!("Successfully wrote file atomically: {:?}", path);
        Ok(())
    }

    /// Async version of atomic file writing
    /// 
    /// Performs the same atomic write operation but uses spawn_blocking to avoid
    /// blocking the async runtime during file I/O operations.
    pub async fn write_atomic_async<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
        let path = path.as_ref().to_path_buf();
        let content = content.to_string();
        let temp_path = Self::get_temp_path(&path);

        // Write to temporary file in blocking task
        tokio::task::spawn_blocking({
            let temp_path = temp_path.clone();
            let content = content.clone();
            move || fs::write(&temp_path, content)
        })
        .await
        .map_err(|e| {
            AppError::file_system_error(
                "spawn_write_task",
                &format!("Failed to spawn write task: {}", e),
            )
        })?
        .map_err(|e| {
            error!("Failed to write to temporary file {:?}: {}", temp_path, e);
            AppError::file_system_error(
                "write_temp_file",
                &format!("Failed to write to temporary file {:?}: {}", temp_path, e),
            )
        })?;

        // Atomically move temp file to final location in blocking task
        tokio::task::spawn_blocking({
            let temp_path = temp_path.clone();
            let path = path.clone();
            move || fs::rename(&temp_path, &path)
        })
        .await
        .map_err(|e| {
            AppError::file_system_error(
                "spawn_rename_task",
                &format!("Failed to spawn rename task: {}", e),
            )
        })?
        .map_err(|e| {
            error!("Failed to move temp file {:?} to final location {:?}: {}", temp_path, path, e);
            
            // Attempt cleanup of temp file on failure
            if let Err(cleanup_err) = fs::remove_file(&temp_path) {
                warn!("Failed to clean up temp file {:?}: {}", temp_path, cleanup_err);
            }
            
            AppError::file_system_error(
                "rename_temp_file",
                &format!("Failed to move temp file {:?} to final location {:?}: {}", temp_path, path, e),
            )
        })?;

        debug!("Successfully wrote file atomically: {:?}", path);
        Ok(())
    }

    /// Generates a temporary file path by adding .tmp extension
    fn get_temp_path<P: AsRef<Path>>(path: P) -> PathBuf {
        path.as_ref().with_extension("tmp")
    }

    /// Cleans up any leftover temporary files
    /// 
    /// Removes temporary files that may have been left behind from
    /// interrupted write operations or system crashes.
    pub fn cleanup_temp_files<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();
        let temp_path = Self::get_temp_path(path);

        if temp_path.exists() {
            fs::remove_file(&temp_path).map_err(|e| {
                warn!("Failed to clean up temp file {:?}: {}", temp_path, e);
                AppError::file_system_error(
                    "cleanup_temp_file",
                    &format!("Failed to clean up temp file {:?}: {}", temp_path, e),
                )
            })?;
            debug!("Cleaned up temp file: {:?}", temp_path);
        }

        Ok(())
    }
}
