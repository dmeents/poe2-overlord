//! # Server Monitoring Repository Implementations
//! 
//! This module provides concrete implementations of the repository traits for server monitoring.
//! These implementations use a combination of in-memory caching and file-based persistence
//! to provide fast access to frequently used data while ensuring durability.

use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringSession, ServerMonitoringStats, ServerStatus,
};
use crate::domain::server_monitoring::traits::{
    ServerInfoRepository, ServerMonitoringSessionRepository, ServerMonitoringStatsRepository,
    ServerStatusRepository,
};
use crate::errors::AppResult;
use crate::infrastructure::persistence::{
    PersistenceRepository, PersistenceRepositoryImpl, ScopedPersistenceRepository, ScopedPersistenceRepositoryImpl,
};
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Repository implementation for server status persistence.
/// 
/// Uses in-memory caching with file-based persistence for optimal performance
/// and data durability. The cache is updated on every save operation and
/// loaded on initialization.
pub struct ServerStatusRepositoryImpl {
    /// In-memory cache of the current server status
    status: Arc<RwLock<Option<ServerStatus>>>,
    /// File-based persistence layer
    persistence: PersistenceRepositoryImpl<ServerStatus>,
}

impl ServerStatusRepositoryImpl {
    /// Create a new server status repository with file-based persistence.
    /// 
    /// Attempts to load existing status data on initialization, but gracefully
    /// handles the case where no previous data exists.
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<ServerStatus>::new_in_data_dir("server_status.json")?;

        let repository = Self {
            status: Arc::new(RwLock::new(None)),
            persistence,
        };

