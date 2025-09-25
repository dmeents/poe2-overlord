use crate::domain::log_analysis::models::{LogAnalysisSession, LogAnalysisStats, LogFileInfo};
use crate::domain::log_analysis::traits::{
    LogAnalysisSessionRepository, LogAnalysisStatsRepository, LogFileRepository,
};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{
    PersistenceRepository, PersistenceRepositoryImpl, ScopedPersistenceRepository,
    ScopedPersistenceRepositoryImpl,
};
use async_trait::async_trait;
use log::{debug, warn};
use std::path::Path;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};
use tokio::sync::RwLock;

/// Implementation of LogFileRepository for file system operations
/// Handles reading and accessing log files from the file system
pub struct LogFileRepositoryImpl {
    /// Base path for resolving relative log file paths
    base_path: String,
}

impl LogFileRepositoryImpl {
    /// Creates a new LogFileRepositoryImpl with the specified base path
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl LogFileRepository for LogFileRepositoryImpl {
    /// Gets metadata information about a log file
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo> {
        // Resolve the full path (absolute or relative to base_path)
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };

        let path_buf = PathBuf::from(&full_path);
        let exists = path_buf.exists();

        // Return file info even if file doesn't exist
        if !exists {
            return Ok(LogFileInfo {
                path: path_buf,
                size: 0,
                last_modified: chrono::Utc::now(),
                exists: false,
            });
        }

        // Get file metadata for existing files
        let metadata = fs::metadata(&path_buf).await.map_err(|e| {
            AppError::file_system_error("Failed to get file metadata: {}", &e.to_string())
        })?;

        let modified_time = metadata
            .modified()
            .map_err(|e| {
                AppError::file_system_error(
                    "Failed to get file modification time: {}",
                    &e.to_string(),
                )
            })?
            .into();

        Ok(LogFileInfo {
            path: path_buf,
            size: metadata.len(),
            last_modified: modified_time,
            exists: true,
        })
    }

    /// Reads a specified number of lines from a log file starting at a given line number
    async fn read_lines(
        &self,
        path: &str,
        start_line: usize,
        count: usize,
    ) -> AppResult<Vec<String>> {
        // Resolve the full path (absolute or relative to base_path)
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };

        if !Path::new(&full_path).exists() {
            return Err(AppError::file_system_error(
                "Log file not found: {}",
                &full_path,
            ));
        }

        let file = fs::File::open(&full_path).await.map_err(|e| {
            AppError::file_system_error("Failed to open log file: {}", &e.to_string())
        })?;

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut current_line = 0;
        let mut line_buffer = String::new();

        // Skip lines until we reach the start_line
        while current_line < start_line {
            match reader.read_line(&mut line_buffer).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    current_line += 1;
                    line_buffer.clear();
                }
                Err(e) => {
                    return Err(AppError::file_system_error(
                        "Failed to read line: {}",
                        &e.to_string(),
                    ));
                }
            }
        }

        // Read the requested number of lines
        for _ in 0..count {
            line_buffer.clear();
            match reader.read_line(&mut line_buffer).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    lines.push(line_buffer.trim_end().to_string());
                }
                Err(e) => {
                    return Err(AppError::file_system_error(
                        "Failed to read line: {}",
                        &e.to_string(),
                    ));
                }
            }
        }

        Ok(lines)
    }

    /// Gets the current size of a log file in bytes
    async fn get_file_size(&self, path: &str) -> AppResult<u64> {
        let file_info = self.get_file_info(path).await?;
        Ok(file_info.size)
    }

    /// Checks whether a log file exists at the specified path
    async fn file_exists(&self, path: &str) -> bool {
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };
        Path::new(&full_path).exists()
    }

    /// Reads all lines from a log file starting at a specific byte position
    /// Used for monitoring new log entries efficiently
    async fn read_from_position(&self, path: &str, position: u64) -> AppResult<Vec<String>> {
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };

        if !Path::new(&full_path).exists() {
            return Err(AppError::file_system_error(
                "Log file not found: {}",
                &full_path,
            ));
        }

        let mut file = fs::File::open(&full_path).await.map_err(|e| {
            AppError::file_system_error("Failed to open log file: {}", &e.to_string())
        })?;

        // Seek to the specified position
        file.seek(std::io::SeekFrom::Start(position))
            .await
            .map_err(|e| {
                AppError::file_system_error("Failed to seek in log file: {}", &e.to_string())
            })?;

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut line_buffer = String::new();

        // Read all lines from the current position to EOF
        loop {
            line_buffer.clear();
            match reader.read_line(&mut line_buffer).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    lines.push(line_buffer.trim_end().to_string());
                }
                Err(e) => {
                    return Err(AppError::file_system_error(
                        "Failed to read line: {}",
                        &e.to_string(),
                    ));
                }
            }
        }

        Ok(lines)
    }

    /// Gets the last modified time of a log file
    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>> {
        let file_info = self.get_file_info(path).await?;
        Ok(file_info.last_modified)
    }
}

