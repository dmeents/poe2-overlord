use crate::models::events::SceneChangeEvent;
use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Player location state for tracking scene and act changes
#[derive(Debug, Clone)]
struct LocationState {
    scene: Option<String>,
    act: Option<String>,
}

/// Location tracker for tracking scene and act changes
#[derive(Clone)]
pub struct LocationTracker {
    state: Arc<RwLock<LocationState>>,
}

impl LocationTracker {
    /// Create a new location tracker
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(LocationState {
                scene: None,
                act: None,
            })),
        }
    }

    /// Reset the previous scene and act tracking
    /// This is useful when you want to clear the history and start fresh
    pub async fn reset_tracking(&self) {
        let mut state = self.state.write().await;
        state.scene = None;
        state.act = None;
        debug!("Scene and act tracking reset");
    }

    /// Get the current scene and act being tracked
    pub async fn get_current_scene_and_act(&self) -> (Option<String>, Option<String>) {
        let state = self.state.read().await;
        (state.scene.clone(), state.act.clone())
    }

    /// Update the scene state and return true if it's a new scene
    pub async fn update_scene(&self, new_scene: &str) -> bool {
        let mut state = self.state.write().await;
        if state.scene.as_ref() != Some(&new_scene.to_string()) {
            state.scene = Some(new_scene.to_string());
            true
        } else {
            false
        }
    }

    /// Update the act state and return true if it's a new act
    pub async fn update_act(&self, new_act: &str) -> bool {
        let mut state = self.state.write().await;
        if state.act.as_ref() != Some(&new_act.to_string()) {
            state.act = Some(new_act.to_string());
            true
        } else {
            false
        }
    }

    /// Get the current scene name
    pub async fn get_current_scene(&self) -> Option<String> {
        let state = self.state.read().await;
        state.scene.clone()
    }

    /// Get the current act name
    pub async fn get_current_act(&self) -> Option<String> {
        let state = self.state.read().await;
        state.act.clone()
    }

    // Synchronous methods for internal use when async is not needed

    /// Get the current scene name synchronously (for internal use)
    pub fn get_current_scene_sync(&self) -> Option<String> {
        self.state.blocking_read().scene.clone()
    }

    /// Get the current act name synchronously (for internal use)
    pub fn get_current_act_sync(&self) -> Option<String> {
        self.state.blocking_read().act.clone()
    }

    /// Validate a scene change event and return it only if it represents an actual change
    /// Returns Some(event) if the scene/act actually changed, None if it's the same as before
    pub async fn validate_scene_change_event(
        &self,
        event: SceneChangeEvent,
    ) -> Option<SceneChangeEvent> {
        let is_change = match &event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Validating hideout change: {}", hideout_event.hideout_name);
                let result = self.update_scene(&hideout_event.hideout_name).await;
                debug!("Hideout change validation result: {}", result);
                result
            }
            SceneChangeEvent::Zone(zone_event) => {
                debug!("Validating zone change: {}", zone_event.zone_name);
                let result = self.update_scene(&zone_event.zone_name).await;
                debug!("Zone change validation result: {}", result);
                result
            }
            SceneChangeEvent::Act(act_event) => {
                debug!("Validating act change: {}", act_event.act_name);
                let result = self.update_act(&act_event.act_name).await;
                debug!("Act change validation result: {}", result);
                result
            }
        };

        if is_change {
            Some(event)
        } else {
            None
        }
    }
}

impl Default for LocationTracker {
    fn default() -> Self {
        Self::new()
    }
}
