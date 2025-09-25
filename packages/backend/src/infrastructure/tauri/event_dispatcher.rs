use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::log_analysis::models::LogEvent;
use log::debug;
use tokio::sync::broadcast;

/// Trait for event dispatching infrastructure
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

/// Event dispatcher that manages event channels and subscriptions
pub struct EventDispatcher {
    pub unified_event_sender: broadcast::Sender<LogEvent>,
    pub ping_event_sender: broadcast::Sender<ServerStatus>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        let (unified_event_sender, _) = broadcast::channel(1000);
        let (ping_event_sender, _) = broadcast::channel(100);

        Self {
            unified_event_sender,
            ping_event_sender,
        }
    }

    /// Get the event receiver for subscribing to all log events
    pub fn subscribe(&self) -> broadcast::Receiver<LogEvent> {
        self.unified_event_sender.subscribe()
    }

    /// Get the event receiver for subscribing to server ping events
    pub fn subscribe_ping_events(&self) -> broadcast::Receiver<ServerStatus> {
        self.ping_event_sender.subscribe()
    }

    /// Broadcast a unified log event to all subscribers
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

        // Convert the Result<usize, SendError> to Result<(), SendError>
        result.map(|_| ())
    }

    /// Broadcast a server ping event to all subscribers
    pub fn broadcast_ping_event(
        &self,
        event: ServerStatus,
    ) -> Result<(), broadcast::error::SendError<ServerStatus>> {
        let result = self.ping_event_sender.send(event.clone());

        if let Err(ref e) = result {
            debug!("Failed to broadcast ping event: {}", e);
        }

        // Convert the Result<usize, SendError> to Result<(), SendError>
        result.map(|_| ())
    }

    /// Get the number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.unified_event_sender.receiver_count()
    }

    /// Get the number of active ping event subscribers
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
