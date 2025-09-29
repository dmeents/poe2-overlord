//! Event Types and Configuration
//!
//! This module defines the core event types and configuration structures
//! for the unified event system.

use crate::domain::character::models::{CharacterData, LocationState, LocationType};
use crate::domain::configuration::models::ConfigurationChangedEvent;
use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::walkthrough::models::{WalkthroughProgress, WalkthroughStepResult};
use serde::{Deserialize, Serialize};

/// All possible events in the application
///
/// This enum serves as the single source of truth for all events
/// that can be published or subscribed to in the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    // Server Monitoring Events
    ServerStatusChanged {
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
        timestamp: String,
    },
    ServerPingCompleted {
        server_status: ServerStatus,
        latency_ms: Option<u64>,
        timestamp: String,
    },

    // Configuration Events
    ConfigurationChanged(ConfigurationChangedEvent),

    // Location Tracking Events
    LocationStateChanged {
        old_state: Option<LocationState>,
        new_state: LocationState,
        timestamp: String,
    },
    SceneChangeDetected {
        scene_type: LocationType,
        scene_name: String,
        timestamp: String,
    },
    ActChangeDetected {
        act_name: String,
        timestamp: String,
    },
    ZoneChangeDetected {
        zone_name: String,
        timestamp: String,
    },
    HideoutChangeDetected {
        hideout_name: String,
        timestamp: String,
    },

    // Character Tracking Events
    CharacterTrackingDataUpdated {
        character_id: String,
        data: CharacterData,
        timestamp: String,
    },

    // Walkthrough Events
    WalkthroughProgressUpdated {
        character_id: String,
        progress: WalkthroughProgress,
        timestamp: String,
    },
    WalkthroughStepCompleted {
        character_id: String,
        step: WalkthroughStepResult,
        timestamp: String,
    },
    WalkthroughStepAdvanced {
        character_id: String,
        from_step_id: Option<String>,
        to_step_id: Option<String>,
        timestamp: String,
    },
    WalkthroughCampaignCompleted {
        character_id: String,
        timestamp: String,
    },

    // Game Monitoring Events
    GameProcessStatusChanged {
        old_status: Option<GameProcessStatus>,
        new_status: GameProcessStatus,
        is_state_change: bool,
        timestamp: String,
    },

    // System Events
    SystemError {
        error_message: String,
        error_type: String,
        timestamp: String,
    },
    SystemShutdown {
        timestamp: String,
    },
}

impl AppEvent {
    /// Get the event type for this event
    pub fn event_type(&self) -> EventType {
        match self {
            AppEvent::ServerStatusChanged { .. } | AppEvent::ServerPingCompleted { .. } => {
                EventType::ServerMonitoring
            }
            AppEvent::ConfigurationChanged(_) => EventType::Configuration,
            AppEvent::LocationStateChanged { .. }
            | AppEvent::SceneChangeDetected { .. }
            | AppEvent::ActChangeDetected { .. }
            | AppEvent::ZoneChangeDetected { .. }
            | AppEvent::HideoutChangeDetected { .. }
            | AppEvent::CharacterTrackingDataUpdated { .. }
            | AppEvent::WalkthroughProgressUpdated { .. }
            | AppEvent::WalkthroughStepCompleted { .. }
            | AppEvent::WalkthroughStepAdvanced { .. }
            | AppEvent::WalkthroughCampaignCompleted { .. } => EventType::LocationTracking,
            AppEvent::GameProcessStatusChanged { .. } => EventType::GameMonitoring,
            AppEvent::SystemError { .. } | AppEvent::SystemShutdown { .. } => EventType::System,
        }
    }

