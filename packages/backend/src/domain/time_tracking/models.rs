use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocationSession {
    pub character_id: String,
    pub location_id: String,
    pub location_name: String,
    pub location_type: LocationType,
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u64>,
}

impl LocationSession {
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

    pub fn is_active(&self) -> bool {
        self.exit_timestamp.is_none()
    }

    pub fn end_session(&mut self) {
        if self.exit_timestamp.is_none() {
            self.exit_timestamp = Some(Utc::now());
            if let Some(exit_time) = self.exit_timestamp {
                self.duration_seconds =
                    Some((exit_time - self.entry_timestamp).num_seconds().max(0) as u64);
            }
        }
    }

    pub fn get_current_duration_seconds(&self) -> u64 {
        if let Some(exit_time) = self.exit_timestamp {
            (exit_time - self.entry_timestamp).num_seconds().max(0) as u64
        } else {
            (Utc::now() - self.entry_timestamp).num_seconds().max(0) as u64
        }
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationStats {
    pub character_id: String,
    pub location_id: String,
    pub location_name: String,
    pub location_type: LocationType,
    pub total_visits: u32,
    pub total_time_seconds: u64,
    pub average_session_seconds: f64,
    pub last_visited: Option<DateTime<Utc>>,
}

impl LocationStats {
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

    pub fn add_session(&mut self, session: &LocationSession) {
        if let Some(duration) = session.duration_seconds {
            self.total_visits += 1;
            self.total_time_seconds += duration;
            self.average_session_seconds =
                self.total_time_seconds as f64 / self.total_visits as f64;
            self.last_visited = Some(session.exit_timestamp.unwrap_or(Utc::now()));
        }
    }

    pub fn update_with_active_session(&mut self, session: &LocationSession) {
        if session.is_active() {
            self.last_visited = Some(session.entry_timestamp);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingData {
    pub character_id: String,
    pub active_sessions: Vec<LocationSession>,
    pub completed_sessions: Vec<LocationSession>,
    pub all_location_stats: Vec<LocationStats>,
    pub summary: TimeTrackingSummary,
}

impl TimeTrackingData {
    pub fn new(character_id: String) -> Self {
        Self {
            character_id: character_id.clone(),
            active_sessions: Vec::new(),
            completed_sessions: Vec::new(),
            all_location_stats: Vec::new(),
            summary: TimeTrackingSummary::new(character_id),
        }
    }

    pub fn update_summary(&mut self) {
        self.summary = TimeTrackingSummary::from_data(
            &self.character_id,
            &self.active_sessions,
            &self.completed_sessions,
            &self.all_location_stats,
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeTrackingSummary {
    pub character_id: String,
    pub active_sessions: Vec<LocationSession>,
    pub top_locations: Vec<LocationStats>,
    pub total_locations_tracked: usize,
    pub total_active_sessions: usize,
    pub total_play_time_seconds: u64,
    pub total_play_time_since_process_start_seconds: u64,
    pub total_hideout_time_seconds: u64,
}

impl TimeTrackingSummary {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterTimeTrackingData {
    pub character_id: String,
    pub completed_sessions: Vec<LocationSession>,
    pub stats: Vec<LocationStats>,
}

impl CharacterTimeTrackingData {
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            completed_sessions: Vec::new(),
            stats: Vec::new(),
        }
    }
}
