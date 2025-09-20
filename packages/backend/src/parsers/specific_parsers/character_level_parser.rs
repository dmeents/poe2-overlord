use crate::parsers::config::ParsersConfig;
use crate::parsers::core::{LogParser, ParseError};
use crate::models::character::CharacterClass;
use log::debug;
use regex::Regex;

/// Character level parser for detecting level-up patterns
/// Matches patterns like "Lylunin (Sorceress) is now level 2"
#[derive(Clone)]
pub struct CharacterLevelParser {
    config: ParsersConfig,
    level_regex: Regex,
}

impl CharacterLevelParser {
    /// Create a new character level parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
            level_regex: Self::create_level_regex(),
        }
    }

    /// Create a new character level parser with custom configuration
    pub fn with_config(config: ParsersConfig) -> Self {
        Self {
            config,
            level_regex: Self::create_level_regex(),
        }
    }

    /// Create the regex pattern for matching level-up messages
    fn create_level_regex() -> Regex {
        // Pattern: "{character_name} ({character_class}) is now level {level}"
        // This will match patterns like:
        // - "Lylunin (Sorceress) is now level 2"
        // - "MyCharacter (Warrior) is now level 15"
        // The pattern matches the message part after [INFO Client X] :
        Regex::new(r"\[INFO Client \d+\]\s*:\s*(\S.*?)\s+\((.+?)\)\s+is\s+now\s+level\s+(\d+)$")
            .expect("Failed to compile character level regex")
    }

    /// Extract character information from a level-up log line
    fn extract_character_info(&self, line: &str) -> Result<(String, CharacterClass, u32), ParseError> {
        debug!("Attempting to extract character info from: {}", line.trim());

        if let Some(captures) = self.level_regex.captures(line.trim()) {
            if captures.len() == 4 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();
                let character_class_str = captures.get(2).unwrap().as_str().trim();
                let level_str = captures.get(3).unwrap().as_str().trim();
                
                let level = level_str.parse::<u32>().map_err(|_| {
                    ParseError::content_extraction_failed(&format!(
                        "Failed to parse level '{}' as number",
                        level_str
                    ))
                })?;

                let character_class = self.parse_character_class(character_class_str)?;

                debug!(
                    "Extracted character info: name='{}', class='{}', level={}",
                    character_name, character_class_str, level
                );

                Ok((character_name, character_class, level))
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups"
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character level-up pattern"
            ))
        }
    }

    /// Parse a character class string into a CharacterClass enum
    fn parse_character_class(&self, class_str: &str) -> Result<CharacterClass, ParseError> {
        match class_str.to_lowercase().as_str() {
            "warrior" => Ok(CharacterClass::Warrior),
            "sorceress" => Ok(CharacterClass::Sorceress),
            "ranger" => Ok(CharacterClass::Ranger),
            "huntress" => Ok(CharacterClass::Huntress),
            "monk" => Ok(CharacterClass::Monk),
            "mercenary" => Ok(CharacterClass::Mercenary),
            "witch" => Ok(CharacterClass::Witch),
            _ => Err(ParseError::content_extraction_failed(&format!(
                "Invalid character class: '{}'",
                class_str
            ))),
        }
    }

}

impl LogParser for CharacterLevelParser {
    type Event = (String, CharacterClass, u32); // (character_name, character_class, level)

    fn should_parse(&self, line: &str) -> bool {
        // Check if the line contains the level-up pattern
        self.config
            .matches_patterns("character_level", line)
            .unwrap_or(false)
    }

    /// Parse a log line and return character level-up information
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Character level parser attempting to parse line: {}",
            line.trim()
        );

        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            debug!("Line does not match character level patterns");
            return Err(ParseError::no_pattern_match("character_level"));
        }

        // Extract character information
        let (character_name, character_class, level) = self.extract_character_info(line)?;

        // Validate level (reasonable range)
        if !(1..=100).contains(&level) {
            return Err(ParseError::content_extraction_failed(&format!(
                "Level {} is outside valid range (1-100)",
                level
            )));
        }

        debug!(
            "Successfully parsed character level-up: {} ({}) -> level {}",
            character_name, character_class, level
        );

        Ok((character_name, character_class, level))
    }

    fn parser_name(&self) -> &'static str {
        "character_level"
    }
}

impl Default for CharacterLevelParser {
    fn default() -> Self {
        Self::new()
    }
}
