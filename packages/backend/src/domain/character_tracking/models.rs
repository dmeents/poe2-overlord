use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Combined character tracking data that includes both location and time tracking
/// This replaces the separate location and time tracking domains with a unified approach
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CharacterTrackingData {
    /// Character this data belongs to
    pub character_id: String,
    /// Current location state
    pub current_location: Option<LocationState>,
    /// Summary statistics
    pub summary: TrackingSummary,
    /// Zone statistics (aggregated data per location)
    pub zones: Vec<ZoneStats>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl CharacterTrackingData {
    /// Creates new empty character tracking data for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id: character_id.clone(),
            current_location: None,
            summary: TrackingSummary::new(character_id),
            zones: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    /// Updates the last_updated timestamp to current time
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Recalculates summary statistics from zone data
    pub fn update_summary(&mut self) {
        self.summary = TrackingSummary::from_zones(&self.character_id, &self.zones);
        self.touch();
    }

    /// Finds a zone by location ID
    pub fn find_zone(&self, location_id: &str) -> Option<&ZoneStats> {
        self.zones
            .iter()
            .find(|zone| zone.location_id == location_id)
    }

    /// Finds a zone by location ID (mutable)
    pub fn find_zone_mut(&mut self, location_id: &str) -> Option<&mut ZoneStats> {
        self.zones
            .iter_mut()
            .find(|zone| zone.location_id == location_id)
    }

    /// Adds or updates a zone with the given statistics
    pub fn upsert_zone(&mut self, zone: ZoneStats) {
        if let Some(existing_zone) = self.find_zone_mut(&zone.location_id) {
            *existing_zone = zone;
        } else {
            self.zones.push(zone);
        }
        self.update_summary();
    }

    /// Gets the current active zone (only one can be active at a time)
    pub fn get_active_zone(&self) -> Option<&ZoneStats> {
        self.zones.iter().find(|zone| zone.is_active)
    }

    /// Gets zones sorted by total time spent (descending)
    pub fn get_zones_by_time(&self) -> Vec<&ZoneStats> {
        let mut zones: Vec<&ZoneStats> = self.zones.iter().collect();
        zones.sort_by(|a, b| b.duration.cmp(&a.duration));
        zones
    }
}

/// Current location state for a character
/// Simplified to focus on current location without complex session management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationState {
    /// Current scene/zone name (hideout, zone, etc.)
    pub scene: Option<String>,
    /// Current act name (Act 1, Act 2, etc.)
    pub act: Option<String>,
    /// Type of location (Zone, Act, Hideout)
    pub location_type: LocationType,
    /// Timestamp of the last location update
    pub last_updated: DateTime<Utc>,
}

impl LocationState {
    /// Creates a new location state with current timestamp
    pub fn new() -> Self {
        Self {
            scene: None,
            act: None,
            location_type: LocationType::Zone,
            last_updated: Utc::now(),
        }
    }

    /// Creates a new location state for a specific location
    pub fn new_for_location(
        scene: Option<String>,
        act: Option<String>,
        location_type: LocationType,
    ) -> Self {
        Self {
            scene,
            act,
            location_type,
            last_updated: Utc::now(),
        }
    }

    /// Updates the current scene and returns true if it actually changed
    /// Returns false if the scene is the same as the current one
    pub fn update_scene(&mut self, new_scene: String, location_type: LocationType) -> bool {
        if self.scene.as_ref() != Some(&new_scene) || self.location_type != location_type {
            self.scene = Some(new_scene);
            self.location_type = location_type;
            self.last_updated = Utc::now();
            true
        } else {
            false
        }
    }

    /// Updates the current act and returns true if it actually changed
    /// Returns false if the act is the same as the current one
    pub fn update_act(&mut self, new_act: String) -> bool {
        if self.act.as_ref() != Some(&new_act) {
            self.act = Some(new_act);
            self.last_updated = Utc::now();
            true
        } else {
            false
        }
    }

    /// Resets the location state to initial values
    /// Clears scene and act, updates timestamps
    pub fn reset(&mut self) {
        self.scene = None;
        self.act = None;
        self.location_type = LocationType::Zone;
        self.last_updated = Utc::now();
    }

    /// Gets a reference to the current scene name
    pub fn get_current_scene(&self) -> Option<&String> {
        self.scene.as_ref()
    }

    /// Gets a reference to the current act name
    pub fn get_current_act(&self) -> Option<&String> {
        self.act.as_ref()
    }
}

/// Summary statistics for character tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TrackingSummary {
    /// Character this summary belongs to
    pub character_id: String,
    /// Total play time across all zones (seconds)
    pub total_play_time: u64,
    /// Total time spent in hideouts (seconds)
    pub total_hideout_time: u64,
    /// Total number of unique zones visited
    pub total_zones_visited: usize,
    /// Total number of deaths across all zones
    pub total_deaths: u32,
    /// Current active zone (if any)
    pub current_zone: Option<String>,
    /// Current active act (if any)
    pub current_act: Option<String>,
}

