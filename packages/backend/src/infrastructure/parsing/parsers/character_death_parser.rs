use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::{LogParser, ParseError};
use log::debug;
use regex::Regex;

#[derive(Clone)]
pub struct CharacterDeathParser {
    death_regex: Regex,
}

impl CharacterDeathParser {
    pub fn new() -> Self {
        Self {
            death_regex: Self::create_death_regex(),
        }
    }

    fn create_death_regex() -> Regex {
        Regex::new(r"\[INFO Client \d+\]\s*:\s*(\S.*?)\s+has\s+been\s+slain\.$")
            .expect("Failed to compile character death regex")
    }

    fn extract_character_name(&self, line: &str) -> Result<String, ParseError> {
        debug!("Attempting to extract character name from: {}", line.trim());

        if let Some(captures) = self.death_regex.captures(line.trim()) {
            if captures.len() == 2 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();

                debug!("Extracted character name: '{character_name}'");

                Ok(character_name)
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups",
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character death pattern",
            ))
        }
    }
}

impl LogParser for CharacterDeathParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.death_regex.is_match(line)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("character_death"));
        }

        let character_name = self.extract_character_name(line)?;

        Ok(ParserResult::CharacterDeath(character_name))
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
