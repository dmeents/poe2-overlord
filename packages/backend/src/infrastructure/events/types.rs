use crate::domain::character::models::CharacterDataResponse;
use crate::domain::configuration::models::ConfigurationChangedEvent;
use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::domain::leveling::models::LevelingStats;
use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::walkthrough::models::WalkthroughStepResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    ServerStatusChanged {
        old_status: Option<ServerStatus>,
        new_status: ServerStatus,
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

    LevelingStatsUpdated {
        character_id: String,
        stats: LevelingStats,
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
            AppEvent::ServerStatusChanged { .. } => EventType::ServerMonitoring,
            AppEvent::ConfigurationChanged(_) => EventType::Configuration,
            AppEvent::CharacterUpdated { .. }
            | AppEvent::CharacterDeleted { .. }
            | AppEvent::WalkthroughStepCompleted { .. }
            | AppEvent::WalkthroughStepAdvanced { .. }
            | AppEvent::WalkthroughCampaignCompleted { .. }
            | AppEvent::LevelingStatsUpdated { .. } => EventType::CharacterTracking,
            AppEvent::GameProcessStatusChanged { .. } => EventType::GameMonitoring,
            AppEvent::SystemError { .. } | AppEvent::SystemShutdown { .. } => EventType::System,
        }
    }

    pub fn timestamp(&self) -> String {
        match self {
            AppEvent::ServerStatusChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::ConfigurationChanged(event) => event.timestamp.clone(),
            AppEvent::CharacterUpdated { timestamp, .. } => timestamp.clone(),
            AppEvent::CharacterDeleted { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughStepCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughStepAdvanced { timestamp, .. } => timestamp.clone(),
            AppEvent::WalkthroughCampaignCompleted { timestamp, .. } => timestamp.clone(),
            AppEvent::GameProcessStatusChanged { timestamp, .. } => timestamp.clone(),
            AppEvent::LevelingStatsUpdated { timestamp, .. } => timestamp.clone(),
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

    pub fn leveling_stats_updated(character_id: String, stats: LevelingStats) -> Self {
        Self::LevelingStatsUpdated {
            character_id,
            stats,
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
}

impl ChannelConfig {
    pub fn for_event_type(event_type: EventType) -> Self {
        Self {
            capacity: event_type.default_capacity(),
        }
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self { capacity: 100 }
    }
}
