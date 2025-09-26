//! Channel Manager
//!
//! This module provides centralized management of broadcast channels for the event system.
//! It handles channel lifecycle, configuration, and statistics tracking.

use crate::domain::events::event_types::{ChannelConfig, ChannelStats, EventType};
use crate::errors::AppResult;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Centralized manager for event channels
///
/// This struct manages all broadcast channels in the event system, providing
/// lifecycle management, configuration, and statistics tracking.
pub struct ChannelManager {
    /// Active channels mapped by event type
    channels: Arc<RwLock<HashMap<EventType, Arc<EventChannel>>>>,
    /// Channel configurations
    configs: Arc<RwLock<HashMap<EventType, ChannelConfig>>>,
    /// Statistics for each channel
    stats: Arc<RwLock<HashMap<EventType, ChannelStats>>>,
}

/// Internal representation of an event channel
#[derive(Debug)]
pub struct EventChannel {
    /// The broadcast sender for this channel
    sender: broadcast::Sender<crate::domain::events::event_types::AppEvent>,
    /// Configuration for this channel
    config: ChannelConfig,
    /// When this channel was created
    created_at: String,
}

impl EventChannel {
    /// Create a new event channel
    pub fn new(_event_type: EventType, config: ChannelConfig) -> Self {
        let (sender, _) = broadcast::channel(config.capacity);

        Self {
            sender,
            config,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Get the number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Check if the channel is at capacity
    pub fn is_at_capacity(&self) -> bool {
        // This is a rough estimate - Tokio doesn't expose exact capacity
        self.subscriber_count() >= self.config.capacity
    }

    /// Get the sender for this channel
    pub fn sender(&self) -> &broadcast::Sender<crate::domain::events::event_types::AppEvent> {
        &self.sender
    }
}

impl ChannelManager {
    /// Create a new channel manager
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create a channel for the specified event type
    ///
    /// If a channel doesn't exist for the event type, it will be created
    /// with the default configuration for that event type.
    pub async fn get_or_create_channel(
        &self,
        event_type: EventType,
    ) -> AppResult<Arc<EventChannel>> {
        // Check if channel already exists
        {
            let channels = self.channels.read().await;
            if let Some(channel) = channels.get(&event_type) {
                return Ok(Arc::clone(channel));
            }
        }

        // Create new channel
        let config = self.get_or_create_config(event_type).await;
        let channel = Arc::new(EventChannel::new(event_type, config.clone()));

        // Store the channel
        {
            let mut channels = self.channels.write().await;
            channels.insert(event_type, Arc::clone(&channel));
        }

        // Initialize statistics
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

        // Store configuration
        {
            let mut configs = self.configs.write().await;
            configs.insert(event_type, config);
        }

        info!("Created new event channel for type: {:?}", event_type);
        Ok(channel)
    }

    /// Get an existing channel for the specified event type
    pub async fn get_channel(&self, event_type: EventType) -> Option<Arc<EventChannel>> {
        let channels = self.channels.read().await;
        channels.get(&event_type).cloned()
    }

    /// Remove a channel for the specified event type
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

    /// Get all active event types
    pub async fn get_active_event_types(&self) -> Vec<EventType> {
        let channels = self.channels.read().await;
        channels.keys().cloned().collect()
    }

    /// Update configuration for a specific event type
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

    /// Get configuration for a specific event type
    pub async fn get_config(&self, event_type: EventType) -> Option<ChannelConfig> {
        let configs = self.configs.read().await;
        configs.get(&event_type).cloned()
    }

    /// Get statistics for a specific event type
    pub async fn get_stats(&self, event_type: EventType) -> Option<ChannelStats> {
        let stats = self.stats.read().await;
        stats.get(&event_type).cloned()
    }

    /// Get statistics for all channels
    pub async fn get_all_stats(&self) -> Vec<ChannelStats> {
        let stats = self.stats.read().await;
        stats.values().cloned().collect()
    }

    /// Increment published events counter for a channel
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

    /// Increment received events counter for a channel
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

    /// Get or create configuration for an event type
    async fn get_or_create_config(&self, event_type: EventType) -> ChannelConfig {
        {
            let configs = self.configs.read().await;
            if let Some(config) = configs.get(&event_type) {
                return config.clone();
            }
        }

        // Create default configuration
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
