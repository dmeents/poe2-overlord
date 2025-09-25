use crate::domain::log_analysis::models::LogEvent;
use crate::domain::server_monitoring::models::ServerStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    pub subscription_id: String,
    pub event_type: EventType,
    pub subscriber_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

impl EventSubscription {
    pub fn new(event_type: EventType, subscriber_name: String) -> Self {
        Self {
            subscription_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            subscriber_name,
            created_at: chrono::Utc::now(),
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    LogEvent,
    ServerPing,
    GameProcessStatus,
    ConfigurationChange,
    CharacterUpdate,
    TimeTrackingUpdate,
}

impl EventType {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChannelConfig {
    pub channel_capacity: usize,
    pub max_subscribers: usize,
    pub enable_persistence: bool,
    pub retention_duration_hours: u64,
}

impl Default for EventChannelConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 1000,
            max_subscribers: 100,
            enable_persistence: false,
            retention_duration_hours: 24,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventChannel {
    pub event_type: EventType,
    pub sender: broadcast::Sender<EventPayload>,
    pub subscriber_count: usize,
    pub config: EventChannelConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EventChannel {
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

    pub fn get_subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    pub fn is_at_capacity(&self) -> bool {
        self.get_subscriber_count() >= self.config.max_subscribers
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    LogEvent(LogEvent),
    ServerPing(ServerStatus),
    GameProcessStatus(crate::domain::game_monitoring::models::GameProcessStatus),
    ConfigurationChange(crate::domain::configuration::models::ConfigurationChangedEvent),
    CharacterUpdate(crate::domain::character::models::Character),
    TimeTrackingUpdate(crate::domain::time_tracking::models::TimeTrackingData),
}

impl EventPayload {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventManagementSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub total_events_published: u64,
    pub total_events_received: u64,
    pub active_subscriptions: HashMap<EventType, u32>,
    pub is_active: bool,
}

impl EventManagementSession {
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

    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

    pub fn increment_published_events(&mut self) {
        self.total_events_published += 1;
    }

    pub fn increment_received_events(&mut self) {
        self.total_events_received += 1;
    }

    pub fn add_subscription(&mut self, event_type: EventType) {
        *self.active_subscriptions.entry(event_type).or_insert(0) += 1;
    }

    pub fn remove_subscription(&mut self, event_type: EventType) {
        if let Some(count) = self.active_subscriptions.get_mut(&event_type) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventManagementStats {
    pub total_sessions: u64,
    pub total_events_published: u64,
    pub total_events_received: u64,
    pub active_channels: u32,
    pub total_subscribers: u32,
    pub last_activity_time: chrono::DateTime<chrono::Utc>,
    pub current_session: Option<EventManagementSession>,
}

impl Default for EventManagementStats {
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

#[derive(Debug, thiserror::Error)]
pub enum EventManagementError {
    #[error("Channel not found: {event_type}")]
    ChannelNotFound { event_type: String },

    #[error("Channel at capacity: {event_type}")]
    ChannelAtCapacity { event_type: String },

    #[error("Subscription not found: {subscription_id}")]
    SubscriptionNotFound { subscription_id: String },

    #[error("Invalid event type: {event_type}")]
    InvalidEventType { event_type: String },

    #[error("Broadcast error: {message}")]
    BroadcastError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}
