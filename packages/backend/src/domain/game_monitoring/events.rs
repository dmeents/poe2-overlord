use crate::domain::game_monitoring::models::GameProcessStatus;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Event data for when a game process status is updated.
/// 
/// This event is published whenever the monitoring system detects a change
/// in the game process status, including both state changes and periodic updates.
/// The `is_state_change` flag helps distinguish between actual state transitions
/// and routine status checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcessStatusUpdated {
    /// The current status of the game process
    pub process_status: GameProcessStatus,
    /// Whether this update represents an actual state change (running <-> stopped)
    pub is_state_change: bool,
    /// Timestamp when this event occurred
    pub occurred_at: SystemTime,
}

impl GameProcessStatusUpdated {
    /// Creates a new process status updated event with the current timestamp.
    /// 
    /// # Arguments
    /// * `process_status` - The current process status
    /// * `is_state_change` - Whether this represents a state change
    /// 
    /// # Returns
    /// * `Self` - New GameProcessStatusUpdated event
    pub fn new(process_status: GameProcessStatus, is_state_change: bool) -> Self {
        Self {
            process_status,
            is_state_change,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Enumeration of all possible game monitoring events.
/// 
/// This enum defines the complete set of events that can be published by the
/// game monitoring system. Currently supports status updates, but can be extended
/// to include other monitoring-related events in the future.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMonitoringEvent {
    /// Event published when the game process status is updated
    StatusUpdated(GameProcessStatusUpdated),
}

impl GameMonitoringEvent {
    /// Extracts the process status from the event.
    /// 
    /// Provides convenient access to the process status regardless of the
    /// specific event variant, enabling consistent handling across event types.
    /// 
    /// # Returns
    /// * `&GameProcessStatus` - Reference to the process status
    pub fn process_status(&self) -> &GameProcessStatus {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => &event.process_status,
        }
    }

    /// Gets the timestamp when this event occurred.
    /// 
    /// Provides consistent access to event timing information across all
    /// event variants, useful for logging and debugging purposes.
    /// 
    /// # Returns
    /// * `SystemTime` - The timestamp when the event occurred
    pub fn occurred_at(&self) -> SystemTime {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => event.occurred_at,
        }
    }
}