    /// Get the timestamp for this event
    pub fn timestamp(&self) -> String {
        match self {
            AppEvent::ServerStatusChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::ServerPingCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::ConfigurationChanged(event) => event.timestamp.to_rfc3339(),
            AppEvent::LocationStateChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::SceneChangeDetected { timestamp, .. } => timestamp.clone(),
            AppEvent::ActChangeDetected { timestamp, .. } => timestamp.clone(),
            AppEvent::ZoneChangeDetected { timestamp, .. } => timestamp.clone(),
            AppEvent::HideoutChangeDetected { timestamp, .. } => timestamp.clone(),
            AppEvent::CharacterTrackingDataUpdated { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughProgressUpdated { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughStepCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughStepAdvanced { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughCampaignCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::GameProcessStatusChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::SystemError { timestamp, .. } => timestamp.clone(),
            AppEvent::SystemShutdown { timestamp, .. } => timestamp.clone(),
        }
    }

    /// Create a server status changed event
    pub fn server_status_changed(
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
    ) -> Self {
        Self::ServerStatusChanged {
            old_status,
            new_status,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a server ping completed event
    pub fn server_ping_completed(server_status: ServerStatus, latency_ms: Option<u64>) -> Self {
        Self::ServerPingCompleted {
            server_status,
            latency_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a location state changed event
    pub fn location_state_changed(
        old_state: Option<LocationState>,
        new_state: LocationState,
    ) -> Self {
        Self::LocationStateChanged {
            old_state,
            new_state,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a scene change detected event
    pub fn scene_change_detected(scene_type: LocationType, scene_name: String) -> Self {
        Self::SceneChangeDetected {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create an act change detected event
    pub fn act_change_detected(act_name: String) -> Self {
        Self::ActChangeDetected {
            act_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a zone change detected event
    pub fn zone_change_detected(zone_name: String) -> Self {
        Self::ZoneChangeDetected {
            zone_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a hideout change detected event
    pub fn hideout_change_detected(hideout_name: String) -> Self {
        Self::HideoutChangeDetected {
            hideout_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a character tracking data updated event
    pub fn character_tracking_data_updated(character_id: String, data: CharacterData) -> Self {
        Self::CharacterTrackingDataUpdated {
            character_id,
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a game process status changed event
    pub fn game_process_status_changed(
        old_status: Option<GameProcessStatus>,
        new_status: GameProcessStatus,
        is_state_change: bool,
    ) -> Self {
        Self::GameProcessStatusChanged {
            old_status,
            new_status,
            is_state_change,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a system error event
    pub fn system_error(error_message: String, error_type: String) -> Self {
        Self::SystemError {
            error_message,
            error_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a system shutdown event
    pub fn system_shutdown() -> Self {
        Self::SystemShutdown {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a walkthrough progress updated event
    pub fn walkthrough_progress_updated(character_id: String, progress: WalkthroughProgress) -> Self {
        Self::WalkthroughProgressUpdated {
            character_id,
            progress,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a walkthrough step completed event
    pub fn walkthrough_step_completed(character_id: String, step: WalkthroughStepResult) -> Self {
        Self::WalkthroughStepCompleted {
            character_id,
            step,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a walkthrough step advanced event
    pub fn walkthrough_step_advanced(
        character_id: String,
        from_step_id: Option<String>,
        to_step_id: Option<String>,
    ) -> Self {
        Self::WalkthroughStepAdvanced {
            character_id,
            from_step_id,
            to_step_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a walkthrough campaign completed event
    pub fn walkthrough_campaign_completed(character_id: String) -> Self {
        Self::WalkthroughCampaignCompleted {
            character_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Event types for channel management
///
/// This enum categorizes events into logical groups for channel management.
/// Each event type gets its own broadcast channel with configurable capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    ServerMonitoring,
    Configuration,
    LocationTracking,
    GameMonitoring,
    System,
}

impl EventType {
    /// Get all available event types
    pub fn all() -> Vec<EventType> {
        vec![
            EventType::ServerMonitoring,
            EventType::Configuration,
            EventType::LocationTracking,
            EventType::GameMonitoring,
            EventType::System,
        ]
    }

    /// Get the default channel capacity for this event type
    pub fn default_capacity(&self) -> usize {
        match self {
            EventType::ServerMonitoring => 100, // Medium volume
            EventType::Configuration => 16,     // Low volume
            EventType::LocationTracking => 500, // Medium-high volume
            EventType::GameMonitoring => 100,   // Medium volume
            EventType::System => 50,            // Low volume
        }
    }
}

/// Configuration for event channels
///
/// This struct defines the configuration for broadcast channels,
/// including capacity, behavior, and monitoring settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Maximum number of events to buffer in the channel
    pub capacity: usize,
    /// Whether to drop old events when channel is full
    pub drop_old_events: bool,
    /// Whether to log channel statistics
    pub enable_logging: bool,
    /// Whether to track subscriber count
    pub track_subscribers: bool,
}

impl ChannelConfig {
    /// Create a default configuration for an event type
    pub fn for_event_type(event_type: EventType) -> Self {
        Self {
            capacity: event_type.default_capacity(),
            drop_old_events: true,
            enable_logging: true,
            track_subscribers: true,
        }
    }

    /// Create a high-capacity configuration for high-volume events
    pub fn high_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            drop_old_events: true,
            enable_logging: false, // Disable logging for performance
            track_subscribers: true,
        }
    }

    /// Create a low-capacity configuration for low-volume events
    pub fn low_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            drop_old_events: false, // Don't drop events for important low-volume events
            enable_logging: true,
            track_subscribers: true,
        }
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            capacity: 100,
            drop_old_events: true,
            enable_logging: true,
            track_subscribers: true,
        }
    }
}

/// Statistics for a specific event channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStats {
    /// The event type this channel handles
    pub event_type: EventType,
    /// Current number of active subscribers
    pub subscriber_count: usize,
    /// Total number of events published through this channel
    pub events_published: u64,
    /// Total number of events received by subscribers
    pub events_received: u64,
    /// When this channel was created
    pub created_at: String,
    /// Timestamp of the last activity on this channel
    pub last_activity: String,
}

/// An active subscription to events of a specific type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    /// Unique identifier for this subscription
    pub subscription_id: String,
    /// The type of events this subscription listens for
    pub event_type: EventType,
    /// Human-readable name of the subscriber
    pub subscriber_name: String,
    /// When this subscription was created
    pub created_at: String,
    /// Whether this subscription is currently active
    pub is_active: bool,
}

impl EventSubscription {
    /// Create a new active subscription
    pub fn new(event_type: EventType, subscriber_name: String) -> Self {
        Self {
            subscription_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            subscriber_name,
            created_at: chrono::Utc::now().to_rfc3339(),
            is_active: true,
        }
    }

    /// Deactivate this subscription
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
