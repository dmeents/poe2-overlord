use crate::errors::{AppError, AppResult};
use crate::models::events::ServerConnectionEvent;
use crate::services::event_dispatcher::EventDispatcher;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// Server status information for both internal storage and frontend events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ip_address: String,
    pub port: u16,
    pub is_online: bool,
    pub latency_ms: Option<u64>,
    pub timestamp: String,
}

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
            .map_err(|e| AppError::FileSystem(format!("Failed to read status file: {}", e)))?;

        let loaded_status: ServerStatus = serde_json::from_str(&contents)
            .map_err(|e| AppError::Serialization(format!("Failed to parse status file: {}", e)))?;

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

        let new_status = ServerStatus {
            ip_address: event.ip_address.clone(),
            port: event.port,
            is_online: true,
            latency_ms: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

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

            // Broadcast the ping event (we'll need to add this to EventDispatcher)
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
                .map_err(|e| AppError::FileSystem(format!("Failed to create directory: {}", e)))?;
        }

        let json = serde_json::to_string_pretty(status)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize status: {}", e)))?;

        fs::write(&self.status_file_path, json)
            .await
            .map_err(|e| AppError::FileSystem(format!("Failed to write status file: {}", e)))?;

        debug!("Server status saved to file");
        Ok(())
    }

    /// Start periodic ping monitoring
    pub async fn start_periodic_ping(&self) {
        let server_manager = Arc::clone(&self.status);
        let event_broadcaster = Arc::clone(&self.event_broadcaster);

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
                    drop(status);

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
