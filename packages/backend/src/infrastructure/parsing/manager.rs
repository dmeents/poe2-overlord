use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::infrastructure::parsing::{LogParser, ParseError, ParserFactory};

#[derive(Debug)]
pub enum ParserResult {
    SceneChange(String),
    ServerConnection(ServerConnectionEvent),
    CharacterLevel((String, u32)),
    CharacterDeath(String),
    ZoneLevel(u32),
}

pub struct LogParserManager {
    parsers: Vec<Box<dyn LogParser<Event = ParserResult> + Send + Sync>>,
}

impl LogParserManager {
    pub fn new() -> Self {
        let parsers = ParserFactory::create_all_parsers();
        Self { parsers }
    }

    /// Returns first successful parse result or None if no parser matches
    pub fn parse_line(&self, line: &str) -> Result<Option<ParserResult>, ParseError> {
        for parser in &self.parsers {
            if parser.should_parse(line) {
                match parser.parse_line(line) {
                    Ok(result) => {
                        return Ok(Some(result));
                    }
                    Err(_e) => {
                        // Parser matched but failed to parse
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn get_active_parser_names(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .map(|parser| parser.parser_name())
            .collect()
    }

    pub fn get_parser_by_name(
        &self,
        parser_name: &str,
    ) -> Option<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> {
        ParserFactory::create_parser(parser_name)
    }
}

impl Clone for LogParserManager {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for LogParserManager {
    fn default() -> Self {
        Self::new()
    }
}
