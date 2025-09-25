//! # Server Monitoring Events
//! 
//! This module defines the event types used in the server monitoring domain.
//! Events enable loose coupling between components and provide real-time
//! notifications about monitoring activities and status changes.

use crate::domain::server_monitoring::models::ServerStatus;
use serde::{Deserialize, Serialize};

/// Events emitted by the server monitoring system.
/// 
/// These events provide real-time notifications about monitoring activities,
/// status changes, and system state updates to interested components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMonitoringEvent {
    /// Server status has changed (online/offline, latency update, etc.)
    StatusChanged {
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
        timestamp: String,
    },
    
    /// A ping operation has completed with results
    PingCompleted {
        server_status: ServerStatus,
        latency_ms: Option<u64>,
        timestamp: String,
    },
    
    /// Periodic ping monitoring has been started
    PeriodicPingStarted {
        interval_seconds: u64,
        timestamp: String,
    },
    
    /// Periodic ping monitoring has been stopped
    PeriodicPingStopped {
        timestamp: String,
    },
    
    /// Server information has been updated (new IP/port)
    ServerInfoUpdated {
        ip_address: String,
        port: u16,
        timestamp: String,
    },
    
    /// A new monitoring session has started
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    /// A monitoring session has ended with summary statistics
    SessionEnded {
        session_id: String,
        total_pings: u64,
        success_rate: f64,
        timestamp: String,
    },
    
    /// Monitoring configuration has been updated
    ConfigurationUpdated {
        timestamp: String,
    },
    
    /// An error occurred during monitoring operations
    MonitoringError {
        error_message: String,
        timestamp: String,
    },
}

impl ServerMonitoringEvent {
    /// Create a status changed event with old and new status
    pub fn status_changed(old_status: Option<ServerStatus>, new_status: ServerStatus) -> Self {
        Self::StatusChanged {
            old_status,
            new_status,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a ping completed event with results
    pub fn ping_completed(server_status: ServerStatus, latency_ms: Option<u64>) -> Self {
        Self::PingCompleted {
            server_status,
            latency_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a periodic ping started event
    pub fn periodic_ping_started(interval_seconds: u64) -> Self {
        Self::PeriodicPingStarted {
            interval_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a periodic ping stopped event
    pub fn periodic_ping_stopped() -> Self {
        Self::PeriodicPingStopped {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a server info updated event
    pub fn server_info_updated(ip_address: String, port: u16) -> Self {
        Self::ServerInfoUpdated {
            ip_address,
            port,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a session started event
    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a session ended event with summary statistics
    pub fn session_ended(session_id: String, total_pings: u64, success_rate: f64) -> Self {
        Self::SessionEnded {
            session_id,
            total_pings,
            success_rate,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a configuration updated event
    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a monitoring error event
    pub fn monitoring_error(error_message: String) -> Self {
        Self::MonitoringError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
