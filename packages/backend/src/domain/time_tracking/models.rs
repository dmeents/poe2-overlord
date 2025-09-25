use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Time tracking session for a specific location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocationSession {
    /// Character this session belongs to
    pub character_id: String,
    /// Unique identifier for zone/act
    pub location_id: String,
    /// Human-readable name
    pub location_name: String,
    /// Zone, Act, or Hideout
    pub location_type: LocationType,
    /// When the session started
    pub entry_timestamp: DateTime<Utc>,
    /// When the session ended (None if currently active)
    pub exit_timestamp: Option<DateTime<Utc>>,
    /// Calculated duration when session ends
    pub duration_seconds: Option<u64>,
}

impl LocationSession {
    /// Create a new location session
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

    /// Check if this session is currently active
    pub fn is_active(&self) -> bool {
        self.exit_timestamp.is_none()
    }

    /// End this session and calculate duration
    pub fn end_session(&mut self) {
        if self.exit_timestamp.is_none() {
            self.exit_timestamp = Some(Utc::now());
            if let Some(exit_time) = self.exit_timestamp {
                self.duration_seconds =
                    Some((exit_time - self.entry_timestamp).num_seconds().max(0) as u64);
            }
        }
    }

    /// Get the current duration of an active session
    pub fn get_current_duration_seconds(&self) -> u64 {
        if let Some(exit_time) = self.exit_timestamp {
            (exit_time - self.entry_timestamp).num_seconds().max(0) as u64
        } else {
            (Utc::now() - self.entry_timestamp).num_seconds().max(0) as u64
        }
    }
}

/// Types of locations that can be tracked
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LocationType {
    Zone,
    Act,
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

/// Aggregated statistics for a location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationStats {
    /// Character these stats belong to
    pub character_id: String,
    /// Unique location identifier
    pub location_id: String,
    /// Human-readable location name
    pub location_name: String,
    /// Type of location
    pub location_type: LocationType,
    /// Total number of visits
    pub total_visits: u32,
    /// Total time spent in seconds
    pub total_time_seconds: u64,
    /// Average session duration in seconds
    pub average_session_seconds: f64,
    /// Last time this location was visited
    pub last_visited: Option<DateTime<Utc>>,
}

impl LocationStats {
    /// Create new location stats
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

    /// Update stats with a new session
    pub fn add_session(&mut self, session: &LocationSession) {
        if let Some(duration) = session.duration_seconds {
            self.total_visits += 1;
            self.total_time_seconds += duration;
            self.average_session_seconds =
                self.total_time_seconds as f64 / self.total_visits as f64;
            self.last_visited = Some(session.exit_timestamp.unwrap_or(Utc::now()));
        }
    }

    /// Update stats with an active session (for current duration)
    pub fn update_with_active_session(&mut self, session: &LocationSession) {
        if session.is_active() {
            // For active sessions, we don't increment visits until they end
            // but we update the last visited time
            self.last_visited = Some(session.entry_timestamp);
        }
    }
}

/// Unified time tracking data containing all information for a specific character
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingData {
    /// Character this data belongs to
    pub character_id: String,
    /// Currently active sessions
    pub active_sessions: Vec<LocationSession>,
    /// Completed sessions
    pub completed_sessions: Vec<LocationSession>,
    /// All location statistics
    pub all_location_stats: Vec<LocationStats>,
    /// Summary of time tracking data
    pub summary: TimeTrackingSummary,
}

impl TimeTrackingData {
    /// Create new time tracking data for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id: character_id.clone(),
            active_sessions: Vec::new(),
            completed_sessions: Vec::new(),
            all_location_stats: Vec::new(),
            summary: TimeTrackingSummary::new(character_id),
        }
    }

    /// Update the summary with current data
    pub fn update_summary(&mut self) {
        self.summary = TimeTrackingSummary::from_data(
            &self.character_id,
            &self.active_sessions,
            &self.completed_sessions,
            &self.all_location_stats,
        );
    }
}

/// Time tracking summary with aggregated metrics for a specific character
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingSummary {
    /// Character this summary belongs to
    pub character_id: String,
    /// Currently active sessions
    pub active_sessions: Vec<LocationSession>,
    /// Top locations by time spent
    pub top_locations: Vec<LocationStats>,
    /// Total number of locations tracked
    pub total_locations_tracked: usize,
    /// Total number of active sessions
    pub total_active_sessions: usize,
    /// Total play time in seconds
    pub total_play_time_seconds: u64,
    /// Total play time since process start in seconds
    pub total_play_time_since_process_start_seconds: u64,
    /// Total hideout time in seconds
    pub total_hideout_time_seconds: u64,
}

impl TimeTrackingSummary {
    /// Create a new empty summary
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

    /// Create summary from time tracking data
    pub fn from_data(
        character_id: &str,
        active_sessions: &[LocationSession],
        completed_sessions: &[LocationSession],
        all_location_stats: &[LocationStats],
    ) -> Self {
        // Get top locations (zones only, sorted by time)
        let mut zone_stats: Vec<LocationStats> = all_location_stats
            .iter()
            .filter(|stat| stat.location_type == LocationType::Zone)
            .cloned()
            .collect();
        zone_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
        let top_locations = zone_stats.into_iter().take(10).collect();

        // Calculate total play time
        let total_play_time_seconds = completed_sessions
            .iter()
            .filter_map(|session| session.duration_seconds)
            .sum();

        // Calculate hideout time
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

/// Character-specific time tracking data structure for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterTimeTrackingData {
    /// Character ID
    pub character_id: String,
    /// Completed sessions
    pub completed_sessions: Vec<LocationSession>,
    /// Location statistics
    pub stats: Vec<LocationStats>,
}

impl CharacterTimeTrackingData {
    /// Create new character time tracking data
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            completed_sessions: Vec::new(),
            stats: Vec::new(),
        }
    }
}
