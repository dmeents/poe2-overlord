use crate::errors::AppResult;
use crate::infrastructure::events::types::{AppEvent, ChannelConfig, EventType};
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub struct ChannelManager {
    channels: Arc<RwLock<HashMap<EventType, Arc<EventChannel>>>>,
    configs: Arc<RwLock<HashMap<EventType, ChannelConfig>>>,
}

#[derive(Debug)]
pub struct EventChannel {
    sender: broadcast::Sender<AppEvent>,
}

impl EventChannel {
    pub fn new(_event_type: EventType, config: ChannelConfig) -> Self {
        let (sender, _) = broadcast::channel(config.capacity);
        Self { sender }
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
        }
    }

    pub async fn get_or_create_channel(
        &self,
        event_type: EventType,
    ) -> AppResult<Arc<EventChannel>> {
        // Take write lock once and do check-then-insert to prevent race condition
        let mut channels = self.channels.write().await;

        // Check if channel already exists
        if let Some(channel) = channels.get(&event_type) {
            return Ok(Arc::clone(channel));
        }

        // Channel doesn't exist - create it
        let config = self.get_or_create_config(event_type).await;
        let channel = Arc::new(EventChannel::new(event_type, config.clone()));

        // Insert channel while holding write lock
        channels.insert(event_type, Arc::clone(&channel));

        // Release channels lock before taking config lock
        drop(channels);

        {
            let mut configs = self.configs.write().await;
            configs.insert(event_type, config);
        }

        info!("Created new event channel for type: {event_type:?}");
        Ok(channel)
    }

    pub async fn get_channel(&self, event_type: EventType) -> Option<Arc<EventChannel>> {
        let channels = self.channels.read().await;
        channels.get(&event_type).cloned()
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
