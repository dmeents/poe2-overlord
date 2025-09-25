use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogAnalysisEvent {
    MonitoringStarted {
        log_file_path: String,
        timestamp: String,
    },
    
    MonitoringStopped {
        timestamp: String,
    },
    
    LogPathUpdated {
        old_path: String,
        new_path: String,
        timestamp: String,
    },
    
    ConfigurationUpdated {
        timestamp: String,
    },
    
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    SessionEnded {
        session_id: String,
        events_processed: u64,
        duration_seconds: u64,
        timestamp: String,
    },
    
    StatisticsUpdated {
        total_events: u64,
        timestamp: String,
    },
    
    AnalysisError {
        error_message: String,
        timestamp: String,
    },
}

impl LogAnalysisEvent {
    pub fn monitoring_started(log_file_path: String) -> Self {
        Self::MonitoringStarted {
            log_file_path,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn monitoring_stopped() -> Self {
        Self::MonitoringStopped {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn log_path_updated(old_path: String, new_path: String) -> Self {
        Self::LogPathUpdated {
            old_path,
            new_path,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_ended(session_id: String, events_processed: u64, duration_seconds: u64) -> Self {
        Self::SessionEnded {
            session_id,
            events_processed,
            duration_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn statistics_updated(total_events: u64) -> Self {
        Self::StatisticsUpdated {
            total_events,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn analysis_error(error_message: String) -> Self {
        Self::AnalysisError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
