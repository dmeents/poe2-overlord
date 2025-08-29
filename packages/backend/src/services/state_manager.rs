use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

/// State manager for tracking scene and act changes
pub struct StateManager {
    pub previous_scene: Arc<RwLock<Option<String>>>,
    pub previous_act: Arc<RwLock<Option<String>>>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        Self {
            previous_scene: Arc::new(RwLock::new(None)),
            previous_act: Arc::new(RwLock::new(None)),
        }
    }

    /// Reset the previous scene and act tracking
    /// This is useful when you want to clear the history and start fresh
    pub async fn reset_tracking(&self) {
        let mut prev_scene = self.previous_scene.write().await;
        *prev_scene = None;
        let mut prev_act = self.previous_act.write().await;
        *prev_act = None;
        debug!("Scene and act tracking reset");
    }

    /// Get the current scene and act being tracked
    pub async fn get_current_scene_and_act(&self) -> (Option<String>, Option<String>) {
        let scene = self.previous_scene.read().await.clone();
        let act = self.previous_act.read().await.clone();
        (scene, act)
    }

    /// Update the scene state and return true if it's a new scene
    pub async fn update_scene(&self, new_scene: &str) -> bool {
        let mut prev_scene = self.previous_scene.write().await;
        if prev_scene.as_ref() != Some(&new_scene.to_string()) {
            *prev_scene = Some(new_scene.to_string());
            true
        } else {
            false
        }
    }

    /// Update the act state and return true if it's a new act
    pub async fn update_act(&self, new_act: &str) -> bool {
        let mut prev_act = self.previous_act.write().await;
        if prev_act.as_ref() != Some(&new_act.to_string()) {
            *prev_act = Some(new_act.to_string());
            true
        } else {
            false
        }
    }

    /// Get the current scene name
    pub async fn get_current_scene(&self) -> Option<String> {
        self.previous_scene.read().await.clone()
    }

    /// Get the current act name
    pub async fn get_current_act(&self) -> Option<String> {
        self.previous_act.read().await.clone()
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}