/// Implementation of LogAnalysisSessionRepository for managing log analysis sessions
/// Handles persistence and retrieval of monitoring session data
pub struct LogAnalysisSessionRepositoryImpl {
    /// In-memory cache of the currently active session
    active_session: Arc<RwLock<Option<LogAnalysisSession>>>,
    /// Repository for storing individual sessions by ID
    persistence: ScopedPersistenceRepositoryImpl<LogAnalysisSession, String>,
    /// Repository for storing the currently active session
    active_session_persistence: PersistenceRepositoryImpl<LogAnalysisSession>,
    /// Flag to track whether data has been loaded from disk
    data_loaded: Arc<AtomicBool>,
}

impl LogAnalysisSessionRepositoryImpl {
    /// Creates a new LogAnalysisSessionRepositoryImpl with persistence setup
    pub fn new() -> AppResult<Self> {
        let persistence =
            ScopedPersistenceRepositoryImpl::<LogAnalysisSession, String>::new_in_data_dir(
                "log_analysis_session_",
                ".json",
            )?;

        let active_session_persistence =
            PersistenceRepositoryImpl::<LogAnalysisSession>::new_in_data_dir(
                "active_log_analysis_session.json",
            )?;

        Ok(Self {
            active_session: Arc::new(RwLock::new(None)),
            persistence,
            active_session_persistence,
            data_loaded: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Ensures that data has been loaded from disk.
    ///
    /// This method checks if data has already been loaded and loads it if necessary.
    /// It's safe to call multiple times and will only load data once.
    ///
    /// # Returns
    /// * `Ok(())` - Data is loaded and ready
    /// * `Err(AppError)` - If data loading fails
    async fn ensure_data_loaded(&self) -> AppResult<()> {
        if !self.data_loaded.load(Ordering::Relaxed) {
            if let Err(e) = self.get_active_session().await {
                debug!(
                    "Failed to load active log analysis session, starting fresh: {}",
                    e
                );
                // Don't return error - allow repository to work with empty data
            }
        }
        Ok(())
    }
}

#[async_trait]
impl LogAnalysisSessionRepository for LogAnalysisSessionRepositoryImpl {
    /// Saves a log analysis session to persistent storage
    async fn save_session(&self, session: &LogAnalysisSession) -> AppResult<()> {
        self.persistence
            .save_scoped(&session.session_id, session)
            .await?;
        debug!("Saved log analysis session: {}", session.session_id);
        Ok(())
    }

    /// Loads a specific log analysis session by its ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<LogAnalysisSession>> {
        self.persistence.load_scoped(&session_id.to_string()).await
    }

    /// Gets the currently active log analysis session, if any
    async fn get_active_session(&self) -> AppResult<Option<LogAnalysisSession>> {
        self.ensure_data_loaded().await?;
        // Check in-memory cache first
        let active_session = self.active_session.read().await.clone();
        if active_session.is_some() {
            return Ok(active_session);
        }

        // Try to load from persistence
        if self.active_session_persistence.exists().await? {
            let session = self.active_session_persistence.load().await?;
            let mut current_active = self.active_session.write().await;
            *current_active = Some(session.clone());
            // Mark data as loaded
            self.data_loaded.store(true, Ordering::Relaxed);
            return Ok(Some(session));
        }

        // Mark data as loaded even if no data exists
        self.data_loaded.store(true, Ordering::Relaxed);
        Ok(None)
    }

    /// Updates an existing log analysis session
    async fn update_session(&self, session: &LogAnalysisSession) -> AppResult<()> {
        self.save_session(session).await
    }

    /// Ends the current active session and marks it as completed
    async fn end_current_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.get_active_session().await? {
            session.end_session();
            self.update_session(&session).await?;

            // Clear active session from memory
            {
                let mut active_session = self.active_session.write().await;
                *active_session = None;
            }

            // Delete active session file
            self.active_session_persistence.delete().await?;
        }
        Ok(())
    }

    /// Gets all stored log analysis sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<LogAnalysisSession>> {
        self.ensure_data_loaded().await?;
        // Note: This is a simplified implementation. In a real scenario, you might want to
        // maintain a list of all session IDs or scan the data directory.
        // For now, we'll return an empty vector as the scoped persistence doesn't provide
        // a direct way to list all keys.
        Ok(Vec::new())
    }
}

