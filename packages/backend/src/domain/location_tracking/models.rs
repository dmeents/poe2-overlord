use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Scene Type Definition
// ============================================================================

/// Represents the different types of scenes that can be detected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SceneType {
    Hideout,
    Act,
    Zone,
}

// ============================================================================
// Location Tracking Models
// ============================================================================

/// Location state for tracking current scene and act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationState {
    pub scene: Option<String>,
    pub act: Option<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub session_start_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl LocationState {
    pub fn new() -> Self {
        Self {
            scene: None,
            act: None,
            last_updated: chrono::Utc::now(),
            session_start_time: Some(chrono::Utc::now()),
        }
    }

    pub fn update_scene(&mut self, new_scene: String) -> bool {
        if self.scene.as_ref() != Some(&new_scene) {
            self.scene = Some(new_scene);
            self.last_updated = chrono::Utc::now();
            true
        } else {
            false
        }
    }

    pub fn update_act(&mut self, new_act: String) -> bool {
        if self.act.as_ref() != Some(&new_act) {
            self.act = Some(new_act);
            self.last_updated = chrono::Utc::now();
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.scene = None;
        self.act = None;
        self.last_updated = chrono::Utc::now();
        self.session_start_time = Some(chrono::Utc::now());
    }

    pub fn get_current_scene(&self) -> Option<&String> {
        self.scene.as_ref()
    }

    pub fn get_current_act(&self) -> Option<&String> {
        self.act.as_ref()
    }
}

/// Scene type configuration for location tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTypeConfig {
    pub hideout_keywords: Vec<String>,
    pub act_keywords: Vec<String>,
    pub zone_keywords: Vec<String>,
}

impl SceneTypeConfig {
    pub fn new() -> Self {
        Self {
            hideout_keywords: Vec::new(),
            act_keywords: Vec::new(),
            zone_keywords: Vec::new(),
        }
    }

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

/// Location tracking session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingSession {
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub total_scene_changes: u64,
    pub total_act_changes: u64,
    pub total_zone_changes: u64,
    pub total_hideout_changes: u64,
    pub current_location_state: LocationState,
    pub location_history: Vec<LocationHistoryEntry>,
    pub is_active: bool,
}

impl LocationTrackingSession {
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

    pub fn end_session(&mut self) {
        self.end_time = Some(chrono::Utc::now());
        self.is_active = false;
    }

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
        
        // Add to history
        self.location_history.push(LocationHistoryEntry {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now(),
        });
    }

    pub fn get_session_duration(&self) -> Option<chrono::Duration> {
        if let Some(end_time) = self.end_time {
            Some(end_time - self.start_time)
        } else {
            Some(chrono::Utc::now() - self.start_time)
        }
    }

    pub fn get_total_changes(&self) -> u64 {
        self.total_scene_changes
    }
}

/// Location history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHistoryEntry {
    pub scene_type: SceneType,
    pub scene_name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Location tracking statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingStats {
    pub total_sessions: u64,
    pub total_scene_changes: u64,
    pub total_act_changes: u64,
    pub total_zone_changes: u64,
    pub total_hideout_changes: u64,
    pub average_session_duration_seconds: f64,
    pub most_visited_scenes: HashMap<String, u64>,
    pub last_activity_time: chrono::DateTime<chrono::Utc>,
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

/// Location tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationTrackingConfig {
    pub scene_type_config: SceneTypeConfig,
    pub enable_history_tracking: bool,
    pub max_history_entries: usize,
    pub enable_statistics: bool,
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

/// Location tracking error types
#[derive(Debug, thiserror::Error)]
pub enum LocationTrackingError {
    #[error("Invalid scene type: {scene_type}")]
    InvalidSceneType { scene_type: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Session error: {message}")]
    SessionError { message: String },
    
    #[error("History tracking error: {message}")]
    HistoryTrackingError { message: String },
    
    #[error("Statistics error: {message}")]
    StatisticsError { message: String },
}
