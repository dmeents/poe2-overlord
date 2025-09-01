/// Common trait for parsing log lines into events
pub trait LogParser {
    type Event;

    /// Parse a log line and return an event if valid
    fn parse_line(&self, line: &str) -> Option<Self::Event>;
}
