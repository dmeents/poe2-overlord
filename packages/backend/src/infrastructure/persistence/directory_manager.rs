use crate::errors::{AppError, AppResult};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

/// Directory management utilities
pub struct DirectoryManager;

impl DirectoryManager {
    /// Create directory and all parent directories if they don't exist
    pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();
        
        if path.exists() {
            if path.is_dir() {
                debug!("Directory already exists: {:?}", path);
                return Ok(());
            } else {
                return Err(AppError::file_system_error(
                    "ensure_directory_exists",
                    &format!("Path exists but is not a directory: {:?}", path),
                ));
            }
        }

        fs::create_dir_all(path).map_err(|e| {
            AppError::file_system_error(
                "create_directory",
                &format!("Failed to create directory {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully created directory: {:?}", path);
        Ok(())
    }

    /// Create directory if it doesn't exist, but don't create parent directories
    pub fn create_directory<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();
        
        if path.exists() {
            if path.is_dir() {
                debug!("Directory already exists: {:?}", path);
                return Ok(());
            } else {
                return Err(AppError::file_system_error(
                    "create_directory",
                    &format!("Path exists but is not a directory: {:?}", path),
                ));
            }
        }

        fs::create_dir(path).map_err(|e| {
            AppError::file_system_error(
                "create_directory",
                &format!("Failed to create directory {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully created directory: {:?}", path);
        Ok(())
    }

    /// Remove directory and all its contents
    pub fn remove_directory<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();
        
        if !path.exists() {
            debug!("Directory does not exist, nothing to remove: {:?}", path);
            return Ok(());
        }

        if !path.is_dir() {
            return Err(AppError::file_system_error(
                "remove_directory",
                &format!("Path is not a directory: {:?}", path),
            ));
        }

        fs::remove_dir_all(path).map_err(|e| {
            AppError::file_system_error(
                "remove_directory",
                &format!("Failed to remove directory {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully removed directory: {:?}", path);
        Ok(())
    }

    /// Check if directory exists
    pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_dir()
    }

    /// Get system config directory with fallback
    pub fn get_config_directory() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
    }

    /// Get system data directory with fallback
    pub fn get_data_directory() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
    }

    /// Get system cache directory with fallback
    pub fn get_cache_directory() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
    }

    /// Ensure config directory exists and return its path
    pub fn ensure_config_directory() -> AppResult<PathBuf> {
        let config_dir = Self::get_config_directory();
        Self::ensure_directory_exists(&config_dir)?;
        Ok(config_dir)
    }

    /// Ensure data directory exists and return its path
    pub fn ensure_data_directory() -> AppResult<PathBuf> {
        let data_dir = Self::get_data_directory();
        Self::ensure_directory_exists(&data_dir)?;
        Ok(data_dir)
    }

    /// Ensure cache directory exists and return its path
    pub fn ensure_cache_directory() -> AppResult<PathBuf> {
        let cache_dir = Self::get_cache_directory();
        Self::ensure_directory_exists(&cache_dir)?;
        Ok(cache_dir)
    }

    /// List files in directory
    pub fn list_files<P: AsRef<Path>>(path: P) -> AppResult<Vec<PathBuf>> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(AppError::file_system_error(
                "list_files",
                &format!("Directory not found: {:?}", path),
            ));
        }

        if !path.is_dir() {
            return Err(AppError::file_system_error(
                "list_files",
                &format!("Path is not a directory: {:?}", path),
            ));
        }

        let mut files = Vec::new();
        let entries = fs::read_dir(path).map_err(|e| {
            AppError::file_system_error(
                "list_files",
                &format!("Failed to read directory {:?}: {}", path, e),
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                AppError::file_system_error(
                    "list_files",
                    &format!("Failed to read directory entry: {}", e),
                )
            })?;

            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }

        Ok(files)
    }

    /// List directories in directory
    pub fn list_directories<P: AsRef<Path>>(path: P) -> AppResult<Vec<PathBuf>> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(AppError::file_system_error(
                "list_directories",
                &format!("Directory not found: {:?}", path),
            ));
        }

        if !path.is_dir() {
            return Err(AppError::file_system_error(
                "list_directories",
                &format!("Path is not a directory: {:?}", path),
            ));
        }

        let mut directories = Vec::new();
        let entries = fs::read_dir(path).map_err(|e| {
            AppError::file_system_error(
                "list_directories",
                &format!("Failed to read directory {:?}: {}", path, e),
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                AppError::file_system_error(
                    "list_directories",
                    &format!("Failed to read directory entry: {}", e),
                )
            })?;

            let path = entry.path();
            if path.is_dir() {
                directories.push(path);
            }
        }

        Ok(directories)
    }
}
