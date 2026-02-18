use crate::errors::{AppError, AppResult};
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::fs;

pub struct FileService;

impl FileService {
    pub async fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> AppResult<T> {
        let path = path.as_ref();
        let content = Self::read_file(path).await?;

        serde_json::from_str(&content).map_err(|e| {
            AppError::internal_error(
                "json_deserialize",
                &format!("Failed to deserialize JSON from {:?}: {}", path, e),
            )
        })
    }

    async fn read_file(path: &Path) -> AppResult<String> {
        fs::read_to_string(path).await.map_err(|e| {
            let operation = if e.kind() == std::io::ErrorKind::NotFound {
                "file_not_found"
            } else {
                "read_file"
            };
            AppError::file_system_error(
                operation,
                &format!("Failed to read file {:?}: {}", path, e),
            )
        })
    }
}
