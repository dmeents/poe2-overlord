use crate::domain::log_analysis::models::LogFileInfo;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait LogAnalysisService: Send + Sync {
    async fn start_monitoring(&self) -> AppResult<()>;

    async fn stop_monitoring(&self) -> AppResult<()>;

    async fn is_monitoring(&self) -> bool;

    async fn update_log_path(&self, new_path: String) -> AppResult<()>;
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
