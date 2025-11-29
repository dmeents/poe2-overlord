//! Server monitoring service trait definitions.

use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;
    async fn ping_current_server(&self) -> AppResult<()>;
    async fn start_ping_monitoring(&self) -> AppResult<()>;
}
