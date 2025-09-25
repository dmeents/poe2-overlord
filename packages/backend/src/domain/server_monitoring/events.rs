use crate::domain::server_monitoring::models::ServerStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMonitoringEvent {
    StatusChanged {
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
        timestamp: String,
    },
    
    PingCompleted {
        server_status: ServerStatus,
        latency_ms: Option<u64>,
        timestamp: String,
    },
    
    PeriodicPingStarted {
        interval_seconds: u64,
        timestamp: String,
    },
    
    PeriodicPingStopped {
        timestamp: String,
    },
    
    ServerInfoUpdated {
        ip_address: String,
        port: u16,
        timestamp: String,
    },
    
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    SessionEnded {
        session_id: String,
        total_pings: u64,
        success_rate: f64,
        timestamp: String,
    },
    
    ConfigurationUpdated {
        timestamp: String,
    },
    
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
