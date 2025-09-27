//! # Server Monitoring Models
//!
//! This module contains the core data structures for server monitoring functionality.
//! Simplified to only track server status with IP from logs and ping results.

use serde::{Deserialize, Serialize};

/// Represents the current status of a game server.
///
/// This model captures the essential connectivity information including
/// server address, online status, latency measurements, and timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    /// Server IP address
    pub ip_address: String,
    /// Server port number
    pub port: u16,
    /// Whether the server is currently reachable
    pub is_online: bool,
    /// Measured latency in milliseconds (None if offline or not measured)
    pub latency_ms: Option<u64>,
    /// ISO 8601 timestamp of when this status was recorded
    pub timestamp: String,
}

impl Default for ServerStatus {
    fn default() -> Self {
        Self {
            ip_address: "127.0.0.1".to_string(),
            port: 6112,
            is_online: false,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl ServerStatus {
    /// Create a new server status with the given address and port (initially offline)
    pub fn new(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            is_online: false,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create server status from a log analysis connection event
    pub fn from_connection_event(
        event: &crate::domain::log_analysis::models::ServerConnectionEvent,
    ) -> Self {
        Self {
            ip_address: event.ip_address.clone(),
            port: event.port,
            is_online: true,
            latency_ms: None,
            timestamp: event.timestamp.clone(),
        }
    }

    /// Create server status with measured latency (implies online)
    pub fn with_latency(ip_address: String, port: u16, latency_ms: u64) -> Self {
        Self {
            ip_address,
            port,
            is_online: true,
            latency_ms: Some(latency_ms),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create an offline server status
    pub fn offline(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            is_online: false,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Update the status with new latency measurement (marks as online)
    pub fn update_latency(&mut self, latency_ms: u64) {
        self.latency_ms = Some(latency_ms);
        self.is_online = true;
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the server as offline and clear latency
    pub fn set_offline(&mut self) {
        self.is_online = false;
        self.latency_ms = None;
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }
}

/// Simple model for persisting only the last known server IP address
///
/// This model is used for lightweight persistence of server IP addresses
/// discovered from game logs. Only the IP address and discovery timestamp
/// are persisted, as other status information is transient and measured in real-time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIp {
    /// The last known server IP address from game logs
    pub ip_address: String,
    /// When this IP was last discovered
    pub discovered_at: String,
}

impl Default for ServerIp {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            discovered_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl ServerIp {
    /// Create a new ServerIp with the given IP address
    pub fn new(ip_address: String) -> Self {
        Self {
            ip_address,
            discovered_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
