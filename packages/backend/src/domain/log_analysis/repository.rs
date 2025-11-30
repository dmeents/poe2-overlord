use crate::domain::log_analysis::models::LogFileInfo;
use crate::domain::log_analysis::traits::LogFileRepository;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};

/// Simple implementation of LogFileRepository for file system operations
pub struct LogFileRepositoryImpl;

impl LogFileRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LogFileRepository for LogFileRepositoryImpl {
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo> {
        let metadata = fs::metadata(path).await.map_err(|e| {
            AppError::file_system_error(
                "get_file_info",
                &format!("Failed to get file metadata for '{}': {}", path, e),
            )
        })?;

        Ok(LogFileInfo {
            path: std::path::PathBuf::from(path),
            size: metadata.len(),
            exists: true,
            last_modified: chrono::DateTime::<chrono::Utc>::from(metadata.modified().map_err(
                |e| {
                    AppError::file_system_error(
                        "get_file_info",
                        &format!("Failed to get modified time for '{}': {}", path, e),
                    )
                },
            )?),
        })
    }

    async fn read_lines(
        &self,
        path: &str,
        start_line: usize,
        count: usize,
    ) -> AppResult<Vec<String>> {
        let file = fs::File::open(path).await.map_err(|e| {
            AppError::file_system_error(
                "read_lines",
                &format!("Failed to open file '{}': {}", path, e),
            )
        })?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut result = Vec::new();
        let mut current_line = 0;

        while let Some(line) = lines.next_line().await.map_err(|e| {
            AppError::file_system_error(
                "read_lines",
                &format!("Failed to read line from '{}': {}", path, e),
            )
        })? {
            if current_line >= start_line && result.len() < count {
                result.push(line);
            }
            current_line += 1;
        }

        Ok(result)
    }

    async fn get_file_size(&self, path: &str) -> AppResult<u64> {
        let metadata = fs::metadata(path).await.map_err(|e| {
            AppError::file_system_error(
                "get_file_size",
                &format!("Failed to get file size for '{}': {}", path, e),
            )
        })?;
        Ok(metadata.len())
    }

    async fn file_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    async fn read_from_position(&self, path: &str, position: u64) -> AppResult<Vec<String>> {
        let file = fs::File::open(path).await.map_err(|e| {
            AppError::file_system_error(
                "read_from_position",
                &format!("Failed to open file '{}': {}", path, e),
            )
        })?;

        let mut reader = BufReader::new(file);
        reader
            .seek(std::io::SeekFrom::Start(position))
            .await
            .map_err(|e| {
                AppError::file_system_error(
                    "read_from_position",
                    &format!(
                        "Failed to seek to position {} in '{}': {}",
                        position, path, e
                    ),
                )
            })?;

        let mut lines = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line).await.map_err(|e| {
            AppError::file_system_error(
                "read_from_position",
                &format!("Failed to read line from '{}': {}", path, e),
            )
        })? > 0
        {
            lines.push(line.trim_end().to_string());
            line.clear();
        }

        Ok(lines)
    }

    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>> {
        let metadata = fs::metadata(path).await.map_err(|e| {
            AppError::file_system_error(
                "get_file_modified_time",
                &format!("Failed to get file metadata for '{}': {}", path, e),
            )
        })?;

        Ok(chrono::DateTime::<chrono::Utc>::from(
            metadata.modified().map_err(|e| {
                AppError::file_system_error(
                    "get_file_modified_time",
                    &format!("Failed to get modified time for '{}': {}", path, e),
                )
            })?,
        ))
    }
}
