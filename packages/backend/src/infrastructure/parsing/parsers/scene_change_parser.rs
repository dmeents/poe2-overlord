use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::utils::extract_content_between_delimiters;
use crate::infrastructure::parsing::{LogParser, ParseError};
use log::{debug, info};

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
        debug!(
            "SCENE PARSER: extract_scene_content - checking line: {}",
            line
        );
        for pattern in &self.patterns {
            debug!("SCENE PARSER: Trying pattern: '{}'", pattern);
            if line.contains(pattern) {
                debug!("SCENE PARSER: Pattern '{}' found in line", pattern);
                match extract_content_between_delimiters(line, pattern, '[', ']') {
                    Ok(content) => {
                        info!(
                            "SCENE PARSER: Successfully extracted content: '{}'",
                            content
                        );
                        return Ok(content.into_owned());
                    }
                    Err(e) => {
                        debug!(
                            "SCENE PARSER: Failed to extract content with pattern '{}': {:?}",
                            pattern, e
                        );
                    }
                }
            } else {
                debug!("SCENE PARSER: Pattern '{}' NOT found in line", pattern);
            }
        }
        debug!("SCENE PARSER: All patterns failed, returning error");
        Err(ParseError::content_extraction_failed(line))
    }
}

impl LogParser for SceneChangeParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        // Check if line contains [INFO] or [DEBUG] log levels with SCENE
        let has_scene = line.contains("[SCENE]");
        if has_scene {
            debug!("SCENE PARSER: Line contains [SCENE]: {}", line);
        }

        let matches = self.patterns.iter().any(|pattern| line.contains(pattern));
        if matches {
            info!(
                "SCENE PARSER: ✓ Line matches scene change pattern: {}",
                line
            );
        } else if has_scene {
            debug!(
                "SCENE PARSER: ✗ Line has [SCENE] but doesn't match patterns: {}",
                line
            );
            for pattern in &self.patterns {
                debug!(
                    "SCENE PARSER:   Pattern '{}' match: {}",
                    pattern,
                    line.contains(pattern)
                );
            }
        }
        matches
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!("SCENE PARSER: Attempting to parse line: {}", line);

        if !self.should_parse(line) {
            debug!("SCENE PARSER: No pattern match for line");
            return Err(ParseError::no_pattern_match("scene_change"));
        }

        let content = self.extract_scene_content(line)?;
        info!(
            "SCENE PARSER: Successfully extracted scene content: '{}'",
            content
        );
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
