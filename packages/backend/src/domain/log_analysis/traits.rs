use crate::domain::events::AppEvent;
use crate::domain::log_analysis::models::{
    LogAnalysisConfig, LogAnalysisSession, LogAnalysisStats, LogFileInfo,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Service trait for log analysis operations
/// Handles monitoring game log files and extracting meaningful events
#[async_trait]
pub trait LogAnalysisService: Send + Sync {
    /// Starts monitoring the configured log file for new events
    async fn start_monitoring(&self) -> AppResult<()>;

    /// Stops the current log monitoring session
    async fn stop_monitoring(&self) -> AppResult<()>;

    /// Returns whether log monitoring is currently active
    async fn is_monitoring(&self) -> bool;

    /// Gets information about the currently monitored log file
    async fn get_log_file_info(&self) -> AppResult<LogFileInfo>;

    /// Reads a specified number of lines from the log file starting at a given line
    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>>;

    /// Gets current statistics about log analysis activity
    async fn get_analysis_stats(&self) -> AppResult<LogAnalysisStats>;

    /// Subscribes to log events published by the service
    async fn subscribe_to_events(&self) -> AppResult<broadcast::Receiver<AppEvent>>;

    /// Updates the path to the log file being monitored
    async fn update_log_path(&self, new_path: String) -> AppResult<()>;

    /// Gets the current log analysis configuration
    async fn get_config(&self) -> LogAnalysisConfig;

    /// Updates the log analysis configuration
    async fn update_config(&self, config: LogAnalysisConfig) -> AppResult<()>;
}

/// Repository trait for file system operations on log files
/// Provides abstraction for reading and accessing log file data
#[async_trait]
pub trait LogFileRepository: Send + Sync {
    /// Gets metadata information about a log file
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo>;

    /// Reads a specified number of lines from a log file starting at a given line number
    async fn read_lines(
        &self,
        path: &str,
        start_line: usize,
        count: usize,
    ) -> AppResult<Vec<String>>;

    /// Gets the current size of a log file in bytes
    async fn get_file_size(&self, path: &str) -> AppResult<u64>;

    /// Checks whether a log file exists at the specified path
    async fn file_exists(&self, path: &str) -> bool;

    /// Reads all lines from a log file starting at a specific byte position
    async fn read_from_position(&self, path: &str, position: u64) -> AppResult<Vec<String>>;

    /// Gets the last modified time of a log file
    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>>;
}

/// Repository trait for managing log analysis sessions
/// Handles persistence and retrieval of monitoring session data
#[async_trait]
pub trait LogAnalysisSessionRepository: Send + Sync {
    /// Saves a log analysis session to persistent storage
    async fn save_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    /// Loads a specific log analysis session by its ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<LogAnalysisSession>>;

    /// Gets the currently active log analysis session, if any
    async fn get_active_session(&self) -> AppResult<Option<LogAnalysisSession>>;

    /// Updates an existing log analysis session
    async fn update_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    /// Ends the current active session and marks it as completed
    async fn end_current_session(&self) -> AppResult<()>;

    /// Gets all stored log analysis sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<LogAnalysisSession>>;
}

/// Repository trait for managing log analysis statistics
/// Handles persistence and updates of analysis performance metrics
#[async_trait]
pub trait LogAnalysisStatsRepository: Send + Sync {
    /// Saves log analysis statistics to persistent storage
    async fn save_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    /// Loads the current log analysis statistics
    async fn load_stats(&self) -> AppResult<LogAnalysisStats>;

    /// Updates the log analysis statistics
    async fn update_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    /// Increments the count for a specific event type
    async fn increment_event_count(&self, event_type: &str) -> AppResult<()>;

    /// Resets all statistics to their default values
    async fn reset_stats(&self) -> AppResult<()>;
}
