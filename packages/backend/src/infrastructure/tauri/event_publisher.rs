use crate::domain::game_monitoring::{
    events::GameMonitoringEvent, models::GameProcessStatus, traits::GameMonitoringEventPublisher,
};
use crate::errors::AppResult;
use crate::domain::log_analysis::models::LogEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use log::error;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

pub struct EventPublisher {
    log_event_sender: broadcast::Sender<LogEvent>,
    ping_event_sender: broadcast::Sender<ServerStatus>,
    window: Option<WebviewWindow>,
}

impl EventPublisher {
    pub fn new() -> Self {
        let (log_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            log_event_sender,
            ping_event_sender,
            window: None,
        }
    }

    pub fn with_window(window: WebviewWindow) -> Self {
        let (log_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            log_event_sender,
            ping_event_sender,
            window: Some(window),
        }
    }

    pub fn broadcast_log_event(&self, event: LogEvent) -> Result<(), broadcast::error::SendError<LogEvent>> {
        let result = self.log_event_sender.send(event.clone());
        
        if let Err(ref e) = result {
            error!("Failed to broadcast log event: {}", e);
        }

        if let Some(ref window) = self.window {
            if let Err(e) = window.emit("log-event", &event) {
                error!("Failed to emit log event to frontend: {}", e);
            }
        }

        result.map(|_| ())
    }

    pub fn broadcast_ping_event(&self, event: ServerStatus) -> Result<(), broadcast::error::SendError<ServerStatus>> {
        let result = self.ping_event_sender.send(event.clone());
        
        if let Err(ref e) = result {
            error!("Failed to broadcast ping event: {}", e);
        }

        if let Some(ref window) = self.window {
            if let Err(e) = window.emit("server-ping", &event) {
                error!("Failed to emit ping event to frontend: {}", e);
            }
        }

        result.map(|_| ())
    }

    pub fn subscribe_to_log_events(&self) -> broadcast::Receiver<LogEvent> {
        self.log_event_sender.subscribe()
    }

    pub fn subscribe_to_ping_events(&self) -> broadcast::Receiver<ServerStatus> {
        self.ping_event_sender.subscribe()
    }

    pub fn log_subscriber_count(&self) -> usize {
        self.log_event_sender.receiver_count()
    }

    pub fn ping_subscriber_count(&self) -> usize {
        self.ping_event_sender.receiver_count()
    }
}

impl Default for EventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TauriGameMonitoringEventPublisher {
    window: WebviewWindow,
}

impl TauriGameMonitoringEventPublisher {
    pub fn new(window: WebviewWindow) -> Self {
        Self { window }
    }

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
        let (_, receiver) = broadcast::channel(100);
        receiver
    }
}