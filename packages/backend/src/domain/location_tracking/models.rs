use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the different types of scenes/locations in Path of Exile 2
/// Used to categorize and track different areas the player visits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SceneType {
    /// Player's personal hideout area
    Hideout,
    /// Story act areas (Act 1, Act 2, etc.)
    Act,
    /// General zone areas within acts
    Zone,
}

/// Current location state tracking the player's position in the game
/// Maintains the current scene, act, and timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationState {
    /// Current scene/zone name (hideout, zone, etc.)
    pub scene: Option<String>,
    /// Current act name (Act 1, Act 2, etc.)
    pub act: Option<String>,
    /// Timestamp of the last location update
    pub last_updated: chrono::DateTime<chrono::Utc>,
    /// When the current tracking session started
    pub session_start_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl LocationState {
    /// Creates a new location state with current timestamp
    pub fn new() -> Self {
        Self {
            scene: None,
            act: None,
            last_updated: chrono::Utc::now(),
            session_start_time: Some(chrono::Utc::now()),
        }
    }

    /// Updates the current scene and returns true if it actually changed
    /// Returns false if the scene is the same as the current one
    pub fn update_scene(&mut self, new_scene: String) -> bool {
        if self.scene.as_ref() != Some(&new_scene) {
            self.scene = Some(new_scene);
            self.last_updated = chrono::Utc::now();
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
            self.last_updated = chrono::Utc::now();
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
        self.last_updated = chrono::Utc::now();
        self.session_start_time = Some(chrono::Utc::now());
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

/// Represents a tracking session for monitoring location changes
/// Tracks statistics and history for a single play session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingSession {
    /// Unique identifier for this session
    pub session_id: String,
    /// When the session started
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// When the session ended (None if still active)
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Total number of scene changes in this session
    pub total_scene_changes: u64,
    /// Total number of act changes in this session
    pub total_act_changes: u64,
    /// Total number of zone changes in this session
    pub total_zone_changes: u64,
    /// Total number of hideout changes in this session
    pub total_hideout_changes: u64,
    /// Current location state for this session
    pub current_location_state: LocationState,
    /// History of all location changes in this session
    pub location_history: Vec<LocationHistoryEntry>,
    /// Whether this session is currently active
    pub is_active: bool,
}

impl LocationTrackingSession {
    /// Creates a new tracking session with a unique ID and current timestamp
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            total_scene_changes: 0,
            total_act_changes: 0,
            total_zone_changes: 0,
            total_hideout_changes: 0,
            current_location_state: LocationState::new(),
            location_history: Vec::new(),
            is_active: true,
        }
    }

    /// Ends the current session and marks it as inactive
    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

    /// Records a scene change and updates relevant counters and state
    /// Updates the location state and adds entry to history
    pub fn record_scene_change(&mut self, scene_type: SceneType, scene_name: String) {
        match scene_type {
            SceneType::Act => {
                self.total_act_changes += 1;
                self.current_location_state.update_act(scene_name.clone());
            }
            SceneType::Zone => {
                self.total_zone_changes += 1;
                self.current_location_state.update_scene(scene_name.clone());
            }
            SceneType::Hideout => {
                self.total_hideout_changes += 1;
                self.current_location_state.update_scene(scene_name.clone());
            }
        }

        self.total_scene_changes += 1;

        self.location_history.push(LocationHistoryEntry {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now(),
        });
    }

    /// Calculates the duration of the session
    /// Returns None if session hasn't started, or duration from start to end/now
    pub fn get_session_duration(&self) -> Option<chrono::Duration> {
        if let Some(end_time) = self.end_time {
            Some(end_time - self.start_time)
        } else {
            Some(chrono::Utc::now() - self.start_time)
        }
    }

    /// Gets the total number of scene changes in this session
    pub fn get_total_changes(&self) -> u64 {
        self.total_scene_changes
    }
}

/// Represents a single entry in the location tracking history
/// Records when and where a scene change occurred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHistoryEntry {
    /// Type of scene that was entered
    pub scene_type: SceneType,
    /// Name of the scene/zone
    pub scene_name: String,
    /// When this change occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Aggregated statistics for location tracking across all sessions
/// Provides insights into player behavior and location patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingStats {
    /// Total number of tracking sessions
    pub total_sessions: u64,
    /// Total scene changes across all sessions
    pub total_scene_changes: u64,
    /// Total act changes across all sessions
    pub total_act_changes: u64,
    /// Total zone changes across all sessions
    pub total_zone_changes: u64,
    /// Total hideout changes across all sessions
    pub total_hideout_changes: u64,
    /// Average session duration in seconds
    pub average_session_duration_seconds: f64,
    /// Map of scene names to visit counts
    pub most_visited_scenes: HashMap<String, u64>,
    /// Timestamp of the last recorded activity
    pub last_activity_time: chrono::DateTime<chrono::Utc>,
    /// Current active session if any
    pub current_session: Option<LocationTrackingSession>,
}

impl Default for LocationTrackingStats {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            total_scene_changes: 0,
            total_act_changes: 0,
            total_zone_changes: 0,
            total_hideout_changes: 0,
            average_session_duration_seconds: 0.0,
            most_visited_scenes: HashMap::new(),
            last_activity_time: chrono::Utc::now(),
            current_session: None,
        }
    }
}

/// Configuration settings for location tracking behavior
/// Controls what data is collected and how long it's retained
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingConfig {
    /// Configuration for scene type detection
    pub scene_type_config: SceneTypeConfig,
    /// Whether to track and store location history
    pub enable_history_tracking: bool,
    /// Maximum number of history entries to keep
    pub max_history_entries: usize,
    /// Whether to collect and maintain statistics
    pub enable_statistics: bool,
    /// Session timeout in minutes before auto-ending
    pub session_timeout_minutes: u64,
}

impl Default for LocationTrackingConfig {
    fn default() -> Self {
        Self {
            scene_type_config: SceneTypeConfig::default(),
            enable_history_tracking: true,
            max_history_entries: 1000,
            enable_statistics: true,
            session_timeout_minutes: 60,
        }
    }
}

/// Error types for location tracking operations
/// Provides specific error information for different failure scenarios
#[derive(Debug, thiserror::Error)]
pub enum LocationTrackingError {
    /// Scene type detection failed or invalid scene type provided
    #[error("Invalid scene type: {scene_type}")]
    InvalidSceneType { scene_type: String },

    /// Configuration-related errors (invalid settings, etc.)
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    /// Session management errors (start/end failures, etc.)
    #[error("Session error: {message}")]
    SessionError { message: String },

    /// History tracking errors (storage failures, etc.)
    #[error("History tracking error: {message}")]
    HistoryTrackingError { message: String },

    /// Statistics calculation or storage errors
    #[error("Statistics error: {message}")]
    StatisticsError { message: String },
}
