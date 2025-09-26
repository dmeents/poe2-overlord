use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use crate::errors::{AppError, AppResult};
use crate::domain::events::EventBus;
use async_trait::async_trait;
use log::{debug, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// Monitors server connectivity and maintains server status information
///
/// Handles server discovery from log events, persistent status storage,
/// periodic ping monitoring, and real-time connectivity testing.
/// Integrates with the event system to broadcast status changes.
pub struct ServerMonitor {
    /// Current server status with thread-safe access
    status: Arc<RwLock<Option<ServerStatus>>>,
    /// Path to persistent status storage file
    status_file_path: PathBuf,
    /// Event broadcaster for status change notifications
    event_bus: Arc<EventBus>,
}

impl ServerMonitor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        let status_file_path = Self::get_status_file_path();
        let status = Arc::new(RwLock::new(None));

        Self {
            status,
            status_file_path,
            event_bus,
        }
    }

    /// Constructs the path for persistent server status storage
    ///
    /// Uses the system config directory with a fallback to current directory.
    /// Creates a dedicated subdirectory for the application's data.
    fn get_status_file_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("poe2-overlord");
        path.push("server_status.json");
        path
    }

    pub async fn load_status(&self) -> AppResult<()> {
        if !self.status_file_path.exists() {
            debug!("No server status file found, starting fresh");
            return Ok(());
        }

        let contents = fs::read_to_string(&self.status_file_path)
            .await
            .map_err(|e| {
                AppError::file_system_error("Failed to read status file: {}", &e.to_string())
            })?;

        let loaded_status: ServerStatus = serde_json::from_str(&contents).map_err(|e| {
            AppError::serialization_error("Failed to parse status file: {}", &e.to_string())
        })?;

        let mut status = self.status.write().await;
        *status = Some(loaded_status.clone());
        drop(status);

        info!(
            "Loaded server status: {}:{}",
            loaded_status.ip_address, loaded_status.port
        );
        Ok(())
    }

    pub async fn update_server_info(&self, event: &ServerConnectionEvent) -> AppResult<()> {
        debug!(
            "Updating server info from connection event: {}:{}",
            event.ip_address, event.port
        );

        let new_status = ServerStatus::from_connection_event(event);

        let mut status = self.status.write().await;
        *status = Some(new_status.clone());
        drop(status);

        if let Err(e) = self.save_status_to_file(&new_status).await {
            warn!("Failed to save server status: {}", e);
        }

        debug!("Updated server info: {}:{}", event.ip_address, event.port);
        Ok(())
    }

    pub async fn get_server_status(&self) -> Option<ServerStatus> {
        let status = self.status.read().await;
        status.clone()
    }

    pub async fn get_last_known_server(&self) -> Option<(String, u16)> {
        let status = self.status.read().await;
        status.as_ref().map(|s| (s.ip_address.clone(), s.port))
    }

    /// Performs a TCP connection test to measure server latency
    ///
    /// Attempts to establish a TCP connection to the server and measures
    /// the round-trip time. Returns the latency in milliseconds or an error
    /// if the connection fails or times out.
    async fn ping_server_internal(
        ip: &str,
        port: u16,
        timeout_duration: Duration,
    ) -> Result<u64, String> {
        let start = std::time::Instant::now();
        let addr = format!("{}:{}", ip, port);

        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_stream)) => {
                let ping_ms = start.elapsed().as_millis() as u64;
                debug!("Server ping successful: {}ms to {}:{}", ping_ms, ip, port);
                Ok(ping_ms)
            }
            Ok(Err(e)) => {
                debug!("Server ping failed: {}:{} - {}", ip, port, e);
                Err(format!("Connection failed: {}", e))
            }
            Err(_) => {
                debug!("Server ping timeout: {}:{}", ip, port);
                Err("Connection timeout".to_string())
            }
        }
    }

    pub async fn ping_server(&self) -> AppResult<Option<u64>> {
        let server_info = self.get_last_known_server().await;

        if let Some((ip, port)) = server_info {
            let timeout_duration = Duration::from_secs(5);
            let ping_result = Self::ping_server_internal(&ip, port, timeout_duration).await;

            let (is_online, latency_ms) = match ping_result {
                Ok(ping_ms) => (true, Some(ping_ms)),
                Err(_) => (false, None),
            };

            let mut status = self.status.write().await;
            if let Some(ref mut s) = *status {
                s.is_online = is_online;
                s.latency_ms = latency_ms;
                s.timestamp = chrono::Utc::now().to_rfc3339();
            }
            drop(status);

            let ping_event = ServerStatus {
                ip_address: ip.clone(),
                port,
                is_online,
                latency_ms,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            // Note: ServerMonitor now uses the unified event system
            // Ping events are published through the server monitoring service
            debug!("Server ping completed: {:?}ms", ping_event.latency_ms);

            if let Some(status_to_save) = self.get_server_status().await {
                if let Err(e) = self.save_status_to_file(&status_to_save).await {
                    warn!("Failed to save server status: {}", e);
                }
            }

            Ok(latency_ms)
        } else {
            debug!("No server information available for ping");
            Ok(None)
        }
    }

    async fn save_status_to_file(&self, status: &ServerStatus) -> AppResult<()> {
        if let Some(parent) = self.status_file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error("Failed to create directory: {}", &e.to_string())
            })?;
        }

        let json = serde_json::to_string_pretty(status).map_err(|e| {
            AppError::serialization_error("Failed to serialize status: {}", &e.to_string())
        })?;

        fs::write(&self.status_file_path, json).await.map_err(|e| {
            AppError::file_system_error("Failed to write status file: {}", &e.to_string())
        })?;

        debug!("Server status saved to file");
        Ok(())
    }

    /// Starts a background task for periodic server ping monitoring
    ///
    /// Spawns an async task that pings the server every 30 seconds.
    /// Updates status, saves to file, and broadcasts events on each ping.
    /// The task runs indefinitely until the application shuts down.
    pub async fn start_periodic_ping(&self) {
        let server_manager = Arc::clone(&self.status);
        // Event bus is available for future use if needed
        let status_file_path = self.status_file_path.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30)); // Ping every 30 seconds

            loop {
                interval.tick().await;

                let server_info = {
                    let status = server_manager.read().await;
                    status.as_ref().map(|s| (s.ip_address.clone(), s.port))
                };

                if let Some((ip, port)) = server_info {
                    let timeout_duration = Duration::from_secs(5);
                    let ping_result = Self::ping_server_internal(&ip, port, timeout_duration).await;

                    let (is_online, latency_ms) = match ping_result {
                        Ok(ping_ms) => (true, Some(ping_ms)),
                        Err(_) => (false, None),
                    };

                    // Update status with ping results
                    let mut status = server_manager.write().await;
                    if let Some(ref mut s) = *status {
                        s.is_online = is_online;
                        s.latency_ms = latency_ms;
                        s.timestamp = chrono::Utc::now().to_rfc3339();
                    }
                    let status_to_save = status.clone();
                    drop(status);

                    // Persist status to file
                    if let Some(ref status) = status_to_save {
                        if let Some(parent) = status_file_path.parent() {
                            if let Err(e) = fs::create_dir_all(parent).await {
                                warn!("Failed to create directory for status file: {}", e);
                            }
                        }

                        if let Ok(json) = serde_json::to_string_pretty(status) {
                            if let Err(e) = fs::write(&status_file_path, json).await {
                                warn!("Failed to save server status to file: {}", e);
                            } else {
                                debug!("Server status saved to file during periodic ping");
                            }
                        }
                    }

                    // Broadcast ping event to subscribers
                    let ping_event = ServerStatus {
                        ip_address: ip,
                        port,
                        is_online,
                        latency_ms,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };

                    // Note: ServerMonitor now uses the unified event system
                    // Ping events are published through the server monitoring service
                    debug!("Periodic server ping completed: {:?}ms", ping_event.latency_ms);
                }
            }
        });
    }
}

