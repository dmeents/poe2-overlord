use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Time tracking session for a specific location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSession {
    pub character_id: String,        // Character this session belongs to
    pub location_id: String,         // Unique identifier for zone/act
    pub location_name: String,       // Human-readable name
    pub location_type: LocationType, // Zone or Act
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>, // None if currently active
    pub duration_seconds: Option<u64>,         // Calculated when session ends
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LocationType {
    Zone,
    Act,
    Hideout,
}

/// Aggregated statistics for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStats {
    pub character_id: String,        // Character these stats belong to
    pub location_id: String,
    pub location_name: String,
    pub location_type: LocationType,
    pub total_visits: u32,
    pub total_time_seconds: u64,
    pub average_session_seconds: f64,
    pub last_visited: Option<DateTime<Utc>>,
}

/// Unified time tracking data containing all information for a specific character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingData {
    pub character_id: String,        // Character this data belongs to
    pub active_sessions: Vec<LocationSession>,
    pub completed_sessions: Vec<LocationSession>,
    pub all_location_stats: Vec<LocationStats>,
    pub summary: TimeTrackingSummary,
}

/// Time tracking summary with aggregated metrics for a specific character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingSummary {
    pub character_id: String,        // Character this summary belongs to
    pub active_sessions: Vec<LocationSession>,
    pub top_locations: Vec<LocationStats>,
    pub total_locations_tracked: usize,
    pub total_active_sessions: usize,
    pub total_play_time_seconds: u64,
    pub total_play_time_since_process_start_seconds: u64,
    pub total_hideout_time_seconds: u64,
}

/// Time tracking events for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTrackingEvent {
    SessionStarted(LocationSession),
    SessionEnded(LocationSession),
    StatsUpdated(LocationStats),
}
