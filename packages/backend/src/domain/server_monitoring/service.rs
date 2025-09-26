//! # Server Monitoring Service Implementation
//!
//! This module provides the main service implementation for server monitoring functionality.
//! It orchestrates repository operations, manages monitoring sessions, handles periodic
//! ping operations, and provides real-time status updates through events.

use crate::domain::server_monitoring::models::{
    ServerInfo, ServerMonitoringConfig, ServerMonitoringSession, ServerMonitoringStats,
    ServerStatus,
};
use crate::domain::server_monitoring::repository::{
    ServerInfoRepositoryImpl, ServerMonitoringSessionRepositoryImpl,
    ServerMonitoringStatsRepositoryImpl, ServerStatusRepositoryImpl,
};
use crate::domain::server_monitoring::traits::{
    NetworkConnectivity, ServerInfoRepository, ServerMonitoringService,
    ServerMonitoringSessionRepository, ServerMonitoringStatsRepository, ServerStatusRepository,
};
use crate::domain::events::{AppEvent, EventBus, EventType};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::{broadcast, RwLock};
use tokio::time;

/// Main service implementation for server monitoring operations.
///
/// Orchestrates all server monitoring functionality including status tracking,
/// session management, periodic ping operations, and event broadcasting.
pub struct ServerMonitoringServiceImpl {
    /// Configuration settings for monitoring operations
    config: Arc<RwLock<ServerMonitoringConfig>>,
    /// Repository for server status persistence
    status_repository: Arc<dyn ServerStatusRepository>,
    /// Repository for server information persistence
    info_repository: Arc<dyn ServerInfoRepository>,
    /// Repository for monitoring session persistence
    session_repository: Arc<dyn ServerMonitoringSessionRepository>,
    /// Repository for monitoring statistics persistence
    stats_repository: Arc<dyn ServerMonitoringStatsRepository>,
    /// Network connectivity service for ping operations
    network_connectivity: Arc<dyn NetworkConnectivity>,
    /// Event bus for publishing monitoring events
    event_bus: Arc<EventBus>,
    /// Current server status cache
    current_status: Arc<RwLock<Option<ServerStatus>>>,
    /// Current monitoring session cache
    current_session: Arc<RwLock<Option<ServerMonitoringSession>>>,
    /// Flag indicating if periodic ping monitoring is active
    is_ping_monitoring_active: Arc<RwLock<bool>>,
    /// Broadcast channel for status change notifications
    status_change_sender: broadcast::Sender<ServerStatus>,
}

