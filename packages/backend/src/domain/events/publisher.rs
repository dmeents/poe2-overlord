//! Event Publisher
//!
//! This module provides a simple publisher implementation for the unified event system.

use crate::domain::events::event_types::AppEvent;
use crate::domain::events::traits::EventPublisherTrait;
use crate::errors::AppResult;
use std::sync::Arc;

/// Simple event publisher implementation
///
/// This struct provides a simple implementation of the EventPublisherTrait
/// that delegates to an EventBus.
pub struct EventPublisher {
    event_bus: Arc<crate::domain::events::event_bus::EventBus>,
}

impl EventPublisher {
    /// Create a new event publisher
    pub fn new(event_bus: Arc<crate::domain::events::event_bus::EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait::async_trait]
impl EventPublisherTrait for EventPublisher {
    async fn publish(&self, event: AppEvent) -> AppResult<()> {
        self.event_bus.publish(event).await
    }

    async fn publish_batch(&self, events: Vec<AppEvent>) -> AppResult<()> {
        for event in events {
            self.event_bus.publish(event).await?;
        }
        Ok(())
    }
}
