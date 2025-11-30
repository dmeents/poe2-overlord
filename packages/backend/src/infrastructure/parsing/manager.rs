use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::infrastructure::parsing::{LogParser, ParseError, ParserFactory};

/// Results produced by log parsers
///
/// Contains the parsed data from different types of log events.
/// Each variant represents a specific type of game event that was successfully parsed.
#[derive(Debug)]
pub enum ParserResult {
    SceneChange(String), // Raw scene change content
    ServerConnection(ServerConnectionEvent),
    CharacterLevel((String, u32)), // (character_name, level)
    CharacterDeath(String),        // character_name
    ZoneLevel(u32),                // zone level
}

/// Manages a collection of log parsers for processing game log events
///
/// Coordinates multiple parsers to handle different types of log events.
/// Provides a unified interface for parsing log lines and extracting game events.
pub struct LogParserManager {
    /// Collection of active parsers as trait objects
    parsers: Vec<Box<dyn LogParser<Event = ParserResult> + Send + Sync>>,
}

impl LogParserManager {
    pub fn new() -> Self {
        let parsers = ParserFactory::create_all_parsers();
        Self { parsers }
    }

    /// Attempts to parse a log line using all available parsers
    ///
    /// Iterates through all parsers to find one that can handle the log line.
    /// Returns the first successful parse result or None if no parser matches.
    pub fn parse_line(&self, line: &str) -> Result<Option<ParserResult>, ParseError> {
        for parser in &self.parsers {
            if parser.should_parse(line) {
                match parser.parse_line(line) {
                    Ok(result) => {
                        // Log specific event details for debugging
                        match &result {
                            ParserResult::SceneChange(_content) => {
                                // Scene change content parsed successfully
                            }
                            ParserResult::ServerConnection(_event) => {
                                // Server connection event detected
                            }
                            ParserResult::CharacterLevel((_name, _level)) => {
                                // Character level-up detected
                            }
                            ParserResult::CharacterDeath(_name) => {
                                // Character death detected
                            }
                            ParserResult::ZoneLevel(_level) => {
                                // Zone level detected
                            }
                        }

                        return Ok(Some(result));
                    }
                    Err(_e) => {
                        // Parser matched but failed to parse line
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
