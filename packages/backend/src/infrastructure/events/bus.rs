use crate::errors::AppResult;
use crate::infrastructure::events::channels::ChannelManager;
use crate::infrastructure::events::types::{AppEvent, ChannelConfig, EventSubscription, EventType};
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub struct EventBus {
    channel_manager: Arc<ChannelManager>,
    subscriptions: Arc<RwLock<HashMap<String, EventSubscription>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            channel_manager: Arc::new(ChannelManager::new()),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn publish(&self, event: AppEvent) -> AppResult<()> {
        let event_type = event.event_type();
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;

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

    pub async fn subscribe(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription> {
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;

        if channel.is_at_capacity() {
            return Err(crate::errors::AppError::internal_error(
                "subscribe",
                &format!("Channel for event type {:?} is at capacity", event_type),
            ));
        }

        let subscription = EventSubscription::new(event_type, subscriber_name);
        let subscription_id = subscription.subscription_id.clone();

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

    pub async fn get_subscriber_count(&self, event_type: EventType) -> usize {
        if let Some(channel) = self.channel_manager.get_channel(event_type).await {
            channel.subscriber_count()
        } else {
            0
        }
    }

    pub async fn get_active_subscriptions(&self) -> Vec<EventSubscription> {
        let subscriptions = self.subscriptions.read().await;
        subscriptions
            .values()
            .filter(|sub| sub.is_active)
            .cloned()
            .collect()
    }

    pub async fn get_stats(
        &self,
        event_type: EventType,
    ) -> Option<crate::infrastructure::events::types::ChannelStats> {
        self.channel_manager.get_stats(event_type).await
    }

    pub async fn get_all_stats(&self) -> Vec<crate::infrastructure::events::types::ChannelStats> {
        self.channel_manager.get_all_stats().await
    }

    pub async fn update_channel_config(
        &self,
        event_type: EventType,
        config: ChannelConfig,
    ) -> AppResult<()> {
        self.channel_manager.update_config(event_type, config).await
    }

    pub async fn get_channel_config(&self, event_type: EventType) -> Option<ChannelConfig> {
        self.channel_manager.get_config(event_type).await
    }

    pub async fn get_active_event_types(&self) -> Vec<EventType> {
        self.channel_manager.get_active_event_types().await
    }

    pub async fn remove_channel(&self, event_type: EventType) -> AppResult<()> {
        self.channel_manager.remove_channel(event_type).await
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
