use crate::domain::log_analysis::models::LogEvent;
use crate::domain::log_analysis::models::{
    LogAnalysisConfig, LogAnalysisSession, LogAnalysisStats, LogFileInfo,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait LogAnalysisService: Send + Sync {
    async fn start_monitoring(&self) -> AppResult<()>;

    async fn stop_monitoring(&self) -> AppResult<()>;

    async fn is_monitoring(&self) -> bool;

    async fn get_log_file_info(&self) -> AppResult<LogFileInfo>;

    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>>;

    async fn get_analysis_stats(&self) -> AppResult<LogAnalysisStats>;

    fn subscribe_to_events(&self) -> broadcast::Receiver<LogEvent>;

    async fn update_log_path(&self, new_path: String) -> AppResult<()>;

    async fn get_config(&self) -> LogAnalysisConfig;

    async fn update_config(&self, config: LogAnalysisConfig) -> AppResult<()>;
}

#[async_trait]
pub trait LogFileRepository: Send + Sync {
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo>;

    async fn read_lines(
        &self,
        path: &str,
        start_line: usize,
        count: usize,
    ) -> AppResult<Vec<String>>;

    async fn get_file_size(&self, path: &str) -> AppResult<u64>;

    async fn file_exists(&self, path: &str) -> bool;

    async fn read_from_position(&self, path: &str, position: u64) -> AppResult<Vec<String>>;

    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>>;
}

#[async_trait]
pub trait LogAnalysisSessionRepository: Send + Sync {
    async fn save_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    async fn load_session(&self, session_id: &str) -> AppResult<Option<LogAnalysisSession>>;

    async fn get_active_session(&self) -> AppResult<Option<LogAnalysisSession>>;

    async fn update_session(&self, session: &LogAnalysisSession) -> AppResult<()>;

    async fn end_current_session(&self) -> AppResult<()>;

    async fn get_all_sessions(&self) -> AppResult<Vec<LogAnalysisSession>>;
}

#[async_trait]
pub trait LogAnalysisStatsRepository: Send + Sync {
    async fn save_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    async fn load_stats(&self) -> AppResult<LogAnalysisStats>;

    async fn update_stats(&self, stats: &LogAnalysisStats) -> AppResult<()>;

    async fn increment_event_count(&self, event_type: &str) -> AppResult<()>;

    async fn reset_stats(&self) -> AppResult<()>;
}
