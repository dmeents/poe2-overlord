use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Basic process information structure.
///
/// Contains essential details about a running process including name, PID, and status.
/// This is a simpler version used for basic process detection operations.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// The name of the process
    pub name: String,
    /// Process ID (PID) of the running process
    pub pid: u32,
    /// Whether the process is currently running
    pub running: bool,
}

/// State information for overlay windows.
///
/// Tracks the visibility and positioning of overlay windows that may be used
/// for displaying game monitoring information to the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayState {
    /// Whether the overlay is currently visible
    pub visible: bool,
    /// Screen position coordinates (x, y)
    pub position: (i32, i32),
    /// Window dimensions (width, height)
    pub size: (u32, u32),
    /// Whether the overlay should stay on top of other windows
    pub always_on_top: bool,
}

/// Represents the current status of a game process.
///
/// This is the primary data structure used throughout the game monitoring system
/// to track the state of Path of Exile 2 processes. It includes timing information
/// for accurate state change detection and time tracking integration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcessStatus {
    /// The name of the detected process
    pub name: String,
    /// Process ID of the running game process
    pub pid: u32,
    /// Whether the process is currently running
    pub running: bool,
    /// Timestamp when this status was detected
    pub detected_at: SystemTime,
}

impl GameProcessStatus {
    /// Creates a new game process status with the current timestamp.
    ///
    /// # Arguments
    /// * `name` - The process name
    /// * `pid` - The process ID
    /// * `running` - Whether the process is running
    ///
    /// # Returns
    /// * `Self` - New GameProcessStatus instance
    pub fn new(name: String, pid: u32, running: bool) -> Self {
        Self {
            name,
            pid,
            running,
            detected_at: SystemTime::now(),
        }
    }

    /// Creates a status representing a non-running process.
    ///
    /// Used when no game process is detected, providing a consistent
    /// representation of the "not running" state.
    ///
    /// # Returns
    /// * `Self` - GameProcessStatus representing no running process
    pub fn not_running() -> Self {
        Self {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
            detected_at: SystemTime::now(),
        }
    }

    /// Checks if the process is currently running.
    ///
    /// # Returns
    /// * `bool` - True if the process is running, false otherwise
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Determines if this status represents a state change from a previous status.
    ///
    /// Compares the running state to detect transitions between running/stopped states.
    /// This is used to trigger appropriate actions when the game starts or stops.
    ///
    /// # Arguments
    /// * `previous` - The previous process status to compare against
    ///
    /// # Returns
    /// * `bool` - True if the running state has changed, false otherwise
    pub fn is_state_change(&self, previous: &GameProcessStatus) -> bool {
        self.running != previous.running
    }
}

/// Configuration settings for game process monitoring.
///
/// Defines the parameters used by the monitoring system including check intervals
/// and the list of process names to search for when detecting POE2 instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMonitoringConfig {
    /// Interval in seconds between process status checks when no game is detected (fast detection)
    pub detection_interval_seconds: u64,
    /// Interval in seconds between process status checks when game is running (slow monitoring)
    pub monitoring_interval_seconds: u64,
    /// List of process names to search for when detecting POE2
    pub process_names: Vec<String>,
}

impl Default for GameMonitoringConfig {
    /// Creates default configuration for POE2 monitoring.
    ///
    /// Sets up reasonable defaults including fast detection (3s) when no game is running
    /// and slow monitoring (60s) when game is running, plus comprehensive list of
    /// common POE2 process names across different platforms.
    ///
    /// # Returns
    /// * `Self` - Default GameMonitoringConfig instance
    fn default() -> Self {
        Self {
            detection_interval_seconds: 3,   // Fast detection when no game running
            monitoring_interval_seconds: 60, // Slow monitoring when game is running
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
