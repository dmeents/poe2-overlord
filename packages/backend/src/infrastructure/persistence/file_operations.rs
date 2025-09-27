use crate::errors::{AppError, AppResult};
use log::debug;
use std::fs;
use std::path::Path;

pub struct FileOperations;

impl FileOperations {
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }

    pub fn get_file_size<P: AsRef<Path>>(path: P) -> AppResult<u64> {
        let metadata = fs::metadata(path).map_err(|e| {
            AppError::file_system_error(
                "get_file_size",
                &format!("Failed to get file size: {}", e),
            )
        })?;
        Ok(metadata.len())
    }


    pub fn read_file_content<P: AsRef<Path>>(path: P) -> AppResult<String> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(AppError::file_system_error(
                "read_file_content",
                &format!("File not found: {:?}", path),
            ));
        }

        fs::read_to_string(path).map_err(|e| {
            AppError::file_system_error(
                "read_file",
                &format!("Failed to read file {:?}: {}", path, e),
            )
        })
    }

    pub fn write_file_content<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
        let path = path.as_ref();

        fs::write(path, content).map_err(|e| {
            AppError::file_system_error(
                "write_file",
                &format!("Failed to write file {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully wrote file: {:?}", path);
        Ok(())
    }

    pub fn delete_file<P: AsRef<Path>>(path: P) -> AppResult<()> {
        let path = path.as_ref();

        if !path.exists() {
            debug!("File does not exist, nothing to delete: {:?}", path);
            return Ok(());
        }

        fs::remove_file(path).map_err(|e| {
            AppError::file_system_error(
                "delete_file",
                &format!("Failed to delete file {:?}: {}", path, e),
            )
        })?;

        debug!("Successfully deleted file: {:?}", path);
        Ok(())
    }

}
