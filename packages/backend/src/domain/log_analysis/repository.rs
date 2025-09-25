use crate::domain::log_analysis::models::{
    LogAnalysisSession, LogAnalysisStats, LogFileInfo,
};
use crate::domain::log_analysis::traits::{
    LogAnalysisSessionRepository, LogAnalysisStatsRepository, LogFileRepository,
};
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use log::{debug, warn};
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};

pub struct LogFileRepositoryImpl {
    base_path: String,
}

impl LogFileRepositoryImpl {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl LogFileRepository for LogFileRepositoryImpl {
    async fn get_file_info(&self, path: &str) -> AppResult<LogFileInfo> {
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };

        let path_buf = PathBuf::from(&full_path);
        let exists = path_buf.exists();

        if !exists {
            return Ok(LogFileInfo {
                path: path_buf,
                size: 0,
                last_modified: chrono::Utc::now(),
                exists: false,
            });
        }

        let metadata = fs::metadata(&path_buf).await.map_err(|e| {
            AppError::file_system_error("Failed to get file metadata: {}", &e.to_string())
        })?;

        let modified_time = metadata
            .modified()
            .map_err(|e| {
                AppError::file_system_error("Failed to get file modification time: {}", &e.to_string())
            })?
            .into();

        Ok(LogFileInfo {
            path: path_buf,
            size: metadata.len(),
            last_modified: modified_time,
            exists: true,
        })
    }

    async fn read_lines(&self, path: &str, start_line: usize, count: usize) -> AppResult<Vec<String>> {
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

    async fn get_file_size(&self, path: &str) -> AppResult<u64> {
        let file_info = self.get_file_info(path).await?;
        Ok(file_info.size)
    }

    async fn file_exists(&self, path: &str) -> bool {
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", self.base_path, path)
        };
        Path::new(&full_path).exists()
    }

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

        file.seek(std::io::SeekFrom::Start(position)).await.map_err(|e| {
            AppError::file_system_error("Failed to seek in log file: {}", &e.to_string())
        })?;

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut line_buffer = String::new();

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

    async fn get_file_modified_time(&self, path: &str) -> AppResult<chrono::DateTime<chrono::Utc>> {
        let file_info = self.get_file_info(path).await?;
        Ok(file_info.last_modified)
    }
}

pub struct LogAnalysisSessionRepositoryImpl {
    sessions_dir: String,
}

impl LogAnalysisSessionRepositoryImpl {
    pub fn new(sessions_dir: String) -> Self {
        Self { sessions_dir }
    }

    fn get_session_file_path(&self, session_id: &str) -> String {
        format!("{}/session_{}.json", self.sessions_dir, session_id)
    }

    fn get_active_session_file_path(&self) -> String {
        format!("{}/active_session.json", self.sessions_dir)
    }
}

#[async_trait]
impl LogAnalysisSessionRepository for LogAnalysisSessionRepositoryImpl {
    async fn save_session(&self, session: &LogAnalysisSession) -> AppResult<()> {
        let file_path = self.get_session_file_path(&session.session_id);
        
        if let Some(parent) = Path::new(&file_path).parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("Failed to create sessions directory: {}", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(session).map_err(|e| {
            AppError::serialization_error("Failed to serialize session: {}", &e.to_string())
        })?;

        fs::write(&file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write session file: {}", &e.to_string())
        })?;

        debug!("Saved log analysis session: {}", session.session_id);
        Ok(())
    }

    async fn load_session(&self, session_id: &str) -> AppResult<Option<LogAnalysisSession>> {
        let file_path = self.get_session_file_path(session_id);
        
        if !Path::new(&file_path).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read session file: {}", &e.to_string())
        })?;

        let session: LogAnalysisSession = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse session file: {}", &e.to_string())
        })?;

        Ok(Some(session))
    }

    async fn get_active_session(&self) -> AppResult<Option<LogAnalysisSession>> {
        let file_path = self.get_active_session_file_path();
        
        if !Path::new(&file_path).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read active session file: {}", &e.to_string())
        })?;

        let session: LogAnalysisSession = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse active session file: {}", &e.to_string())
        })?;

        Ok(Some(session))
    }

    async fn update_session(&self, session: &LogAnalysisSession) -> AppResult<()> {
        self.save_session(session).await
    }

    async fn end_current_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.get_active_session().await? {
            session.end_session();
            self.update_session(&session).await?;
            
            let active_file_path = self.get_active_session_file_path();
            if Path::new(&active_file_path).exists() {
                fs::remove_file(&active_file_path).await.map_err(|e| {
                    AppError::file_system_error("Failed to remove active session file: {}", &e.to_string())
                })?;
            }
        }
        Ok(())
    }

    async fn get_all_sessions(&self) -> AppResult<Vec<LogAnalysisSession>> {
        let mut sessions = Vec::new();
        
        if !Path::new(&self.sessions_dir).exists() {
            return Ok(sessions);
        }

        let mut entries = fs::read_dir(&self.sessions_dir).await.map_err(|e| {
            AppError::file_system_error("Failed to read sessions directory: {}", &e.to_string())
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::file_system_error("Failed to read directory entry: {}", &e.to_string())
        })? {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(session_id) = path.file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.strip_prefix("session_"))
                {
                    if let Ok(Some(session)) = self.load_session(session_id).await {
                        sessions.push(session);
                    }
                }
            }
        }

        sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));
        Ok(sessions)
    }
}

pub struct LogAnalysisStatsRepositoryImpl {
    stats_file_path: String,
}

impl LogAnalysisStatsRepositoryImpl {
    pub fn new(stats_file_path: String) -> Self {
        Self { stats_file_path }
    }
}

#[async_trait]
impl LogAnalysisStatsRepository for LogAnalysisStatsRepositoryImpl {
    async fn save_stats(&self, stats: &LogAnalysisStats) -> AppResult<()> {
        if let Some(parent) = Path::new(&self.stats_file_path).parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("Failed to create stats directory: {}", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(stats).map_err(|e| {
            AppError::serialization_error("Failed to serialize stats: {}", &e.to_string())
        })?;

        fs::write(&self.stats_file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write stats file: {}", &e.to_string())
        })?;

        debug!("Saved log analysis statistics");
        Ok(())
    }

    async fn load_stats(&self) -> AppResult<LogAnalysisStats> {
        if !Path::new(&self.stats_file_path).exists() {
            return Ok(LogAnalysisStats::default());
        }

        let contents = fs::read_to_string(&self.stats_file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read stats file: {}", &e.to_string())
        })?;

        let stats: LogAnalysisStats = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse stats file: {}", &e.to_string())
        })?;

        Ok(stats)
    }

    async fn update_stats(&self, stats: &LogAnalysisStats) -> AppResult<()> {
        self.save_stats(stats).await
    }

    async fn increment_event_count(&self, event_type: &str) -> AppResult<()> {
        let mut stats = self.load_stats().await?;
        
        match event_type {
            "scene_change" => stats.scene_changes_detected += 1,
            "server_connection" => stats.server_connections_detected += 1,
            "character_level_up" => stats.character_level_ups_detected += 1,
            "character_death" => stats.character_deaths_detected += 1,
            _ => {
                warn!("Unknown event type for statistics: {}", event_type);
            }
        }
        
        stats.total_events_processed += 1;
        stats.last_analysis_time = chrono::Utc::now();
        
        self.update_stats(&stats).await
    }

    async fn reset_stats(&self) -> AppResult<()> {
        let stats = LogAnalysisStats::default();
        self.save_stats(&stats).await
    }
}