impl ServerMonitoringServiceImpl {
    pub fn new(event_bus: Arc<EventBus>) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(ServerMonitoringConfig::default()));

        let status_repository = Arc::new(ServerStatusRepositoryImpl::new()?);
        let info_repository = Arc::new(ServerInfoRepositoryImpl::new()?);
        let session_repository = Arc::new(ServerMonitoringSessionRepositoryImpl::new()?);
        let stats_repository = Arc::new(ServerMonitoringStatsRepositoryImpl::new()?);
        let network_connectivity = Arc::new(NetworkConnectivityImpl::new());

        let (status_change_sender, _) = broadcast::channel(100);

        Ok(Self {
            config,
            status_repository,
            info_repository,
            session_repository,
            stats_repository,
            network_connectivity,
            event_bus,
            current_status: Arc::new(RwLock::new(None)),
            current_session: Arc::new(RwLock::new(None)),
            is_ping_monitoring_active: Arc::new(RwLock::new(false)),
            status_change_sender,
        })
    }

    pub fn with_repositories(
        config: ServerMonitoringConfig,
        status_repository: Arc<dyn ServerStatusRepository>,
        info_repository: Arc<dyn ServerInfoRepository>,
        session_repository: Arc<dyn ServerMonitoringSessionRepository>,
        stats_repository: Arc<dyn ServerMonitoringStatsRepository>,
        network_connectivity: Arc<dyn NetworkConnectivity>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let (status_change_sender, _) = broadcast::channel(100);

        Self {
            config,
            status_repository,
            info_repository,
            session_repository,
            stats_repository,
            network_connectivity,
            event_bus,
            current_status: Arc::new(RwLock::new(None)),
            current_session: Arc::new(RwLock::new(None)),
            is_ping_monitoring_active: Arc::new(RwLock::new(false)),
            status_change_sender,
        }
    }

    /// Start a new monitoring session and persist it
    async fn start_monitoring_session(&self) -> AppResult<()> {
        let session = ServerMonitoringSession::new();
        self.session_repository.save_session(&session).await?;

        let mut current_session = self.current_session.write().await;
        *current_session = Some(session);

        info!("Started new server monitoring session");
        Ok(())
    }

    /// End the current monitoring session and update statistics
    async fn end_monitoring_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;

            // Update aggregated statistics with session data
            let mut stats = self.stats_repository.load_stats().await?;
            stats.total_sessions += 1;
            stats.total_pings += session.total_pings;
            stats.successful_pings += session.successful_pings;
            stats.failed_pings += session.failed_pings;
            self.stats_repository.update_stats(&stats).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = None;

            info!("Ended server monitoring session");
        }
        Ok(())
    }

    /// Update server information and associate it with the current session
    async fn update_server_info_internal(&self, ip_address: String, port: u16) -> AppResult<()> {
        let server_info = match self.info_repository.load_server_info().await? {
            Some(mut info) => {
                if info.ip_address == ip_address && info.port == port {
                    // Same server, just record another connection
                    info.record_connection();
                } else {
                    // Different server, create new info
                    info = ServerInfo::new(ip_address.clone(), port);
                }
                info
            }
            None => ServerInfo::new(ip_address.clone(), port),
        };

        self.info_repository.save_server_info(&server_info).await?;

        // Associate server info with current session if one exists
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.set_server_info(server_info);
            self.session_repository.update_session(&session).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = Some(session);
        }

        debug!("Updated server info: {}:{}", ip_address, port);
        Ok(())
    }

    /// Start a background task to emit server status events to the frontend
    pub async fn start_frontend_event_emission(&self, window: WebviewWindow) {
        let mut status_receiver = self.subscribe_to_status_changes().await.unwrap_or_else(|_| {
            // Create a dummy receiver if subscription fails
            let (_, receiver) = broadcast::channel(1);
            receiver
        });
        let window_clone = window.clone();

        tokio::spawn(async move {
            debug!("Server monitoring frontend event emission started");

            while let Ok(event) = status_receiver.recv().await {
                if let AppEvent::ServerStatusChanged { new_status, .. } = event {
                    Self::emit_server_status_event(&window_clone, &new_status);
                }
            }

            debug!("Server monitoring frontend event emission stopped");
        });
    }

    /// Emit a server status event to the frontend window
    fn emit_server_status_event(window: &WebviewWindow, status: &ServerStatus) {
        if let Err(e) = window.emit("server-status-updated", status) {
            warn!("Failed to emit server status event: {}", e);
        }
    }
}

#[async_trait]
impl ServerMonitoringService for ServerMonitoringServiceImpl {
    async fn get_current_status(&self) -> ServerStatus {
        let status = self.current_status.read().await;
        status
            .clone()
            .unwrap_or_else(|| ServerStatus::new("".to_string(), 0))
    }

    async fn update_status(&self, status: ServerStatus) -> AppResult<()> {
        // Get old status before updating
        let old_status = {
            let current_status = self.current_status.read().await;
            current_status.clone()
        };

        // Update in-memory cache
        {
            let mut current_status = self.current_status.write().await;
            *current_status = Some(status.clone());
        }

        // Persist to storage
        self.status_repository.save_status(&status).await?;

        // Broadcast status change to subscribers
        if let Err(e) = self.status_change_sender.send(status.clone()) {
            warn!("Failed to broadcast status change: {}", e);
        }

        // Publish server status changed event
        let event = AppEvent::server_status_changed(old_status, status.clone());
        if let Err(e) = self.event_bus.publish(event).await {
            warn!("Failed to publish server status event: {}", e);
        }

        debug!(
            "Updated server status: {}:{}",
            status.ip_address, status.port
        );
        Ok(())
    }

