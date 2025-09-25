use serde::{Deserialize, Serialize};
use std::time::SystemTime;


#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayState {
    pub visible: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub always_on_top: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcessStatus {
    pub name: String,
    pub pid: u32,
    pub running: bool,
    pub detected_at: SystemTime,
}

impl GameProcessStatus {
    pub fn new(name: String, pid: u32, running: bool) -> Self {
        Self {
            name,
            pid,
            running,
            detected_at: SystemTime::now(),
        }
    }

    pub fn not_running() -> Self {
        Self {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
            detected_at: SystemTime::now(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_state_change(&self, previous: &GameProcessStatus) -> bool {
        self.running != previous.running
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMonitoringConfig {
    pub check_interval_seconds: u64,
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
