use crate::models::events::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent,
};
use crate::parsers::detection::types::SceneType;

/// Factory for creating scene change events
pub struct EventFactory;

impl EventFactory {
    /// Create a scene change event based on the detected scene type
    pub fn create_scene_change_event(content: &str, scene_type: SceneType) -> SceneChangeEvent {
        let timestamp = chrono::Utc::now().to_rfc3339();

        match scene_type {
            SceneType::Hideout => SceneChangeEvent::Hideout(HideoutChangeEvent {
                hideout_name: content.to_string(),
                timestamp,
            }),
            SceneType::Act => SceneChangeEvent::Act(ActChangeEvent {
                act_name: content.to_string(),
                timestamp,
            }),
            SceneType::Zone => SceneChangeEvent::Zone(ZoneChangeEvent {
                zone_name: content.to_string(),
                timestamp,
            }),
        }
    }
}
