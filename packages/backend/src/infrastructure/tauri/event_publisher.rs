use crate::domain::game_monitoring::{
    events::GameMonitoringEvent, models::GameProcessStatus, traits::GameMonitoringEventPublisher,
};
use crate::errors::AppResult;
use log::error;
// Arc not needed for this implementation
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

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
            return Err(crate::errors::AppError::event_emission_error(&format!(
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
