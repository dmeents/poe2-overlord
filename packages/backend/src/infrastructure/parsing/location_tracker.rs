use crate::domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent,
};
use crate::domain::location_tracking::models::SceneType;
use crate::infrastructure::parsing::ParsersConfig;
use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct LocationState {
    scene: Option<String>,
    act: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SceneTypeConfig {
    pub hideout_keywords: Vec<String>,
    pub act_keywords: Vec<String>,
    pub zone_keywords: Vec<String>,
}

#[derive(Clone)]
pub struct LocationTracker {
    state: Arc<RwLock<LocationState>>,
    scene_config: SceneTypeConfig,
}

impl LocationTracker {
    pub fn new() -> Self {
        let parser_config = ParsersConfig::default();
        let scene_config = SceneTypeConfig {
            hideout_keywords: parser_config.hideout_keywords().clone(),
            act_keywords: parser_config.act_keywords().clone(),
            zone_keywords: parser_config.zone_keywords().clone(),
        };
        Self::with_config(scene_config)
    }

    pub fn with_config(scene_config: SceneTypeConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(LocationState {
                scene: None,
                act: None,
            })),
            scene_config,
        }
    }

    pub async fn reset_tracking(&self) {
        let mut state = self.state.write().await;
        state.scene = None;
        state.act = None;
        debug!("Scene and act tracking reset");
    }

    pub async fn get_current_scene_and_act(&self) -> (Option<String>, Option<String>) {
        let state = self.state.read().await;
        (state.scene.clone(), state.act.clone())
    }

    pub async fn update_scene(&self, new_scene: &str) -> bool {
        let mut state = self.state.write().await;
        if state.scene.as_ref() != Some(&new_scene.to_string()) {
            state.scene = Some(new_scene.to_string());
            true
        } else {
            false
        }
    }

    pub async fn update_act(&self, new_act: &str) -> bool {
        let mut state = self.state.write().await;
        if state.act.as_ref() != Some(&new_act.to_string()) {
            state.act = Some(new_act.to_string());
            true
        } else {
            false
        }
    }

    pub async fn get_current_scene(&self) -> Option<String> {
        let state = self.state.read().await;
        state.scene.clone()
    }

    pub async fn get_current_act(&self) -> Option<String> {
        let state = self.state.read().await;
        state.act.clone()
    }


    pub fn get_current_scene_sync(&self) -> Option<String> {
        self.state.blocking_read().scene.clone()
    }

    pub fn get_current_act_sync(&self) -> Option<String> {
        self.state.blocking_read().act.clone()
    }

    pub fn detect_scene_type(&self, content: &str) -> SceneType {
        let lower_content = content.to_lowercase();

        if self.is_hideout_content(&lower_content) {
            return SceneType::Hideout;
        }

        if self.is_act_content(&lower_content) {
            return SceneType::Act;
        }

        SceneType::Zone
    }

    pub fn create_scene_change_event(&self, content: &str) -> SceneChangeEvent {
        let scene_type = self.detect_scene_type(content);
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

    pub async fn process_scene_content(&self, content: &str) -> Option<SceneChangeEvent> {
        let event = self.create_scene_change_event(content);
        let result = self.validate_scene_change_event(event).await;

        match &result {
            Some(validated_event) => {
                debug!(
                    "Scene change validated as actual change: {:?}",
                    validated_event
                );
            }
            None => {
                debug!("Scene change content was not an actual change, skipping broadcast");
            }
        }

        result
    }

    fn is_hideout_content(&self, lower_content: &str) -> bool {
        self.scene_config
            .hideout_keywords
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    fn is_act_content(&self, lower_content: &str) -> bool {
        self.scene_config
            .act_keywords
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    pub async fn validate_scene_change_event(
        &self,
        event: SceneChangeEvent,
    ) -> Option<SceneChangeEvent> {
        match &event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Validating hideout change: {}", hideout_event.hideout_name);
                let result = self.update_scene(&hideout_event.hideout_name).await;
                debug!("Hideout change validation result: {}", result);
                if result {
                    Some(event)
                } else {
                    None
                }
            }
            SceneChangeEvent::Zone(zone_event) => {
                debug!("Validating zone change: {}", zone_event.zone_name);
                let result = self.update_scene(&zone_event.zone_name).await;
                debug!("Zone change validation result: {}", result);
                if result {
                    Some(event)
                } else {
                    None
                }
            }
            SceneChangeEvent::Act(act_event) => {
                debug!("Processing act event: {}", act_event.act_name);
                let _ = self.update_act(&act_event.act_name).await;
                debug!("Act event always returned for session continuity");
                Some(event)
            }
        }
    }
}

impl Default for LocationTracker {
    fn default() -> Self {
        Self::new()
    }
}
