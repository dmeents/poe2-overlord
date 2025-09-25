use crate::domain::game_monitoring::models::GameProcessStatus;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Domain event indicating that the game process status has been updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcessStatusUpdated {
    /// The current process status
    pub process_status: GameProcessStatus,
    /// Whether this represents a state change
    pub is_state_change: bool,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl GameProcessStatusUpdated {
    pub fn new(process_status: GameProcessStatus, is_state_change: bool) -> Self {
        Self {
            process_status,
            is_state_change,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Enum representing all possible game monitoring domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMonitoringEvent {
    StatusUpdated(GameProcessStatusUpdated),
}

impl GameMonitoringEvent {
    /// Get the process status from any game monitoring event
    pub fn process_status(&self) -> &GameProcessStatus {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => &event.process_status,
        }
    }

    /// Get the timestamp when this event occurred
    pub fn occurred_at(&self) -> SystemTime {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => event.occurred_at,
        }
    }
}
