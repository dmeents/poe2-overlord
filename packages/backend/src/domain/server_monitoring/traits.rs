use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringConfig, ServerMonitoringSession, ServerMonitoringStats, ServerStatus,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    async fn get_current_status(&self) -> ServerStatus;
    
    async fn update_status(&self, status: ServerStatus) -> AppResult<()>;
    
    async fn save_status(&self) -> AppResult<()>;
    
    async fn load_status(&self) -> AppResult<()>;
    
    async fn ping_server(&self) -> AppResult<Option<u64>>;
    
    async fn start_periodic_ping(&self) -> AppResult<()>;
    
    async fn stop_periodic_ping(&self) -> AppResult<()>;
    
    async fn is_ping_monitoring_active(&self) -> bool;
    
    async fn get_server_info(&self) -> Option<ServerInfo>;
    
    async fn update_server_info(&self, ip_address: String, port: u16) -> AppResult<()>;
    
    async fn get_monitoring_stats(&self) -> AppResult<ServerMonitoringStats>;
    
    async fn get_config(&self) -> ServerMonitoringConfig;
    
    async fn update_config(&self, config: ServerMonitoringConfig) -> AppResult<()>;
    
    fn subscribe_to_status_changes(&self) -> broadcast::Receiver<ServerStatus>;
}

#[async_trait]
pub trait ServerStatusRepository: Send + Sync {
    async fn save_status(&self, status: &ServerStatus) -> AppResult<()>;
    
    async fn load_status(&self) -> AppResult<Option<ServerStatus>>;
    
    async fn delete_status(&self) -> AppResult<()>;
    
    async fn status_exists(&self) -> bool;
}

#[async_trait]
pub trait ServerInfoRepository: Send + Sync {
    async fn save_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;
    
    async fn load_server_info(&self) -> AppResult<Option<ServerInfo>>;
    
    async fn update_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;
    
    async fn delete_server_info(&self) -> AppResult<()>;
}

#[async_trait]
pub trait ServerMonitoringSessionRepository: Send + Sync {
    async fn save_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;
    
    async fn load_session(&self, session_id: &str) -> AppResult<Option<ServerMonitoringSession>>;
    
    async fn get_active_session(&self) -> AppResult<Option<ServerMonitoringSession>>;
    
    async fn update_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;
    
    async fn end_current_session(&self) -> AppResult<()>;
    
    async fn get_all_sessions(&self) -> AppResult<Vec<ServerMonitoringSession>>;
}

#[async_trait]
pub trait ServerMonitoringStatsRepository: Send + Sync {
    async fn save_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;
    
    async fn load_stats(&self) -> AppResult<ServerMonitoringStats>;
    
    async fn update_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;
    
    async fn increment_ping_count(&self, success: bool) -> AppResult<()>;
    
    async fn update_average_latency(&self, latency_ms: u64) -> AppResult<()>;
    
    async fn reset_stats(&self) -> AppResult<()>;
}

#[async_trait]
pub trait NetworkConnectivity: Send + Sync {
    async fn ping_server(&self, ip_address: &str, port: u16, timeout_seconds: u64) -> AppResult<Option<u64>>;
    
    async fn is_server_reachable(&self, ip_address: &str, port: u16, timeout_seconds: u64) -> AppResult<bool>;
    
    fn get_network_config(&self) -> NetworkConfig;
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub default_timeout_seconds: u64,
    pub max_retry_attempts: u32,
    pub retry_delay_seconds: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            default_timeout_seconds: 5,
            max_retry_attempts: 3,
            retry_delay_seconds: 1,
        }
    }
}
