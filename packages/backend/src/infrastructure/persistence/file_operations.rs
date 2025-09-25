use crate::errors::{AppError, AppResult};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

/// Generic file operations utilities
pub struct FileOperations;

impl FileOperations {
    /// Check if a file exists
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }

    /// Get file size in bytes
    pub fn get_file_size<P: AsRef<Path>>(path: P) -> AppResult<u64> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(AppError::file_system_error(
                "get_file_size",
                &format!("File not found: {:?}", path),
            ));
        }

        let metadata = fs::metadata(path).map_err(|e| {
            AppError::file_system_error(
                "get_file_metadata",
                &format!("Failed to get metadata for {:?}: {}", path, e),
            )
        })?;

        Ok(metadata.len())
    }

    /// Read file content as string
    pub fn read_file_content<P: AsRef<Path>>(path: P) -> AppResult<String> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(AppError::file_system_error(
                "read_file_content",
                &format!("File not found: {:?}", path),
            ));
        }

        fs::read_to_string(path).map_err(|e| {
            AppError::file_system_error(
                "read_file",
                &format!("Failed to read file {:?}: {}", path, e),
            )
        })
    }

    /// Write content to file
    pub fn write_file_content<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
        let path = path.as_ref();

        fs::write(path, content).map_err(|e| {
            AppError::file_system_error(
                "write_file",
                &format!("Failed to write file {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully wrote file: {:?}", path);
        Ok(())
    }

    /// Delete a file
    pub fn delete_file<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();

        if !path.exists() {
            debug!("File does not exist, nothing to delete: {:?}", path);
            return Ok(());
        }

        fs::remove_file(path).map_err(|e| {
            AppError::file_system_error(
                "delete_file",
                &format!("Failed to delete file {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully deleted file: {:?}", path);
        Ok(())
    }

    /// Get file metadata
    pub fn get_file_metadata<P: AsRef<Path>>(path: P) -> AppResult<std::fs::Metadata> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(AppError::file_system_error(
                "get_file_metadata",
                &format!("File not found: {:?}", path),
            ));
        }

        fs::metadata(path).map_err(|e| {
            AppError::file_system_error(
                "get_file_metadata",
                &format!("Failed to get metadata for {:?}: {}", path, e),
            )
        })
    }

    /// Check if path is a file
    pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().is_file()
    }

    /// Check if path is a directory
    pub fn is_directory<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().is_dir()
    }

    /// Get file extension
    pub fn get_file_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get file stem (filename without extension)
    pub fn get_file_stem<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }

    /// Get parent directory
    pub fn get_parent_directory<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
        path.as_ref().parent().map(|p| p.to_path_buf())
    }
}
