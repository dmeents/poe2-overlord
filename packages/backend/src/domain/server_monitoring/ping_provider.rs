//! Ping provider implementations for server connectivity testing.

use crate::domain::server_monitoring::traits::PingProvider;
use crate::errors::AppError;
use async_trait::async_trait;

const PING_COUNT: &str = "1";

#[cfg(target_family = "unix")]
const PING_COUNT_FLAG: &str = "-c";
#[cfg(target_family = "windows")]
const PING_COUNT_FLAG: &str = "-n";

#[cfg(target_family = "unix")]
const PING_TIMEOUT_FLAG: &str = "-W";
#[cfg(target_family = "windows")]
const PING_TIMEOUT_FLAG: &str = "-w";

// Unix expects seconds, Windows expects milliseconds
#[cfg(target_family = "unix")]
const PING_TIMEOUT_VALUE: &str = "5";
#[cfg(target_family = "windows")]
const PING_TIMEOUT_VALUE: &str = "5000";

/// System-level ping provider that uses the operating system's ping command.
#[derive(Debug, Clone)]
pub struct SystemPingProvider;

impl SystemPingProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemPingProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PingProvider for SystemPingProvider {
    async fn ping(&self, ip_address: &str) -> crate::errors::AppResult<u64> {
        let start = std::time::Instant::now();

        let output = tokio::process::Command::new("ping")
            .arg(PING_COUNT_FLAG)
            .arg(PING_COUNT)
            .arg(PING_TIMEOUT_FLAG)
            .arg(PING_TIMEOUT_VALUE)
            .arg(ip_address)
            .output()
            .await;

        match output {
            Ok(result) => {
                if result.status.success() {
                    let ping_ms = start.elapsed().as_millis() as u64;
                    Ok(ping_ms)
                } else {
                    Err(AppError::network_error("ping", "server unreachable"))
                }
            }
            Err(e) => Err(AppError::network_error("ping_command", &format!("{e}"))),
        }
    }
}
