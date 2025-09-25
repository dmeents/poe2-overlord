use crate::application::services::GameMonitoringApplicationService;
use crate::domain::game_monitoring::{
    events::GameMonitoringEvent,
    models::GameProcessStatus,
    traits::GameMonitoringEventPublisher,
};
use crate::errors::AppResult;
use crate::handlers::{runtime_manager::RuntimeManager, task_manager::TaskManager};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

/// Infrastructure handler for game monitoring that integrates with Tauri
/// This handles the technical concerns of event emission and task management
pub struct GameMonitoringHandler;

impl GameMonitoringHandler {
    /// Start game monitoring with Tauri integration
    pub async fn start_monitoring(
        _window: WebviewWindow,
        application_service: Arc<GameMonitoringApplicationService>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) -> AppResult<()> {
        info!("Starting game monitoring with Tauri integration");

        // The event publisher is already created in the domain service
        // We just need to start the monitoring
        
        // Start the monitoring task
        let app_service_clone = application_service.clone();
        let handle = runtime_manager.spawn_background_task(
            "game_process_monitoring".to_string(),
            move || async move {
                if let Err(e) = app_service_clone.start_monitoring().await {
                    error!("Game monitoring application service failed: {}", e);
                }
            },
        );

        // Register the task for lifecycle management
        task_manager
            .register_task("game_process_monitoring".to_string(), handle)
            .await;

        info!("Game monitoring started successfully");
        Ok(())
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
            GameMonitoringEvent::ProcessStarted(started_event) => {
                info!("Publishing game process started event");
                self.emit_game_process_status(&started_event.process_status).await?;
            }
            GameMonitoringEvent::ProcessStopped(stopped_event) => {
                info!("Publishing game process stopped event");
                self.emit_game_process_status(&stopped_event.process_status).await?;
            }
            GameMonitoringEvent::StatusUpdated(updated_event) => {
                debug!("Publishing game process status updated event");
                self.emit_game_process_status(&updated_event.process_status).await?;
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
