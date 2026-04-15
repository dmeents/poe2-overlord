use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::utils::extract_content_between_delimiters;
use crate::infrastructure::parsing::{LogParser, ParseError};
use log::debug;

#[derive(Clone)]
pub struct SceneChangeParser {
    patterns: Vec<String>,
}

impl SceneChangeParser {
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    fn default_patterns() -> Vec<String> {
        vec![
            "[SCENE] Set Source [".to_string(),
            "[SCENE] Load Source [".to_string(),
        ]
    }

    fn extract_scene_content(&self, line: &str) -> Result<String, ParseError> {
        for pattern in &self.patterns {
            if line.contains(pattern) {
                if let Ok(content) = extract_content_between_delimiters(line, pattern, '[', ']') {
                    debug!("Extracted scene: {content}");
                    return Ok(content.into_owned());
                }
            }
        }
        Err(ParseError::content_extraction_failed(line))
    }
}

impl LogParser for SceneChangeParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.patterns.iter().any(|pattern| line.contains(pattern))
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        // Trust that should_parse() was already called by the manager
        let content = self.extract_scene_content(line)?;
        debug!("Scene change: {content}");
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
