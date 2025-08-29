use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Path to the POE client.txt file
    pub poe_client_log_path: String,
    /// Log level for the application
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            poe_client_log_path: crate::utils::PoeClientLogPaths::get_default_path_string(),
            log_level: "info".to_string(),
        }
    }
}

/// Time tracking session for a specific location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSession {
    pub location_id: String,        // Unique identifier for zone/act
    pub location_name: String,      // Human-readable name
    pub location_type: LocationType, // Zone or Act
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>, // None if currently active
    pub duration_seconds: Option<u64>,         // Calculated when session ends
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LocationType {
    Zone,
    Act,
}

/// Aggregated statistics for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStats {
    pub location_id: String,
    pub location_name: String,
    pub location_type: LocationType,
    pub total_visits: u32,
    pub total_time_seconds: u64,
    pub average_session_seconds: f64,
    pub last_visited: Option<DateTime<Utc>>,
}

/// Time tracking events for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTrackingEvent {
    SessionStarted(LocationSession),
    SessionEnded(LocationSession),
    StatsUpdated(LocationStats),
}

pub mod events;
