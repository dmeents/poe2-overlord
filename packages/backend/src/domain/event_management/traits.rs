use crate::domain::event_management::models::{
    EventChannelConfig, EventManagementSession, EventManagementStats, EventPayload, EventSubscription,
    EventType,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Trait for event management service operations
#[async_trait]
pub trait EventManagementService: Send + Sync {
    /// Publish an event to the appropriate channel
    async fn publish_event(&self, event: EventPayload) -> AppResult<()>;
    
    /// Subscribe to events of a specific type
    async fn subscribe_to_events(&self, event_type: EventType, subscriber_name: String) -> AppResult<EventSubscription>;
    
    /// Unsubscribe from events
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()>;
    
    /// Get a receiver for a specific event type
    fn get_event_receiver(&self, event_type: EventType) -> Option<broadcast::Receiver<EventPayload>>;
    
    /// Get the number of subscribers for an event type
    async fn get_subscriber_count(&self, event_type: EventType) -> usize;
    
    /// Get all active subscriptions
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;
    
    /// Get event management statistics
    async fn get_stats(&self) -> AppResult<EventManagementStats>;
    
    /// Start event management session
    async fn start_session(&self) -> AppResult<()>;
    
    /// End current event management session
    async fn end_session(&self) -> AppResult<()>;
    
    /// Check if a session is active
    async fn is_session_active(&self) -> bool;
    
    /// Get current session
    async fn get_current_session(&self) -> Option<EventManagementSession>;
    
    /// Update channel configuration
    async fn update_channel_config(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    /// Get channel configuration
    async fn get_channel_config(&self, event_type: EventType) -> Option<EventChannelConfig>;
}

/// Trait for event subscription repository operations
#[async_trait]
pub trait EventSubscriptionRepository: Send + Sync {
    /// Save event subscription
    async fn save_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;
    
    /// Load subscription by ID
    async fn load_subscription(&self, subscription_id: &str) -> AppResult<Option<EventSubscription>>;
    
    /// Get all subscriptions for an event type
    async fn get_subscriptions_by_type(&self, event_type: EventType) -> AppResult<Vec<EventSubscription>>;
    
    /// Get all active subscriptions
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;
    
    /// Update subscription
    async fn update_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;
    
    /// Delete subscription
    async fn delete_subscription(&self, subscription_id: &str) -> AppResult<()>;
    
    /// Deactivate subscription
    async fn deactivate_subscription(&self, subscription_id: &str) -> AppResult<()>;
}

/// Trait for event management session repository operations
#[async_trait]
pub trait EventManagementSessionRepository: Send + Sync {
    /// Save event management session
    async fn save_session(&self, session: &EventManagementSession) -> AppResult<()>;
    
    /// Load session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<EventManagementSession>>;
    
    /// Get current active session
    async fn get_active_session(&self) -> AppResult<Option<EventManagementSession>>;
    
    /// Update session
    async fn update_session(&self, session: &EventManagementSession) -> AppResult<()>;
    
    /// End current session
    async fn end_current_session(&self) -> AppResult<()>;
    
    /// Get all sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<EventManagementSession>>;
}

/// Trait for event management statistics repository operations
#[async_trait]
pub trait EventManagementStatsRepository: Send + Sync {
    /// Save event management statistics
    async fn save_stats(&self, stats: &EventManagementStats) -> AppResult<()>;
    
    /// Load event management statistics
    async fn load_stats(&self) -> AppResult<EventManagementStats>;
    
    /// Update statistics
    async fn update_stats(&self, stats: &EventManagementStats) -> AppResult<()>;
    
    /// Increment published events counter
    async fn increment_published_events(&self) -> AppResult<()>;
    
    /// Increment received events counter
    async fn increment_received_events(&self) -> AppResult<()>;
    
    /// Update active channels count
    async fn update_active_channels(&self, count: u32) -> AppResult<()>;
    
    /// Update total subscribers count
    async fn update_total_subscribers(&self, count: u32) -> AppResult<()>;
    
    /// Reset statistics
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Trait for event channel management
#[async_trait]
pub trait EventChannelManager: Send + Sync {
    /// Create a new event channel
    async fn create_channel(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    /// Get event channel
    async fn get_channel(&self, event_type: EventType) -> Option<Arc<crate::domain::event_management::models::EventChannel>>;
    
    /// Remove event channel
    async fn remove_channel(&self, event_type: EventType) -> AppResult<()>;
    
    /// Get all active channels
    async fn get_all_channels(&self) -> AppResult<Vec<EventType>>;
    
    /// Update channel configuration
    async fn update_channel_config(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    /// Get channel statistics
    async fn get_channel_stats(&self, event_type: EventType) -> AppResult<Option<ChannelStats>>;
}

/// Channel statistics
#[derive(Debug, Clone)]
pub struct ChannelStats {
    pub event_type: EventType,
    pub subscriber_count: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}
