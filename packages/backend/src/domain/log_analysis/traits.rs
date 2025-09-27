use crate::domain::events::AppEvent;
use crate::domain::log_analysis::models::{LogAnalysisConfig, LogFileInfo};
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
