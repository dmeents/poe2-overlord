use crate::domain::event_management::models::EventType;
use serde::{Deserialize, Serialize};

/// Events related to event management operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventManagementEvent {
    /// Event published
    EventPublished {
        event_type: EventType,
        timestamp: String,
    },
    
    /// Event subscription created
    SubscriptionCreated {
        subscription_id: String,
        event_type: EventType,
        subscriber_name: String,
        timestamp: String,
    },
    
    /// Event subscription removed
    SubscriptionRemoved {
        subscription_id: String,
        event_type: EventType,
        timestamp: String,
    },
    
    /// Event channel created
    ChannelCreated {
        event_type: EventType,
        channel_capacity: usize,
        timestamp: String,
    },
    
    /// Event channel removed
    ChannelRemoved {
        event_type: EventType,
        timestamp: String,
    },
    
    /// Event management session started
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    /// Event management session ended
    SessionEnded {
        session_id: String,
        events_published: u64,
        events_received: u64,
        timestamp: String,
    },
    
    /// Channel configuration updated
    ChannelConfigUpdated {
        event_type: EventType,
        timestamp: String,
    },
    
    /// Error occurred during event management
    ManagementError {
        error_message: String,
        timestamp: String,
    },
}

impl EventManagementEvent {
    pub fn event_published(event_type: EventType) -> Self {
        Self::EventPublished {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn subscription_created(subscription_id: String, event_type: EventType, subscriber_name: String) -> Self {
        Self::SubscriptionCreated {
            subscription_id,
            event_type,
            subscriber_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn subscription_removed(subscription_id: String, event_type: EventType) -> Self {
        Self::SubscriptionRemoved {
            subscription_id,
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn channel_created(event_type: EventType, channel_capacity: usize) -> Self {
        Self::ChannelCreated {
            event_type,
            channel_capacity,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn channel_removed(event_type: EventType) -> Self {
        Self::ChannelRemoved {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_ended(session_id: String, events_published: u64, events_received: u64) -> Self {
        Self::SessionEnded {
            session_id,
            events_published,
            events_received,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn channel_config_updated(event_type: EventType) -> Self {
        Self::ChannelConfigUpdated {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn management_error(error_message: String) -> Self {
        Self::ManagementError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