    async fn save_status(&self) -> AppResult<()> {
        if let Some(status) = self.current_status.read().await.clone() {
            self.status_repository.save_status(&status).await?;
        }
        Ok(())
    }

    async fn load_status(&self) -> AppResult<()> {
        if let Some(loaded_status) = self.status_repository.load_status().await? {
            let mut current_status = self.current_status.write().await;
            *current_status = Some(loaded_status.clone());

            info!(
                "Loaded server status: {}:{}",
                loaded_status.ip_address, loaded_status.port
            );
        } else {
            debug!("No server status file found, starting fresh");
        }
        Ok(())
    }

    async fn ping_server(&self) -> AppResult<Option<u64>> {
        // Get server information from current status
        let current_status = self.current_status.read().await;
        let server_info = current_status
            .as_ref()
            .map(|s| (s.ip_address.clone(), s.port));
        drop(current_status);

        if let Some((ip, port)) = server_info {
            let config = self.config.read().await;
            let timeout_seconds = config.ping_timeout_seconds;
            drop(config);

            // Perform the ping operation
            match self
                .network_connectivity
                .ping_server(&ip, port, timeout_seconds)
                .await
            {
                Ok(Some(latency_ms)) => {
                    // Server is online with measured latency
                    let status = ServerStatus::with_latency(ip.clone(), port, latency_ms);
                    self.update_status(status.clone()).await?;

                    // Update statistics
                    self.stats_repository.increment_ping_count(true).await?;
                    self.stats_repository
                        .update_average_latency(latency_ms)
                        .await?;

                    // Update current session if active
                    if let Some(mut session) = self.current_session.read().await.clone() {
                        session.record_ping(true);
                        self.session_repository.update_session(&session).await?;

                        let mut current_session = self.current_session.write().await;
                        *current_session = Some(session);
                    }

                    Ok(Some(latency_ms))
                }
                Ok(None) => {
                    // Server is offline or unreachable
                    let status = ServerStatus::offline(ip.clone(), port);
                    self.update_status(status).await?;

                    // Update statistics
                    self.stats_repository.increment_ping_count(false).await?;

                    // Update current session if active
                    if let Some(mut session) = self.current_session.read().await.clone() {
                        session.record_ping(false);
                        self.session_repository.update_session(&session).await?;

                        let mut current_session = self.current_session.write().await;
                        *current_session = Some(session);
                    }

                    Ok(None)
                }
                Err(e) => {
                    error!("Failed to ping server: {}", e);
                    Err(e)
                }
            }
        } else {
            debug!("No server information available for ping");
            Ok(None)
        }
    }

    async fn start_periodic_ping(&self) -> AppResult<()> {
        let mut is_active = self.is_ping_monitoring_active.write().await;
        if *is_active {
            warn!("Periodic ping monitoring is already active");
            return Ok(());
        }

        *is_active = true;
        drop(is_active);

        // Start a new monitoring session
        self.start_monitoring_session().await?;

        let config = self.config.read().await;
        let ping_interval = Duration::from_secs(config.ping_interval_seconds);
        drop(config);

        // Spawn background task for periodic ping monitoring
        let service = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = time::interval(ping_interval);

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                if !*service.is_ping_monitoring_active.read().await {
                    debug!("Periodic ping monitoring stopped, exiting loop");
                    break;
                }

                // Perform ping operation
                if let Err(e) = service.ping_server().await {
                    error!("Failed to ping server during periodic monitoring: {}", e);
                }
            }