#[async_trait]
impl ServerMonitoringService for ServerMonitor {
    async fn get_current_status(&self) -> ServerStatus {
        if let Some(status) = self.get_server_status().await {
            status
        } else {
            ServerStatus::new("0.0.0.0".to_string(), 0)
        }
    }

    async fn update_status(&self, status: ServerStatus) -> AppResult<()> {
        let mut current_status = self.status.write().await;
        *current_status = Some(status);
        Ok(())
    }

    async fn save_status(&self) -> AppResult<()> {
        if let Some(status) = self.get_server_status().await {
            self.save_status_to_file(&status).await
        } else {
            Ok(())
        }
    }

    async fn load_status(&self) -> AppResult<()> {
        self.load_status().await
    }

    async fn ping_server(&self) -> AppResult<Option<u64>> {
        self.ping_server().await
    }

    async fn start_periodic_ping(&self) -> AppResult<()> {
        self.start_periodic_ping().await;
        Ok(())
    }

    async fn stop_periodic_ping(&self) -> AppResult<()> {
        Ok(())
    }

    async fn is_ping_monitoring_active(&self) -> bool {
        false
    }

    async fn get_server_info(
        &self,
    ) -> Option<crate::domain::server_monitoring::models::ServerInfo> {
        if let Some(status) = self.get_server_status().await {
            Some(crate::domain::server_monitoring::models::ServerInfo::new(
                status.ip_address,
                status.port,
            ))
        } else {
            None
        }
    }

    async fn update_server_info(&self, ip_address: String, port: u16) -> AppResult<()> {
        let new_status = ServerStatus::new(ip_address, port);
        self.update_status(new_status).await
    }

    async fn get_monitoring_stats(
        &self,
    ) -> AppResult<crate::domain::server_monitoring::models::ServerMonitoringStats> {
        Ok(crate::domain::server_monitoring::models::ServerMonitoringStats::default())
    }

    async fn get_config(&self) -> crate::domain::server_monitoring::models::ServerMonitoringConfig {
        crate::domain::server_monitoring::models::ServerMonitoringConfig::default()
    }

    async fn update_config(
        &self,
        _config: crate::domain::server_monitoring::models::ServerMonitoringConfig,
    ) -> AppResult<()> {
        Ok(())
    }

    async fn subscribe_to_status_changes(&self) -> AppResult<tokio::sync::broadcast::Receiver<crate::domain::events::AppEvent>> {
        self.event_bus.get_receiver(crate::domain::events::EventType::ServerMonitoring).await
    }
}
