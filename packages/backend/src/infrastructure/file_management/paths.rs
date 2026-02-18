use crate::errors::{AppError, AppResult};
use log::debug;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Expands tilde (~) in paths to the user's home directory
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    if let Some(stripped) = path_str.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    } else if path_str == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }

    path.to_path_buf()
}

pub struct AppPaths;

impl AppPaths {
    const APP_NAME: &'static str = "poe2-overlord";

    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
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

    pub async fn ensure_data_dir() -> AppResult<PathBuf> {
        let data_dir = Self::data_dir();
        Self::ensure_dir(&data_dir).await?;
        Ok(data_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde_with_path() {
        let path = "~/test/file.txt";
        let expanded = expand_tilde(path);

        assert!(!expanded.to_string_lossy().starts_with("~"));
        assert!(expanded.to_string_lossy().ends_with("test/file.txt"));
    }

    #[test]
    fn test_expand_tilde_alone() {
        let path = "~";
        let expanded = expand_tilde(path);

        assert_ne!(expanded.to_string_lossy(), "~");
    }

    #[test]
    fn test_expand_tilde_no_slash() {
        let path = "~test/file.txt";
        let expanded = expand_tilde(path);

        assert_eq!(expanded.to_string_lossy(), "~test/file.txt");
    }

    #[test]
    fn test_expand_tilde_absolute_path() {
        let path = "/absolute/path/file.txt";
        let expanded = expand_tilde(path);

        assert_eq!(expanded.to_string_lossy(), "/absolute/path/file.txt");
    }
}
