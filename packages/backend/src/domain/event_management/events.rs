//! Event Management Domain Events
//!
//! This module defines domain events that are emitted by the event management system itself.
//! These events represent significant occurrences within the event management domain and can
//! be used for monitoring, auditing, and integration with other parts of the system.

use crate::domain::event_management::models::EventType;
use serde::{Deserialize, Serialize};

/// Domain events emitted by the event management system
///
/// These events represent important state changes and operations within the event management
/// system, providing visibility into system behavior and enabling external monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventManagementEvent {
    /// An event was successfully published to subscribers
    EventPublished {
        event_type: EventType,
        timestamp: String,
    },

    /// A new subscription was created for an event type
    SubscriptionCreated {
        subscription_id: String,
        event_type: EventType,
        subscriber_name: String,
        timestamp: String,
    },

    /// A subscription was removed or deactivated
    SubscriptionRemoved {
        subscription_id: String,
        event_type: EventType,
        timestamp: String,
    },

    /// A new event channel was created
    ChannelCreated {
        event_type: EventType,
        channel_capacity: usize,
        timestamp: String,
    },

    /// An event channel was removed or destroyed
    ChannelRemoved {
        event_type: EventType,
        timestamp: String,
    },

    /// A new event management session was started
    SessionStarted {
        session_id: String,
        timestamp: String,
    },

    /// An event management session was ended with final statistics
    SessionEnded {
        session_id: String,
        events_published: u64,
        events_received: u64,
        timestamp: String,
    },

    /// A channel's configuration was updated
    ChannelConfigUpdated {
        event_type: EventType,
        timestamp: String,
    },

    /// An error occurred in the event management system
    ManagementError {
        error_message: String,
        timestamp: String,
    },
}

impl EventManagementEvent {
    /// Creates an event published domain event
    pub fn event_published(event_type: EventType) -> Self {
        Self::EventPublished {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a subscription created domain event
    pub fn subscription_created(
        subscription_id: String,
        event_type: EventType,
        subscriber_name: String,
    ) -> Self {
        Self::SubscriptionCreated {
            subscription_id,
            event_type,
            subscriber_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a subscription removed domain event
    pub fn subscription_removed(subscription_id: String, event_type: EventType) -> Self {
        Self::SubscriptionRemoved {
            subscription_id,
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a channel created domain event
    pub fn channel_created(event_type: EventType, channel_capacity: usize) -> Self {
        Self::ChannelCreated {
            event_type,
            channel_capacity,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a channel removed domain event
    pub fn channel_removed(event_type: EventType) -> Self {
        Self::ChannelRemoved {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session started domain event
    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session ended domain event with final statistics
    pub fn session_ended(session_id: String, events_published: u64, events_received: u64) -> Self {
        Self::SessionEnded {
            session_id,
            events_published,
            events_received,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a channel config updated domain event
    pub fn channel_config_updated(event_type: EventType) -> Self {
        Self::ChannelConfigUpdated {
            event_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a management error domain event
    pub fn management_error(error_message: String) -> Self {
        Self::ManagementError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
