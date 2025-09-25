use crate::models::events::LogEvent;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Log file information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub exists: bool,
}

/// Log analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnalysisConfig {
    pub log_file_path: String,
    pub monitoring_interval_ms: u64,
    pub max_file_size_mb: u64,
    pub buffer_size: usize,
}

impl Default for LogAnalysisConfig {
    fn default() -> Self {
        Self {
            log_file_path: String::new(),
            monitoring_interval_ms: 100,
            max_file_size_mb: 100,
            buffer_size: 1000,
        }
    }
}

/// Log analysis session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnalysisSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub events_processed: u64,
    pub last_position: u64,
    pub is_active: bool,
}

impl LogAnalysisSession {
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            events_processed: 0,
            last_position: 0,
            is_active: true,
        }
    }

    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }
}

/// Log analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnalysisStats {
    pub total_events_processed: u64,
    pub scene_changes_detected: u64,
    pub server_connections_detected: u64,
    pub character_level_ups_detected: u64,
    pub character_deaths_detected: u64,
    pub last_analysis_time: chrono::DateTime<chrono::Utc>,
    pub current_session: Option<LogAnalysisSession>,
}

impl Default for LogAnalysisStats {
    fn default() -> Self {
        Self {
            total_events_processed: 0,
            scene_changes_detected: 0,
            server_connections_detected: 0,
            character_level_ups_detected: 0,
            character_deaths_detected: 0,
            last_analysis_time: chrono::Utc::now(),
            current_session: None,
        }
    }
}

/// Log analysis result for a single line
#[derive(Debug, Clone)]
pub struct LogLineAnalysis {
    pub line_number: usize,
    pub content: String,
    pub parsed_event: Option<LogEvent>,
    pub processing_time_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Log analysis error types
#[derive(Debug, thiserror::Error)]
pub enum LogAnalysisError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("File access error: {message}")]
    FileAccessError { message: String },
    
    #[error("Parsing error: {message}")]
    ParsingError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Monitoring error: {message}")]
    MonitoringError { message: String },
}
