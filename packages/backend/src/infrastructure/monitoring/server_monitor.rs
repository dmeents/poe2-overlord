use crate::errors::{AppError, AppResult};
use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::infrastructure::tauri::EventDispatcher;
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use async_trait::async_trait;
use log::{debug, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// Server monitor for tracking server status and connectivity
pub struct ServerMonitor {
    status: Arc<RwLock<Option<ServerStatus>>>,
    status_file_path: PathBuf,
    event_broadcaster: Arc<EventDispatcher>,
}

impl ServerMonitor {
    /// Create a new server monitor
    pub fn new(event_broadcaster: Arc<EventDispatcher>) -> Self {
        let status_file_path = Self::get_status_file_path();
        let status = Arc::new(RwLock::new(None));

        Self {
            status,
            status_file_path,
            event_broadcaster,
        }
    }

    /// Get the path to the server status file
    fn get_status_file_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("poe2-overlord");
        path.push("server_status.json");
        path
    }

    /// Load server status from file on startup
    pub async fn load_status(&self) -> AppResult<()> {
        if !self.status_file_path.exists() {
            debug!("No server status file found, starting fresh");
            return Ok(());
        }

        let contents = fs::read_to_string(&self.status_file_path)
            .await
            .map_err(|e| AppError::file_system_error("Failed to read status file: {}", &e.to_string()))?;

        let loaded_status: ServerStatus = serde_json::from_str(&contents)
            .map_err(|e| AppError::serialization_error("Failed to parse status file: {}", &e.to_string()))?;

        let mut status = self.status.write().await;
        *status = Some(loaded_status.clone());
        drop(status);

        info!(
            "Loaded server status: {}:{}",
            loaded_status.ip_address, loaded_status.port
        );
        Ok(())
    }

    /// Update server information from a connection event (extract IP from logs)
    pub async fn update_server_info(&self, event: &ServerConnectionEvent) -> AppResult<()> {
        debug!(
            "Updating server info from connection event: {}:{}",
            event.ip_address, event.port
        );

        let new_status = ServerStatus::from_connection_event(event);

        // Update in-memory status
        let mut status = self.status.write().await;
        *status = Some(new_status.clone());
        drop(status);

        // Save to file
        if let Err(e) = self.save_status_to_file(&new_status).await {
            warn!("Failed to save server status: {}", e);
        }

        debug!("Updated server info: {}:{}", event.ip_address, event.port);
        Ok(())
    }

    /// Get the current server status
    pub async fn get_server_status(&self) -> Option<ServerStatus> {
        let status = self.status.read().await;
        status.clone()
    }

    /// Get the last known server address
    pub async fn get_last_known_server(&self) -> Option<(String, u16)> {
        let status = self.status.read().await;
        status.as_ref().map(|s| (s.ip_address.clone(), s.port))
    }

    /// Ping a server and return the ping time in milliseconds
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

    /// Ping the current server and emit event to frontend
    pub async fn ping_server(&self) -> AppResult<Option<u64>> {
        let server_info = self.get_last_known_server().await;

        if let Some((ip, port)) = server_info {
            let timeout_duration = Duration::from_secs(5);
            let ping_result = Self::ping_server_internal(&ip, port, timeout_duration).await;

            let (is_online, latency_ms) = match ping_result {
                Ok(ping_ms) => (true, Some(ping_ms)),
                Err(_) => (false, None),
            };

            // Update status in memory
            let mut status = self.status.write().await;
            if let Some(ref mut s) = *status {
                s.is_online = is_online;
                s.latency_ms = latency_ms;
                s.timestamp = chrono::Utc::now().to_rfc3339();
            }
            drop(status);

            // Emit ping event to frontend
            let ping_event = ServerStatus {
                ip_address: ip.clone(),
                port,
                is_online,
                latency_ms,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            // Broadcast the ping event
            if let Err(e) = self.event_broadcaster.broadcast_ping_event(ping_event) {
                warn!("Failed to broadcast ping event: {}", e);
            }

            // Save status to file periodically (not on every ping)
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

    /// Save server status to file
    async fn save_status_to_file(&self, status: &ServerStatus) -> AppResult<()> {
        // Ensure the directory exists
        if let Some(parent) = self.status_file_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::file_system_error("Failed to create directory: {}", &e.to_string()))?;
        }

        let json = serde_json::to_string_pretty(status)
            .map_err(|e| AppError::serialization_error("Failed to serialize status: {}", &e.to_string()))?;

        fs::write(&self.status_file_path, json)
            .await
            .map_err(|e| AppError::file_system_error("Failed to write status file: {}", &e.to_string()))?;

        debug!("Server status saved to file");
        Ok(())
    }

    /// Start periodic ping monitoring
    pub async fn start_periodic_ping(&self) {
        let server_manager = Arc::clone(&self.status);
        let event_broadcaster = Arc::clone(&self.event_broadcaster);
        let status_file_path = self.status_file_path.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30)); // Ping every 30 seconds

            loop {
                interval.tick().await;

                // Get current server info
                let server_info = {
                    let status = server_manager.read().await;
                    status.as_ref().map(|s| (s.ip_address.clone(), s.port))
                };

                if let Some((ip, port)) = server_info {
                    // Perform ping with 5 second timeout
                    let timeout_duration = Duration::from_secs(5);
                    let ping_result = Self::ping_server_internal(&ip, port, timeout_duration).await;

                    let (is_online, latency_ms) = match ping_result {
                        Ok(ping_ms) => (true, Some(ping_ms)),
                        Err(_) => (false, None),
                    };

                    // Update status in memory
                    let mut status = server_manager.write().await;
                    if let Some(ref mut s) = *status {
                        s.is_online = is_online;
                        s.latency_ms = latency_ms;
                        s.timestamp = chrono::Utc::now().to_rfc3339();
                    }
                    let status_to_save = status.clone();
                    drop(status);

                    // Save status to file
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

                    // Emit ping event to frontend
                    let ping_event = ServerStatus {
                        ip_address: ip,
                        port,
                        is_online,
                        latency_ms,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };

                    if let Err(e) = event_broadcaster.broadcast_ping_event(ping_event) {
                        warn!("Failed to broadcast ping event: {}", e);
                    }
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
        // TODO: Implement stop functionality
        Ok(())
    }

    async fn is_ping_monitoring_active(&self) -> bool {
        // TODO: Implement monitoring state tracking
        false
    }

    async fn get_server_info(&self) -> Option<crate::domain::server_monitoring::models::ServerInfo> {
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

    async fn get_monitoring_stats(&self) -> AppResult<crate::domain::server_monitoring::models::ServerMonitoringStats> {
        Ok(crate::domain::server_monitoring::models::ServerMonitoringStats::default())
    }

    async fn get_config(&self) -> crate::domain::server_monitoring::models::ServerMonitoringConfig {
        crate::domain::server_monitoring::models::ServerMonitoringConfig::default()
    }

    async fn update_config(&self, _config: crate::domain::server_monitoring::models::ServerMonitoringConfig) -> AppResult<()> {
        // TODO: Implement config update
        Ok(())
    }

    fn subscribe_to_status_changes(&self) -> tokio::sync::broadcast::Receiver<ServerStatus> {
        // TODO: Implement status change subscription
        let (_, receiver) = tokio::sync::broadcast::channel(1);
        receiver
    }
}
