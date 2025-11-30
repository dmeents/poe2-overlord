//! Manages broadcast channels for different event types.

use crate::errors::AppResult;
use crate::infrastructure::events::types::{AppEvent, ChannelConfig, ChannelStats, EventType};
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub struct ChannelManager {
    channels: Arc<RwLock<HashMap<EventType, Arc<EventChannel>>>>,
    configs: Arc<RwLock<HashMap<EventType, ChannelConfig>>>,
    stats: Arc<RwLock<HashMap<EventType, ChannelStats>>>,
}

#[derive(Debug)]
pub struct EventChannel {
    sender: broadcast::Sender<AppEvent>,
    config: ChannelConfig,
    created_at: String,
}

impl EventChannel {
    pub fn new(_event_type: EventType, config: ChannelConfig) -> Self {
        let (sender, _) = broadcast::channel(config.capacity);

        Self {
            sender,
            config,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    pub fn is_at_capacity(&self) -> bool {
        self.subscriber_count() >= self.config.capacity
    }

    pub fn sender(&self) -> &broadcast::Sender<AppEvent> {
        &self.sender
    }
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_channel(
        &self,
        event_type: EventType,
    ) -> AppResult<Arc<EventChannel>> {
        {
            let channels = self.channels.read().await;
            if let Some(channel) = channels.get(&event_type) {
                return Ok(Arc::clone(channel));
            }
        }

        let config = self.get_or_create_config(event_type).await;
        let channel = Arc::new(EventChannel::new(event_type, config.clone()));

        {
            let mut channels = self.channels.write().await;
            channels.insert(event_type, Arc::clone(&channel));
        }

        {
            let mut stats = self.stats.write().await;
            stats.insert(
                event_type,
                ChannelStats {
                    event_type,
                    subscriber_count: 0,
                    events_published: 0,
                    events_received: 0,
                    created_at: channel.created_at.clone(),
                    last_activity: channel.created_at.clone(),
                },
            );
        }

        {
            let mut configs = self.configs.write().await;
            configs.insert(event_type, config);
        }

        info!("Created new event channel for type: {:?}", event_type);
        Ok(channel)
    }

    pub async fn get_channel(&self, event_type: EventType) -> Option<Arc<EventChannel>> {
        let channels = self.channels.read().await;
        channels.get(&event_type).cloned()
    }

    pub async fn remove_channel(&self, event_type: EventType) -> AppResult<()> {
        {
            let mut channels = self.channels.write().await;
            channels.remove(&event_type);
        }

        {
            let mut configs = self.configs.write().await;
            configs.remove(&event_type);
        }

        {
            let mut stats = self.stats.write().await;
            stats.remove(&event_type);
        }

        debug!("Removed event channel for type: {:?}", event_type);
        Ok(())
    }

    pub async fn get_active_event_types(&self) -> Vec<EventType> {
        let channels = self.channels.read().await;
        channels.keys().cloned().collect()
    }

    pub async fn update_config(
        &self,
        event_type: EventType,
        config: ChannelConfig,
    ) -> AppResult<()> {
        {
            let mut configs = self.configs.write().await;
            configs.insert(event_type, config);
        }

        debug!("Updated configuration for event type: {:?}", event_type);
        Ok(())
    }

    pub async fn get_config(&self, event_type: EventType) -> Option<ChannelConfig> {
        let configs = self.configs.read().await;
        configs.get(&event_type).cloned()
    }

    pub async fn get_stats(&self, event_type: EventType) -> Option<ChannelStats> {
        let stats = self.stats.read().await;
        stats.get(&event_type).cloned()
    }

    pub async fn get_all_stats(&self) -> Vec<ChannelStats> {
        let stats = self.stats.read().await;
        stats.values().cloned().collect()
    }

    pub async fn increment_published_events(&self, event_type: EventType) {
        if let Some(channel) = self.get_channel(event_type).await {
            let mut stats = self.stats.write().await;
            if let Some(stat) = stats.get_mut(&event_type) {
                stat.events_published += 1;
                stat.last_activity = chrono::Utc::now().to_rfc3339();
                stat.subscriber_count = channel.subscriber_count();
            }
        }
    }

    pub async fn increment_received_events(&self, event_type: EventType) {
        if let Some(channel) = self.get_channel(event_type).await {
            let mut stats = self.stats.write().await;
            if let Some(stat) = stats.get_mut(&event_type) {
                stat.events_received += 1;
                stat.last_activity = chrono::Utc::now().to_rfc3339();
                stat.subscriber_count = channel.subscriber_count();
            }
        }
    }

    async fn get_or_create_config(&self, event_type: EventType) -> ChannelConfig {
        {
            let configs = self.configs.read().await;
            if let Some(config) = configs.get(&event_type) {
                return config.clone();
            }
        }

        let config = ChannelConfig::for_event_type(event_type);

        {
            let mut configs = self.configs.write().await;
            configs.insert(event_type, config.clone());
        }

        config
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}