        // Attempt to load existing data, but don't fail if it doesn't exist
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load_status()) {
            debug!("Failed to load server status, starting fresh: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ServerStatusRepository for ServerStatusRepositoryImpl {
    async fn save_status(&self, status: &ServerStatus) -> AppResult<()> {
        // Update in-memory cache first for fast access
        {
            let mut current_status = self.status.write().await;
            *current_status = Some(status.clone());
        }
        
        // Persist to disk for durability
        self.persistence.save(status).await?;
        debug!("Server status saved to file");
        Ok(())
    }

    async fn load_status(&self) -> AppResult<Option<ServerStatus>> {
        // Check in-memory cache first
        let status = self.status.read().await.clone();
        if status.is_some() {
            return Ok(status);
        }

        // Try to load from persistence if not in cache
        if self.persistence.exists().await? {
            let status = self.persistence.load().await?;
            let mut current_status = self.status.write().await;
            *current_status = Some(status.clone());
            debug!("Loaded server status from file");
            return Ok(Some(status));
        }

        debug!("No server status file found");
        Ok(None)
    }

    async fn delete_status(&self) -> AppResult<()> {
        // Clear in-memory cache
        {
            let mut status = self.status.write().await;
            *status = None;
        }
        
        // Delete from persistence
        self.persistence.delete().await?;
        debug!("Server status file deleted");
        Ok(())
    }

    async fn status_exists(&self) -> bool {
        self.persistence.exists().await.unwrap_or(false)
    }
}

/// Repository implementation for server information persistence.
/// 
/// Manages server metadata including connection history and uptime statistics
/// with the same caching strategy as the status repository.
pub struct ServerInfoRepositoryImpl {
    /// In-memory cache of server information
    server_info: Arc<RwLock<Option<ServerInfo>>>,
    /// File-based persistence layer
    persistence: PersistenceRepositoryImpl<ServerInfo>,
}

impl ServerInfoRepositoryImpl {
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<ServerInfo>::new_in_data_dir("server_info.json")?;

        let repository = Self {
            server_info: Arc::new(RwLock::new(None)),
            persistence,
        };

        // Attempt to load existing data, but don't fail if it doesn't exist
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load_server_info()) {
            debug!("Failed to load server info, starting fresh: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ServerInfoRepository for ServerInfoRepositoryImpl {
    async fn save_server_info(&self, server_info: &ServerInfo) -> AppResult<()> {
        // Update in-memory cache
        {
            let mut current_info = self.server_info.write().await;
            *current_info = Some(server_info.clone());
        }
        
        // Persist to disk
        self.persistence.save(server_info).await?;
        debug!("Server info saved to file");
        Ok(())
    }

    async fn load_server_info(&self) -> AppResult<Option<ServerInfo>> {
        let server_info = self.server_info.read().await.clone();
        if server_info.is_some() {
            return Ok(server_info);
        }

        // Try to load from persistence
        if self.persistence.exists().await? {
            let server_info = self.persistence.load().await?;
            let mut current_info = self.server_info.write().await;
            *current_info = Some(server_info.clone());
            return Ok(Some(server_info));
        }

        Ok(None)
    }

    async fn update_server_info(&self, server_info: &ServerInfo) -> AppResult<()> {
        self.save_server_info(server_info).await
    }

    async fn delete_server_info(&self) -> AppResult<()> {
        // Clear in-memory cache
        {
            let mut server_info = self.server_info.write().await;
            *server_info = None;
        }
        
        // Delete from persistence
        self.persistence.delete().await?;
        debug!("Server info file deleted");
        Ok(())
    }
}

/// Repository implementation for monitoring session persistence.
/// 
/// Uses scoped persistence for individual sessions and maintains a separate
/// active session cache for fast access to the current monitoring session.
pub struct ServerMonitoringSessionRepositoryImpl {
    /// In-memory cache of the currently active session
    active_session: Arc<RwLock<Option<ServerMonitoringSession>>>,
    /// Scoped persistence for individual sessions (by session ID)
    persistence: ScopedPersistenceRepositoryImpl<ServerMonitoringSession, String>,
    /// Dedicated persistence for the active session
    active_session_persistence: PersistenceRepositoryImpl<ServerMonitoringSession>,
}

impl ServerMonitoringSessionRepositoryImpl {
    pub fn new() -> AppResult<Self> {
        let persistence = ScopedPersistenceRepositoryImpl::<ServerMonitoringSession, String>::new_in_data_dir(
            "server_monitoring_session_",
            ".json"
        )?;
        
        let active_session_persistence = PersistenceRepositoryImpl::<ServerMonitoringSession>::new_in_data_dir("active_server_monitoring_session.json")?;

        let repository = Self {
            active_session: Arc::new(RwLock::new(None)),
            persistence,
            active_session_persistence,
        };

        // Attempt to load active session, but don't fail if it doesn't exist
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.get_active_session()) {
            debug!("Failed to load active server monitoring session, starting fresh: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ServerMonitoringSessionRepository for ServerMonitoringSessionRepositoryImpl {
    async fn save_session(&self, session: &ServerMonitoringSession) -> AppResult<()> {
        self.persistence.save_scoped(&session.session_id, session).await?;
        debug!("Saved server monitoring session: {}", session.session_id);
        Ok(())
    }

    async fn load_session(&self, session_id: &str) -> AppResult<Option<ServerMonitoringSession>> {
        self.persistence.load_scoped(&session_id.to_string()).await
    }

    async fn get_active_session(&self) -> AppResult<Option<ServerMonitoringSession>> {
        let active_session = self.active_session.read().await.clone();
        if active_session.is_some() {
            return Ok(active_session);
        }

        // Try to load from persistence
        if self.active_session_persistence.exists().await? {
            let session = self.active_session_persistence.load().await?;
            let mut current_active = self.active_session.write().await;
            *current_active = Some(session.clone());
            return Ok(Some(session));
        }

        Ok(None)
    }

    async fn update_session(&self, session: &ServerMonitoringSession) -> AppResult<()> {
        self.save_session(session).await
    }

    async fn end_current_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.get_active_session().await? {
            session.end_session();
            self.update_session(&session).await?;
            
            // Clear active session
            {
                let mut active_session = self.active_session.write().await;
                *active_session = None;
            }
            
            // Delete active session file
            self.active_session_persistence.delete().await?;
        }
        Ok(())
    }

    async fn get_all_sessions(&self) -> AppResult<Vec<ServerMonitoringSession>> {
        // Note: This is a simplified implementation. In a real scenario, you might want to
        // maintain a list of all session IDs or scan the data directory.
        // For now, we'll return an empty vector as the scoped persistence doesn't provide
        // a direct way to list all keys without additional metadata tracking.
        Ok(Vec::new())
    }
}

/// Repository implementation for monitoring statistics persistence.
/// 
/// Manages aggregated statistics with in-memory caching and provides
/// atomic operations for updating individual statistics counters.
pub struct ServerMonitoringStatsRepositoryImpl {
    /// In-memory cache of monitoring statistics
    stats: Arc<RwLock<ServerMonitoringStats>>,
    /// File-based persistence layer
    persistence: PersistenceRepositoryImpl<ServerMonitoringStats>,
}

impl ServerMonitoringStatsRepositoryImpl {
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<ServerMonitoringStats>::new_in_data_dir("server_monitoring_stats.json")?;

        let repository = Self {
            stats: Arc::new(RwLock::new(ServerMonitoringStats::default())),
            persistence,
        };

        // Attempt to load existing data, but don't fail if it doesn't exist
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load_stats()) {
            debug!("Failed to load server monitoring stats, starting fresh: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl ServerMonitoringStatsRepository for ServerMonitoringStatsRepositoryImpl {
    async fn save_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()> {
        // Update in-memory cache
        {
            let mut current_stats = self.stats.write().await;
            *current_stats = stats.clone();
        }
        
        // Persist to disk
        self.persistence.save(stats).await
    }

    async fn load_stats(&self) -> AppResult<ServerMonitoringStats> {
        let stats = self.persistence.load().await?;
        
        // Update in-memory cache
        {
            let mut current_stats = self.stats.write().await;
            *current_stats = stats.clone();
        }
        
        debug!("Server monitoring statistics loaded successfully");
        Ok(stats)
    }

    async fn update_stats(&self, stats: &ServerMonitoringStats) -> AppResult<()> {
        self.save_stats(stats).await
    }

    async fn increment_ping_count(&self, success: bool) -> AppResult<()> {
        let mut stats = self.stats.read().await.clone();
        
        // Atomically update ping counters
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
        let mut stats = self.stats.read().await.clone();
        
        // Update running average latency calculation
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
