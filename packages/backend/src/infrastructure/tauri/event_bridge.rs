//! Tauri Event Bridge
//!
//! This module provides a bridge between the unified event system and the Tauri frontend.
//! It subscribes to all events and forwards them to the frontend using Tauri's emit functionality.

use crate::domain::events::{AppEvent, EventBus, EventType};
use crate::errors::AppResult;
use log::{debug, error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

/// Bridge between the unified event system and Tauri frontend
///
/// This struct subscribes to all events from the EventBus and forwards them
/// to the Tauri frontend using the WebviewWindow's emit functionality.
pub struct TauriEventBridge {
    event_bus: Arc<EventBus>,
    window: WebviewWindow,
}

impl TauriEventBridge {
    /// Create a new Tauri event bridge
    pub fn new(event_bus: Arc<EventBus>, window: WebviewWindow) -> Self {
        Self { event_bus, window }
    }
    
    /// Start forwarding events to the frontend
    ///
    /// This method subscribes to all event types and starts forwarding them
    /// to the Tauri frontend. It runs as a background task.
    pub async fn start_forwarding(&self) -> AppResult<()> {
        info!("Starting Tauri event bridge forwarding");
        
        // Subscribe to all event types
        for event_type in EventType::all() {
            let subscription = self.event_bus
                .subscribe(event_type, "tauri-bridge".to_string())
                .await?;
            
            debug!("Subscribed to {:?} events with ID: {}", event_type, subscription.subscription_id);
            
            // Start forwarding task for this event type
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
    
    /// Forward a single event to the frontend
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
    
    /// Get the Tauri event name for an AppEvent
    fn get_event_name(event: &AppEvent) -> String {
        match event {
            AppEvent::LogParsed(_) => "log-event".to_string(),
            AppEvent::LogAnalysisError { .. } => "log-analysis-error".to_string(),
            AppEvent::ServerStatusChanged { .. } => "server-status-changed".to_string(),
            AppEvent::ServerPingCompleted { .. } => "server-ping-completed".to_string(),
            AppEvent::ConfigurationChanged(_) => "configuration-changed".to_string(),
            AppEvent::LocationStateChanged { .. } => "location-state-changed".to_string(),
            AppEvent::SceneChangeDetected { .. } => "scene-change-detected".to_string(),
            AppEvent::ActChangeDetected { .. } => "act-change-detected".to_string(),
            AppEvent::ZoneChangeDetected { .. } => "zone-change-detected".to_string(),
            AppEvent::HideoutChangeDetected { .. } => "hideout-change-detected".to_string(),
            AppEvent::GameProcessStatusChanged { .. } => "game-process-status-changed".to_string(),
            AppEvent::SystemError { .. } => "system-error".to_string(),
            AppEvent::SystemShutdown { .. } => "system-shutdown".to_string(),
        }
    }
    
    /// Publish an event through the event bus
    ///
    /// This method provides a convenient way to publish events from Tauri commands
    /// that will be forwarded to the frontend.
    pub async fn publish_event(&self, event: AppEvent) -> AppResult<()> {
        self.event_bus.publish(event).await
    }
    
    /// Get the event bus for direct access
    pub fn get_event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }
}
