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

/// Hideout change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HideoutChangeEvent {
    pub hideout_name: String,
    pub timestamp: String,
}

/// Server connection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConnectionEvent {
    pub ip_address: String,
    pub port: u16,
    pub timestamp: String,
}

/// Combined scene change event that can represent either a zone, act, or hideout change
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SceneChangeEvent {
    Zone(ZoneChangeEvent),
    Act(ActChangeEvent),
    Hideout(HideoutChangeEvent),
}

/// Unified log event that can represent either a scene change or server connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum LogEvent {
    SceneChange(SceneChangeEvent),
    ServerConnection(ServerConnectionEvent),
}

impl SceneChangeEvent {
    /// Get the name of the scene (zone, act, or hideout)
    pub fn get_name(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.zone_name,
            SceneChangeEvent::Act(event) => &event.act_name,
            SceneChangeEvent::Hideout(event) => &event.hideout_name,
        }
    }

    /// Get the timestamp of the event
    pub fn get_timestamp(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.timestamp,
            SceneChangeEvent::Act(event) => &event.timestamp,
            SceneChangeEvent::Hideout(event) => &event.timestamp,
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

    /// Check if this is a hideout change event
    pub fn is_hideout(&self) -> bool {
        matches!(self, SceneChangeEvent::Hideout(_))
    }
}
