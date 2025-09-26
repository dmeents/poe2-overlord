//! Event Subscriber
//!
//! This module provides a simple subscriber implementation for the unified event system.

use crate::domain::events::event_types::{AppEvent, EventType, EventSubscription};
use crate::domain::events::traits::EventSubscriberTrait;
use crate::errors::AppResult;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Simple event subscriber implementation
///
/// This struct provides a simple implementation of the EventSubscriberTrait
/// that delegates to an EventBus.
pub struct EventSubscriber {
    event_bus: Arc<crate::domain::events::event_bus::EventBus>,
}

impl EventSubscriber {
    /// Create a new event subscriber
    pub fn new(event_bus: Arc<crate::domain::events::event_bus::EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl EventSubscriberTrait for EventSubscriber {
    async fn subscribe(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription> {
        self.event_bus.subscribe(event_type, subscriber_name).await
    }
    
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()> {
        self.event_bus.unsubscribe(subscription_id).await
    }
    
    async fn get_receiver(&self, event_type: EventType) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(event_type).await
    }
    
    async fn get_subscriber_count(&self, event_type: EventType) -> usize {
        self.event_bus.get_subscriber_count(event_type).await
    }
}
