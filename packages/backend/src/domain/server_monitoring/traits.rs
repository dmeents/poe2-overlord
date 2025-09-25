use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringConfig, ServerMonitoringSession, ServerMonitoringStats, ServerStatus,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for server monitoring service operations
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    /// Get the current server status
    async fn get_current_status(&self) -> ServerStatus;
    
    /// Update server status
    async fn update_status(&self, status: ServerStatus) -> AppResult<()>;
    
    /// Save server status to persistent storage
    async fn save_status(&self) -> AppResult<()>;
    
    /// Load server status from persistent storage
    async fn load_status(&self) -> AppResult<()>;
    
    /// Ping the current server and update status
    async fn ping_server(&self) -> AppResult<Option<u64>>;
    
    /// Start periodic ping monitoring
    async fn start_periodic_ping(&self) -> AppResult<()>;
    
    /// Stop periodic ping monitoring
    async fn stop_periodic_ping(&self) -> AppResult<()>;
    
    /// Check if periodic ping monitoring is active
    async fn is_ping_monitoring_active(&self) -> bool;
    
    /// Get server information
    async fn get_server_info(&self) -> Option<ServerInfo>;
    
    /// Update server information from connection event
    async fn update_server_info(&self, ip_address: String, port: u16) -> AppResult<()>;
    
    /// Get monitoring statistics
    async fn get_monitoring_stats(&self) -> AppResult<ServerMonitoringStats>;
    
    /// Get current configuration
    async fn get_config(&self) -> ServerMonitoringConfig;
    
    /// Update configuration
    async fn update_config(&self, config: ServerMonitoringConfig) -> AppResult<()>;
    
    /// Subscribe to server status changes
    fn subscribe_to_status_changes(&self) -> broadcast::Receiver<ServerStatus>;
}

/// Trait for server status repository operations
#[async_trait]
pub trait ServerStatusRepository: Send + Sync {
    /// Save server status to persistent storage
    async fn save_status(&self, status: &ServerStatus) -> AppResult<()>;
    
    /// Load server status from persistent storage
    async fn load_status(&self) -> AppResult<Option<ServerStatus>>;
    
    /// Delete server status from persistent storage
    async fn delete_status(&self) -> AppResult<()>;
    
    /// Check if status file exists
    async fn status_exists(&self) -> bool;
}

/// Trait for server information repository operations
#[async_trait]
pub trait ServerInfoRepository: Send + Sync {
    /// Save server information
    async fn save_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;
    
    /// Load server information
    async fn load_server_info(&self) -> AppResult<Option<ServerInfo>>;
    
    /// Update server information
    async fn update_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;
    
    /// Delete server information
    async fn delete_server_info(&self) -> AppResult<()>;
}

/// Trait for server monitoring session repository operations
#[async_trait]
pub trait ServerMonitoringSessionRepository: Send + Sync {
    /// Save monitoring session
    async fn save_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;
    
    /// Load session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<ServerMonitoringSession>>;
    
    /// Get current active session
    async fn get_active_session(&self) -> AppResult<Option<ServerMonitoringSession>>;
    
    /// Update session
    async fn update_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;
    
    /// End current session
    async fn end_current_session(&self) -> AppResult<()>;
    
    /// Get all sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<ServerMonitoringSession>>;
}

/// Trait for server monitoring statistics repository operations
#[async_trait]
pub trait ServerMonitoringStatsRepository: Send + Sync {
    /// Save monitoring statistics
    async fn save_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;
    
    /// Load monitoring statistics
    async fn load_stats(&self) -> AppResult<ServerMonitoringStats>;
    
    /// Update statistics
    async fn update_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;
    
    /// Increment ping counter
    async fn increment_ping_count(&self, success: bool) -> AppResult<()>;
    
    /// Update average latency
    async fn update_average_latency(&self, latency_ms: u64) -> AppResult<()>;
    
    /// Reset statistics
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Trait for network connectivity operations
#[async_trait]
pub trait NetworkConnectivity: Send + Sync {
    /// Ping a server and return latency in milliseconds
    async fn ping_server(&self, ip_address: &str, port: u16, timeout_seconds: u64) -> AppResult<Option<u64>>;
    
    /// Check if a server is reachable
    async fn is_server_reachable(&self, ip_address: &str, port: u16, timeout_seconds: u64) -> AppResult<bool>;
    
    /// Get network configuration
    fn get_network_config(&self) -> NetworkConfig;
}

/// Network configuration for connectivity operations
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
