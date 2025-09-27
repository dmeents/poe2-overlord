use crate::domain::log_analysis::models::LogFileInfo;
use crate::errors::AppResult;
use async_trait::async_trait;

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

    /// Updates the path to the log file being monitored
    async fn update_log_path(&self, new_path: String) -> AppResult<()>;
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
