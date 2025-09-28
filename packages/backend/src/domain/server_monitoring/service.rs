//! # Server Monitoring Service Implementation
//!
//! This module provides a simplified service implementation for server monitoring functionality.
//! Handles server status tracking, ping operations, and event publishing.

use crate::domain::events::{AppEvent, EventBus};
use crate::domain::server_monitoring::models::{ServerIp, ServerStatus};
use crate::domain::server_monitoring::repository::{ServerIpRepository, ServerIpRepositoryTrait};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

/// Simplified service implementation for server monitoring operations.
///
/// Handles server status tracking, ping operations, and event publishing.
pub struct ServerMonitoringServiceImpl {
    /// Repository for server IP persistence
    ip_repository: Arc<dyn ServerIpRepositoryTrait>,
    /// Event bus for publishing monitoring events
    event_bus: Arc<EventBus>,
    /// Current server status cache (memory only)
    current_status: Arc<RwLock<Option<ServerStatus>>>,
    /// Flag indicating if periodic ping monitoring is active
    is_monitoring: Arc<RwLock<bool>>,
}

impl ServerMonitoringServiceImpl {
    /// Create a new server monitoring service instance.
    pub fn new(event_bus: Arc<EventBus>) -> AppResult<Self> {
        let ip_repository = Arc::new(ServerIpRepository::new()?);

        Ok(Self {
            ip_repository,
            event_bus,
            current_status: Arc::new(RwLock::new(None)),
            is_monitoring: Arc::new(RwLock::new(false)),
        })
    }

    /// Update server status in memory and publish event
    async fn update_status(&self, status: ServerStatus) -> AppResult<()> {
        // Update in-memory cache
        {
            let mut current = self.current_status.write().await;
            *current = Some(status.clone());
        }

        // Publish event
        let event = AppEvent::server_status_changed(None, status.clone());
        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish server status event: {}", e);
        }

        Ok(())
    }

    /// Load the last known server IP and create status in memory
    async fn load_last_known_ip(&self) -> AppResult<()> {
        if let Some(server_ip) = self.ip_repository.load_ip().await? {
            let ip_address = server_ip.ip_address.clone();
            let status = ServerStatus::new(ip_address.clone(), 6112); // Standard POE2 port
            let mut current = self.current_status.write().await;
            *current = Some(status);
        }
        Ok(())
    }

    /// Perform a ping to the server IP address
    async fn ping_server(&self, ip_address: &str, _port: u16) -> AppResult<Option<u64>> {
        let start = std::time::Instant::now();

        // Use the system ping command for ICMP ping
        let output = tokio::process::Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("-W")
            .arg("5") // 5 second timeout
            .arg(ip_address)
            .output()
            .await;

        match output {
            Ok(result) => {
                if result.status.success() {
                    let ping_ms = start.elapsed().as_millis() as u64;
                    Ok(Some(ping_ms))
                } else {
                    Ok(None)
                }
            }
            Err(_e) => {
                Ok(None)
            }
        }
    }
}

#[async_trait]
impl ServerMonitoringService for ServerMonitoringServiceImpl {
    /// Get current server status from memory cache
    async fn get_current_status(&self) -> ServerStatus {
        let status = self.current_status.read().await;
        status
            .clone()
            .unwrap_or_else(|| ServerStatus::new("".to_string(), 0))
    }

    /// Update server status from log analysis (when IP is found)
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()> {
        // Save IP to file
        let server_ip = ServerIp::new(ip_address.clone());
        self.ip_repository.save_ip(&server_ip).await?;

        // Create status in memory
        let status = ServerStatus::new(ip_address, port);
        self.update_status(status).await
    }

    /// Ping current server and update status
    async fn ping_current_server(&self) -> AppResult<()> {
        let current_status = self.current_status.read().await;
        if let Some(status) = current_status.as_ref() {
            if status.ip_address.is_empty() {
                return Ok(());
            }

            let ip = status.ip_address.clone();
            let port = status.port;
            drop(current_status);

            match self.ping_server(&ip, port).await {
                Ok(Some(latency_ms)) => {
                    // Server is online with measured latency
                    let new_status = ServerStatus::with_latency(ip, port, latency_ms);
                    self.update_status(new_status).await?;
                }
                Ok(None) => {
                    // Server is offline or unreachable
                    let new_status = ServerStatus::offline(ip, port);
                    self.update_status(new_status).await?;
                }
                Err(e) => {
                    error!("Failed to ping server: {}", e);
                    return Err(e);
                }
            }
        } else {
        }
        Ok(())
    }

    /// Start periodic ping monitoring
    async fn start_ping_monitoring(&self) -> AppResult<()> {
        let mut is_active = self.is_monitoring.write().await;
        if *is_active {
            return Ok(());
        }

        *is_active = true;
        drop(is_active);

        // Load last known IP on startup
        self.load_last_known_ip().await?;

        // Spawn background task for periodic ping monitoring
        let service = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30)); // Ping every 30 seconds

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                if !*service.is_monitoring.read().await {
                    break;
                }

                // Perform ping operation
                if let Err(e) = service.ping_current_server().await {
                    error!("Failed to ping server during monitoring: {}", e);
                }
            }
        });

        info!("Started periodic ping monitoring");
        Ok(())
    }

    /// Stop periodic ping monitoring
    async fn stop_ping_monitoring(&self) -> AppResult<()> {
        let mut is_active = self.is_monitoring.write().await;
        if !*is_active {
            return Ok(());
        }

        *is_active = false;
        info!("Stopped periodic ping monitoring");
        Ok(())
    }
}

impl Clone for ServerMonitoringServiceImpl {
    fn clone(&self) -> Self {
        Self {
            ip_repository: Arc::clone(&self.ip_repository),
            event_bus: Arc::clone(&self.event_bus),
            current_status: Arc::clone(&self.current_status),
            is_monitoring: Arc::clone(&self.is_monitoring),
        }
    }
}

/// Simplified service trait for server monitoring operations.
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    /// Get current server status
    async fn get_current_status(&self) -> ServerStatus;

    /// Update server status from log analysis (when IP is found)
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;

    /// Ping current server and update status
    async fn ping_current_server(&self) -> AppResult<()>;

    /// Start periodic ping monitoring
    async fn start_ping_monitoring(&self) -> AppResult<()>;

    /// Stop periodic ping monitoring  
    async fn stop_ping_monitoring(&self) -> AppResult<()>;
}
