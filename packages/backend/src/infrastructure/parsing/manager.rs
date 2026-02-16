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
                    Err(e) => {
                        // Parser matched but failed to parse - log the error
                        log::warn!(
                            "Parser '{}' matched but failed to parse line: {}",
                            parser.parser_name(),
                            e
                        );
                    }
                }
            }
        }

        Ok(None)
    }
}

impl Clone for LogParserManager {
    fn clone(&self) -> Self {
        // Creates a new instance rather than cloning state because:
        // 1. Parsers are stateless and cheap to reconstruct
        // 2. Box<dyn LogParser> is not Clone, so we can't clone the Vec
        Self::new()
    }
}

impl Default for LogParserManager {
    fn default() -> Self {
        Self::new()
    }
}
