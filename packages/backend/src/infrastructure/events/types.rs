use crate::domain::character::models::{CharacterData, LocationState, LocationType};
use crate::domain::configuration::models::ConfigurationChangedEvent;
use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::walkthrough::models::{WalkthroughProgress, WalkthroughStepResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
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

    ConfigurationChanged(ConfigurationChangedEvent),

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

    CharacterTrackingDataUpdated {
        character_id: String,
        data: CharacterData,
        timestamp: String,
    },

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

    GameProcessStatusChanged {
        old_status: Option<GameProcessStatus>,
        new_status: GameProcessStatus,
        is_state_change: bool,
        timestamp: String,
    },

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

    pub fn server_ping_completed(server_status: ServerStatus, latency_ms: Option<u64>) -> Self {
        Self::ServerPingCompleted {
            server_status,
            latency_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

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

    pub fn scene_change_detected(scene_type: LocationType, scene_name: String) -> Self {
        Self::SceneChangeDetected {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn act_change_detected(act_name: String) -> Self {
        Self::ActChangeDetected {
            act_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn zone_change_detected(zone_name: String) -> Self {
        Self::ZoneChangeDetected {
            zone_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn hideout_change_detected(hideout_name: String) -> Self {
        Self::HideoutChangeDetected {
            hideout_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn character_tracking_data_updated(character_id: String, data: CharacterData) -> Self {
        Self::CharacterTrackingDataUpdated {
            character_id,
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

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

    pub fn system_error(error_message: String, error_type: String) -> Self {
        Self::SystemError {
            error_message,
            error_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn system_shutdown() -> Self {
        Self::SystemShutdown {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn walkthrough_progress_updated(
        character_id: String,
        progress: WalkthroughProgress,
    ) -> Self {
        Self::WalkthroughProgressUpdated {
            character_id,
            progress,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn walkthrough_step_completed(character_id: String, step: WalkthroughStepResult) -> Self {
        Self::WalkthroughStepCompleted {
            character_id,
            step,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

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

    pub fn walkthrough_campaign_completed(character_id: String) -> Self {
        Self::WalkthroughCampaignCompleted {
            character_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    ServerMonitoring,
    Configuration,
    LocationTracking,
    GameMonitoring,
    System,
}

impl EventType {
    pub fn all() -> Vec<EventType> {
        vec![
            EventType::ServerMonitoring,
            EventType::Configuration,
            EventType::LocationTracking,
            EventType::GameMonitoring,
            EventType::System,
        ]
    }

    pub fn default_capacity(&self) -> usize {
        match self {
            EventType::ServerMonitoring => 100,
            EventType::Configuration => 16,
            EventType::LocationTracking => 500,
            EventType::GameMonitoring => 100,
            EventType::System => 50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub capacity: usize,
    /// Drops oldest events when channel is full
    pub drop_old_events: bool,
    pub enable_logging: bool,
    pub track_subscribers: bool,
}

impl ChannelConfig {
    pub fn for_event_type(event_type: EventType) -> Self {
        Self {
            capacity: event_type.default_capacity(),
            drop_old_events: true,
            enable_logging: true,
            track_subscribers: true,
        }
    }

    /// Disables logging for performance on high-volume events
    pub fn high_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            drop_old_events: true,
            enable_logging: false,
            track_subscribers: true,
        }
    }

    /// Preserves all events for critical low-volume events
    pub fn low_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            drop_old_events: false,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStats {
    pub event_type: EventType,
    pub subscriber_count: usize,
    pub events_published: u64,
    pub events_received: u64,
    pub created_at: String,
    pub last_activity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    pub subscription_id: String,
    pub event_type: EventType,
    pub subscriber_name: String,
    pub created_at: String,
    pub is_active: bool,
}

impl EventSubscription {
    pub fn new(event_type: EventType, subscriber_name: String) -> Self {
        Self {
            subscription_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            subscriber_name,
            created_at: chrono::Utc::now().to_rfc3339(),
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
