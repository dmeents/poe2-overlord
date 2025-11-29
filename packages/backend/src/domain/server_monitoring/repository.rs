//! File-based persistence for server status in the app data directory.

use crate::domain::server_monitoring::models::ServerStatus;
use crate::errors::{AppError, AppResult};
use log::{debug, error};
use std::path::PathBuf;
use tokio::fs;

pub struct ServerStatusRepository {
    file_path: PathBuf,
}

impl ServerStatusRepository {
    pub fn new() -> AppResult<Self> {
        let data_dir = dirs::data_local_dir().ok_or_else(|| {
            AppError::internal_error(
                "get_data_directory",
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

    pub async fn save(&self, status: &ServerStatus) -> AppResult<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("create_status_directory", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(status)
            .map_err(|e| AppError::internal_error("serialize_server_status", &e.to_string()))?;

        fs::write(&self.file_path, json)
            .await
            .map_err(|e| AppError::file_system_error("write_status_file", &e.to_string()))?;

        debug!("Server status saved to: {}", self.file_path.display());
        Ok(())
    }

    pub async fn load(&self) -> AppResult<Option<ServerStatus>> {
        if !self.file_path.exists() {
            debug!(
                "Server status file does not exist: {}",
                self.file_path.display()
            );
            return Ok(None);
        }

        let contents = fs::read_to_string(&self.file_path)
            .await
            .map_err(|e| AppError::file_system_error("read_status_file", &e.to_string()))?;

        let status = serde_json::from_str(&contents).map_err(|e| {
            error!(
                "Failed to parse server status file, it may be corrupted: {}",
                e
            );
            AppError::internal_error("parse_status_file", &e.to_string())
        })?;

        debug!("Server status loaded from: {}", self.file_path.display());
        Ok(Some(status))
    }
}

impl Default for ServerStatusRepository {
    fn default() -> Self {
        Self::new().expect("Failed to create ServerStatusRepository")
    }
}
