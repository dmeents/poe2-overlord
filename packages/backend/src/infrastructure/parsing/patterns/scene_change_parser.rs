use crate::parsers::config::ParsersConfig;
use crate::infrastructure::parsing::{LogParser, ParseError};
use crate::infrastructure::parsing::extraction::extract_content_by_patterns;

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

    /// Extract content from scene change patterns
    fn extract_scene_content(&self, line: &str) -> Result<String, ParseError> {
        let content =
            extract_content_by_patterns(line, &self.config.scene_change.patterns, '[', ']')?;

        Ok(content.into_owned())
    }
}

impl LogParser for SceneChangeParser {
    type Event = String; // Now returns raw content instead of SceneChangeEvent

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("scene_change", line)
            .unwrap_or(false)
    }

    /// Parse a log line and return the extracted scene content
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("scene_change"));
        }

        let content = self.extract_scene_content(line)?;
        Ok(content)
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