            // End the monitoring session when stopping
            if let Err(e) = service.end_monitoring_session().await {
                error!("Failed to end monitoring session: {}", e);
            }
        });

        info!("Started periodic ping monitoring");
        Ok(())
    }

    async fn stop_periodic_ping(&self) -> AppResult<()> {
        let mut is_active = self.is_ping_monitoring_active.write().await;
        if !*is_active {
            warn!("Periodic ping monitoring is not active");
            return Ok(());
        }

        *is_active = false;
        drop(is_active);

        self.end_monitoring_session().await?;

        info!("Stopped periodic ping monitoring");
        Ok(())
    }

    async fn is_ping_monitoring_active(&self) -> bool {
        *self.is_ping_monitoring_active.read().await
    }

    async fn get_server_info(&self) -> Option<ServerInfo> {
        self.info_repository
            .load_server_info()
            .await
            .unwrap_or(None)
    }

    async fn update_server_info(&self, ip_address: String, port: u16) -> AppResult<()> {
        self.update_server_info_internal(ip_address, port).await
    }

    async fn get_monitoring_stats(&self) -> AppResult<ServerMonitoringStats> {
        self.stats_repository.load_stats().await
    }

    async fn get_config(&self) -> ServerMonitoringConfig {
        self.config.read().await.clone()
    }

    async fn update_config(&self, new_config: ServerMonitoringConfig) -> AppResult<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }

    async fn subscribe_to_status_changes(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(EventType::ServerMonitoring).await
    }
}

/// Implementation of network connectivity operations using TCP connections.
///
/// Provides low-level network operations for server connectivity testing
/// with configurable timeouts and error handling.
pub struct NetworkConnectivityImpl {
    /// Network configuration settings
    config: crate::domain::server_monitoring::traits::NetworkConfig,
}

impl Default for NetworkConnectivityImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkConnectivityImpl {
    pub fn new() -> Self {
        Self {
            config: crate::domain::server_monitoring::traits::NetworkConfig::default(),
        }
    }
}

#[async_trait]
impl NetworkConnectivity for NetworkConnectivityImpl {
    async fn ping_server(
        &self,
        ip_address: &str,
        port: u16,
        timeout_seconds: u64,
    ) -> AppResult<Option<u64>> {
        let start = std::time::Instant::now();
        let addr = format!("{}:{}", ip_address, port);

        // Attempt TCP connection with timeout
        match tokio::time::timeout(
            Duration::from_secs(timeout_seconds),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_stream)) => {
                // Connection successful, measure latency
                let ping_ms = start.elapsed().as_millis() as u64;
                debug!(
                    "Server ping successful: {}ms to {}:{}",
                    ping_ms, ip_address, port
                );
                Ok(Some(ping_ms))
            }
            Ok(Err(e)) => {
                // Connection failed (server unreachable, port closed, etc.)
                debug!("Server ping failed: {}:{} - {}", ip_address, port, e);
                Ok(None)
            }
            Err(_) => {
                // Connection timed out
                debug!("Server ping timeout: {}:{}", ip_address, port);
                Ok(None)
            }
        }
    }

    async fn is_server_reachable(
        &self,
        ip_address: &str,
        port: u16,
        timeout_seconds: u64,
    ) -> AppResult<bool> {
        // Use ping_server to determine reachability (latency doesn't matter)
        match self.ping_server(ip_address, port, timeout_seconds).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn get_network_config(&self) -> crate::domain::server_monitoring::traits::NetworkConfig {
        self.config.clone()
    }
}

impl Clone for ServerMonitoringServiceImpl {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            status_repository: Arc::clone(&self.status_repository),
            info_repository: Arc::clone(&self.info_repository),
            session_repository: Arc::clone(&self.session_repository),
            stats_repository: Arc::clone(&self.stats_repository),
            network_connectivity: Arc::clone(&self.network_connectivity),
            event_bus: Arc::clone(&self.event_bus),
            current_status: Arc::clone(&self.current_status),
            current_session: Arc::clone(&self.current_session),
            is_ping_monitoring_active: Arc::clone(&self.is_ping_monitoring_active),
            status_change_sender: self.status_change_sender.clone(),
        }
    }
}
