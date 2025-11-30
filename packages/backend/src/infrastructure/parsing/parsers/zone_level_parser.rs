use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::{LogParser, ParseError};
use regex::Regex;

#[derive(Clone)]
pub struct ZoneLevelParser {
    regex: Regex,
}

impl ZoneLevelParser {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"Generating level (\d+) area").unwrap(),
        }
    }

    fn extract_zone_level(&self, line: &str) -> Result<u32, ParseError> {
        if let Some(captures) = self.regex.captures(line) {
            if let Some(level_str) = captures.get(1) {
                let level = level_str.as_str().parse::<u32>().map_err(|_| {
                    ParseError::invalid_content("Failed to parse zone level as number")
                })?;
                return Ok(level);
            }
        }
        Err(ParseError::invalid_content("No zone level found in line"))
    }
}

impl LogParser for ZoneLevelParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        if !self.should_parse(line) {
            return Err(ParseError::no_pattern_match("zone_level"));
        }

        let level = self.extract_zone_level(line)?;
        Ok(ParserResult::ZoneLevel(level))
    }

    fn parser_name(&self) -> &'static str {
        "zone_level"
    }
}

impl Default for ZoneLevelParser {
    fn default() -> Self {
        Self::new()
    }
}
