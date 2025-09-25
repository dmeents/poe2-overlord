use crate::infrastructure::parsing::errors::ParseError;

pub trait LogParser {
    type Event;

    fn should_parse(&self, line: &str) -> bool;

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError>;

    fn parser_name(&self) -> &'static str;
}
