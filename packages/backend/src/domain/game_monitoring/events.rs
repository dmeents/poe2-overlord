use crate::domain::game_monitoring::models::GameProcessStatus;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Domain event indicating that the game process has started
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcessStarted {
    /// The process status when it was detected as started
    pub process_status: GameProcessStatus,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl GameProcessStarted {
    pub fn new(process_status: GameProcessStatus) -> Self {
        Self {
            process_status,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that the game process has stopped
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcessStopped {
    /// The process status when it was detected as stopped
    pub process_status: GameProcessStatus,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl GameProcessStopped {
    pub fn new(process_status: GameProcessStatus) -> Self {
        Self {
            process_status,
            occurred_at: SystemTime::now(),
        }
    }
}

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
    ProcessStarted(GameProcessStarted),
    ProcessStopped(GameProcessStopped),
    StatusUpdated(GameProcessStatusUpdated),
}

impl GameMonitoringEvent {
    /// Get the process status from any game monitoring event
    pub fn process_status(&self) -> &GameProcessStatus {
        match self {
            GameMonitoringEvent::ProcessStarted(event) => &event.process_status,
            GameMonitoringEvent::ProcessStopped(event) => &event.process_status,
            GameMonitoringEvent::StatusUpdated(event) => &event.process_status,
        }
    }

    /// Get the timestamp when this event occurred
    pub fn occurred_at(&self) -> SystemTime {
        match self {
            GameMonitoringEvent::ProcessStarted(event) => event.occurred_at,
            GameMonitoringEvent::ProcessStopped(event) => event.occurred_at,
            GameMonitoringEvent::StatusUpdated(event) => event.occurred_at,
        }
    }
}
