// Game monitoring imports removed - using unified event system
use crate::domain::log_analysis::models::LogEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use log::error;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

/// Event publisher that bridges backend events to the Tauri frontend
/// 
/// Handles both internal event broadcasting and Tauri-specific event emission
/// to the frontend application. Provides dual-channel event distribution.
pub struct EventPublisher {
    /// Internal broadcast channel for log events
    log_event_sender: broadcast::Sender<LogEvent>,
    /// Internal broadcast channel for ping events
    ping_event_sender: broadcast::Sender<ServerStatus>,
    /// Optional Tauri window for frontend communication
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

    /// Broadcasts log events both internally and to the frontend
    /// 
    /// Sends events to internal subscribers and emits them to the Tauri frontend
    /// if a window is available. Handles errors gracefully for both channels.
    pub fn broadcast_log_event(&self, event: LogEvent) -> Result<(), broadcast::error::SendError<LogEvent>> {
        let result = self.log_event_sender.send(event.clone());
        
        if let Err(ref e) = result {
            error!("Failed to broadcast log event: {}", e);
        }

        // Emit to frontend if window is available
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

// Note: ServerMonitoringEventPublisher implementation removed - using unified event system

// TauriGameMonitoringEventPublisher removed - using unified event system