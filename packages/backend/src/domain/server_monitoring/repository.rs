//! File-based persistence for server status in the app data directory.

use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::ServerStatusRepository as ServerStatusRepositoryTrait;
use crate::errors::AppResult;
use crate::infrastructure::file_management::{AppPaths, FileService};
use async_trait::async_trait;
use log::debug;
use std::path::PathBuf;

pub struct ServerStatusRepository {
    file_path: PathBuf,
}

impl ServerStatusRepository {
    pub async fn new() -> AppResult<Self> {
        let data_dir = AppPaths::ensure_data_dir().await?;
        let file_path = data_dir.join("server_status.json");

        debug!(
            "Server status repository initialized at: {}",
            file_path.display()
        );

        Ok(Self { file_path })
    }
}

#[async_trait]
impl ServerStatusRepositoryTrait for ServerStatusRepository {
    async fn save(&self, status: &ServerStatus) -> AppResult<()> {
        FileService::write_json(&self.file_path, status).await?;
        debug!("Server status saved to: {}", self.file_path.display());
        Ok(())
    }

    async fn load(&self) -> AppResult<Option<ServerStatus>> {
        let status = FileService::read_json_optional(&self.file_path).await?;

        if status.is_some() {
            debug!("Server status loaded from: {}", self.file_path.display());
        } else {
            debug!(
                "Server status file does not exist: {}",
                self.file_path.display()
            );
        }

        Ok(status)
    }
}
