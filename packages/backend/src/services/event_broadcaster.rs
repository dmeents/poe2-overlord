use crate::models::events::{ActChangeEvent, SceneChangeEvent, ZoneChangeEvent, ServerConnectionEvent};
use log::debug;
use tokio::sync::broadcast;

/// Event broadcaster that manages event channels and subscriptions
pub struct EventBroadcaster {
    pub scene_event_sender: broadcast::Sender<SceneChangeEvent>,
    pub server_event_sender: broadcast::Sender<ServerConnectionEvent>,
}

impl EventBroadcaster {
    /// Create a new event broadcaster
    pub fn new() -> Self {
        let (scene_event_sender, _) = broadcast::channel(1000);
        let (server_event_sender, _) = broadcast::channel(100);

        Self { 
            scene_event_sender,
            server_event_sender,
        }
    }

    /// Get the event receiver for subscribing to scene change events
    pub fn subscribe(&self) -> broadcast::Receiver<SceneChangeEvent> {
        self.scene_event_sender.subscribe()
    }

    /// Get the event receiver for subscribing to zone change events
    pub fn subscribe_zones(&self) -> broadcast::Receiver<ZoneChangeEvent> {
        let (zone_sender, zone_receiver) = broadcast::channel(100);
        let mut scene_receiver = self.scene_event_sender.subscribe();

        // Spawn a task to filter zone events with better resource management
        tokio::spawn(async move {
            while let Ok(event) = scene_receiver.recv().await {
                if let SceneChangeEvent::Zone(zone_event) = event {
                    // Use send to avoid blocking if receiver is slow
                    if zone_sender.send(zone_event).is_err() {
                        // Receiver is not keeping up, skip this event
                        break;
                    }
                }
            }
        });

        zone_receiver
    }

    /// Get the event receiver for subscribing to act change events
    pub fn subscribe_acts(&self) -> broadcast::Receiver<ActChangeEvent> {
        let (act_sender, act_receiver) = broadcast::channel(100);
        let mut scene_receiver = self.scene_event_sender.subscribe();

        // Spawn a task to filter act events with better resource management
        tokio::spawn(async move {
            while let Ok(event) = scene_receiver.recv().await {
                if let SceneChangeEvent::Act(act_event) = event {
                    // Use send to avoid blocking if receiver is slow
                    if act_sender.send(act_event).is_err() {
                        // Receiver is not keeping up, skip this event
                        break;
                    }
                }
            }
        });

        act_receiver
    }

    /// Get the event receiver for subscribing to server connection events
    pub fn subscribe_server_events(&self) -> broadcast::Receiver<ServerConnectionEvent> {
        self.server_event_sender.subscribe()
    }

    /// Broadcast a scene change event to all subscribers
    pub fn broadcast_event(
        &self,
        event: SceneChangeEvent,
    ) -> Result<(), broadcast::error::SendError<SceneChangeEvent>> {
        let result = self.scene_event_sender.send(event.clone());

        if let Err(ref e) = result {
            debug!("Failed to broadcast scene change event: {}", e);
        }

        // Convert the Result<usize, SendError> to Result<(), SendError>
        result.map(|_| ())
    }

    /// Broadcast a server connection event to all subscribers
    pub fn broadcast_server_event(
        &self,
        event: ServerConnectionEvent,
    ) -> Result<(), broadcast::error::SendError<ServerConnectionEvent>> {
        let result = self.server_event_sender.send(event.clone());

        if let Err(ref e) = result {
            debug!("Failed to broadcast server connection event: {}", e);
        }

        // Convert the Result<usize, SendError> to Result<(), SendError>
        result.map(|_| ())
    }

    /// Get the number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.scene_event_sender.receiver_count()
    }

    /// Get the number of active server event subscribers
    pub fn server_subscriber_count(&self) -> usize {
        self.server_event_sender.receiver_count()
    }
}

impl Default for EventBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}
