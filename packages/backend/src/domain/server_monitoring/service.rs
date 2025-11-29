//! # Server Monitoring Service Implementation
//!
//! Simplified service for server monitoring functionality.
//!
//! Responsibilities:
//! - Update server status when detected from game logs
//! - Periodically ping server to check connectivity
//! - Persist status to file (server_status.json in app data directory)
//! - Emit events on status changes

use crate::domain::events::{AppEvent, EventBus};
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::repository::ServerStatusRepository;
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

/// Service implementation for server monitoring operations.
///
/// Handles server status tracking, ping operations, persistence, and event publishing.
pub struct ServerMonitoringServiceImpl {
    /// Repository for server status persistence
    repository: Arc<ServerStatusRepository>,
    /// Event bus for publishing monitoring events
    event_bus: Arc<EventBus>,
    /// Current server status cache (in-memory)
    current_status: Arc<RwLock<Option<ServerStatus>>>,
    /// Flag indicating if periodic ping monitoring is active
    is_monitoring: Arc<RwLock<bool>>,
}

impl ServerMonitoringServiceImpl {
    /// Create a new server monitoring service instance.
    pub fn new(event_bus: Arc<EventBus>) -> AppResult<Self> {
        let repository = Arc::new(ServerStatusRepository::new()?);

        Ok(Self {
            repository,
            event_bus,
            current_status: Arc::new(RwLock::new(None)),
            is_monitoring: Arc::new(RwLock::new(false)),
        })
    }

    /// Load status from file on startup and emit event.
    async fn load_from_file(&self) -> AppResult<()> {
        if let Some(status) = self.repository.load().await? {
            info!(
                "Loaded server status from file: {}:{}",
                status.ip_address, status.port
            );

            // Update in-memory cache
            {
                let mut current = self.current_status.write().await;
                *current = Some(status.clone());
            }

            // Emit initial event so frontend gets the status
            let event = AppEvent::server_status_changed(None, status);
            if let Err(e) = self.event_bus.publish(event).await {
                error!("Failed to publish initial server status event: {}", e);
            }
        } else {
            debug!("No existing server status file found");
        }
        Ok(())
    }

    /// Update status, persist to file, and emit event.
    ///
    /// This is the central method that ensures consistency:
    /// 1. Save to file first (so we have persistence even if event fails)
    /// 2. Update in-memory cache
    /// 3. Emit event to frontend
    async fn update_and_persist(&self, status: ServerStatus) -> AppResult<()> {
        // Save to file
        self.repository.save(&status).await?;

        // Update in-memory cache and capture old status
        let old_status = {
            let mut current = self.current_status.write().await;
            let old = current.clone();
            *current = Some(status.clone());
            old
        };

        // Emit event
        let event = AppEvent::server_status_changed(old_status, status);
        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish server status event: {}", e);
        }

        Ok(())
    }

    /// Perform a ping to the server using system ping command.
    ///
    /// Uses ICMP ping with 1 attempt and 5 second timeout.
    /// Returns latency in milliseconds on success, or error on failure.
    async fn ping_server_internal(&self, ip_address: &str) -> Result<u64, String> {
        let start = std::time::Instant::now();

        // Use system ping command for ICMP ping
        // Note: This requires the ping command to be available on the system
        let output = tokio::process::Command::new("ping")
            .arg("-c") // Count (Unix/Linux/macOS)
            .arg("1")
            .arg("-W") // Timeout in seconds (Unix/Linux/macOS)
            .arg("5")
            .arg(ip_address)
            .output()
            .await;

        match output {
            Ok(result) => {
                if result.status.success() {
                    let ping_ms = start.elapsed().as_millis() as u64;
                    Ok(ping_ms)
                } else {
                    Err("Ping failed: server unreachable".to_string())
                }
            }
            Err(e) => Err(format!("Ping command failed: {}", e)),
        }
    }
}

impl Clone for ServerMonitoringServiceImpl {
    fn clone(&self) -> Self {
        Self {
            repository: Arc::clone(&self.repository),
            event_bus: Arc::clone(&self.event_bus),
            current_status: Arc::clone(&self.current_status),
            is_monitoring: Arc::clone(&self.is_monitoring),
        }
    }
}

/// Simplified service trait for server monitoring operations.
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    /// Update server status from log detection (when IP is found in logs)
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()>;

    /// Ping current server and update status
    async fn ping_current_server(&self) -> AppResult<()>;

    /// Start periodic ping monitoring
    async fn start_ping_monitoring(&self) -> AppResult<()>;
}

#[async_trait]
impl ServerMonitoringService for ServerMonitoringServiceImpl {
    /// Update server status from log analysis (when IP is detected in logs).
    ///
    /// Called by LogAnalysisService when it detects a server connection line.
    /// Creates a new status with the detected IP and port, saves to file, and emits event.
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()> {
        info!(
            "Server connection detected from logs: {}:{}",
            ip_address, port
        );

        // Create new status (initially unknown online status, will be checked by ping)
        let status = ServerStatus::new(ip_address, port);

        // Update, persist, and emit event
        self.update_and_persist(status).await
    }

    /// Ping current server and update status.
    ///
    /// Called periodically by the background monitoring task.
    /// Updates the status with current latency and online/offline state.
    async fn ping_current_server(&self) -> AppResult<()> {
        let current_status = self.current_status.read().await;

        if let Some(mut status) = current_status.clone() {
            // Skip if no valid IP
            if !status.is_valid() {
                debug!("No valid server IP to ping");
                return Ok(());
            }

            let ip = status.ip_address.clone();
            drop(current_status);

            // Perform ping
            match self.ping_server_internal(&ip).await {
                Ok(latency_ms) => {
                    debug!("Server ping successful: {}ms", latency_ms);
                    status.mark_as_online(latency_ms);
                }
                Err(e) => {
                    debug!("Server ping failed: {}", e);
                    status.mark_as_offline();
                }
            }

            // Update, persist, and emit event
            self.update_and_persist(status).await?;
        } else {
            debug!("No server status available to ping");
        }

        Ok(())
    }

    /// Start periodic ping monitoring.
    ///
    /// Spawns a background task that pings the server every 30 seconds.
    /// Also loads any existing server status from file on startup.
    async fn start_ping_monitoring(&self) -> AppResult<()> {
        let mut is_active = self.is_monitoring.write().await;
        if *is_active {
            warn!("Ping monitoring is already active");
            return Ok(());
        }

        *is_active = true;
        drop(is_active);

        // Load last known server status from file on startup
        self.load_from_file().await?;

        // Spawn background task for periodic ping monitoring
        let service = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30)); // Ping every 30 seconds

            info!("Started periodic server ping monitoring (30s interval)");

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                if !*service.is_monitoring.read().await {
                    info!("Stopping server ping monitoring");
                    break;
                }

                // Perform ping operation
                if let Err(e) = service.ping_current_server().await {
                    error!("Failed to ping server during monitoring: {}", e);
                }
            }
        });

        Ok(())
    }
}
