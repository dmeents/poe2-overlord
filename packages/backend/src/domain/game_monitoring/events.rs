use crate::domain::game_monitoring::models::GameProcessStatus;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcessStatusUpdated {
    pub process_status: GameProcessStatus,
    pub is_state_change: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMonitoringEvent {
    StatusUpdated(GameProcessStatusUpdated),
}

impl GameMonitoringEvent {
    pub fn process_status(&self) -> &GameProcessStatus {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => &event.process_status,
        }
    }

    pub fn occurred_at(&self) -> SystemTime {
        match self {
            GameMonitoringEvent::StatusUpdated(event) => event.occurred_at,
        }
    }
}
