use crate::errors::{AppError, AppResult};
use log::debug;
use std::path::PathBuf;
use tokio::fs;

pub struct AppPaths;

impl AppPaths {
    const APP_NAME: &'static str = "poe2-overlord";

    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(Self::APP_NAME)
    }

    pub async fn ensure_dir(path: impl AsRef<std::path::Path>) -> AppResult<()> {
        let path = path.as_ref();

        match fs::metadata(path).await {
            Ok(metadata) if metadata.is_dir() => {
                debug!("Directory already exists: {:?}", path);
                Ok(())
            }
            Ok(_) => Err(AppError::file_system_error(
                "ensure_dir",
                &format!("Path exists but is not a directory: {:?}", path),
            )),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir_all(path).await.map_err(|e| {
                    AppError::file_system_error(
                        "ensure_dir",
                        &format!("Failed to create directory {:?}: {}", path, e),
                    )
                })?;
                debug!("Created directory: {:?}", path);
                Ok(())
            }
            Err(e) => Err(AppError::file_system_error(
                "ensure_dir",
                &format!("Failed to check directory {:?}: {}", path, e),
            )),
        }
    }

    pub async fn ensure_config_dir() -> AppResult<PathBuf> {
        let config_dir = Self::config_dir();
        Self::ensure_dir(&config_dir).await?;
        Ok(config_dir)
    }

    pub async fn ensure_data_dir() -> AppResult<PathBuf> {
        let data_dir = Self::data_dir();
        Self::ensure_dir(&data_dir).await?;
        Ok(data_dir)
    }

    pub async fn ensure_cache_dir() -> AppResult<PathBuf> {
        let cache_dir = Self::cache_dir();
        Self::ensure_dir(&cache_dir).await?;
        Ok(cache_dir)
    }
}
