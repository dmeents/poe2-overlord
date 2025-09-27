//! Event Bus
//!
//! This module provides the central event bus for the unified event system.
//! It coordinates between publishers, subscribers, and channel management.

use crate::domain::events::channel_manager::ChannelManager;
use crate::domain::events::event_types::{AppEvent, ChannelConfig, EventSubscription, EventType};
use crate::errors::AppResult;
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Central event bus for the application
///
/// The EventBus is the single point of coordination for all events in the application.
/// It manages channels, handles publishing and subscribing, and provides statistics.
pub struct EventBus {
    /// Channel manager for handling broadcast channels
    channel_manager: Arc<ChannelManager>,
    /// Active subscriptions tracking
    subscriptions: Arc<RwLock<HashMap<String, EventSubscription>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            channel_manager: Arc::new(ChannelManager::new()),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Publish an event to all subscribers of its type
    ///
    /// This method will:
    /// 1. Get or create a channel for the event type
    /// 2. Send the event to all subscribers
    /// 3. Update statistics
    /// 4. Handle any errors gracefully
    pub async fn publish(&self, event: AppEvent) -> AppResult<()> {
        let event_type = event.event_type();

        // Get or create the channel for this event type
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;

        // Send the event to all subscribers
        match channel.sender().send(event.clone()) {
            Ok(receiver_count) => {
                if receiver_count > 0 {
                    debug!(
                        "Published event of type {:?} to {} subscribers",
                        event_type, receiver_count
                    );
                } else {
                    debug!("Published event of type {:?} (no subscribers)", event_type);
                }

                // Update statistics
                self.channel_manager
                    .increment_published_events(event_type)
                    .await;

                Ok(())
            }
            Err(broadcast::error::SendError(_event)) => {
                error!(
                    "Failed to publish event of type {:?}: no receivers",
                    event_type
                );
                Err(crate::errors::AppError::internal_error(
                    "publish_event",
                    &format!("No receivers for event type {:?}", event_type),
                ))
            }
        }
    }

    /// Subscribe to events of a specific type
    ///
    /// This method will:
    /// 1. Get or create a channel for the event type
    /// 2. Create a new subscription
    /// 3. Return a receiver for the events
    /// 4. Track the subscription
    pub async fn subscribe(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription> {
        // Get or create the channel for this event type
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;

        // Check if the channel is at capacity
        if channel.is_at_capacity() {
            return Err(crate::errors::AppError::internal_error(
                "subscribe",
                &format!("Channel for event type {:?} is at capacity", event_type),
            ));
        }

        // Create the subscription
        let subscription = EventSubscription::new(event_type, subscriber_name);
        let subscription_id = subscription.subscription_id.clone();

        // Store the subscription
        {
            let mut subscriptions = self.subscriptions.write().await;
            subscriptions.insert(subscription_id.clone(), subscription.clone());
        }

        debug!(
            "Created subscription {} for event type {:?}",
            subscription_id, event_type
        );

        Ok(subscription)
    }

    /// Unsubscribe from events by subscription ID
    ///
    /// This method will:
    /// 1. Find the subscription by ID
    /// 2. Deactivate the subscription
    /// 3. Remove it from tracking
    pub async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()> {
        let mut subscriptions = self.subscriptions.write().await;

        if let Some(mut subscription) = subscriptions.remove(subscription_id) {
            subscription.deactivate();
            debug!(
                "Unsubscribed from event type: {:?}",
                subscription.event_type
            );
        } else {
            debug!("Subscription not found: {}", subscription_id);
        }

        Ok(())
    }

    /// Get a receiver for events of a specific type
    ///
    /// This method provides direct access to the broadcast receiver
    /// for cases where you need more control over event handling.
    pub async fn get_receiver(
        &self,
        event_type: EventType,
    ) -> AppResult<broadcast::Receiver<AppEvent>> {
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;
        Ok(channel.sender().subscribe())
    }

    /// Get the number of subscribers for a specific event type
    pub async fn get_subscriber_count(&self, event_type: EventType) -> usize {
        if let Some(channel) = self.channel_manager.get_channel(event_type).await {
            channel.subscriber_count()
        } else {
            0
        }
    }

    /// Get all active subscriptions
    pub async fn get_active_subscriptions(&self) -> Vec<EventSubscription> {
        let subscriptions = self.subscriptions.read().await;
        subscriptions
            .values()
            .filter(|sub| sub.is_active)
            .cloned()
            .collect()
    }

    /// Get statistics for a specific event type
    pub async fn get_stats(
        &self,
        event_type: EventType,
    ) -> Option<crate::domain::events::event_types::ChannelStats> {
        self.channel_manager.get_stats(event_type).await
    }

    /// Get statistics for all event types
    pub async fn get_all_stats(&self) -> Vec<crate::domain::events::event_types::ChannelStats> {
        self.channel_manager.get_all_stats().await
    }

    /// Update configuration for a specific event type
    pub async fn update_channel_config(
        &self,
        event_type: EventType,
        config: ChannelConfig,
    ) -> AppResult<()> {
        self.channel_manager.update_config(event_type, config).await
    }

    /// Get configuration for a specific event type
    pub async fn get_channel_config(&self, event_type: EventType) -> Option<ChannelConfig> {
        self.channel_manager.get_config(event_type).await
    }

    /// Get all active event types
    pub async fn get_active_event_types(&self) -> Vec<EventType> {
        self.channel_manager.get_active_event_types().await
    }

    /// Remove a channel for a specific event type
    pub async fn remove_channel(&self, event_type: EventType) -> AppResult<()> {
        self.channel_manager.remove_channel(event_type).await
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
