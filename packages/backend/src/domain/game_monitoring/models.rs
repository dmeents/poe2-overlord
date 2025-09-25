use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Domain model representing the status of the Path of Exile 2 game process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcessStatus {
    /// The name of the process
    pub name: String,
    /// The process ID
    pub pid: u32,
    /// Whether the process is currently running
    pub running: bool,
    /// When this status was detected
    pub detected_at: SystemTime,
}

impl GameProcessStatus {
    /// Create a new game process status
    pub fn new(name: String, pid: u32, running: bool) -> Self {
        Self {
            name,
            pid,
            running,
            detected_at: SystemTime::now(),
        }
    }

    /// Create a status indicating the game is not running
    pub fn not_running() -> Self {
        Self {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
            detected_at: SystemTime::now(),
        }
    }

    /// Check if the game process is currently running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Check if this represents a state change from the previous status
    pub fn is_state_change(&self, previous: &GameProcessStatus) -> bool {
        self.running != previous.running
    }
}

/// Domain model for game monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMonitoringConfig {
    /// How often to check for the game process (in seconds)
    pub check_interval_seconds: u64,
    /// Process names to look for when detecting POE2
    pub process_names: Vec<String>,
}

impl Default for GameMonitoringConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 5,
            process_names: vec![
                "pathofexile2".to_string(),
                "poe2".to_string(),
                "pathofexile".to_string(),
                "pathofexilesteam".to_string(),
                "pathofexilesteam.exe".to_string(),
                "pathofexile2.exe".to_string(),
                "pathofexile.exe".to_string(),
            ],
        }
    }
}
