use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::{LogParser, ParseError};
use regex::Regex;

#[derive(Clone)]
pub struct CharacterLevelParser {
    level_regex: Regex,
}

impl CharacterLevelParser {
    pub fn new() -> Self {
        Self {
            level_regex: Self::create_level_regex(),
        }
    }

    fn create_level_regex() -> Regex {
        // Regex captures: (character_name) (class_or_ascendency - ignored) (level)
        Regex::new(r"\[INFO Client \d+\]\s*:\s*(.+?)\s+\(.+?\)\s+is\s+now\s+level\s+(\d+)$")
            .expect("Failed to compile character level regex")
    }

    fn extract_character_info(&self, line: &str) -> Result<(String, u32), ParseError> {
        if let Some(captures) = self.level_regex.captures(line.trim()) {
            if captures.len() == 3 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();
                let level_str = captures.get(2).unwrap().as_str().trim();

                let level = level_str.parse::<u32>().map_err(|_| {
                    ParseError::content_extraction_failed(&format!(
                        "Failed to parse level '{}' as number",
                        level_str
                    ))
                })?;

                Ok((character_name, level))
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups",
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character level-up pattern",
            ))
        }
    }
}

impl LogParser for CharacterLevelParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.level_regex.is_match(line)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("character_level"));
        }

        let (character_name, level) = self.extract_character_info(line)?;

        if !(1..=100).contains(&level) {
            return Err(ParseError::content_extraction_failed(&format!(
                "Level {} is outside valid range (1-100)",
                level
            )));
        }

        Ok(ParserResult::CharacterLevel((character_name, level)))
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