impl TrackingSummary {
    /// Creates new empty summary for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            total_play_time: 0,
            total_hideout_time: 0,
            total_zones_visited: 0,
            total_deaths: 0,
            current_zone: None,
            current_act: None,
        }
    }

    /// Creates summary from zone data
    pub fn from_zones(character_id: &str, zones: &[ZoneStats]) -> Self {
        let total_play_time = zones.iter().map(|zone| zone.duration).sum();
        let total_hideout_time = zones
            .iter()
            .filter(|zone| zone.location_type == LocationType::Hideout)
            .map(|zone| zone.duration)
            .sum();
        let total_deaths = zones.iter().map(|zone| zone.deaths).sum();

        // Find current active zone and act
        let active_zones: Vec<&ZoneStats> = zones.iter().filter(|zone| zone.is_active).collect();
        let current_zone = active_zones.first().map(|zone| zone.location_name.clone());
        let current_act = active_zones.first().and_then(|zone| zone.act.clone());

        Self {
            character_id: character_id.to_string(),
            total_play_time,
            total_hideout_time,
            total_zones_visited: zones.len(),
            total_deaths,
            current_zone,
            current_act,
        }
    }
}

/// Aggregated statistics for a specific zone/location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ZoneStats {
    /// Unique identifier for the location
    pub location_id: String,
    /// Human-readable name of the location
    pub location_name: String,
    /// Type of location (Zone, Act, Hideout)
    pub location_type: LocationType,
    /// Act this zone belongs to (1, 2, 3, 4, interlude, atlas)
    pub act: Option<String>,
    /// Total time spent in this zone (seconds)
    pub duration: u64,
    /// Number of deaths in this zone
    pub deaths: u32,
    /// Number of visits to this zone
    pub visits: u32,
    /// Timestamp of first visit
    pub first_visited: DateTime<Utc>,
    /// Timestamp of most recent visit
    pub last_visited: DateTime<Utc>,
    /// Whether this zone is currently active
    pub is_active: bool,
}

impl ZoneStats {
    /// Creates new zone stats for a location
    pub fn new(
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            location_id,
            location_name,
            location_type,
            act,
            duration: 0,
            deaths: 0,
            visits: 0,
            first_visited: now,
            last_visited: now,
            is_active: true,
        }
    }

    /// Adds time to the zone duration
    pub fn add_time(&mut self, seconds: u64) {
        self.duration += seconds;
        self.last_visited = Utc::now();
    }

    /// Records a death in this zone
    pub fn record_death(&mut self) {
        self.deaths += 1;
        self.last_visited = Utc::now();
    }

    /// Records a visit to this zone
    pub fn record_visit(&mut self) {
        self.visits += 1;
        self.last_visited = Utc::now();
    }

    /// Activates the zone (character enters)
    pub fn activate(&mut self) {
        self.is_active = true;
        self.record_visit();
    }

    /// Deactivates the zone (character leaves)
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.last_visited = Utc::now();
    }
}

/// Types of locations that can be tracked in the game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum LocationType {
    /// A playable game zone/area
    #[default]
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

/// Configuration for detecting different scene types based on keyword matching
/// Used by the scene type detector to categorize game content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTypeConfig {
    /// Keywords that indicate hideout content
    pub hideout_keywords: Vec<String>,
    /// Keywords that indicate act content
    pub act_keywords: Vec<String>,
    /// Keywords that indicate zone content
    pub zone_keywords: Vec<String>,
}

impl SceneTypeConfig {
    /// Creates a new empty scene type configuration
    pub fn new() -> Self {
        Self {
            hideout_keywords: Vec::new(),
            act_keywords: Vec::new(),
            zone_keywords: Vec::new(),
        }
    }

    /// Creates a new scene type configuration with provided keywords
    pub fn with_keywords(
        hideout_keywords: Vec<String>,
        act_keywords: Vec<String>,
        zone_keywords: Vec<String>,
    ) -> Self {
        Self {
            hideout_keywords,
            act_keywords,
            zone_keywords,
        }
    }
}

impl Default for SceneTypeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for character tracking operations
#[derive(Debug, thiserror::Error)]
pub enum CharacterTrackingError {
    /// Scene type detection failed or invalid scene type provided
    #[error("Invalid scene type: {scene_type}")]
    InvalidSceneType { scene_type: String },

    /// Configuration-related errors (invalid settings, etc.)
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    /// Character ID is required but not provided
    #[error("Character ID is required: {message}")]
    CharacterIdRequired { message: String },

    /// Zone not found for the given character
    #[error("Zone not found: {zone_id} for character {character_id}")]
    ZoneNotFound {
        zone_id: String,
        character_id: String,
    },
}
