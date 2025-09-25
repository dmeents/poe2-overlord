use crate::domain::location_tracking::models::LocationState;
use crate::domain::location_tracking::models::SceneType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationTrackingEvent {
    LocationStateChanged {
        old_state: Option<LocationState>,
        new_state: LocationState,
        timestamp: String,
    },
    
    SceneChangeDetected {
        scene_type: SceneType,
        scene_name: String,
        timestamp: String,
    },
    
    ActChangeDetected {
        act_name: String,
        timestamp: String,
    },
    
    ZoneChangeDetected {
        zone_name: String,
        timestamp: String,
    },
    
    HideoutChangeDetected {
        hideout_name: String,
        timestamp: String,
    },
    
    SessionStarted {
        session_id: String,
        timestamp: String,
    },
    
    SessionEnded {
        session_id: String,
        total_changes: u64,
        duration_seconds: u64,
        timestamp: String,
    },
    
    TrackingReset {
        timestamp: String,
    },
    
    ConfigurationUpdated {
        timestamp: String,
    },
    
    HistoryCleared {
        timestamp: String,
    },
    
    TrackingError {
        error_message: String,
        timestamp: String,
    },
}

impl LocationTrackingEvent {
    pub fn location_state_changed(old_state: Option<LocationState>, new_state: LocationState) -> Self {
        Self::LocationStateChanged {
            old_state,
            new_state,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn scene_change_detected(scene_type: SceneType, scene_name: String) -> Self {
        Self::SceneChangeDetected {
            scene_type,
            scene_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn act_change_detected(act_name: String) -> Self {
        Self::ActChangeDetected {
            act_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn zone_change_detected(zone_name: String) -> Self {
        Self::ZoneChangeDetected {
            zone_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn hideout_change_detected(hideout_name: String) -> Self {
        Self::HideoutChangeDetected {
            hideout_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_started(session_id: String) -> Self {
        Self::SessionStarted {
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn session_ended(session_id: String, total_changes: u64, duration_seconds: u64) -> Self {
        Self::SessionEnded {
            session_id,
            total_changes,
            duration_seconds,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn tracking_reset() -> Self {
        Self::TrackingReset {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn configuration_updated() -> Self {
        Self::ConfigurationUpdated {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn history_cleared() -> Self {
        Self::HistoryCleared {
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn tracking_error(error_message: String) -> Self {
        Self::TrackingError {
            error_message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
