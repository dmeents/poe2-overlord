//! # Server Monitoring Repository Implementation
//!
//! This module provides a simple repository implementation for server IP persistence.
//! Uses file-based persistence to store only the last known server IP address.

use crate::domain::server_monitoring::models::ServerIp;
use crate::errors::AppResult;
use crate::infrastructure::persistence::{PersistenceRepository, PersistenceRepositoryImpl};
use async_trait::async_trait;
use log::debug;

/// Simple repository for server IP persistence.
///
/// Uses file-based persistence to store only the last known server IP address.
pub struct ServerIpRepository {
    /// File-based persistence layer
    persistence: PersistenceRepositoryImpl<ServerIp>,
}

impl ServerIpRepository {
    /// Create a new server IP repository with file-based persistence.
    pub fn new() -> AppResult<Self> {
        let persistence =
            PersistenceRepositoryImpl::<ServerIp>::new_in_config_dir("server_ip.json")?;

        Ok(Self { persistence })
    }
}

#[async_trait]
impl ServerIpRepositoryTrait for ServerIpRepository {
    async fn save_ip(&self, server_ip: &ServerIp) -> AppResult<()> {
        self.persistence.save(server_ip).await?;
        debug!("Server IP saved to file");
        Ok(())
    }

    async fn load_ip(&self) -> AppResult<Option<ServerIp>> {
        if self.persistence.exists().await? {
            let server_ip = self.persistence.load().await?;
            debug!("Loaded server IP from file");
            Ok(Some(server_ip))
        } else {
            debug!("No server IP file found");
            Ok(None)
        }
    }
}

/// Trait for server IP repository operations.
#[async_trait]
pub trait ServerIpRepositoryTrait: Send + Sync {
    /// Save server IP to persistence
    async fn save_ip(&self, server_ip: &ServerIp) -> AppResult<()>;

    /// Load server IP from persistence
    async fn load_ip(&self) -> AppResult<Option<ServerIp>>;
}
