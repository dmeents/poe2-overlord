use serde::{Deserialize, Serialize};

/// Status of the game process with human-readable timestamp
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcessStatus {
    pub name: String,
    pub pid: u32,
    pub running: bool,
    /// RFC3339 formatted timestamp for frontend compatibility
    pub detected_at: String,
}

impl GameProcessStatus {
    fn current_timestamp() -> String {
        chrono::Utc::now().to_rfc3339()
    }

    pub fn new(name: String, pid: u32, running: bool) -> Self {
        Self {
            name,
            pid,
            running,
            detected_at: Self::current_timestamp(),
        }
    }

    pub fn not_running() -> Self {
        Self {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
            detected_at: Self::current_timestamp(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Detects transitions between running and stopped states
    pub fn is_state_change(&self, previous: &GameProcessStatus) -> bool {
        self.running != previous.running
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMonitoringConfig {
    /// Fast polling when no game is detected
    pub detection_interval_seconds: u64,
    /// Slow polling when game is running
    pub monitoring_interval_seconds: u64,
    pub process_names: Vec<String>,
}

impl Default for GameMonitoringConfig {
    fn default() -> Self {
        Self {
            detection_interval_seconds: 3,
            monitoring_interval_seconds: 5,
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
