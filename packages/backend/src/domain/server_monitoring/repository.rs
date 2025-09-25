use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringSession, ServerMonitoringStats, ServerStatus,
};
use crate::domain::server_monitoring::traits::{
    ServerInfoRepository, ServerMonitoringSessionRepository, ServerMonitoringStatsRepository,
    ServerStatusRepository,
};
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use log::debug;
use std::path::Path;
use tokio::fs;

pub struct ServerStatusRepositoryImpl {
    status_file_path: String,
}

impl ServerStatusRepositoryImpl {
    pub fn new(status_file_path: String) -> Self {
        Self { status_file_path }
    }
}

#[async_trait]
impl ServerStatusRepository for ServerStatusRepositoryImpl {
    async fn save_status(&self, status: &ServerStatus) -> AppResult<()> {
        if let Some(parent) = Path::new(&self.status_file_path).parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("Failed to create directory: {}", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(status).map_err(|e| {
            AppError::serialization_error("Failed to serialize server status: {}", &e.to_string())
        })?;

        fs::write(&self.status_file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write status file: {}", &e.to_string())
        })?;

        debug!("Server status saved to file");
        Ok(())
    }

    async fn load_status(&self) -> AppResult<Option<ServerStatus>> {
        if !Path::new(&self.status_file_path).exists() {
            debug!("No server status file found");
            return Ok(None);
        }

        let contents = fs::read_to_string(&self.status_file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read status file: {}", &e.to_string())
        })?;

        let status: ServerStatus = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse status file: {}", &e.to_string())
        })?;

        debug!("Loaded server status from file");
        Ok(Some(status))
    }

    async fn delete_status(&self) -> AppResult<()> {
        if Path::new(&self.status_file_path).exists() {
            fs::remove_file(&self.status_file_path).await.map_err(|e| {
                AppError::file_system_error("Failed to delete status file: {}", &e.to_string())
            })?;
            debug!("Server status file deleted");
        }
        Ok(())
    }

    async fn status_exists(&self) -> bool {
        Path::new(&self.status_file_path).exists()
    }
}

pub struct ServerInfoRepositoryImpl {
    info_file_path: String,
}

impl ServerInfoRepositoryImpl {
    pub fn new(info_file_path: String) -> Self {
        Self { info_file_path }
    }
}

#[async_trait]
impl ServerInfoRepository for ServerInfoRepositoryImpl {
    async fn save_server_info(&self, server_info: &ServerInfo) -> AppResult<()> {
        if let Some(parent) = Path::new(&self.info_file_path).parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("Failed to create directory: {}", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(server_info).map_err(|e| {
            AppError::serialization_error("Failed to serialize server info: {}", &e.to_string())
        })?;

        fs::write(&self.info_file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write server info file: {}", &e.to_string())
        })?;

        debug!("Server info saved to file");
        Ok(())
    }

    async fn load_server_info(&self) -> AppResult<Option<ServerInfo>> {
        if !Path::new(&self.info_file_path).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&self.info_file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read server info file: {}", &e.to_string())
        })?;

        let server_info: ServerInfo = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse server info file: {}", &e.to_string())
        })?;

        Ok(Some(server_info))
    }

    async fn update_server_info(&self, server_info: &ServerInfo) -> AppResult<()> {
        self.save_server_info(server_info).await
    }

    async fn delete_server_info(&self) -> AppResult<()> {
        if Path::new(&self.info_file_path).exists() {
            fs::remove_file(&self.info_file_path).await.map_err(|e| {
                AppError::file_system_error("Failed to delete server info file: {}", &e.to_string())
            })?;
            debug!("Server info file deleted");
        }
        Ok(())
    }
}

pub struct ServerMonitoringSessionRepositoryImpl {
    sessions_dir: String,
}

impl ServerMonitoringSessionRepositoryImpl {
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
impl ServerMonitoringSessionRepository for ServerMonitoringSessionRepositoryImpl {
    async fn save_session(&self, session: &ServerMonitoringSession) -> AppResult<()> {
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

        debug!("Saved server monitoring session: {}", session.session_id);
        Ok(())
    }

    async fn load_session(&self, session_id: &str) -> AppResult<Option<ServerMonitoringSession>> {
        let file_path = self.get_session_file_path(session_id);
        
        if !Path::new(&file_path).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read session file: {}", &e.to_string())
        })?;

        let session: ServerMonitoringSession = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse session file: {}", &e.to_string())
        })?;

        Ok(Some(session))
    }

    async fn get_active_session(&self) -> AppResult<Option<ServerMonitoringSession>> {
        let file_path = self.get_active_session_file_path();
        
        if !Path::new(&file_path).exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read active session file: {}", &e.to_string())
        })?;

        let session: ServerMonitoringSession = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse active session file: {}", &e.to_string())
        })?;

        Ok(Some(session))
    }

    async fn update_session(&self, session: &ServerMonitoringSession) -> AppResult<()> {
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

    async fn get_all_sessions(&self) -> AppResult<Vec<ServerMonitoringSession>> {
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

pub struct ServerMonitoringStatsRepositoryImpl {
    stats_file_path: String,
}

impl ServerMonitoringStatsRepositoryImpl {
    pub fn new(stats_file_path: String) -> Self {
        Self { stats_file_path }
    }
}

#[async_trait]
impl ServerMonitoringStatsRepository for ServerMonitoringStatsRepositoryImpl {
    async fn save_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()> {
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

        debug!("Saved server monitoring statistics");
        Ok(())
    }

    async fn load_stats(&self) -> AppResult<ServerMonitoringStats> {
        if !Path::new(&self.stats_file_path).exists() {
            return Ok(ServerMonitoringStats::default());
        }

        let contents = fs::read_to_string(&self.stats_file_path).await.map_err(|e| {
            AppError::file_system_error("Failed to read stats file: {}", &e.to_string())
        })?;

        let stats: ServerMonitoringStats = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse stats file: {}", &e.to_string())
        })?;

        Ok(stats)
    }

    async fn update_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()> {
        self.save_stats(stats).await
    }

    async fn increment_ping_count(&self, success: bool) -> AppResult<()> {
        let mut stats = self.load_stats().await?;
        
        stats.total_pings += 1;
        if success {
            stats.successful_pings += 1;
        } else {
            stats.failed_pings += 1;
        }
        
        stats.last_monitoring_time = chrono::Utc::now();
        
        self.update_stats(&stats).await
    }

    async fn update_average_latency(&self, latency_ms: u64) -> AppResult<()> {
        let mut stats = self.load_stats().await?;
        
        if let Some(current_avg) = stats.average_latency_ms {
            stats.average_latency_ms = Some((current_avg + latency_ms as f64) / 2.0);
        } else {
            stats.average_latency_ms = Some(latency_ms as f64);
        }
        
        self.update_stats(&stats).await
    }

    async fn reset_stats(&self) -> AppResult<()> {
        let stats = ServerMonitoringStats::default();
        self.save_stats(&stats).await
    }
}
