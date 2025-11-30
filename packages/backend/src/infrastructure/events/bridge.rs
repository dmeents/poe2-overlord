//! Forwards events from EventBus to Tauri frontend.

use crate::errors::AppResult;
use crate::infrastructure::events::{AppEvent, EventBus, EventType};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct TauriEventBridge {
    event_bus: Arc<EventBus>,
    window: WebviewWindow,
}

impl TauriEventBridge {
    pub fn new(event_bus: Arc<EventBus>, window: WebviewWindow) -> Self {
        Self { event_bus, window }
    }

    pub async fn start_forwarding(&self) -> AppResult<()> {
        info!("Starting Tauri event bridge forwarding");

        for event_type in EventType::all() {
            let subscription = self
                .event_bus
                .subscribe(event_type, "tauri-bridge".to_string())
                .await?;

            debug!(
                "Subscribed to {:?} events with ID: {}",
                event_type, subscription.subscription_id
            );

            let event_bus = Arc::clone(&self.event_bus);
            let window = self.window.clone();

            tokio::spawn(async move {
                if let Ok(mut receiver) = event_bus.get_receiver(event_type).await {
                    while let Ok(event) = receiver.recv().await {
                        Self::forward_event_to_frontend(&window, &event).await;
                    }
                }
            });
        }

        Ok(())
    }

    async fn forward_event_to_frontend(window: &WebviewWindow, event: &AppEvent) {
        let event_name = Self::get_event_name(event);

        match window.emit(&event_name, event) {
            Ok(_) => {
                debug!("Forwarded event {} to frontend", event_name);
            }
            Err(e) => {
                error!("Failed to forward event {} to frontend: {}", event_name, e);
            }
        }
    }

    fn get_event_name(event: &AppEvent) -> String {
        match event {
            AppEvent::ServerStatusChanged { .. } => "server-status-changed".to_string(),
            AppEvent::ServerPingCompleted { .. } => "server-ping-completed".to_string(),
            AppEvent::ConfigurationChanged(_) => "configuration-changed".to_string(),
            AppEvent::LocationStateChanged { .. } => "location-state-changed".to_string(),
            AppEvent::SceneChangeDetected { .. } => "scene-change-detected".to_string(),
            AppEvent::ActChangeDetected { .. } => "act-change-detected".to_string(),
            AppEvent::ZoneChangeDetected { .. } => "zone-change-detected".to_string(),
            AppEvent::HideoutChangeDetected { .. } => "hideout-change-detected".to_string(),
            AppEvent::CharacterTrackingDataUpdated { .. } => {
                "character-tracking-data-updated".to_string()
            }
            AppEvent::WalkthroughProgressUpdated { .. } => {
                "walkthrough-progress-updated".to_string()
            }
            AppEvent::WalkthroughStepCompleted { .. } => "walkthrough-step-completed".to_string(),
            AppEvent::WalkthroughStepAdvanced { .. } => "walkthrough-step-advanced".to_string(),
            AppEvent::WalkthroughCampaignCompleted { .. } => {
                "walkthrough-campaign-completed".to_string()
            }
            AppEvent::GameProcessStatusChanged { .. } => "game-process-status-changed".to_string(),
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
