use crate::domain::character::models::CharacterDataResponse;
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

    CharacterUpdated {
        character_id: String,
        data: CharacterDataResponse,
        timestamp: String,
    },
    CharacterDeleted {
        character_id: String,
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
            AppEvent::CharacterUpdated { .. }
            | AppEvent::CharacterDeleted { .. }
            | AppEvent::WalkthroughProgressUpdated { .. }
            | AppEvent::WalkthroughStepCompleted { .. }
            | AppEvent::WalkthroughStepAdvanced { .. }
            | AppEvent::WalkthroughCampaignCompleted { .. } => EventType::CharacterTracking,
            AppEvent::GameProcessStatusChanged { .. } => EventType::GameMonitoring,
            AppEvent::SystemError { .. } | AppEvent::SystemShutdown { .. } => EventType::System,
        }
    }

    pub fn timestamp(&self) -> String {
        match self {
            AppEvent::ServerStatusChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::ServerPingCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::ConfigurationChanged(event) => event.timestamp.to_rfc3339(),
            AppEvent::CharacterUpdated { timestamp, .. } => timestamp.clone(),
            AppEvent::CharacterDeleted { timestamp, .. } => timestamp.clone(),
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

    pub fn character_updated(character_id: String, data: CharacterDataResponse) -> Self {
        Self::CharacterUpdated {
            character_id,
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn character_deleted(character_id: String) -> Self {
        Self::CharacterDeleted {
            character_id,
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
    CharacterTracking,
    GameMonitoring,
    System,
}

impl EventType {
    pub fn all() -> Vec<EventType> {
        vec![
            EventType::ServerMonitoring,
            EventType::Configuration,
            EventType::CharacterTracking,
            EventType::GameMonitoring,
            EventType::System,
        ]
    }

    pub fn default_capacity(&self) -> usize {
        match self {
            EventType::ServerMonitoring => 100,
            EventType::Configuration => 16,
            EventType::CharacterTracking => 500,
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
