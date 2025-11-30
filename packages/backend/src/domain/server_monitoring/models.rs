//! Server monitoring data structures for tracking server status.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

pub const DEFAULT_SERVER_PORT: u16 = 6112;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ip_address: String,
    pub port: u16,
    pub is_online: bool,
    pub latency_ms: Option<u64>,
    pub timestamp: String,
}

impl Default for ServerStatus {
    fn default() -> Self {
        Self {
            ip_address: "127.0.0.1".to_string(),
            port: DEFAULT_SERVER_PORT,
            is_online: false,
            latency_ms: None,
            timestamp: Self::current_timestamp(),
        }
    }
}

impl ServerStatus {
    fn current_timestamp() -> String {
        chrono::Utc::now().to_rfc3339()
    }

    pub fn new(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            is_online: false,
            latency_ms: None,
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.ip_address.is_empty() || self.ip_address == "0.0.0.0" {
            return false;
        }

        // Validate IP address format
        self.ip_address.parse::<IpAddr>().is_ok()
    }

    pub fn mark_as_online(&mut self, latency_ms: u64) {
        self.is_online = true;
        self.latency_ms = Some(latency_ms);
        self.timestamp = Self::current_timestamp();
    }

    pub fn mark_as_offline(&mut self) {
        self.is_online = false;
        self.latency_ms = None;
        self.timestamp = Self::current_timestamp();
    }
}
