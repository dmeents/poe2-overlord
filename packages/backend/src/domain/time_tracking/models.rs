use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a single session of time spent in a specific location by a character.
/// Tracks entry/exit times and calculates duration for time tracking analytics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocationSession {
    /// Unique identifier for the character
    pub character_id: String,
    /// Unique identifier for the location (generated from name and type)
    pub location_id: String,
    /// Human-readable name of the location
    pub location_name: String,
    /// Type of location (Zone, Act, Hideout)
    pub location_type: LocationType,
    /// When the character entered this location
    pub entry_timestamp: DateTime<Utc>,
    /// When the character left this location (None if still active)
    pub exit_timestamp: Option<DateTime<Utc>>,
    /// Total time spent in this location in seconds (None if still active)
    pub duration_seconds: Option<u64>,
}

impl LocationSession {
    /// Creates a new active location session with current timestamp as entry time
    pub fn new(
        character_id: String,
        location_id: String,
        location_name: String,
        location_type: LocationType,
    ) -> Self {
        Self {
            character_id,
            location_id,
            location_name,
            location_type,
            entry_timestamp: Utc::now(),
            exit_timestamp: None,
            duration_seconds: None,
        }
    }

    /// Returns true if the session is still active (no exit timestamp)
    pub fn is_active(&self) -> bool {
        self.exit_timestamp.is_none()
    }

    /// Ends the session by setting exit timestamp and calculating duration
    pub fn end_session(&mut self) {
        if self.exit_timestamp.is_none() {
            self.exit_timestamp = Some(Utc::now());
            if let Some(exit_time) = self.exit_timestamp {
                self.duration_seconds =
                    Some((exit_time - self.entry_timestamp).num_seconds().max(0) as u64);
            }
        }
    }

    /// Gets the current duration in seconds, either from stored duration or calculated from timestamps
    pub fn get_current_duration_seconds(&self) -> u64 {
        if let Some(exit_time) = self.exit_timestamp {
            (exit_time - self.entry_timestamp).num_seconds().max(0) as u64
        } else {
            (Utc::now() - self.entry_timestamp).num_seconds().max(0) as u64
        }
    }
}

/// Types of locations that can be tracked in the game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LocationType {
    /// A playable game zone/area
    Zone,
    /// A major story act
    Act,
    /// Player's personal hideout
    Hideout,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationType::Zone => write!(f, "Zone"),
            LocationType::Act => write!(f, "Act"),
            LocationType::Hideout => write!(f, "Hideout"),
        }
    }
}

/// Aggregated statistics for a specific location across all sessions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationStats {
    /// Character this stats belongs to
    pub character_id: String,
    /// Unique identifier for the location
    pub location_id: String,
    /// Human-readable name of the location
    pub location_name: String,
    /// Type of location
    pub location_type: LocationType,
    /// Total number of visits to this location
    pub total_visits: u32,
    /// Total time spent in this location across all sessions (seconds)
    pub total_time_seconds: u64,
    /// Average time per session in this location (seconds)
    pub average_session_seconds: f64,
    /// Timestamp of the most recent visit
    pub last_visited: Option<DateTime<Utc>>,
}

impl LocationStats {
    /// Creates new empty location stats for a location
    pub fn new(
        character_id: String,
        location_id: String,
        location_name: String,
        location_type: LocationType,
    ) -> Self {
        Self {
            character_id,
            location_id,
            location_name,
            location_type,
            total_visits: 0,
            total_time_seconds: 0,
            average_session_seconds: 0.0,
            last_visited: None,
        }
    }

    /// Adds a completed session to the stats, updating all aggregated values
    pub fn add_session(&mut self, session: &LocationSession) {
        if let Some(duration) = session.duration_seconds {
            self.total_visits += 1;
            self.total_time_seconds += duration;
            self.average_session_seconds =
                self.total_time_seconds as f64 / self.total_visits as f64;
            self.last_visited = Some(session.exit_timestamp.unwrap_or(Utc::now()));
        }
    }

