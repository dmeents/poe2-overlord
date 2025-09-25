use crate::infrastructure::parsing::ParsersConfig;
use crate::infrastructure::parsing::{LogParser, ParseError};
use crate::infrastructure::parsing::utils::extract_content_by_patterns;
use crate::infrastructure::parsing::manager::ParserResult;

#[derive(Clone)]
pub struct SceneChangeParser {
    config: ParsersConfig,
}

impl SceneChangeParser {
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
        }
    }

    pub fn with_config(config: ParsersConfig) -> Self {
        Self { config }
    }

    fn extract_scene_content(&self, line: &str) -> Result<String, ParseError> {
        let content =
            extract_content_by_patterns(line, &self.config.scene_change.patterns, '[', ']')?;

        Ok(content.into_owned())
    }
}

impl LogParser for SceneChangeParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("scene_change", line)
            .unwrap_or(false)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("scene_change"));
        }

        let content = self.extract_scene_content(line)?;
        Ok(ParserResult::SceneChange(content))
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
