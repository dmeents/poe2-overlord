use serde::{Deserialize, Serialize};

/// Zone change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneChangeEvent {
    pub zone_name: String,
    pub timestamp: String,
}

/// Act change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActChangeEvent {
    pub act_name: String,
    pub timestamp: String,
}

/// Combined scene change event that can represent either a zone or act change
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SceneChangeEvent {
    Zone(ZoneChangeEvent),
    Act(ActChangeEvent),
}

impl SceneChangeEvent {
    /// Get the name of the scene (zone or act)
    pub fn get_name(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.zone_name,
            SceneChangeEvent::Act(event) => &event.act_name,
        }
    }

    /// Get the timestamp of the event
    pub fn get_timestamp(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.timestamp,
            SceneChangeEvent::Act(event) => &event.timestamp,
        }
    }

    /// Check if this is a zone change event
    pub fn is_zone(&self) -> bool {
        matches!(self, SceneChangeEvent::Zone(_))
    }

    /// Check if this is an act change event
    pub fn is_act(&self) -> bool {
        matches!(self, SceneChangeEvent::Act(_))
    }
}
