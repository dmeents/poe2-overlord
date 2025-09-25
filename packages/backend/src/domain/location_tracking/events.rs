use crate::domain::location_tracking::models::LocationState;
use crate::domain::location_tracking::models::SceneType;
use serde::{Deserialize, Serialize};

/// Events emitted by the location tracking system
/// Used for notifying other components about location changes and system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationTrackingEvent {
    /// Emitted when the overall location state changes
    LocationStateChanged {
        old_state: Option<LocationState>,
        new_state: LocationState,
        timestamp: String,
    },
    
    /// Emitted when any scene change is detected
    SceneChangeDetected {
        scene_type: SceneType,
        scene_name: String,
        timestamp: String,
    },
    
    /// Emitted when an act change is detected
    ActChangeDetected {
        act_name: String,
        timestamp: String,
    },
    
    /// Emitted when a zone change is detected
    ZoneChangeDetected {
        zone_name: String,
        timestamp: String,
    },
    
    /// Emitted when a hideout change is detected
    HideoutChangeDetected {
        hideout_name: String,
        timestamp: String,
    },
    
    /// Emitted when a new tracking session starts
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    /// Emitted when a tracking session ends
    SessionEnded {
        session_id: String,
        total_changes: u64,
        duration_seconds: u64,
        timestamp: String,
    },
    
    /// Emitted when tracking is reset
    TrackingReset {
        timestamp: String,
    },
    
    /// Emitted when configuration is updated
    ConfigurationUpdated {
        timestamp: String,
    },
    
    /// Emitted when history is cleared
    HistoryCleared {
        timestamp: String,
    },
    
    /// Emitted when a tracking error occurs
    TrackingError {
        error_message: String,
        timestamp: String,
    },
}

impl LocationTrackingEvent {
    /// Creates a location state changed event
    pub fn location_state_changed(old_state: Option<LocationState>, new_state: LocationState) -> Self {
        Self::LocationStateChanged {
            old_state,
            new_state,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a scene change detected event
    pub fn scene_change_detected(scene_type: SceneType, scene_name: String) -> Self {
        Self::SceneChangeDetected {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates an act change detected event
    pub fn act_change_detected(act_name: String) -> Self {
        Self::ActChangeDetected {
            act_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a zone change detected event
    pub fn zone_change_detected(zone_name: String) -> Self {
        Self::ZoneChangeDetected {
            zone_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a hideout change detected event
    pub fn hideout_change_detected(hideout_name: String) -> Self {
        Self::HideoutChangeDetected {
            hideout_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session started event
    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a session ended event
    pub fn session_ended(session_id: String, total_changes: u64, duration_seconds: u64) -> Self {
        Self::SessionEnded {
            session_id,
            total_changes,
            duration_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a tracking reset event
    pub fn tracking_reset() -> Self {
        Self::TrackingReset {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a configuration updated event
    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a history cleared event
    pub fn history_cleared() -> Self {
        Self::HistoryCleared {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Creates a tracking error event
    pub fn tracking_error(error_message: String) -> Self {
        Self::TrackingError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
