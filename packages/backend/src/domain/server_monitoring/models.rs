use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ip_address: String,
    pub port: u16,
    pub is_online: bool,
    pub latency_ms: Option<u64>,
    pub timestamp: String,
}

impl ServerStatus {
    pub fn new(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            is_online: false,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn from_connection_event(event: &crate::domain::log_analysis::models::ServerConnectionEvent) -> Self {
        Self {
            ip_address: event.ip_address.clone(),
            port: event.port,
            is_online: true,
            latency_ms: None,
            timestamp: event.timestamp.clone(),
        }
    }

    pub fn with_latency(ip_address: String, port: u16, latency_ms: u64) -> Self {
        Self {
            ip_address,
            port,
            is_online: true,
            latency_ms: Some(latency_ms),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn offline(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            is_online: false,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn update_latency(&mut self, latency_ms: u64) {
        self.latency_ms = Some(latency_ms);
        self.is_online = true;
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }

    pub fn set_offline(&mut self) {
        self.is_online = false;
        self.latency_ms = None;
        self.timestamp = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringConfig {
    pub ping_interval_seconds: u64,
    pub ping_timeout_seconds: u64,
    pub status_file_path: PathBuf,
    pub max_retry_attempts: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub ip_address: String,
    pub port: u16,
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,
    pub connection_count: u64,
    pub total_uptime_seconds: u64,
}

impl ServerInfo {
    pub fn new(ip_address: String, port: u16) -> Self {
        Self {
            ip_address,
            port,
            last_connected: Some(chrono::Utc::now()),
            connection_count: 1,
            total_uptime_seconds: 0,
        }
    }

    pub fn record_connection(&mut self) {
        self.last_connected = Some(chrono::Utc::now());
        self.connection_count += 1;
    }

    pub fn add_uptime(&mut self, seconds: u64) {
        self.total_uptime_seconds += seconds;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub server_info: Option<ServerInfo>,
    pub total_pings: u64,
    pub successful_pings: u64,
    pub failed_pings: u64,
    pub is_active: bool,
}

impl ServerMonitoringSession {
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

    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

    pub fn record_ping(&mut self, success: bool) {
        self.total_pings += 1;
        if success {
            self.successful_pings += 1;
        } else {
            self.failed_pings += 1;
        }
    }

    pub fn set_server_info(&mut self, server_info: ServerInfo) {
        self.server_info = Some(server_info);
    }

    pub fn get_success_rate(&self) -> f64 {
        if self.total_pings == 0 {
            return 0.0;
        }
        (self.successful_pings as f64 / self.total_pings as f64) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMonitoringStats {
    pub total_sessions: u64,
    pub total_pings: u64,
    pub successful_pings: u64,
    pub failed_pings: u64,
    pub average_latency_ms: Option<f64>,
    pub last_monitoring_time: chrono::DateTime<chrono::Utc>,
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

#[derive(Debug, thiserror::Error)]
pub enum ServerMonitoringError {
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Persistence error: {message}")]
    PersistenceError { message: String },
    
    #[error("Timeout error: {message}")]
    TimeoutError { message: String },
    
    #[error("Invalid server address: {address}")]
    InvalidServerAddress { address: String },
}