/// Implementation of LogAnalysisStatsRepository for managing log analysis statistics
/// Handles persistence and updates of analysis performance metrics
pub struct LogAnalysisStatsRepositoryImpl {
    /// In-memory cache of the current statistics
    stats: Arc<RwLock<LogAnalysisStats>>,
    /// Repository for persisting statistics to disk
    persistence: PersistenceRepositoryImpl<LogAnalysisStats>,
    /// Flag to track whether data has been loaded from disk
    data_loaded: Arc<AtomicBool>,
}

impl LogAnalysisStatsRepositoryImpl {
    /// Creates a new LogAnalysisStatsRepositoryImpl with persistence setup
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<LogAnalysisStats>::new_in_data_dir(
            "log_analysis_stats.json",
        )?;

        Ok(Self {
            stats: Arc::new(RwLock::new(LogAnalysisStats::default())),
            persistence,
            data_loaded: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Ensures that data has been loaded from disk.
    ///
    /// This method checks if data has already been loaded and loads it if necessary.
    /// It's safe to call multiple times and will only load data once.
    ///
    /// # Returns
    /// * `Ok(())` - Data is loaded and ready
    /// * `Err(AppError)` - If data loading fails
    async fn ensure_data_loaded(&self) -> AppResult<()> {
        if !self.data_loaded.load(Ordering::Relaxed) {
            if let Err(e) = self.load_stats().await {
                debug!("Failed to load log analysis stats, starting fresh: {}", e);
                // Don't return error - allow repository to work with empty data
            }
        }
        Ok(())
    }
}

#[async_trait]
impl LogAnalysisStatsRepository for LogAnalysisStatsRepositoryImpl {
    /// Saves log analysis statistics to persistent storage
    async fn save_stats(&self, stats: &LogAnalysisStats) -> AppResult<()> {
        // Update in-memory cache
        {
            let mut current_stats = self.stats.write().await;
            *current_stats = stats.clone();
        }

        // Persist to disk
        self.persistence.save(stats).await
    }

    /// Loads the current log analysis statistics
    async fn load_stats(&self) -> AppResult<LogAnalysisStats> {
        let stats = self.persistence.load().await?;

        // Update in-memory cache
        {
            let mut current_stats = self.stats.write().await;
            *current_stats = stats.clone();
        }

        // Mark data as loaded
        self.data_loaded.store(true, Ordering::Relaxed);
        debug!("Log analysis statistics loaded successfully");
        Ok(stats)
    }

    /// Updates the log analysis statistics
    async fn update_stats(&self, stats: &LogAnalysisStats) -> AppResult<()> {
        self.save_stats(stats).await
    }

    /// Increments the count for a specific event type
    async fn increment_event_count(&self, event_type: &str) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let mut stats = self.stats.read().await.clone();

        // Increment the appropriate counter based on event type
        match event_type {
            "scene_change" => stats.scene_changes_detected += 1,
            "server_connection" => stats.server_connections_detected += 1,
            "character_level_up" => stats.character_level_ups_detected += 1,
            "character_death" => stats.character_deaths_detected += 1,
            _ => {
                warn!("Unknown event type for statistics: {}", event_type);
            }
        }

        // Update total events and timestamp
        stats.total_events_processed += 1;
        stats.last_analysis_time = chrono::Utc::now();

        self.update_stats(&stats).await
    }

    /// Resets all statistics to their default values
    async fn reset_stats(&self) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let stats = LogAnalysisStats::default();
        self.save_stats(&stats).await
    }
}
