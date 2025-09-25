use crate::domain::log_analysis::models::{
    LogAnalysisConfig, LogAnalysisSession, LogAnalysisStats, LogFileInfo,
};
use crate::errors::AppResult;
use crate::models::events::LogEvent;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for log analysis service operations
#[async_trait]
pub trait LogAnalysisService: Send + Sync {
    /// Start monitoring the log file
    async fn start_monitoring(&self) -> AppResult<()>;

    /// Stop monitoring the log file
    async fn stop_monitoring(&self) -> AppResult<()>;

    /// Check if monitoring is currently active
    async fn is_monitoring(&self) -> bool;

    /// Get current log file information
    async fn get_log_file_info(&self) -> AppResult<LogFileInfo>;

    /// Read log lines from the file
    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>>;

    /// Get log analysis statistics
    async fn get_analysis_stats(&self) -> AppResult<LogAnalysisStats>;

    /// Subscribe to log events
    fn subscribe_to_events(&self) -> broadcast::Receiver<LogEvent>;

    /// Update log file path configuration
    async fn update_log_path(&self, new_path: String) -> AppResult<()>;

    /// Get current configuration
    async fn get_config(&self) -> LogAnalysisConfig;

    /// Update configuration
    async fn update_config(&self, config: LogAnalysisConfig) -> AppResult<()>;
}

/// Trait for log file repository operations
#[async_trait]
pub trait LogFileRepository: Send + Sync {
    /// Get log file information
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo>;

    /// Read lines from log file
    async fn read_lines(
        &self,
        path: &str,
        start_line: usize,
        count: usize,
    ) -> AppResult<Vec<String>>;

    /// Get file size
    async fn get_file_size(&self, path: &str) -> AppResult<u64>;

    /// Check if file exists
    async fn file_exists(&self, path: &str) -> bool;

    /// Read file from specific position
    async fn read_from_position(&self, path: &str, position: u64) -> AppResult<Vec<String>>;

    /// Get file modification time
    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>>;
}

/// Trait for log analysis session management
#[async_trait]
pub trait LogAnalysisSessionRepository: Send + Sync {
    /// Save analysis session
    async fn save_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    /// Load analysis session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<LogAnalysisSession>>;

    /// Get current active session
    async fn get_active_session(&self) -> AppResult<Option<LogAnalysisSession>>;

    /// Update session
    async fn update_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    /// End current session
    async fn end_current_session(&self) -> AppResult<()>;

    /// Get all sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<LogAnalysisSession>>;
}

/// Trait for log analysis statistics repository
#[async_trait]
pub trait LogAnalysisStatsRepository: Send + Sync {
    /// Save analysis statistics
    async fn save_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    /// Load analysis statistics
    async fn load_stats(&self) -> AppResult<LogAnalysisStats>;

    /// Update statistics
    async fn update_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    /// Increment event counter
    async fn increment_event_count(&self, event_type: &str) -> AppResult<()>;

    /// Reset statistics
    async fn reset_stats(&self) -> AppResult<()>;
}
