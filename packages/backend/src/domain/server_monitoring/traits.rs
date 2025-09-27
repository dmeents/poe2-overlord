//! # Server Monitoring Traits
//!
//! This module defines the core traits for simplified server monitoring functionality.

use crate::errors::AppResult;
use async_trait::async_trait;

/// Simplified service trait for server monitoring operations.
///
/// This trait defines the main business operations for monitoring server connectivity
/// and providing real-time status updates.
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    /// Get current server status
    async fn get_current_status(&self) -> crate::domain::server_monitoring::models::ServerStatus;

    /// Update server status from log analysis (when IP is found)
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;

    /// Ping current server and update status
    async fn ping_current_server(&self) -> AppResult<()>;

    /// Start periodic ping monitoring
    async fn start_ping_monitoring(&self) -> AppResult<()>;

    /// Stop periodic ping monitoring  
    async fn stop_ping_monitoring(&self) -> AppResult<()>;
}
