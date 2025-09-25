//! Event Management Traits
//!
//! This module defines the core interfaces for the event management system, following the
//! repository pattern and service layer architecture. These traits provide a clean separation
//! between the business logic and data persistence layers.

use crate::domain::event_management::models::{
    EventChannelConfig, EventManagementSession, EventManagementStats, EventPayload,
    EventSubscription, EventType,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Core service interface for event management operations
///
/// This trait defines the main business logic for publishing events, managing subscriptions,
/// and tracking event management sessions. It serves as the primary interface for components
/// that need to interact with the event system.
#[async_trait]
pub trait EventManagementService: Send + Sync {
    /// Publishes an event to all subscribers of the event's type
    async fn publish_event(&self, event: EventPayload) -> AppResult<()>;

    /// Creates a new subscription for the specified event type
    async fn subscribe_to_events(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription>;

    /// Removes an existing subscription by its ID
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()>;

    /// Gets a broadcast receiver for the specified event type (blocking operation)
    fn get_event_receiver(
        &self,
        event_type: EventType,
    ) -> Option<broadcast::Receiver<EventPayload>>;

    /// Returns the current number of subscribers for an event type
    async fn get_subscriber_count(&self, event_type: EventType) -> usize;

    /// Retrieves all currently active subscriptions
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;

    /// Gets comprehensive statistics about the event management system
    async fn get_stats(&self) -> AppResult<EventManagementStats>;

    /// Starts a new event management session for tracking metrics
    async fn start_session(&self) -> AppResult<()>;

    /// Ends the current session and updates statistics
    async fn end_session(&self) -> AppResult<()>;

    /// Checks if there's an active session running
    async fn is_session_active(&self) -> bool;

    /// Gets the current active session, if any
    async fn get_current_session(&self) -> Option<EventManagementSession>;

    /// Updates configuration for a specific event channel
    async fn update_channel_config(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()>;

    /// Retrieves the current configuration for an event channel
    async fn get_channel_config(&self, event_type: EventType) -> Option<EventChannelConfig>;
}

/// Repository interface for managing event subscriptions persistence
///
/// This trait defines the data access layer for event subscriptions, providing CRUD operations
/// and querying capabilities. Implementations should handle persistence to storage systems
/// like databases or file systems.
#[async_trait]
pub trait EventSubscriptionRepository: Send + Sync {
    /// Saves a new subscription to persistent storage
    async fn save_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;

    /// Loads a subscription by its unique ID
    async fn load_subscription(
        &self,
        subscription_id: &str,
    ) -> AppResult<Option<EventSubscription>>;

    /// Retrieves all subscriptions for a specific event type
    async fn get_subscriptions_by_type(
        &self,
        event_type: EventType,
    ) -> AppResult<Vec<EventSubscription>>;

    /// Gets all currently active subscriptions across all event types
    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>>;

    /// Updates an existing subscription in storage
    async fn update_subscription(&self, subscription: &EventSubscription) -> AppResult<()>;

    /// Permanently removes a subscription from storage
    async fn delete_subscription(&self, subscription_id: &str) -> AppResult<()>;

    /// Soft-deletes a subscription by marking it as inactive
    async fn deactivate_subscription(&self, subscription_id: &str) -> AppResult<()>;
}

/// Repository interface for managing event management sessions
///
/// This trait handles persistence of event management sessions, which track metrics and
/// statistics for monitoring the health and performance of the event system.
#[async_trait]
pub trait EventManagementSessionRepository: Send + Sync {
    /// Saves a new session to persistent storage
    async fn save_session(&self, session: &EventManagementSession) -> AppResult<()>;

    /// Loads a specific session by its ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<EventManagementSession>>;

    /// Retrieves the currently active session, if any
    async fn get_active_session(&self) -> AppResult<Option<EventManagementSession>>;

    /// Updates an existing session in storage
    async fn update_session(&self, session: &EventManagementSession) -> AppResult<()>;

    /// Ends the current active session by marking it as completed
    async fn end_current_session(&self) -> AppResult<()>;

    /// Retrieves all sessions from storage (for historical analysis)
    async fn get_all_sessions(&self) -> AppResult<Vec<EventManagementSession>>;
}

/// Repository interface for managing event management statistics
///
/// This trait handles persistence of aggregated statistics about the event management system,
/// providing both full CRUD operations and atomic increment operations for performance.
#[async_trait]
pub trait EventManagementStatsRepository: Send + Sync {
    /// Saves complete statistics to persistent storage
    async fn save_stats(&self, stats: &EventManagementStats) -> AppResult<()>;

    /// Loads the current statistics from storage
    async fn load_stats(&self) -> AppResult<EventManagementStats>;

    /// Updates existing statistics in storage
    async fn update_stats(&self, stats: &EventManagementStats) -> AppResult<()>;

    /// Atomically increments the published events counter
    async fn increment_published_events(&self) -> AppResult<()>;

    /// Atomically increments the received events counter
    async fn increment_received_events(&self) -> AppResult<()>;

    /// Updates the count of active channels
    async fn update_active_channels(&self, count: u32) -> AppResult<()>;

    /// Updates the total subscriber count
    async fn update_total_subscribers(&self, count: u32) -> AppResult<()>;

    /// Resets all statistics to default values
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Manager interface for event channel lifecycle and configuration
///
/// This trait provides operations for managing the lifecycle of event channels, including
/// creation, configuration updates, and statistics gathering. It abstracts the complexity
/// of channel management from the main service.
#[async_trait]
pub trait EventChannelManager: Send + Sync {
    /// Creates a new event channel with the specified configuration
    async fn create_channel(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()>;

    /// Retrieves an existing channel for the specified event type
    async fn get_channel(
        &self,
        event_type: EventType,
    ) -> Option<Arc<crate::domain::event_management::models::EventChannel>>;

    /// Removes a channel and cleans up its resources
    async fn remove_channel(&self, event_type: EventType) -> AppResult<()>;

    /// Gets a list of all currently active event types
    async fn get_all_channels(&self) -> AppResult<Vec<EventType>>;

    /// Updates the configuration for an existing channel
    async fn update_channel_config(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()>;

    /// Retrieves performance statistics for a specific channel
    async fn get_channel_stats(&self, event_type: EventType) -> AppResult<Option<ChannelStats>>;
}

/// Statistics for a specific event channel
///
/// This struct contains performance metrics and metadata for monitoring the health
/// and usage of individual event channels.
#[derive(Debug, Clone)]
pub struct ChannelStats {
    /// The event type this channel handles
    pub event_type: EventType,
    /// Current number of active subscribers
    pub subscriber_count: usize,
    /// Total number of messages sent through this channel
    pub messages_sent: u64,
    /// Total number of messages received by subscribers
    pub messages_received: u64,
    /// When this channel was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp of the last activity on this channel
    pub last_activity: chrono::DateTime<chrono::Utc>,
}
