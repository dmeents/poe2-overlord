//! # Server Monitoring Models
//! 
//! This module contains the core data structures for server monitoring functionality.
//! These models represent the domain entities and their business logic methods.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub fn from_connection_event(event: &crate::domain::log_analysis::models::ServerConnectionEvent) -> Self {
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

/// Configuration settings for server monitoring operations.
/// 
/// Defines timing intervals, file paths, and retry policies used
/// throughout the monitoring system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringConfig {
    /// Interval between periodic ping attempts in seconds
    pub ping_interval_seconds: u64,
    /// Timeout for individual ping operations in seconds
    pub ping_timeout_seconds: u64,
    /// File path where server status is persisted
    pub status_file_path: PathBuf,
    /// Maximum number of retry attempts for failed operations
    pub max_retry_attempts: u32,
    /// Delay between retry attempts in seconds
    pub retry_delay_seconds: u64,
}

impl Default for ServerMonitoringConfig {
    fn default() -> Self {
        let mut status_file_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        status_file_path.push("poe2-overlord");
        status_file_path.push("server_status.json");

        Self {
            ping_interval_seconds: 30,
            ping_timeout_seconds: 5,
            status_file_path,
            max_retry_attempts: 3,
            retry_delay_seconds: 10,
        }
    }
}

/// Metadata and historical information about a server.
/// 
/// Tracks connection history, uptime statistics, and other server-related
/// information for analytics and monitoring purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server IP address
    pub ip_address: String,
    /// Server port number
    pub port: u16,
    /// Timestamp of the last successful connection
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,
    /// Total number of connections made to this server
    pub connection_count: u64,
    /// Total uptime in seconds across all monitoring sessions
    pub total_uptime_seconds: u64,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            ip_address: "127.0.0.1".to_string(),
            port: 6112,
            last_connected: None,
            connection_count: 0,
            total_uptime_seconds: 0,
        }
    }
}

impl ServerInfo {
    /// Create new server info for a first-time connection
    pub fn new(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            last_connected: Some(chrono::Utc::now()),
            connection_count: 1,
            total_uptime_seconds: 0,
        }
    }

    /// Record a new connection to this server
    pub fn record_connection(&mut self) {
        self.last_connected = Some(chrono::Utc::now());
        self.connection_count += 1;
    }

    /// Add uptime duration to the total uptime counter
    pub fn add_uptime(&mut self, seconds: u64) {
        self.total_uptime_seconds += seconds;
    }
}

/// Represents a monitoring session with ping statistics and metadata.
/// 
/// Tracks individual monitoring periods, recording ping success rates,
/// latency data, and session duration for analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringSession {
    /// Unique identifier for this monitoring session
    pub session_id: String,
    /// When the monitoring session started
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// When the monitoring session ended (None if still active)
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Server information associated with this session
    pub server_info: Option<ServerInfo>,
    /// Total number of ping attempts made during this session
    pub total_pings: u64,
    /// Number of successful ping attempts
    pub successful_pings: u64,
    /// Number of failed ping attempts
    pub failed_pings: u64,
    /// Whether this session is currently active
    pub is_active: bool,
}

impl Default for ServerMonitoringSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerMonitoringSession {
    /// Create a new monitoring session with a unique ID
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            server_info: None,
            total_pings: 0,
            successful_pings: 0,
            failed_pings: 0,
            is_active: true,
        }
    }

    /// End the monitoring session and record the end time
    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

    /// Record a ping attempt and its result
    pub fn record_ping(&mut self, success: bool) {
        self.total_pings += 1;
        if success {
            self.successful_pings += 1;
        } else {
            self.failed_pings += 1;
        }
    }

    /// Associate server information with this session
    pub fn set_server_info(&mut self, server_info: ServerInfo) {
        self.server_info = Some(server_info);
    }

    /// Calculate the ping success rate as a percentage
    pub fn get_success_rate(&self) -> f64 {
        if self.total_pings == 0 {
            return 0.0;
        }
        (self.successful_pings as f64 / self.total_pings as f64) * 100.0
    }
}

/// Aggregated statistics across all monitoring sessions.
/// 
/// Provides comprehensive metrics including total sessions, ping statistics,
/// average latency, and references to the current active session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringStats {
    /// Total number of monitoring sessions completed
    pub total_sessions: u64,
    /// Total number of ping attempts across all sessions
    pub total_pings: u64,
    /// Total number of successful ping attempts
    pub successful_pings: u64,
    /// Total number of failed ping attempts
    pub failed_pings: u64,
    /// Running average latency across all successful pings
    pub average_latency_ms: Option<f64>,
    /// Timestamp of the last monitoring activity
    pub last_monitoring_time: chrono::DateTime<chrono::Utc>,
    /// Reference to the currently active monitoring session
    pub current_session: Option<ServerMonitoringSession>,
}

impl Default for ServerMonitoringStats {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            total_pings: 0,
            successful_pings: 0,
            failed_pings: 0,
            average_latency_ms: None,
            last_monitoring_time: chrono::Utc::now(),
            current_session: None,
        }
    }
}

/// Error types specific to server monitoring operations.
/// 
/// Provides detailed error information for different failure scenarios
/// in the server monitoring domain.
#[derive(Debug, thiserror::Error)]
pub enum ServerMonitoringError {
    /// Network-related errors (connection failures, DNS issues, etc.)
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    /// Configuration-related errors (invalid settings, missing files, etc.)
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    /// Persistence-related errors (file I/O, serialization, etc.)
    #[error("Persistence error: {message}")]
    PersistenceError { message: String },
    
    /// Timeout-related errors (ping timeouts, operation timeouts, etc.)
    #[error("Timeout error: {message}")]
    TimeoutError { message: String },
    
    /// Invalid server address format or unreachable address
    #[error("Invalid server address: {address}")]
    InvalidServerAddress { address: String },
}
