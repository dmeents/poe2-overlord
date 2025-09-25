//! Event Management Data Models
//!
//! This module defines the core data structures used throughout the event management system.
//! These models represent the entities, value objects, and data transfer objects that enable
//! the publish-subscribe pattern and provide type safety for event handling.

use crate::domain::log_analysis::models::LogEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;

/// Represents a subscription to a specific event type
///
/// This struct tracks the relationship between a subscriber and an event type, including
/// metadata about when the subscription was created and its current status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    /// Unique identifier for this subscription
    pub subscription_id: String,
    /// The type of events this subscription listens for
    pub event_type: EventType,
    /// Human-readable name of the subscriber
    pub subscriber_name: String,
    /// When this subscription was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether this subscription is currently active
    pub is_active: bool,
}

impl EventSubscription {
    /// Creates a new active subscription with a generated UUID
    pub fn new(event_type: EventType, subscriber_name: String) -> Self {
        Self {
            subscription_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            subscriber_name,
            created_at: chrono::Utc::now(),
            is_active: true,
        }
    }

    /// Deactivates this subscription (soft delete)
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// Enumeration of all supported event types in the system
///
/// This enum defines the different categories of events that can be published and subscribed to.
/// Each variant corresponds to a specific domain area and ensures type safety when routing events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Events related to log analysis and parsing
    LogEvent,
    /// Server connectivity and ping status events
    ServerPing,
    /// Game process monitoring and status updates
    GameProcessStatus,
    /// Configuration changes and updates
    ConfigurationChange,
    /// Character-related updates and changes
    CharacterUpdate,
    /// Time tracking data and updates
    TimeTrackingUpdate,
}

impl EventType {
    /// Returns a string representation of the event type for serialization/logging
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::LogEvent => "log_event",
            EventType::ServerPing => "server_ping",
            EventType::GameProcessStatus => "game_process_status",
            EventType::ConfigurationChange => "configuration_change",
            EventType::CharacterUpdate => "character_update",
            EventType::TimeTrackingUpdate => "time_tracking_update",
        }
    }
}

/// Configuration settings for event channels
///
/// This struct defines the operational parameters for event channels, including capacity limits,
/// subscriber limits, and persistence settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChannelConfig {
    /// Maximum number of events that can be queued in the channel
    pub channel_capacity: usize,
    /// Maximum number of concurrent subscribers allowed
    pub max_subscribers: usize,
    /// Whether events should be persisted to storage
    pub enable_persistence: bool,
    /// How long to retain events in storage (in hours)
    pub retention_duration_hours: u64,
}

impl Default for EventChannelConfig {
    /// Provides sensible default configuration values for event channels
    fn default() -> Self {
        Self {
            channel_capacity: 1000,
            max_subscribers: 100,
            enable_persistence: false,
            retention_duration_hours: 24,
        }
    }
}

/// Represents an active event channel for a specific event type
///
/// This struct wraps a Tokio broadcast channel with metadata about the channel's configuration,
/// subscriber count, and creation time. It serves as the runtime representation of an event channel.
#[derive(Debug, Clone)]
pub struct EventChannel {
    /// The event type this channel handles
    pub event_type: EventType,
    /// The broadcast sender for publishing events to subscribers
    pub sender: broadcast::Sender<EventPayload>,
    /// Current number of active subscribers (cached for performance)
    pub subscriber_count: usize,
    /// Configuration settings for this channel
    pub config: EventChannelConfig,
    /// When this channel was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EventChannel {
    /// Creates a new event channel with the specified configuration
    pub fn new(event_type: EventType, config: EventChannelConfig) -> Self {
        let (sender, _) = broadcast::channel(config.channel_capacity);

        Self {
            event_type,
            sender,
            subscriber_count: 0,
            config,
            created_at: chrono::Utc::now(),
        }
    }

    /// Gets the current number of active subscribers
    pub fn get_subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Checks if the channel has reached its maximum subscriber capacity
    pub fn is_at_capacity(&self) -> bool {
        self.get_subscriber_count() >= self.config.max_subscribers
    }
}

/// Union type representing all possible event payloads
///
/// This enum serves as a type-safe container for different event types, ensuring that
/// only valid event data can be published and consumed through the event system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    /// Log analysis events (scene changes, connections, etc.)
    LogEvent(LogEvent),
    /// Server connectivity status updates
    ServerPing(ServerStatus),
    /// Game process monitoring events
    GameProcessStatus(crate::domain::game_monitoring::models::GameProcessStatus),
    /// Configuration change notifications
    ConfigurationChange(crate::domain::configuration::models::ConfigurationChangedEvent),
    /// Character-related updates
    CharacterUpdate(crate::domain::character::models::Character),
    /// Time tracking data updates
    TimeTrackingUpdate(crate::domain::time_tracking::models::TimeTrackingData),
}

