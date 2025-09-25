use serde::{Deserialize, Serialize};

/// Events related to log analysis service operations
/// These events are published when significant changes occur in the log analysis system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogAnalysisEvent {
    /// Log monitoring has been started
    MonitoringStarted {
        /// Path to the log file being monitored
        log_file_path: String,
        /// ISO 8601 timestamp when monitoring started
        timestamp: String,
    },
    
    /// Log monitoring has been stopped
    MonitoringStopped {
        /// ISO 8601 timestamp when monitoring stopped
        timestamp: String,
    },
    
    /// The log file path has been updated
    LogPathUpdated {
        /// The previous log file path
        old_path: String,
        /// The new log file path
        new_path: String,
        /// ISO 8601 timestamp when the path was updated
        timestamp: String,
    },
    
    /// The log analysis configuration has been updated
    ConfigurationUpdated {
        /// ISO 8601 timestamp when configuration was updated
        timestamp: String,
    },
    
    /// A new log analysis session has started
    SessionStarted {
        /// Unique identifier for the session
        session_id: String,
        /// ISO 8601 timestamp when the session started
        timestamp: String,
    },
    
    /// A log analysis session has ended
    SessionEnded {
        /// Unique identifier for the session
        session_id: String,
        /// Total number of events processed in this session
        events_processed: u64,
        /// Duration of the session in seconds
        duration_seconds: u64,
        /// ISO 8601 timestamp when the session ended
        timestamp: String,
    },
    
    /// Log analysis statistics have been updated
    StatisticsUpdated {
        /// Total number of events processed across all sessions
        total_events: u64,
        /// ISO 8601 timestamp when statistics were updated
        timestamp: String,
    },
    
    /// An error occurred during log analysis
    AnalysisError {
        /// Description of the error that occurred
        error_message: String,
        /// ISO 8601 timestamp when the error occurred
        timestamp: String,
    },
}

impl LogAnalysisEvent {
    /// Creates a monitoring started event with the current timestamp
    pub fn monitoring_started(log_file_path: String) -> Self {
        Self::MonitoringStarted {
            log_file_path,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a monitoring stopped event with the current timestamp
    pub fn monitoring_stopped() -> Self {
        Self::MonitoringStopped {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a log path updated event with the current timestamp
    pub fn log_path_updated(old_path: String, new_path: String) -> Self {
        Self::LogPathUpdated {
            old_path,
            new_path,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a configuration updated event with the current timestamp
    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session started event with the current timestamp
    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session ended event with the current timestamp
    pub fn session_ended(session_id: String, events_processed: u64, duration_seconds: u64) -> Self {
        Self::SessionEnded {
            session_id,
            events_processed,
            duration_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a statistics updated event with the current timestamp
    pub fn statistics_updated(total_events: u64) -> Self {
        Self::StatisticsUpdated {
            total_events,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates an analysis error event with the current timestamp
    pub fn analysis_error(error_message: String) -> Self {
        Self::AnalysisError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
