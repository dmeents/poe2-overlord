//! Server monitoring service for tracking server status, ping operations, and event publishing.

use crate::domain::events::{AppEvent, EventBus};
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::{
    PingProvider, ServerMonitoringService, ServerStatusRepository,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

const MONITORING_INTERVAL_SECS: u64 = 30;

pub struct ServerMonitoringServiceImpl {
    repository: Arc<dyn ServerStatusRepository>,
    event_bus: Arc<EventBus>,
    ping_provider: Arc<dyn PingProvider>,
    cached_status: Arc<RwLock<Option<ServerStatus>>>,
    monitoring_active: Arc<RwLock<bool>>,
}

impl ServerMonitoringServiceImpl {
    pub async fn new(
        event_bus: Arc<EventBus>,
        ping_provider: Arc<dyn PingProvider>,
        repository: Arc<dyn ServerStatusRepository>,
    ) -> AppResult<Self> {
        Ok(Self {
            repository,
            event_bus,
            ping_provider,
            cached_status: Arc::new(RwLock::new(None)),
            monitoring_active: Arc::new(RwLock::new(false)),
        })
    }
}

impl Clone for ServerMonitoringServiceImpl {
    fn clone(&self) -> Self {
        Self {
            repository: Arc::clone(&self.repository),
            event_bus: Arc::clone(&self.event_bus),
            ping_provider: Arc::clone(&self.ping_provider),
            cached_status: Arc::clone(&self.cached_status),
            monitoring_active: Arc::clone(&self.monitoring_active),
        }
    }
}

impl ServerMonitoringServiceImpl {
    async fn load_from_file(&self) -> AppResult<()> {
        if let Some(status) = self.repository.load().await? {
            info!(
                "Loaded server status from file: {}:{}",
                status.ip_address, status.port
            );

            *self.cached_status.write().await = Some(status.clone());

            let event = AppEvent::server_status_changed(None, status);
            if let Err(e) = self.event_bus.publish(event).await {
                error!("Failed to publish initial server status event: {}", e);
            }
        } else {
            debug!("No existing server status file found");
        }
        Ok(())
    }

    async fn update_and_persist(&self, status: ServerStatus) -> AppResult<()> {
        self.repository.save(&status).await?;

        let old_status = {
            let mut cached = self.cached_status.write().await;
            let old = cached.clone();
            *cached = Some(status.clone());
            old
        };

        let event = AppEvent::server_status_changed(old_status, status);
        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish server status event: {}", e);
        }

        Ok(())
    }

    async fn ping_server(&self, ip_address: &str) -> Result<u64, String> {
        self.ping_provider.ping(ip_address).await
    }
}

#[async_trait]
impl ServerMonitoringService for ServerMonitoringServiceImpl {
    async fn update_server_from_log(&self, ip_address: String, port: u16) -> AppResult<()> {
        info!(
            "Server connection detected from logs: {}:{}",
            ip_address, port
        );

        let status = ServerStatus::new(ip_address, port);
        self.update_and_persist(status).await
    }

    async fn ping_current_server(&self) -> AppResult<()> {
        let cached_status = self.cached_status.read().await;

        let mut status = match cached_status.as_ref() {
            Some(s) => s.clone(),
            None => {
                debug!("No server status available to ping");
                return Ok(());
            }
        };

        if !status.is_valid() {
            debug!("No valid server IP to ping");
            return Ok(());
        }

        let ip = status.ip_address.clone();
        drop(cached_status);

        match self.ping_server(&ip).await {
            Ok(latency_ms) => {
                debug!("Server ping successful: {}ms", latency_ms);
                status.mark_as_online(latency_ms);
            }
            Err(e) => {
                debug!("Server ping failed: {}", e);
                status.mark_as_offline();
            }
        }

        self.update_and_persist(status).await
    }

    async fn start_ping_monitoring(&self) -> AppResult<()> {
        let mut is_active = self.monitoring_active.write().await;
        if *is_active {
            warn!("Ping monitoring is already active");
            return Ok(());
        }

        *is_active = true;
        drop(is_active);

        self.load_from_file().await?;

        let service = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(MONITORING_INTERVAL_SECS));

            info!(
                "Started periodic server ping monitoring ({}s interval)",
                MONITORING_INTERVAL_SECS
            );

            loop {
                interval.tick().await;

                if !*service.monitoring_active.read().await {
                    info!("Stopping server ping monitoring");
                    break;
                }

                if let Err(e) = service.ping_current_server().await {
                    error!("Failed to ping server during monitoring: {}", e);
                }
            }
        });

        Ok(())
    }

    async fn stop_ping_monitoring(&self) -> AppResult<()> {
        let mut is_active = self.monitoring_active.write().await;
        if !*is_active {
            debug!("Ping monitoring is not active");
            return Ok(());
        }

        *is_active = false;
        info!("Ping monitoring stopped");
        Ok(())
    }
}