impl EventPayload {
    /// Returns the event type corresponding to this payload
    pub fn get_event_type(&self) -> EventType {
        match self {
            EventPayload::LogEvent(_) => EventType::LogEvent,
            EventPayload::ServerPing(_) => EventType::ServerPing,
            EventPayload::GameProcessStatus(_) => EventType::GameProcessStatus,
            EventPayload::ConfigurationChange(_) => EventType::ConfigurationChange,
            EventPayload::CharacterUpdate(_) => EventType::CharacterUpdate,
            EventPayload::TimeTrackingUpdate(_) => EventType::TimeTrackingUpdate,
        }
    }

    /// Extracts a timestamp from the event payload for logging and ordering
    pub fn get_timestamp(&self) -> String {
        match self {
            EventPayload::LogEvent(event) => match event {
                LogEvent::SceneChange(scene_event) => scene_event.get_timestamp().to_string(),
                LogEvent::ServerConnection(conn_event) => conn_event.timestamp.clone(),
                LogEvent::CharacterLevelUp(level_event) => level_event.timestamp.clone(),
                LogEvent::CharacterDeath(death_event) => death_event.timestamp.clone(),
            },
            EventPayload::ServerPing(status) => status.timestamp.clone(),
            EventPayload::GameProcessStatus(status) => status
                .detected_at
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .unwrap_or_else(|| chrono::Utc::now())
                        .to_rfc3339()
                })
                .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339()),
            EventPayload::ConfigurationChange(event) => event.timestamp.to_rfc3339(),
            EventPayload::CharacterUpdate(_) => chrono::Utc::now().to_rfc3339(),
            EventPayload::TimeTrackingUpdate(_) => chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Represents a session of event management activity
///
/// This struct tracks metrics and statistics for a specific period of event management
/// activity, providing insights into system usage and performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventManagementSession {
    /// Unique identifier for this session
    pub session_id: String,
    /// When the session started
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// When the session ended (None if still active)
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Total number of events published during this session
    pub total_events_published: u64,
    /// Total number of events received during this session
    pub total_events_received: u64,
    /// Count of active subscriptions by event type
    pub active_subscriptions: HashMap<EventType, u32>,
    /// Whether this session is currently active
    pub is_active: bool,
}

impl EventManagementSession {
    /// Creates a new active session with a generated UUID
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            total_events_published: 0,
            total_events_received: 0,
            active_subscriptions: HashMap::new(),
            is_active: true,
        }
    }

    /// Ends the session by setting the end time and marking as inactive
    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

    /// Increments the published events counter
    pub fn increment_published_events(&mut self) {
        self.total_events_published += 1;
    }

    /// Increments the received events counter
    pub fn increment_received_events(&mut self) {
        self.total_events_received += 1;
    }

    /// Adds a subscription to the active subscriptions count
    pub fn add_subscription(&mut self, event_type: EventType) {
        *self.active_subscriptions.entry(event_type).or_insert(0) += 1;
    }

    /// Removes a subscription from the active subscriptions count
    pub fn remove_subscription(&mut self, event_type: EventType) {
        if let Some(count) = self.active_subscriptions.get_mut(&event_type) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }
}

/// Aggregated statistics for the event management system
///
/// This struct contains cumulative statistics about the event management system's performance
/// and usage, including totals across all sessions and current system state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventManagementStats {
    /// Total number of sessions that have been created
    pub total_sessions: u64,
    /// Total number of events published across all sessions
    pub total_events_published: u64,
    /// Total number of events received across all sessions
    pub total_events_received: u64,
    /// Current number of active event channels
    pub active_channels: u32,
    /// Current total number of subscribers across all channels
    pub total_subscribers: u32,
    /// Timestamp of the last activity in the system
    pub last_activity_time: chrono::DateTime<chrono::Utc>,
    /// The current active session, if any
    pub current_session: Option<EventManagementSession>,
}

impl Default for EventManagementStats {
    /// Provides default values for a new statistics instance
    fn default() -> Self {
        Self {
            total_sessions: 0,
            total_events_published: 0,
            total_events_received: 0,
            active_channels: 0,
            total_subscribers: 0,
            last_activity_time: chrono::Utc::now(),
            current_session: None,
        }
    }
}

/// Error types specific to event management operations
///
/// This enum defines all possible errors that can occur during event management operations,
/// providing detailed error information for debugging and error handling.
#[derive(Debug, thiserror::Error)]
pub enum EventManagementError {
    /// The requested event channel does not exist
    #[error("Channel not found: {event_type}")]
    ChannelNotFound { event_type: String },

    /// The event channel has reached its maximum subscriber capacity
    #[error("Channel at capacity: {event_type}")]
    ChannelAtCapacity { event_type: String },

    /// The requested subscription does not exist
    #[error("Subscription not found: {subscription_id}")]
    SubscriptionNotFound { subscription_id: String },

    /// An invalid event type was provided
    #[error("Invalid event type: {event_type}")]
    InvalidEventType { event_type: String },

    /// An error occurred during broadcast operations
    #[error("Broadcast error: {message}")]
    BroadcastError { message: String },

    /// A configuration-related error occurred
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}
