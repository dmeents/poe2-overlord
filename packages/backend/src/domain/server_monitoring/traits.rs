use crate::domain::server_monitoring::models::ServerStatus;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ServerStatusRepository: Send + Sync {
    async fn save(&self, status: &ServerStatus) -> AppResult<()>;

    async fn load(&self) -> AppResult<Option<ServerStatus>>;
}

#[async_trait]
pub trait PingProvider: Send + Sync {
    async fn ping(&self, ip_address: &str) -> Result<u64, String>;
}

#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;
    async fn ping_current_server(&self) -> AppResult<()>;
    async fn start_ping_monitoring(&self) -> AppResult<()>;
    async fn stop_ping_monitoring(&self) -> AppResult<()>;
}
