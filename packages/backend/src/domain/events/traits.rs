//! Event System Traits
//!
//! This module defines the core traits for the unified event system,
//! providing clean interfaces for event publishing and subscribing.

use crate::domain::events::event_types::{AppEvent, EventType, EventSubscription, ChannelConfig};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for event publishing operations
///
/// This trait defines the interface for publishing events in the unified event system.
/// It provides a clean abstraction over the event bus for components that need to
/// publish events.
#[async_trait]
pub trait EventPublisherTrait: Send + Sync {
    /// Publish an event to all subscribers
    async fn publish(&self, event: AppEvent) -> AppResult<()>;
    
    /// Publish multiple events in batch
    async fn publish_batch(&self, events: Vec<AppEvent>) -> AppResult<()> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}

/// Trait for event subscribing operations
///
/// This trait defines the interface for subscribing to events in the unified event system.
/// It provides a clean abstraction over the event bus for components that need to
/// subscribe to events.
#[async_trait]
pub trait EventSubscriberTrait: Send + Sync {
    /// Subscribe to events of a specific type
    async fn subscribe(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription>;
    
    /// Unsubscribe from events by subscription ID
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()>;
    
    /// Get a receiver for events of a specific type
    async fn get_receiver(&self, event_type: EventType) -> AppResult<broadcast::Receiver<AppEvent>>;
    
    /// Get the number of subscribers for a specific event type
    async fn get_subscriber_count(&self, event_type: EventType) -> usize;
}

/// Trait for event bus operations
///
/// This trait combines publishing and subscribing operations into a single interface
/// for components that need both capabilities.
#[async_trait]
pub trait EventBusTrait: EventPublisherTrait + EventSubscriberTrait {
    /// Get all active subscriptions
    async fn get_active_subscriptions(&self) -> Vec<EventSubscription>;
    
    /// Get statistics for a specific event type
    async fn get_stats(&self, event_type: EventType) -> Option<crate::domain::events::event_types::ChannelStats>;
    
    /// Get statistics for all event types
    async fn get_all_stats(&self) -> Vec<crate::domain::events::event_types::ChannelStats>;
    
    /// Update configuration for a specific event type
    async fn update_channel_config(
        &self,
        event_type: EventType,
        config: ChannelConfig,
    ) -> AppResult<()>;
    
    /// Get configuration for a specific event type
    async fn get_channel_config(&self, event_type: EventType) -> Option<ChannelConfig>;
    
    /// Get all active event types
    async fn get_active_event_types(&self) -> Vec<EventType>;
}

/// Trait for event handling
///
/// This trait defines the interface for components that handle specific types of events.
/// It provides a clean way to implement event handlers that can be registered with
/// the event system.
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle_event(&self, event: AppEvent) -> AppResult<()>;
    
    /// Get the event types this handler is interested in
    fn get_interested_event_types(&self) -> Vec<EventType>;
    
    /// Get the name of this handler
    fn get_handler_name(&self) -> String;
}

/// Trait for event filtering
///
/// This trait defines the interface for filtering events before they are processed.
/// It allows for implementing custom filtering logic for event handlers.
pub trait EventFilter: Send + Sync {
    /// Check if an event should be processed
    fn should_process(&self, event: &AppEvent) -> bool;
    
    /// Get the name of this filter
    fn get_filter_name(&self) -> String;
}

/// Trait for event transformation
///
/// This trait defines the interface for transforming events before they are processed.
/// It allows for implementing custom transformation logic for event handlers.
pub trait EventTransformer: Send + Sync {
    /// Transform an event
    fn transform(&self, event: AppEvent) -> AppEvent;
    
    /// Get the name of this transformer
    fn get_transformer_name(&self) -> String;
}

/// Trait for event persistence
///
/// This trait defines the interface for persisting events to storage.
/// It allows for implementing custom persistence logic for events.
#[async_trait]
pub trait EventPersistence: Send + Sync {
    /// Persist an event to storage
    async fn persist_event(&self, event: &AppEvent) -> AppResult<()>;
    
    /// Load events from storage
    async fn load_events(
        &self,
        event_type: Option<EventType>,
        limit: Option<usize>,
    ) -> AppResult<Vec<AppEvent>>;
    
    /// Clear events from storage
    async fn clear_events(&self, event_type: Option<EventType>) -> AppResult<()>;
}
