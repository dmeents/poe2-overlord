use crate::domain::game_monitoring::{
    events::GameMonitoringEvent, models::GameProcessStatus, traits::GameMonitoringEventPublisher,
};
use crate::errors::AppResult;
use crate::domain::log_analysis::models::LogEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use log::error;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

/// Generic event publisher for handling various types of events
pub struct EventPublisher {
    log_event_sender: broadcast::Sender<LogEvent>,
    ping_event_sender: broadcast::Sender<ServerStatus>,
    window: Option<WebviewWindow>,
}

impl EventPublisher {
    /// Create a new event publisher
    pub fn new() -> Self {
        let (log_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            log_event_sender,
            ping_event_sender,
            window: None,
        }
    }

    /// Create a new event publisher with Tauri window
    pub fn with_window(window: WebviewWindow) -> Self {
        let (log_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            log_event_sender,
            ping_event_sender,
            window: Some(window),
        }
    }

    /// Broadcast a log event
    pub fn broadcast_log_event(&self, event: LogEvent) -> Result<(), broadcast::error::SendError<LogEvent>> {
        let result = self.log_event_sender.send(event.clone());
        
        if let Err(ref e) = result {
            error!("Failed to broadcast log event: {}", e);
        }

        // Emit to Tauri window if available
        if let Some(ref window) = self.window {
            if let Err(e) = window.emit("log-event", &event) {
                error!("Failed to emit log event to frontend: {}", e);
            }
        }

        result.map(|_| ())
    }

    /// Broadcast a ping event
    pub fn broadcast_ping_event(&self, event: ServerStatus) -> Result<(), broadcast::error::SendError<ServerStatus>> {
        let result = self.ping_event_sender.send(event.clone());
        
        if let Err(ref e) = result {
            error!("Failed to broadcast ping event: {}", e);
        }

        // Emit to Tauri window if available
        if let Some(ref window) = self.window {
            if let Err(e) = window.emit("server-ping", &event) {
                error!("Failed to emit ping event to frontend: {}", e);
            }
        }

        result.map(|_| ())
    }

    /// Subscribe to log events
    pub fn subscribe_to_log_events(&self) -> broadcast::Receiver<LogEvent> {
        self.log_event_sender.subscribe()
    }

    /// Subscribe to ping events
    pub fn subscribe_to_ping_events(&self) -> broadcast::Receiver<ServerStatus> {
        self.ping_event_sender.subscribe()
    }

    /// Get the number of log event subscribers
    pub fn log_subscriber_count(&self) -> usize {
        self.log_event_sender.receiver_count()
    }

    /// Get the number of ping event subscribers
    pub fn ping_subscriber_count(&self) -> usize {
        self.ping_event_sender.receiver_count()
    }
}

impl Default for EventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

/// Tauri-specific implementation of the game monitoring event publisher
/// This handles emitting events to the frontend through Tauri's event system
pub struct TauriGameMonitoringEventPublisher {
    window: WebviewWindow,
}

impl TauriGameMonitoringEventPublisher {
    /// Create a new Tauri event publisher
    pub fn new(window: WebviewWindow) -> Self {
        Self { window }
    }

    /// Emit a game process status event to the frontend
    async fn emit_game_process_status(&self, status: &GameProcessStatus) -> AppResult<()> {
        if let Err(e) = self.window.emit("game-process-status", status) {
            error!("Failed to emit game process status: {}", e);
              return Err(crate::errors::AppError::event_emission_error("emit_game_process_status", &format!(
                "Failed to emit game process status: {}",
                e
            )));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl GameMonitoringEventPublisher for TauriGameMonitoringEventPublisher {
    async fn publish_event(&self, event: GameMonitoringEvent) -> AppResult<()> {
        match event {
            GameMonitoringEvent::StatusUpdated(updated_event) => {
                if updated_event.is_state_change {
                    log::info!("Publishing game process status change event");
                } else {
                    log::debug!("Publishing game process status update event");
                }
                self.emit_game_process_status(&updated_event.process_status)
                    .await?;
            }
        }
        Ok(())
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<GameMonitoringEvent> {
        // For Tauri integration, we don't need to provide a receiver
        // since events are emitted directly to the frontend
        // This could be implemented if we need internal event subscription
        let (_, receiver) = broadcast::channel(100);
        receiver
    }
}