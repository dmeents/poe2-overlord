use crate::models::events::ServerConnectionEvent;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// Simple server status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ip_address: String,
    pub port: u16,
    pub is_online: bool,
    pub last_ping_ms: Option<u64>,
    pub last_checked: String,
}

/// Simple server status manager
pub struct ServerStatusManager {
    status: Arc<RwLock<Option<ServerStatus>>>,
    status_file_path: PathBuf,
}

impl ServerStatusManager {
    /// Create a new server status manager
    pub fn new() -> Self {
        let status_file_path = Self::get_status_file_path();
        let status = Arc::new(RwLock::new(None));

        Self {
            status,
            status_file_path,
        }
    }

    /// Get the path to the server status file
    fn get_status_file_path() -> PathBuf {
        let mut path = dirs::data_dir().unwrap_or_else(|| std::env::temp_dir());
        path.push("poe2-overlord");
        path.push("server_status.json");
        path
    }

    /// Load server status from file on startup
    pub async fn load_status(&self) -> Result<(), String> {
        if !self.status_file_path.exists() {
            debug!("No server status file found, starting fresh");
            return Ok(());
        }

        let contents = fs::read_to_string(&self.status_file_path)
            .await
            .map_err(|e| format!("Failed to read status file: {}", e))?;
        let loaded_status: ServerStatus = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse status file: {}", e))?;

        let mut status = self.status.write().await;
        *status = Some(loaded_status.clone());
        drop(status);

        info!(
            "Loaded server status: {}:{}",
            loaded_status.ip_address, loaded_status.port
        );
        Ok(())
    }

    /// Save server status to file
    async fn save_status(&self, status: &ServerStatus) -> Result<(), String> {
        // Ensure the directory exists
        if let Some(parent) = self.status_file_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(status)
            .map_err(|e| format!("Failed to serialize status: {}", e))?;
        fs::write(&self.status_file_path, json)
            .await
            .map_err(|e| format!("Failed to write status file: {}", e))?;
        debug!("Server status saved to file");
        Ok(())
    }

    /// Update server information from a connection event (extract IP from logs)
    pub async fn update_server_info(&self, event: &ServerConnectionEvent) -> Result<(), String> {
        debug!(
            "Updating server info from connection event: {}:{}",
            event.ip_address, event.port
        );

        let new_status = ServerStatus {
            ip_address: event.ip_address.clone(),
            port: event.port,
            is_online: true,
            last_ping_ms: None,
            last_checked: chrono::Utc::now().to_rfc3339(),
        };

        // Update in-memory status
        let mut status = self.status.write().await;
        *status = Some(new_status.clone());
        drop(status);

        // Save to file
        if let Err(e) = self.save_status(&new_status).await {
            warn!("Failed to save server status: {}", e);
        }

        debug!(
            "Updated server info: {}:{}",
            event.ip_address, event.port
        );
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

    /// Ping the current server and update status
    pub async fn ping_server(&self) -> Result<Option<u64>, String> {
        let server_info = self.get_last_known_server().await;

        if let Some((ip, port)) = server_info {
            match self.perform_ping(&ip, port).await {
                Ok(ping_ms) => {
                    self.update_ping_status(ping_ms).await?;
                    Ok(Some(ping_ms))
                }
                Err(e) => {
                    warn!("Failed to ping server {}:{} - {}", ip, port, e);
                    self.update_ping_status_failed().await?;
                    Ok(None)
                }
            }
        } else {
            debug!("No server information available for ping");
            Ok(None)
        }
    }

    /// Perform a TCP ping to the specified server
    async fn perform_ping(&self, ip: &str, port: u16) -> Result<u64, String> {
        let start = std::time::Instant::now();
        let addr = format!("{}:{}", ip, port);
        let timeout_duration = Duration::from_secs(5);

        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_stream)) => {
                let ping_ms = start.elapsed().as_millis() as u64;
                debug!("Server ping successful: {}ms to {}:{}", ping_ms, ip, port);
                Ok(ping_ms)
            }
            Ok(Err(e)) => Err(format!("Connection failed: {}", e)),
            Err(_) => Err("Ping timeout".to_string()),
        }
    }

    /// Update ping status with successful result
    async fn update_ping_status(&self, ping_ms: u64) -> Result<(), String> {
        let mut status = self.status.write().await;

        if let Some(ref mut s) = *status {
            s.is_online = true;
            s.last_ping_ms = Some(ping_ms);
            s.last_checked = chrono::Utc::now().to_rfc3339();

            let status_to_save = s.clone();
            drop(status);

            if let Err(e) = self.save_status(&status_to_save).await {
                warn!("Failed to save ping status: {}", e);
            }
        }

        Ok(())
    }

    /// Update ping status with failed result
    async fn update_ping_status_failed(&self) -> Result<(), String> {
        let mut status = self.status.write().await;

        if let Some(ref mut s) = *status {
            s.is_online = false;
            s.last_ping_ms = None;
            s.last_checked = chrono::Utc::now().to_rfc3339();

            let status_to_save = s.clone();
            drop(status);

            if let Err(e) = self.save_status(&status_to_save).await {
                warn!("Failed to save failed ping status: {}", e);
            }
        }

        Ok(())
    }
}

impl Default for ServerStatusManager {
    fn default() -> Self {
        Self::new()
    }
}
