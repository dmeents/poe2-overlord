use crate::domain::server_monitoring::models::ServerStatus;
use serde::{Deserialize, Serialize};

/// Events related to server monitoring operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMonitoringEvent {
    /// Server status changed
    StatusChanged {
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
        timestamp: String,
    },
    
    /// Server ping completed
    PingCompleted {
        server_status: ServerStatus,
        latency_ms: Option<u64>,
        timestamp: String,
    },
    
    /// Periodic ping monitoring started
    PeriodicPingStarted {
        interval_seconds: u64,
        timestamp: String,
    },
    
    /// Periodic ping monitoring stopped
    PeriodicPingStopped {
        timestamp: String,
    },
    
    /// Server information updated
    ServerInfoUpdated {
        ip_address: String,
        port: u16,
        timestamp: String,
    },
    
    /// Monitoring session started
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    /// Monitoring session ended
    SessionEnded {
        session_id: String,
        total_pings: u64,
        success_rate: f64,
        timestamp: String,
    },
    
    /// Configuration updated
    ConfigurationUpdated {
        timestamp: String,
    },
    
    /// Error occurred during monitoring
    MonitoringError {
        error_message: String,
        timestamp: String,
    },
}

impl ServerMonitoringEvent {
    pub fn status_changed(old_status: Option<ServerStatus>, new_status: ServerStatus) -> Self {
        Self::StatusChanged {
            old_status,
            new_status,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn ping_completed(server_status: ServerStatus, latency_ms: Option<u64>) -> Self {
        Self::PingCompleted {
            server_status,
            latency_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn periodic_ping_started(interval_seconds: u64) -> Self {
        Self::PeriodicPingStarted {
            interval_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn periodic_ping_stopped() -> Self {
        Self::PeriodicPingStopped {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn server_info_updated(ip_address: String, port: u16) -> Self {
        Self::ServerInfoUpdated {
            ip_address,
            port,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_ended(session_id: String, total_pings: u64, success_rate: f64) -> Self {
        Self::SessionEnded {
            session_id,
            total_pings,
            success_rate,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn monitoring_error(error_message: String) -> Self {
        Self::MonitoringError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
