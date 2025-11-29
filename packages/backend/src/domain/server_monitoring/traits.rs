//! Server monitoring service trait definitions.

use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait for abstracting ping operations to allow for testing and alternative implementations.
#[async_trait]
pub trait PingProvider: Send + Sync {
    /// Pings the specified IP address and returns the latency in milliseconds.
    /// Returns an error if the ping fails or times out.
    async fn ping(&self, ip_address: &str) -> Result<u64, String>;
}

#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;
    async fn ping_current_server(&self) -> AppResult<()>;
    async fn start_ping_monitoring(&self) -> AppResult<()>;
    async fn stop_ping_monitoring(&self) -> AppResult<()>;
}
