use crate::errors::{AppError, AppResult};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek};
use std::path::Path;

/// Log file watcher that watches for file changes and provides file I/O operations
pub struct LogFileWatcher {
    pub log_path: String,
}

impl LogFileWatcher {
    /// Create a new log file watcher for the specified log path
    pub fn new(log_path: String) -> Self {
        Self { log_path }
    }

    /// Get the current log file size
    pub fn get_log_file_size(&self) -> AppResult<u64> {
        let path = Path::new(&self.log_path);

        if !path.exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                self.log_path
            )));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to get file metadata: {}", e)))?;
        Ok(metadata.len())
    }

    /// Read the last N lines from the log file
    pub fn read_last_lines(&self, count: usize) -> AppResult<Vec<String>> {
        let path = Path::new(&self.log_path);

        if !path.exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                self.log_path
            )));
        }

        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to open log file: {}", e)))?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

        let start = if lines.len() > count {
            lines.len() - count
        } else {
            0
        };

        Ok(lines[start..].to_vec())
    }

    /// Process new lines from the log file starting from the last known position
    pub async fn process_new_lines<F>(
        &self,
        last_position: &mut u64,
        line_processor: F,
    ) -> AppResult<()>
    where
        F: Fn(&str) + Send + Sync,
    {
        let path = Path::new(&self.log_path);
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to open log file: {}", e)))?;

        let mut reader = BufReader::new(file);

        // Seek to last known position
        reader
            .seek(io::SeekFrom::Start(*last_position))
            .map_err(|e| AppError::FileSystem(format!("Failed to seek in log file: {}", e)))?;

        for line in reader.lines() {
            let line =
                line.map_err(|e| AppError::FileSystem(format!("Failed to read line: {}", e)))?;
            
            line_processor(&line);
        }

        // Update position to current file size
        *last_position = self.get_log_file_size()?;

        Ok(())
    }

    /// Check if the log file exists
    pub fn file_exists(&self) -> bool {
        Path::new(&self.log_path).exists()
    }

    /// Get the log path
    pub fn get_log_path(&self) -> &str {
        &self.log_path
    }
}
