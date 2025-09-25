use crate::domain::configuration::models::{AppConfig, ConfigurationChangedEvent};
use crate::errors::AppResult;
use log::{debug, info, warn};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Configuration event handler for managing configuration change events
pub struct ConfigurationEventHandler {
    event_sender: broadcast::Sender<ConfigurationChangedEvent>,
}

impl ConfigurationEventHandler {
    /// Create a new configuration event handler
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(16);
        Self { event_sender }
    }

    /// Create a new handler with custom channel capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let (event_sender, _) = broadcast::channel(capacity);
        Self { event_sender }
    }

    /// Broadcast a configuration change event
    pub fn broadcast_config_change(
        &self,
        new_config: AppConfig,
        previous_config: AppConfig,
    ) -> AppResult<()> {
        let event = ConfigurationChangedEvent::new(new_config, previous_config);

        match self.event_sender.send(event) {
            Ok(receiver_count) => {
                debug!(
                    "Configuration change event broadcasted to {} receivers",
                    receiver_count
                );
                Ok(())
            }
            Err(broadcast::error::SendError(_event)) => {
                warn!("Failed to broadcast configuration change event: no receivers");
                Err(crate::errors::AppError::event_emission_error(
                    "emit_configuration_change",
                    "No receivers for configuration change event",
                ))
            }
        }
    }

    /// Subscribe to configuration change events
    pub fn subscribe(&self) -> broadcast::Receiver<ConfigurationChangedEvent> {
        self.event_sender.subscribe()
    }

    /// Get the number of active subscribers
    pub fn receiver_count(&self) -> usize {
        self.event_sender.receiver_count()
    }
}

impl Default for ConfigurationEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration event listener for handling configuration changes
pub struct ConfigurationEventListener {
    receiver: broadcast::Receiver<ConfigurationChangedEvent>,
}

impl ConfigurationEventListener {
    /// Create a new configuration event listener
    pub fn new(event_handler: &ConfigurationEventHandler) -> Self {
        Self {
            receiver: event_handler.subscribe(),
        }
    }

    /// Listen for the next configuration change event
    pub async fn listen_for_change(&mut self) -> AppResult<ConfigurationChangedEvent> {
        match self.receiver.recv().await {
            Ok(event) => {
                info!("Received configuration change event at {}", event.timestamp);
                Ok(event)
            }
            Err(broadcast::error::RecvError::Closed) => {
                Err(crate::errors::AppError::event_emission_error(
                    "emit_configuration_change",
                    "Configuration event channel closed",
                ))
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                warn!(
                    "Configuration event listener lagged, skipped {} events",
                    skipped
                );
                // Try to get the latest event
                Box::pin(self.listen_for_change()).await
            }
        }
    }

    /// Try to receive a configuration change event without blocking
    pub fn try_receive(&mut self) -> Option<ConfigurationChangedEvent> {
        match self.receiver.try_recv() {
            Ok(event) => {
                debug!("Received configuration change event at {}", event.timestamp);
                Some(event)
            }
            Err(broadcast::error::TryRecvError::Empty) => None,
            Err(broadcast::error::TryRecvError::Closed) => {
                warn!("Configuration event channel closed");
                None
            }
            Err(broadcast::error::TryRecvError::Lagged(skipped)) => {
                warn!(
                    "Configuration event listener lagged, skipped {} events",
                    skipped
                );
                // Try to get the latest event
                self.try_receive()
            }
        }
    }
}

/// Configuration event manager for coordinating multiple event handlers
pub struct ConfigurationEventManager {
    event_handler: Arc<ConfigurationEventHandler>,
}

impl ConfigurationEventManager {
    /// Create a new configuration event manager
    pub fn new() -> Self {
        Self {
            event_handler: Arc::new(ConfigurationEventHandler::new()),
        }
    }

    /// Create a new manager with custom channel capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            event_handler: Arc::new(ConfigurationEventHandler::with_capacity(capacity)),
        }
    }

    /// Get the event handler
    pub fn get_event_handler(&self) -> Arc<ConfigurationEventHandler> {
        self.event_handler.clone()
    }

    /// Broadcast a configuration change event
    pub fn broadcast_config_change(
        &self,
        new_config: AppConfig,
        previous_config: AppConfig,
    ) -> AppResult<()> {
        self.event_handler
            .broadcast_config_change(new_config, previous_config)
    }

    /// Create a new event listener
    pub fn create_listener(&self) -> ConfigurationEventListener {
        ConfigurationEventListener::new(&self.event_handler)
    }

    /// Get the number of active subscribers
    pub fn receiver_count(&self) -> usize {
        self.event_handler.receiver_count()
    }
}

impl Default for ConfigurationEventManager {
    fn default() -> Self {
        Self::new()
    }
}