    /// Updates last visited timestamp for an active session
    pub fn update_with_active_session(&mut self, session: &LocationSession) {
        if session.is_active() {
            self.last_visited = Some(session.entry_timestamp);
        }
    }
}

/// Complete time tracking data for a character, including all sessions and statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingData {
    /// Character this data belongs to
    pub character_id: String,
    /// Currently active location sessions
    pub active_sessions: Vec<LocationSession>,
    /// Completed location sessions
    pub completed_sessions: Vec<LocationSession>,
    /// Statistics for all visited locations
    pub all_location_stats: Vec<LocationStats>,
    /// Summary of time tracking data
    pub summary: TimeTrackingSummary,
}

impl TimeTrackingData {
    /// Creates new empty time tracking data for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id: character_id.clone(),
            active_sessions: Vec::new(),
            completed_sessions: Vec::new(),
            all_location_stats: Vec::new(),
            summary: TimeTrackingSummary::new(character_id),
        }
    }

    /// Recalculates the summary based on current data
    pub fn update_summary(&mut self) {
        self.summary = TimeTrackingSummary::from_data(
            &self.character_id,
            &self.active_sessions,
            &self.completed_sessions,
            &self.all_location_stats,
        );
    }
}

/// Summary view of time tracking data with key metrics and top locations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingSummary {
    /// Character this summary belongs to
    pub character_id: String,
    /// Currently active sessions
    pub active_sessions: Vec<LocationSession>,
    /// Top 10 locations by time spent
    pub top_locations: Vec<LocationStats>,
    /// Total number of unique locations visited
    pub total_locations_tracked: usize,
    /// Number of currently active sessions
    pub total_active_sessions: usize,
    /// Total play time across all completed sessions (seconds)
    pub total_play_time_seconds: u64,
    /// Total play time since process start (seconds)
    pub total_play_time_since_process_start_seconds: u64,
    /// Total time spent in hideouts (seconds)
    pub total_hideout_time_seconds: u64,
}

impl TimeTrackingSummary {
    /// Creates new empty summary for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            active_sessions: Vec::new(),
            top_locations: Vec::new(),
            total_locations_tracked: 0,
            total_active_sessions: 0,
            total_play_time_seconds: 0,
            total_play_time_since_process_start_seconds: 0,
            total_hideout_time_seconds: 0,
        }
    }

    /// Creates summary from existing time tracking data
    pub fn from_data(
        character_id: &str,
        active_sessions: &[LocationSession],
        completed_sessions: &[LocationSession],
        all_location_stats: &[LocationStats],
    ) -> Self {
        let mut zone_stats: Vec<LocationStats> = all_location_stats
            .iter()
            .filter(|stat| stat.location_type == LocationType::Zone)
            .cloned()
            .collect();
        zone_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
        let top_locations = zone_stats.into_iter().take(10).collect();

        let total_play_time_seconds = completed_sessions
            .iter()
            .filter_map(|session| session.duration_seconds)
            .sum();

        let total_hideout_time_seconds = completed_sessions
            .iter()
            .filter(|session| session.location_type == LocationType::Hideout)
            .filter_map(|session| session.duration_seconds)
            .sum();

        Self {
            character_id: character_id.to_string(),
            active_sessions: active_sessions.to_vec(),
            top_locations,
            total_locations_tracked: all_location_stats.len(),
            total_active_sessions: active_sessions.len(),
            total_play_time_seconds,
            total_play_time_since_process_start_seconds: 0, // Will be calculated by service
            total_hideout_time_seconds,
        }
    }
}

/// Persistent data structure for character time tracking (used for storage)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterTimeTrackingData {
    /// Character this data belongs to
    pub character_id: String,
    /// All completed sessions for persistence
    pub completed_sessions: Vec<LocationSession>,
    /// All location statistics for persistence
    pub stats: Vec<LocationStats>,
}

impl CharacterTimeTrackingData {
    /// Creates new empty persistent data for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            completed_sessions: Vec::new(),
            stats: Vec::new(),
        }
    }
}
