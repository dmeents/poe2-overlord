use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Time tracking session for a specific location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSession {
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
