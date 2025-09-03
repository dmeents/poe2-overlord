use crate::parsers::core::ParseError;

/// Common trait for parsing log lines into events
pub trait LogParser {
    type Event;

    /// Check if a line should be parsed by this parser
    fn should_parse(&self, line: &str) -> bool;

    /// Parse a log line and return an event if valid
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError>;

    /// Get the name of this parser
    fn parser_name(&self) -> &'static str;
}
