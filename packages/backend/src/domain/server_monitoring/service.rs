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
use crate::errors::AppResult;
use crate::infrastructure::tauri::EventPublisher;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time;

/// Server monitoring service implementation
pub struct ServerMonitoringServiceImpl {
    config: Arc<RwLock<ServerMonitoringConfig>>,
    status_repository: Arc<dyn ServerStatusRepository>,
    info_repository: Arc<dyn ServerInfoRepository>,
    session_repository: Arc<dyn ServerMonitoringSessionRepository>,
    stats_repository: Arc<dyn ServerMonitoringStatsRepository>,
    network_connectivity: Arc<dyn NetworkConnectivity>,
    event_publisher: Arc<EventPublisher>,
    current_status: Arc<RwLock<Option<ServerStatus>>>,
    current_session: Arc<RwLock<Option<ServerMonitoringSession>>>,
    is_ping_monitoring_active: Arc<RwLock<bool>>,
    status_change_sender: broadcast::Sender<ServerStatus>,
}

impl ServerMonitoringServiceImpl {
    /// Create a new server monitoring service
    pub fn new(event_publisher: Arc<EventPublisher>) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(ServerMonitoringConfig::default()));

        // Create repositories with default paths
        let status_file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("server_status.json")
            .to_string_lossy()
            .to_string();

        let info_file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("server_info.json")
            .to_string_lossy()
            .to_string();

        let sessions_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("sessions")
            .to_string_lossy()
            .to_string();

        let stats_file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("server_monitoring_stats.json")
            .to_string_lossy()
            .to_string();

        let status_repository = Arc::new(ServerStatusRepositoryImpl::new(status_file_path));
        let info_repository = Arc::new(ServerInfoRepositoryImpl::new(info_file_path));
        let session_repository = Arc::new(ServerMonitoringSessionRepositoryImpl::new(sessions_dir));
        let stats_repository = Arc::new(ServerMonitoringStatsRepositoryImpl::new(stats_file_path));
        let network_connectivity = Arc::new(NetworkConnectivityImpl::new());

        let (status_change_sender, _) = broadcast::channel(100);

        Ok(Self {
            config,
            status_repository,
            info_repository,
            session_repository,
            stats_repository,
            network_connectivity,
            event_publisher,
            current_status: Arc::new(RwLock::new(None)),
            current_session: Arc::new(RwLock::new(None)),
            is_ping_monitoring_active: Arc::new(RwLock::new(false)),
            status_change_sender,
        })
    }

    /// Create a new server monitoring service with custom repositories (for testing)
    pub fn with_repositories(
        config: ServerMonitoringConfig,
        status_repository: Arc<dyn ServerStatusRepository>,
        info_repository: Arc<dyn ServerInfoRepository>,
        session_repository: Arc<dyn ServerMonitoringSessionRepository>,
        stats_repository: Arc<dyn ServerMonitoringStatsRepository>,
        network_connectivity: Arc<dyn NetworkConnectivity>,
        event_publisher: Arc<EventPublisher>,
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
            event_publisher,
            current_status: Arc::new(RwLock::new(None)),
            current_session: Arc::new(RwLock::new(None)),
            is_ping_monitoring_active: Arc::new(RwLock::new(false)),
            status_change_sender,
        }
    }

    /// Start a new monitoring session
    async fn start_monitoring_session(&self) -> AppResult<()> {
        let session = ServerMonitoringSession::new();
        self.session_repository.save_session(&session).await?;

        let mut current_session = self.current_session.write().await;
        *current_session = Some(session);

        info!("Started new server monitoring session");
        Ok(())
    }

    /// End the current monitoring session
    async fn end_monitoring_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;

            // Update statistics
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

    /// Update server information
    async fn update_server_info_internal(&self, ip_address: String, port: u16) -> AppResult<()> {
        let server_info = match self.info_repository.load_server_info().await? {
            Some(mut info) => {
                if info.ip_address == ip_address && info.port == port {
                    info.record_connection();
                } else {
                    info = ServerInfo::new(ip_address.clone(), port);
                }
                info
            }
            None => ServerInfo::new(ip_address.clone(), port),
        };

        self.info_repository.save_server_info(&server_info).await?;

        // Update current session if active
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.set_server_info(server_info);
            self.session_repository.update_session(&session).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = Some(session);
        }

        debug!("Updated server info: {}:{}", ip_address, port);
        Ok(())
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
        // Update in-memory status
        {
            let mut current_status = self.current_status.write().await;
            *current_status = Some(status.clone());
        }

        // Save to persistent storage
        self.status_repository.save_status(&status).await?;

        // Broadcast status change
        if let Err(e) = self.status_change_sender.send(status.clone()) {
            warn!("Failed to broadcast status change: {}", e);
        }

        // Emit to frontend
        if let Err(e) = self.event_publisher.broadcast_ping_event(status.clone()) {
            warn!("Failed to broadcast ping event: {}", e);
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
        let current_status = self.current_status.read().await;
        let server_info = current_status
            .as_ref()
            .map(|s| (s.ip_address.clone(), s.port));
        drop(current_status);

        if let Some((ip, port)) = server_info {
            let config = self.config.read().await;
            let timeout_seconds = config.ping_timeout_seconds;
            drop(config);

            match self
                .network_connectivity
                .ping_server(&ip, port, timeout_seconds)
                .await
            {
                Ok(Some(latency_ms)) => {
                    // Update status with successful ping
                    let status = ServerStatus::with_latency(ip.clone(), port, latency_ms);
                    self.update_status(status.clone()).await?;

                    // Update statistics
                    self.stats_repository.increment_ping_count(true).await?;
                    self.stats_repository
                        .update_average_latency(latency_ms)
                        .await?;

                    // Update session
                    if let Some(mut session) = self.current_session.read().await.clone() {
                        session.record_ping(true);
                        self.session_repository.update_session(&session).await?;

                        let mut current_session = self.current_session.write().await;
                        *current_session = Some(session);
                    }

                    Ok(Some(latency_ms))
                }
                Ok(None) => {
                    // Server unreachable
                    let status = ServerStatus::offline(ip.clone(), port);
                    self.update_status(status).await?;

                    // Update statistics
                    self.stats_repository.increment_ping_count(false).await?;

                    // Update session
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

        // Start monitoring session
        self.start_monitoring_session().await?;

        let config = self.config.read().await;
        let ping_interval = Duration::from_secs(config.ping_interval_seconds);
        drop(config);

        let service = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = time::interval(ping_interval);

            loop {
                interval.tick().await;

                // Check if we should stop monitoring
                if !*service.is_ping_monitoring_active.read().await {
                    debug!("Periodic ping monitoring stopped, exiting loop");
                    break;
                }

                // Perform ping
                if let Err(e) = service.ping_server().await {
                    error!("Failed to ping server during periodic monitoring: {}", e);
                }
            }

            // End monitoring session
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

        // End monitoring session
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

    fn subscribe_to_status_changes(&self) -> broadcast::Receiver<ServerStatus> {
        self.status_change_sender.subscribe()
    }
}

/// Network connectivity implementation using tokio
pub struct NetworkConnectivityImpl {
    config: crate::domain::server_monitoring::traits::NetworkConfig,
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

        match tokio::time::timeout(
            Duration::from_secs(timeout_seconds),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_stream)) => {
                let ping_ms = start.elapsed().as_millis() as u64;
                debug!(
                    "Server ping successful: {}ms to {}:{}",
                    ping_ms, ip_address, port
                );
                Ok(Some(ping_ms))
            }
            Ok(Err(e)) => {
                debug!("Server ping failed: {}:{} - {}", ip_address, port, e);
                Ok(None)
            }
            Err(_) => {
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
        match self.ping_server(ip_address, port, timeout_seconds).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn get_network_config(&self) -> crate::domain::server_monitoring::traits::NetworkConfig {
        self.config.clone()
    }
}

// Implement Clone for the service to allow moving into async tasks
impl Clone for ServerMonitoringServiceImpl {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            status_repository: Arc::clone(&self.status_repository),
            info_repository: Arc::clone(&self.info_repository),
            session_repository: Arc::clone(&self.session_repository),
            stats_repository: Arc::clone(&self.stats_repository),
            network_connectivity: Arc::clone(&self.network_connectivity),
            event_publisher: Arc::clone(&self.event_publisher),
            current_status: Arc::clone(&self.current_status),
            current_session: Arc::clone(&self.current_session),
            is_ping_monitoring_active: Arc::clone(&self.is_ping_monitoring_active),
            status_change_sender: self.status_change_sender.clone(),
        }
    }
}
