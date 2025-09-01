use crate::models::events::ServerConnectionEvent;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// Server status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ip_address: String,
    pub port: u16,
    pub is_online: bool,
    pub last_ping_ms: Option<u64>,
    pub last_seen: String,
    pub last_checked: String,
}

/// Server status manager for tracking and monitoring POE2 servers
pub struct ServerStatusManager {
    status: Arc<RwLock<Option<ServerStatus>>>,
}

impl ServerStatusManager {
    /// Create a new server status manager
    pub fn new() -> Self {
        Self {
            status: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Update server information from a connection event
    pub async fn update_server_info(&self, event: &ServerConnectionEvent) -> AppResult<()> {
        let mut status = self.status.write().await;

        let new_status = ServerStatus {
            ip_address: event.ip_address.clone(),
            port: event.port,
            is_online: true, // Assume online when we see a connection
            last_ping_ms: None,
            last_seen: event.timestamp.clone(),
            last_checked: chrono::Utc::now().to_rfc3339(),
        };

        *status = Some(new_status);

        info!("Updated server info: {}:{}", event.ip_address, event.port);
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
    pub async fn ping_server(&self) -> AppResult<Option<u64>> {
        let server_info = self.get_server_info().await;

        if let Some((ip, port)) = server_info {
            match self.perform_ping(&ip, port).await {
                Ok(ping_ms) => {
                    // Update status with ping result
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
    async fn perform_ping(&self, ip: &str, port: u16) -> AppResult<u64> {
        let start = std::time::Instant::now();

        let addr = format!("{}:{}", ip, port);
        let timeout_duration = Duration::from_secs(5);

        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_stream)) => {
                let ping_ms = start.elapsed().as_millis() as u64;
                debug!("Server ping successful: {}ms to {}:{}", ping_ms, ip, port);
                Ok(ping_ms)
            }
            Ok(Err(e)) => {
                error!("Failed to connect to server {}:{} - {}", ip, port, e);
                Err(AppError::NetworkError(format!("Connection failed: {}", e)))
            }
            Err(_) => {
                error!("Ping timeout to server {}:{}", ip, port);
                Err(AppError::NetworkError("Ping timeout".to_string()))
            }
        }
    }

    /// Update ping status with successful result
    async fn update_ping_status(&self, ping_ms: u64) -> AppResult<()> {
        let mut status = self.status.write().await;

        if let Some(ref mut s) = *status {
            s.is_online = true;
            s.last_ping_ms = Some(ping_ms);
            s.last_checked = chrono::Utc::now().to_rfc3339();
        }

        Ok(())
    }

    /// Update ping status with failed result
    async fn update_ping_status_failed(&self) -> AppResult<()> {
        let mut status = self.status.write().await;

        if let Some(ref mut s) = *status {
            s.is_online = false;
            s.last_ping_ms = None;
            s.last_checked = chrono::Utc::now().to_rfc3339();
        }

        Ok(())
    }

    /// Get server info for ping operations
    async fn get_server_info(&self) -> Option<(String, u16)> {
        let status = self.status.read().await;
        status.as_ref().map(|s| (s.ip_address.clone(), s.port))
    }

    /// Start background monitoring of server status
    pub async fn start_monitoring(&self) -> AppResult<()> {
        let status_manager = Arc::clone(&self.status);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60)); // Check every minute

            loop {
                interval.tick().await;

                // Perform ping check
                if let Some((ip, port)) = {
                    let status = status_manager.read().await;
                    status.as_ref().map(|s| (s.ip_address.clone(), s.port))
                } {
                    let start = std::time::Instant::now();
                    let addr = format!("{}:{}", ip, port);

                    match timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await {
                        Ok(Ok(_stream)) => {
                            let ping_ms = start.elapsed().as_millis() as u64;
                            debug!(
                                "Background ping successful: {}ms to {}:{}",
                                ping_ms, ip, port
                            );

                            // Update status
                            let mut status = status_manager.write().await;
                            if let Some(ref mut s) = *status {
                                s.is_online = true;
                                s.last_ping_ms = Some(ping_ms);
                                s.last_checked = chrono::Utc::now().to_rfc3339();
                            }
                        }
                        Ok(Err(e)) => {
                            warn!("Background ping failed to {}:{} - {}", ip, port, e);

                            // Update status
                            let mut status = status_manager.write().await;
                            if let Some(ref mut s) = *status {
                                s.is_online = false;
                                s.last_ping_ms = None;
                                s.last_checked = chrono::Utc::now().to_rfc3339();
                            }
                        }
                        Err(_) => {
                            warn!("Background ping timeout to {}:{}", ip, port);

                            // Update status
                            let mut status = status_manager.write().await;
                            if let Some(ref mut s) = *status {
                                s.is_online = false;
                                s.last_ping_ms = None;
                                s.last_checked = chrono::Utc::now().to_rfc3339();
                            }
                        }
                    }
                }
            }
        });

        info!("Server status monitoring started");
        Ok(())
    }
}

impl Default for ServerStatusManager {
    fn default() -> Self {
        Self::new()
    }
}

// Error types for the server status service
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub type AppResult<T> = Result<T, AppError>;
