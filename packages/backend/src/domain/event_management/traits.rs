use crate::domain::event_management::models::{
    EventChannelConfig, EventManagementSession, EventManagementStats, EventPayload, EventSubscription,
    EventType,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

#[async_trait]
pub trait EventManagementService: Send + Sync {
    async fn publish_event(&self, event: EventPayload) -> AppResult<()>;
    
    async fn subscribe_to_events(&self, event_type: EventType, subscriber_name: String) -> AppResult<EventSubscription>;
    
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()>;
    
    fn get_event_receiver(&self, event_type: EventType) -> Option<broadcast::Receiver<EventPayload>>;
    
    async fn get_subscriber_count(&self, event_type: EventType) -> usize;
    
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;
    
    async fn get_stats(&self) -> AppResult<EventManagementStats>;
    
    async fn start_session(&self) -> AppResult<()>;
    
    async fn end_session(&self) -> AppResult<()>;
    
    async fn is_session_active(&self) -> bool;
    
    async fn get_current_session(&self) -> Option<EventManagementSession>;
    
    async fn update_channel_config(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    async fn get_channel_config(&self, event_type: EventType) -> Option<EventChannelConfig>;
}

#[async_trait]
pub trait EventSubscriptionRepository: Send + Sync {
    async fn save_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;
    
    async fn load_subscription(&self, subscription_id: &str) -> AppResult<Option<EventSubscription>>;
    
    async fn get_subscriptions_by_type(&self, event_type: EventType) -> AppResult<Vec<EventSubscription>>;
    
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;
    
    async fn update_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;
    
    async fn delete_subscription(&self, subscription_id: &str) -> AppResult<()>;
    
    async fn deactivate_subscription(&self, subscription_id: &str) -> AppResult<()>;
}

#[async_trait]
pub trait EventManagementSessionRepository: Send + Sync {
    async fn save_session(&self, session: &EventManagementSession) -> AppResult<()>;
    
    async fn load_session(&self, session_id: &str) -> AppResult<Option<EventManagementSession>>;
    
    async fn get_active_session(&self) -> AppResult<Option<EventManagementSession>>;
    
    async fn update_session(&self, session: &EventManagementSession) -> AppResult<()>;
    
    async fn end_current_session(&self) -> AppResult<()>;
    
    async fn get_all_sessions(&self) -> AppResult<Vec<EventManagementSession>>;
}

#[async_trait]
pub trait EventManagementStatsRepository: Send + Sync {
    async fn save_stats(&self, stats: &EventManagementStats) -> AppResult<()>;
    
    async fn load_stats(&self) -> AppResult<EventManagementStats>;
    
    async fn update_stats(&self, stats: &EventManagementStats) -> AppResult<()>;
    
    async fn increment_published_events(&self) -> AppResult<()>;
    
    async fn increment_received_events(&self) -> AppResult<()>;
    
    async fn update_active_channels(&self, count: u32) -> AppResult<()>;
    
    async fn update_total_subscribers(&self, count: u32) -> AppResult<()>;
    
    async fn reset_stats(&self) -> AppResult<()>;
}

#[async_trait]
pub trait EventChannelManager: Send + Sync {
    async fn create_channel(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    async fn get_channel(&self, event_type: EventType) -> Option<Arc<crate::domain::event_management::models::EventChannel>>;
    
    async fn remove_channel(&self, event_type: EventType) -> AppResult<()>;
    
    async fn get_all_channels(&self) -> AppResult<Vec<EventType>>;
    
    async fn update_channel_config(&self, event_type: EventType, config: EventChannelConfig) -> AppResult<()>;
    
    async fn get_channel_stats(&self, event_type: EventType) -> AppResult<Option<ChannelStats>>;
}

#[derive(Debug, Clone)]
pub struct ChannelStats {
    pub event_type: EventType,
    pub subscriber_count: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}
