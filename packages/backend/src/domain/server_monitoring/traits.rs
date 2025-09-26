//! # Server Monitoring Traits
//!
//! This module defines the core traits that establish contracts for server monitoring functionality.
//! These traits follow the dependency inversion principle, allowing for flexible implementations
//! and easy testing through dependency injection.

use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringConfig, ServerMonitoringSession, ServerMonitoringStats,
    ServerStatus,
};
use crate::domain::events::AppEvent;
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for publishing server monitoring events.
///
/// This trait defines the contract for publishing server status events,
/// allowing for different implementations in testing and production.
pub trait ServerMonitoringEventPublisher: Send + Sync {
    /// Broadcast a ping event with server status
    fn broadcast_ping_event(&self, status: ServerStatus) -> AppResult<()>;
}

/// Core service trait for server monitoring operations.
///
/// This trait defines the main business operations for monitoring server connectivity,
/// managing monitoring sessions, and providing real-time status updates.
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    /// Get the current server status from memory cache
    async fn get_current_status(&self) -> ServerStatus;

    /// Update the server status and persist changes
    async fn update_status(&self, status: ServerStatus) -> AppResult<()>;

    /// Persist the current status to storage
    async fn save_status(&self) -> AppResult<()>;

    /// Load the last known status from storage
    async fn load_status(&self) -> AppResult<()>;

    /// Perform a single ping to the server and return latency
    async fn ping_server(&self) -> AppResult<Option<u64>>;

    /// Start periodic ping monitoring with configured interval
    async fn start_periodic_ping(&self) -> AppResult<()>;

    /// Stop periodic ping monitoring
    async fn stop_periodic_ping(&self) -> AppResult<()>;

    /// Check if periodic ping monitoring is currently active
    async fn is_ping_monitoring_active(&self) -> bool;

    /// Get server information including connection history
    async fn get_server_info(&self) -> Option<ServerInfo>;

    /// Update server information with new IP/port
    async fn update_server_info(&self, ip_address: String, port: u16) -> AppResult<()>;

    /// Get comprehensive monitoring statistics
    async fn get_monitoring_stats(&self) -> AppResult<ServerMonitoringStats>;

    /// Get current monitoring configuration
    async fn get_config(&self) -> ServerMonitoringConfig;

    /// Update monitoring configuration
    async fn update_config(&self, config: ServerMonitoringConfig) -> AppResult<()>;

    /// Subscribe to real-time status change events
    async fn subscribe_to_status_changes(&self) -> AppResult<broadcast::Receiver<AppEvent>>;
}

/// Repository trait for server status persistence operations.
///
/// Handles the storage and retrieval of server status information,
/// providing a clean abstraction over the persistence layer.
#[async_trait]
pub trait ServerStatusRepository: Send + Sync {
    /// Save server status to persistent storage
    async fn save_status(&self, status: &ServerStatus) -> AppResult<()>;

    /// Load server status from persistent storage
    async fn load_status(&self) -> AppResult<Option<ServerStatus>>;

    /// Delete server status from persistent storage
    async fn delete_status(&self) -> AppResult<()>;

    /// Check if server status exists in storage
    async fn status_exists(&self) -> bool;
}

/// Repository trait for server information persistence operations.
///
/// Manages server metadata including connection history, uptime tracking,
/// and connection counts for analytics and monitoring purposes.
#[async_trait]
pub trait ServerInfoRepository: Send + Sync {
    /// Save server information to persistent storage
    async fn save_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;

    /// Load server information from persistent storage
    async fn load_server_info(&self) -> AppResult<Option<ServerInfo>>;

    /// Update existing server information
    async fn update_server_info(&self, server_info: &ServerInfo) -> AppResult<()>;

    /// Delete server information from storage
    async fn delete_server_info(&self) -> AppResult<()>;
}

/// Repository trait for monitoring session persistence operations.
///
/// Manages monitoring sessions which track individual monitoring periods,
/// including ping statistics, success rates, and session metadata.
#[async_trait]
pub trait ServerMonitoringSessionRepository: Send + Sync {
    /// Save a monitoring session to storage
    async fn save_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;

    /// Load a specific monitoring session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<ServerMonitoringSession>>;

    /// Get the currently active monitoring session
    async fn get_active_session(&self) -> AppResult<Option<ServerMonitoringSession>>;

    /// Update an existing monitoring session
    async fn update_session(&self, session: &ServerMonitoringSession) -> AppResult<()>;

    /// End the current active session and mark it as completed
    async fn end_current_session(&self) -> AppResult<()>;

    /// Retrieve all monitoring sessions (for analytics/history)
    async fn get_all_sessions(&self) -> AppResult<Vec<ServerMonitoringSession>>;
}

/// Repository trait for monitoring statistics persistence operations.
///
/// Manages aggregated monitoring statistics including total ping counts,
/// success rates, average latency, and session summaries.
#[async_trait]
pub trait ServerMonitoringStatsRepository: Send + Sync {
    /// Save monitoring statistics to storage
    async fn save_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;

    /// Load monitoring statistics from storage
    async fn load_stats(&self) -> AppResult<ServerMonitoringStats>;

    /// Update monitoring statistics
    async fn update_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()>;

    /// Increment ping count and update success/failure counters
    async fn increment_ping_count(&self, success: bool) -> AppResult<()>;

    /// Update the running average latency calculation
    async fn update_average_latency(&self, latency_ms: u64) -> AppResult<()>;

    /// Reset all statistics to default values
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Trait for network connectivity operations.
///
/// Provides low-level network operations for server connectivity testing,
/// including ping functionality and reachability checks.
#[async_trait]
pub trait NetworkConnectivity: Send + Sync {
    /// Ping a server and return latency in milliseconds, or None if unreachable
    async fn ping_server(
        &self,
        ip_address: &str,
        port: u16,
        timeout_seconds: u64,
    ) -> AppResult<Option<u64>>;

    /// Check if a server is reachable without measuring latency
    async fn is_server_reachable(
        &self,
        ip_address: &str,
        port: u16,
        timeout_seconds: u64,
    ) -> AppResult<bool>;

    /// Get the current network configuration
    fn get_network_config(&self) -> NetworkConfig;
}

/// Configuration for network connectivity operations.
///
/// Defines timeout values, retry policies, and other network-related settings
/// used by the NetworkConnectivity implementations.
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Default timeout for network operations in seconds
    pub default_timeout_seconds: u64,
    /// Maximum number of retry attempts for failed operations
    pub max_retry_attempts: u32,
    /// Delay between retry attempts in seconds
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
