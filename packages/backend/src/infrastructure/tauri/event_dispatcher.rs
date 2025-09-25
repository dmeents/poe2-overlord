use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::log_analysis::models::LogEvent;
use log::debug;
use tokio::sync::broadcast;

/// Trait for event service operations
/// 
/// Defines the interface for subscribing to and broadcasting different types of events
/// within the application's event system.
pub trait EventService: Send + Sync {
    fn subscribe_to_log_events(&self) -> broadcast::Receiver<LogEvent>;
    fn subscribe_to_ping_events(&self) -> broadcast::Receiver<ServerStatus>;
    fn broadcast_log_event(
        &self,
        event: LogEvent,
    ) -> Result<(), broadcast::error::SendError<LogEvent>>;
    fn broadcast_ping_event(
        &self,
        event: ServerStatus,
    ) -> Result<(), broadcast::error::SendError<ServerStatus>>;
}

/// Central event dispatcher for managing application-wide event broadcasting
/// 
/// Provides a unified interface for broadcasting and subscribing to different types
/// of events. Uses Tokio's broadcast channels for efficient multi-consumer event distribution.
pub struct EventDispatcher {
    /// Sender for log-related events (scene changes, character events, etc.)
    pub unified_event_sender: broadcast::Sender<LogEvent>,
    /// Sender for server ping/connectivity events
    pub ping_event_sender: broadcast::Sender<ServerStatus>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        let (unified_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            unified_event_sender,
            ping_event_sender,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LogEvent> {
        self.unified_event_sender.subscribe()
    }

    pub fn subscribe_ping_events(&self) -> broadcast::Receiver<ServerStatus> {
        self.ping_event_sender.subscribe()
    }

    /// Broadcasts a log event to all subscribers
    /// 
    /// Sends the event to all active subscribers and logs the result.
    /// Returns an error if there are no subscribers to receive the event.
    pub fn broadcast_event(
        &self,
        event: LogEvent,
    ) -> Result<(), broadcast::error::SendError<LogEvent>> {
        debug!("Broadcasting log event: {:?}", event);
        debug!(
            "Current subscriber count: {}",
            self.unified_event_sender.receiver_count()
        );

        let result = self.unified_event_sender.send(event.clone());

        if let Err(ref e) = result {
            debug!("Failed to broadcast log event: {}", e);
        } else {
            debug!("Log event broadcast successfully");
        }

        result.map(|_| ())
    }

    pub fn broadcast_ping_event(
        &self,
        event: ServerStatus,
    ) -> Result<(), broadcast::error::SendError<ServerStatus>> {
        let result = self.ping_event_sender.send(event.clone());

        if let Err(ref e) = result {
            debug!("Failed to broadcast ping event: {}", e);
        }

        result.map(|_| ())
    }

    pub fn subscriber_count(&self) -> usize {
        self.unified_event_sender.receiver_count()
    }

    pub fn ping_subscriber_count(&self) -> usize {
        self.ping_event_sender.receiver_count()
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventService for EventDispatcher {
    fn subscribe_to_log_events(&self) -> broadcast::Receiver<LogEvent> {
        self.subscribe()
    }

    fn subscribe_to_ping_events(&self) -> broadcast::Receiver<ServerStatus> {
        self.subscribe_ping_events()
    }

    fn broadcast_log_event(
        &self,
        event: LogEvent,
    ) -> Result<(), broadcast::error::SendError<LogEvent>> {
        self.broadcast_event(event)
    }

    fn broadcast_ping_event(
        &self,
        event: ServerStatus,
    ) -> Result<(), broadcast::error::SendError<ServerStatus>> {
        self.broadcast_ping_event(event)
    }
}
