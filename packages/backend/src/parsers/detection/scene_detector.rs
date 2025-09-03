use crate::parsers::config::scene_types::SceneTypeConfig;
use crate::parsers::detection::types::SceneType;
use crate::parsers::events::EventFactory;

/// Scene type detector that determines the type of scene based on content and configuration
pub struct SceneTypeDetector {
    config: SceneTypeConfig,
}

impl SceneTypeDetector {
    /// Create a new scene type detector with the given configuration
    pub fn new(config: SceneTypeConfig) -> Self {
        Self { config }
    }

    /// Determine the scene type based on the content
    pub fn detect_scene_type(&self, content: &str) -> SceneType {
        let lower_content = content.to_lowercase();

        // Check for hideout keywords first (most specific)
        if self.is_hideout_content(&lower_content) {
            return SceneType::Hideout;
        }

        // Check for act keywords
        if self.is_act_content(&lower_content) {
            return SceneType::Act;
        }

        // Default to zone (fallback)
        SceneType::Zone
    }

    /// Create a scene change event based on the detected scene type
    pub fn create_scene_change_event(
        &self,
        content: &str,
    ) -> crate::models::events::SceneChangeEvent {
        let scene_type = self.detect_scene_type(content);
        EventFactory::create_scene_change_event(content, scene_type)
    }

    /// Check if the content represents a hideout based on configuration
    fn is_hideout_content(&self, lower_content: &str) -> bool {
        self.config
            .hideout
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    /// Check if the content represents an act based on configuration
    fn is_act_content(&self, lower_content: &str) -> bool {
        self.config
            .act
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }
}
