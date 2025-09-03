use crate::models::events::ServerConnectionEvent;
use crate::parsers::config::ParsersConfig;
use crate::parsers::core::{LogParser, ParseError, ParserFactory};
use crate::parsers::parsers::{SceneChangeParser, ServerConnectionParser};
use log::debug;

/// Enum to represent different parser types
#[derive(Clone)]
pub enum ParserType {
    SceneChange(SceneChangeParser),
    ServerConnection(ServerConnectionParser),
}

impl ParserType {
    /// Get the parser name
    pub fn parser_name(&self) -> &'static str {
        match self {
            ParserType::SceneChange(parser) => parser.parser_name(),
            ParserType::ServerConnection(parser) => parser.parser_name(),
        }
    }

    /// Check if this parser should parse the given line
    pub fn should_parse(&self, line: &str) -> bool {
        match self {
            ParserType::SceneChange(parser) => parser.should_parse(line),
            ParserType::ServerConnection(parser) => parser.should_parse(line),
        }
    }

    /// Parse a line and return the result
    pub fn parse_line(&self, line: &str) -> Result<ParserResult, ParseError> {
        match self {
            ParserType::SceneChange(parser) => parser
                .parse_line(line)
                .map(|event| ParserResult::SceneChange(event)),
            ParserType::ServerConnection(parser) => parser
                .parse_line(line)
                .map(|event| ParserResult::ServerConnection(event)),
        }
    }
}

/// Enum to represent different parser results
#[derive(Debug)]
pub enum ParserResult {
    SceneChange(String), // Now returns raw content instead of SceneChangeEvent
    ServerConnection(ServerConnectionEvent),
}

/// Manager for all log parsers
#[derive(Clone)]
pub struct LogParserManager {
    parsers: Vec<ParserType>,
    config: ParsersConfig,
}

impl LogParserManager {
    /// Create a new parser manager with default configuration
    pub fn new() -> Self {
        let config = ParsersConfig::default();
        Self::with_config(config)
    }

    /// Create a new parser manager with custom configuration
    pub fn with_config(config: ParsersConfig) -> Self {
        let parsers = ParserFactory::create_all_parsers(&config);
        Self { parsers, config }
    }

    /// Parse a log line using all available parsers and return the first matching event
    /// Returns the first event that any parser successfully parses from the line
    pub fn parse_line(&self, line: &str) -> Result<Option<ParserResult>, ParseError> {
        for parser in &self.parsers {
            if parser.should_parse(line) {
                debug!("Parser '{}' matched line", parser.parser_name());

                match parser.parse_line(line) {
                    Ok(result) => {
                        debug!(
                            "Parser '{}' successfully parsed event: {:?}",
                            parser.parser_name(),
                            result
                        );

                        // Add specific logging based on parser result type
                        match &result {
                            ParserResult::SceneChange(content) => {
                                debug!("Scene change content parsed successfully: {}", content);
                            }
                            ParserResult::ServerConnection(event) => {
                                debug!("Server connection event detected: {:?}", event);
                            }
                        }

                        return Ok(Some(result));
                    }
                    Err(e) => {
                        debug!(
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

    /// Get a list of all active parser names
    pub fn get_active_parser_names(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .map(|parser| parser.parser_name())
            .collect()
    }

    /// Get a specific parser by name (returns a new instance with current config)
    pub fn get_parser_by_name(&self, parser_name: &str) -> Option<ParserType> {
        ParserFactory::create_parser(parser_name, &self.config)
    }
}

impl Default for LogParserManager {
    fn default() -> Self {
        Self::new()
    }
}
