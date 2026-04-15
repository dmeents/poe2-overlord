use crate::errors::AppResult;
use crate::infrastructure::events::{AppEvent, EventBus, EventType};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub struct TauriEventBridge {
    event_bus: Arc<EventBus>,
    window: WebviewWindow,
    forwarding_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    cancellation_token: CancellationToken,
}

impl TauriEventBridge {
    pub fn new(event_bus: Arc<EventBus>, window: WebviewWindow) -> Self {
        Self {
            event_bus,
            window,
            forwarding_tasks: Arc::new(RwLock::new(Vec::new())),
            cancellation_token: CancellationToken::new(),
        }
    }

    pub async fn start_forwarding(&self) -> AppResult<()> {
        info!("Starting Tauri event bridge forwarding");

        let mut handles = Vec::new();

        for event_type in EventType::all() {
            let event_bus = Arc::clone(&self.event_bus);
            let window = self.window.clone();
            let cancellation_token = self.cancellation_token.clone();

            debug!("Starting forwarding for event type: {event_type:?}");

            let handle = tokio::spawn(async move {
                if let Ok(mut receiver) = event_bus.get_receiver(event_type).await {
                    loop {
                        tokio::select! {
                            () = cancellation_token.cancelled() => {
                                info!("Forwarding task for {event_type:?} cancelled");
                                break;
                            }
                            result = receiver.recv() => {
                                match result {
                                    Ok(event) => {
                                        Self::forward_event_to_frontend(&window, &event).await;
                                    }
                                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                        error!(
                                            "Event bridge lagged by {n} events for {event_type:?}, continuing"
                                        );
                                        // Continue receiving - receiver is still valid
                                    }
                                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                        error!(
                                            "Event channel closed for {event_type:?}, exiting forwarding task"
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error!("Failed to get receiver for event type: {event_type:?}");
                }
            });

            handles.push(handle);
        }

        // Store handles for later cleanup
        *self.forwarding_tasks.write().await = handles;

        Ok(())
    }

    pub async fn stop_forwarding(&self) -> AppResult<()> {
        info!("Stopping Tauri event bridge forwarding");

        // Signal all tasks to stop
        self.cancellation_token.cancel();

        // Wait for all tasks to complete
        let mut handles = self.forwarding_tasks.write().await;
        for handle in handles.drain(..) {
            if let Err(e) = handle.await {
                error!("Error waiting for forwarding task to complete: {e}");
            }
        }

        info!("All forwarding tasks stopped");
        Ok(())
    }

    async fn forward_event_to_frontend(window: &WebviewWindow, event: &AppEvent) {
        let event_name = Self::get_event_name(event);

        match window.emit(&event_name, event) {
            Ok(()) => {
                debug!("Forwarded event {event_name} to frontend");
            }
            Err(e) => {
                error!("Failed to forward event {event_name} to frontend: {e}");
            }
        }
    }

    fn get_event_name(event: &AppEvent) -> String {
        match event {
            AppEvent::ServerStatusChanged { .. } => "server-status-changed".to_string(),
            AppEvent::ConfigurationChanged(_) => "configuration-changed".to_string(),
            AppEvent::CharacterUpdated { .. } => "character-updated".to_string(),
            AppEvent::CharacterDeleted { .. } => "character-deleted".to_string(),
            AppEvent::WalkthroughStepCompleted { .. } => "walkthrough-step-completed".to_string(),
            AppEvent::WalkthroughStepAdvanced { .. } => "walkthrough-step-advanced".to_string(),
            AppEvent::WalkthroughCampaignCompleted { .. } => {
                "walkthrough-campaign-completed".to_string()
            }
            AppEvent::GameProcessStatusChanged { .. } => "game-process-status-changed".to_string(),
            AppEvent::LevelingStatsUpdated { .. } => "leveling-stats-updated".to_string(),
            AppEvent::SystemError { .. } => "system-error".to_string(),
            AppEvent::SystemShutdown { .. } => "system-shutdown".to_string(),
        }
    }

    pub async fn publish_event(&self, event: AppEvent) -> AppResult<()> {
        self.event_bus.publish(event).await
    }

    pub fn get_event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }
}
