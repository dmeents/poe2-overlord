use crate::models::events::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent,
};
use crate::parsers::config::{ParsersConfig, SceneTypeConfig};
use crate::parsers::errors::ParseError;
use crate::parsers::traits::LogParser;
use crate::parsers::utils::extract_content_with_patterns;

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

    /// Create a new scene change parser with custom configuration
    pub fn with_config(config: ParsersConfig) -> Self {
        Self { config }
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
    fn extract_scene_content(&self, line: &str) -> Result<String, ParseError> {
        let content =
            extract_content_with_patterns(line, &self.config.scene_change.patterns, '[', ']')?;

        Ok(content.into_owned())
    }
}

impl LogParser for SceneChangeParser {
    type Event = SceneChangeEvent;

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("scene_change", line)
            .unwrap_or(false)
    }

    /// Parse a log line and return a scene change event if valid
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("scene_change"));
        }

        // Extract the scene content
        let content = self.extract_scene_content(line)?;

        // Get scene type configuration
        let scene_config = self.config.get_scene_type_config("scene_change")?;

        // Determine if this is an Act, Zone, or Hideout
        let event = if self.is_hideout_content(&content, scene_config) {
            SceneChangeEvent::Hideout(HideoutChangeEvent {
                hideout_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            })
        } else if self.is_act_content(&content, scene_config) {
            SceneChangeEvent::Act(ActChangeEvent {
                act_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            })
        } else {
            SceneChangeEvent::Zone(ZoneChangeEvent {
                zone_name: content,
                timestamp: chrono::Utc::now().to_rfc3339(),
            })
        };

        Ok(event)
    }

    fn parser_name(&self) -> &'static str {
        "scene_change"
    }
}

impl Default for SceneChangeParser {
    fn default() -> Self {
        Self::new()
    }
}
