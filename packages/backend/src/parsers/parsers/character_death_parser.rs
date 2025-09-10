use crate::parsers::config::ParsersConfig;
use crate::parsers::core::{LogParser, ParseError};
use log::debug;
use regex::Regex;

/// Character death parser for detecting death patterns
/// Matches patterns like "Lylunin has been slain."
#[derive(Clone)]
pub struct CharacterDeathParser {
    config: ParsersConfig,
    death_regex: Regex,
}

impl CharacterDeathParser {
    /// Create a new character death parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
            death_regex: Self::create_death_regex(),
        }
    }

    /// Create a new character death parser with custom configuration
    pub fn with_config(config: ParsersConfig) -> Self {
        Self {
            config,
            death_regex: Self::create_death_regex(),
        }
    }

    /// Create the regex pattern for matching death messages
    fn create_death_regex() -> Regex {
        // Pattern: "{character_name} has been slain."
        // This will match patterns like:
        // - "Lylunin has been slain."
        // - "MyCharacter has been slain."
        Regex::new(r"^(.+?)\s+has\s+been\s+slain\.$")
            .expect("Failed to compile character death regex")
    }

    /// Extract character name from a death log line
    fn extract_character_name(&self, line: &str) -> Result<String, ParseError> {
        debug!("Attempting to extract character name from: {}", line.trim());

        if let Some(captures) = self.death_regex.captures(line.trim()) {
            if captures.len() == 2 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();
                
                debug!("Extracted character name: '{}'", character_name);

                Ok(character_name)
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups"
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character death pattern"
            ))
        }
    }
}

impl LogParser for CharacterDeathParser {
    type Event = String; // character_name

    fn should_parse(&self, line: &str) -> bool {
        // Check if the line contains the death pattern
        self.config
            .matches_patterns("character_death", line)
            .unwrap_or(false)
    }

    /// Parse a log line and return character name
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Character death parser attempting to parse line: {}",
            line.trim()
        );

        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            debug!("Line does not match character death patterns");
            return Err(ParseError::no_pattern_match("character_death"));
        }

        // Extract character name
        let character_name = self.extract_character_name(line)?;

        debug!(
            "Successfully parsed character death: {} has been slain",
            character_name
        );

        Ok(character_name)
    }

    fn parser_name(&self) -> &'static str {
        "character_death"
    }
}

impl Default for CharacterDeathParser {
    fn default() -> Self {
        Self::new()
    }
}
