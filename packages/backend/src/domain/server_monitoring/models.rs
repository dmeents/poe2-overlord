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

    /// Check if the server status has a valid IP address
    pub fn is_valid(&self) -> bool {
        !self.ip_address.is_empty() && self.ip_address != "0.0.0.0"
    }

    /// Update status from ping result
    pub fn update_from_ping(&mut self, latency: Option<u64>) {
        match latency {
            Some(ms) => {
                self.is_online = true;
                self.latency_ms = Some(ms);
            }
            None => {
                self.is_online = false;
                self.latency_ms = None;
            }
        }
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the server as online with given latency
    pub fn mark_as_online(&mut self, latency_ms: u64) {
        self.is_online = true;
        self.latency_ms = Some(latency_ms);
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }

    /// Mark the server as offline
    pub fn mark_as_offline(&mut self) {
        self.is_online = false;
        self.latency_ms = None;
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }
}
