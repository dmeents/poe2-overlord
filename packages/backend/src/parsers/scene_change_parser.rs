use crate::models::events::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent,
};
use crate::parsers::config::{ParsersConfig, SceneTypeConfig};

/// Trait for parsing log lines into events
pub trait LogParser {
    type Event;

    /// Parse a log line and return an event if valid
    fn parse_line(&self, line: &str) -> Option<Self::Event>;
}

/// Scene change parser for detecting scene transition patterns
#[derive(Clone)]
pub struct SceneChangeParser {
    config: ParsersConfig,
}

impl SceneChangeParser {
    /// Create a new scene change parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
        }
    }

    /// Check if a line should be parsed by this parser
    pub fn should_parse(&self, line: &str) -> bool {
        self.config.matches_patterns("scene_change", line)
    }

    /// Determine if the content represents a Hideout based on configuration
    fn is_hideout_content(&self, content: &str, scene_config: &SceneTypeConfig) -> bool {
        let lower_content = content.to_lowercase();
        scene_config
            .hideout
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    /// Determine if the content represents an Act based on configuration
    fn is_act_content(&self, content: &str, scene_config: &SceneTypeConfig) -> bool {
        let lower_content = content.to_lowercase();
        scene_config
            .act
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    /// Extract content from scene change patterns
    fn extract_scene_content(&self, line: &str) -> Option<String> {
        // Try each pattern from the configuration
        for pattern in &self.config.scene_change.patterns {
            if let Some(start) = line.find(pattern) {
                let content_start = start + pattern.len();
                if let Some(end) = line[content_start..].find("]") {
                    let content = line[content_start..content_start + end].trim();

                    // Skip null or empty content
                    if content.is_empty()
                        || content == "(null)"
                        || content == "(undefined)"
                        || content == "undefined"
                        || content.to_lowercase() == "null"
                        || content.to_lowercase() == "undefined"
                    {
                        continue;
                    }

                    return Some(content.to_string());
                }
            }
        }
        None
    }
}

impl LogParser for SceneChangeParser {
    type Event = SceneChangeEvent;

    /// Parse a log line and return a scene change event if valid
    fn parse_line(&self, line: &str) -> Option<SceneChangeEvent> {
        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            return None;
        }

        // Extract the scene content
        let content = self.extract_scene_content(line)?;

        // Get scene type configuration
        let scene_config = self.config.get_scene_type_config("scene_change")?;

        // Determine if this is an Act, Zone, or Hideout
        if self.is_hideout_content(&content, scene_config) {
            Some(SceneChangeEvent::Hideout(HideoutChangeEvent {
                hideout_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        } else if self.is_act_content(&content, scene_config) {
            Some(SceneChangeEvent::Act(ActChangeEvent {
                act_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        } else {
            Some(SceneChangeEvent::Zone(ZoneChangeEvent {
                zone_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}

impl Default for SceneChangeParser {
    fn default() -> Self {
        Self::new()
    }
}
