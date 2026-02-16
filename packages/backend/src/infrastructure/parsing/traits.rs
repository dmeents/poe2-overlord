use crate::infrastructure::parsing::errors::ParseError;

/// Trait for parsing log file lines into structured events
///
/// # Contract
/// - `should_parse()` determines if the parser can handle a given line
/// - `parse_line()` is called ONLY when `should_parse()` returns true
/// - Implementations of `parse_line()` can trust that `should_parse()` was already called
///   and should not call it again internally
pub trait LogParser {
    type Event;

    /// Returns true if this parser can handle the given line
    fn should_parse(&self, line: &str) -> bool;

    /// Parses a line into an event
    ///
    /// # Contract
    /// This method is only called when `should_parse()` returns true.
    /// Implementations should not call `should_parse()` internally.
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError>;

    /// Returns the name of this parser for logging and debugging
    fn parser_name(&self) -> &'static str;
}
