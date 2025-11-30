use crate::errors::{AppError, AppResult};
use crate::infrastructure::file_management::traits::FileOperations;
use async_trait::async_trait;
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
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

    pub async fn read_json_optional<T: DeserializeOwned>(
        path: impl AsRef<Path>,
    ) -> AppResult<Option<T>> {
        let path = path.as_ref();

        match fs::try_exists(path).await {
            Ok(true) => Self::read_json(path).await.map(Some),
            Ok(false) => Ok(None),
            Err(e) => Err(AppError::file_system_error(
                "read_json_optional",
                &format!("Failed to check if file exists {:?}: {}", path, e),
            )),
        }
    }

    pub async fn write_json<T: Serialize>(path: impl AsRef<Path>, data: &T) -> AppResult<()> {
        let path = path.as_ref();

        let json_content = serde_json::to_string_pretty(data).map_err(|e| {
            AppError::internal_error(
                "json_serialize",
                &format!("Failed to serialize data: {}", e),
            )
        })?;

        Self::write(path, &json_content).await
    }

    pub async fn delete(path: impl AsRef<Path>) -> AppResult<()> {
        let path = path.as_ref();

        match fs::remove_file(path).await {
            Ok(_) => {
                debug!("Deleted file: {:?}", path);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AppError::file_system_error(
                "delete_file",
                &format!("Failed to delete file {:?}: {}", path, e),
            )),
        }
    }

    pub async fn exists(path: impl AsRef<Path>) -> AppResult<bool> {
        fs::try_exists(path.as_ref()).await.map_err(|e| {
            AppError::file_system_error("exists", &format!("Failed to check existence: {}", e))
        })
    }

    pub async fn ensure_parent_dir(path: impl AsRef<Path>) -> AppResult<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error(
                    "ensure_parent_dir",
                    &format!("Failed to create parent directory {:?}: {}", parent, e),
                )
            })?;
        }

        Ok(())
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

    /// Uses write-temp-rename pattern to prevent corruption
    async fn write(path: &Path, content: &str) -> AppResult<()> {
        let temp_path = Self::get_temp_path(path);

        Self::ensure_parent_dir(path).await?;

        fs::write(&temp_path, content).await.map_err(|e| {
            AppError::file_system_error(
                "write_temp_file",
                &format!("Failed to write to temporary file {:?}: {}", temp_path, e),
            )
        })?;

        fs::rename(&temp_path, path).await.map_err(|e| {
            let _ = std::fs::remove_file(&temp_path);
            AppError::file_system_error(
                "rename_temp_file",
                &format!(
                    "Failed to rename temp file {:?} to {:?}: {}",
                    temp_path, path, e
                ),
            )
        })?;

        debug!("Wrote file atomically: {:?}", path);
        Ok(())
    }

    fn get_temp_path(path: &Path) -> std::path::PathBuf {
        let mut temp_path = path.to_path_buf();
        let mut extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        if !extension.is_empty() {
            extension.push_str(".tmp");
        } else {
            extension = "tmp".to_string();
        }

        temp_path.set_extension(extension);
        temp_path
    }
}

#[async_trait]
impl FileOperations for FileService {
    async fn read_json<T: DeserializeOwned + Send>(&self, path: &Path) -> AppResult<T> {
        Self::read_json(path).await
    }

    async fn read_json_optional<T: DeserializeOwned + Send>(
        &self,
        path: &Path,
    ) -> AppResult<Option<T>> {
        Self::read_json_optional(path).await
    }

    async fn write_json<T: Serialize + Sync>(&self, path: &Path, data: &T) -> AppResult<()> {
        Self::write_json(path, data).await
    }

    async fn delete(&self, path: &Path) -> AppResult<()> {
        Self::delete(path).await
    }

    async fn exists(&self, path: &Path) -> AppResult<bool> {
        Self::exists(path).await
    }

    async fn ensure_parent_dir(&self, path: &Path) -> AppResult<()> {
        Self::ensure_parent_dir(path).await
    }
}
