//! # Server Status Repository
//!
//! Simple file-based persistence for server status using the app data directory.
//! Stores server_status.json in the local data directory (.local on Linux/macOS,
//! AppData\Local on Windows).

use crate::domain::server_monitoring::models::ServerStatus;
use crate::errors::{AppError, AppResult};
use log::{debug, error};
use std::path::PathBuf;
use tokio::fs;

/// Simple repository for server status persistence.
///
/// Stores server status in a single JSON file in the application's data directory.
/// This is separate from the config directory to keep runtime state separate from
/// user configuration.
pub struct ServerStatusRepository {
    /// Path to the server_status.json file
    file_path: PathBuf,
}

impl ServerStatusRepository {
    /// Create a new server status repository.
    ///
    /// Uses the system's local data directory:
    /// - Linux: ~/.local/share/poe2-overlord/server_status.json
    /// - macOS: ~/Library/Application Support/poe2-overlord/server_status.json
    /// - Windows: C:\Users\<user>\AppData\Local\poe2-overlord\server_status.json
    pub fn new() -> AppResult<Self> {
        // Use local data directory (not config directory)
        let data_dir = dirs::data_local_dir().ok_or_else(|| {
            AppError::internal_error(
                "ServerStatusRepository::new",
                "Could not determine local data directory",
            )
        })?;

        let app_data_dir = data_dir.join("poe2-overlord");
        let file_path = app_data_dir.join("server_status.json");

        debug!(
            "Server status repository initialized at: {}",
            file_path.display()
        );

        Ok(Self { file_path })
    }

    /// Save server status to file.
    ///
    /// Creates the directory if it doesn't exist and writes the status as JSON.
    pub async fn save(&self, status: &ServerStatus) -> AppResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error(
                    "Failed to create server status directory: {}",
                    &e.to_string(),
                )
            })?;
        }

        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(status).map_err(|e| {
            AppError::internal_error("Failed to serialize server status: {}", &e.to_string())
        })?;

        // Write to file
        fs::write(&self.file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write server status file: {}", &e.to_string())
        })?;

        debug!("Server status saved to: {}", self.file_path.display());
        Ok(())
    }

    /// Load server status from file.
    ///
    /// Returns None if the file doesn't exist, otherwise deserializes and returns the status.
    pub async fn load(&self) -> AppResult<Option<ServerStatus>> {
        // Check if file exists
        if !self.file_path.exists() {
            debug!(
                "Server status file does not exist: {}",
                self.file_path.display()
            );
            return Ok(None);
        }

        // Read file contents
        let contents = fs::read_to_string(&self.file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read server status file: {}", &e.to_string())
        })?;

        // Deserialize JSON
        let status = serde_json::from_str(&contents).map_err(|e| {
            error!(
                "Failed to parse server status file, it may be corrupted: {}",
                e
            );
            AppError::internal_error("Failed to parse server status file: {}", &e.to_string())
        })?;

        debug!("Server status loaded from: {}", self.file_path.display());
        Ok(Some(status))
    }

    /// Get the file path for the server status file.
    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Check if the server status file exists.
    pub fn exists(&self) -> bool {
        self.file_path.exists()
    }
}

impl Default for ServerStatusRepository {
    fn default() -> Self {
        Self::new().expect("Failed to create ServerStatusRepository")
    }
}
