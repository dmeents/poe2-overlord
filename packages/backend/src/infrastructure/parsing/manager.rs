use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::domain::character::models::CharacterClass;
use crate::infrastructure::parsing::ParsersConfig;
use crate::infrastructure::parsing::{LogParser, ParseError, ParserFactory};
use crate::infrastructure::parsing::patterns::{CharacterDeathParser, CharacterLevelParser, SceneChangeParser, ServerConnectionParser};
use log::debug;

#[derive(Clone)]
pub enum ParserType {
    SceneChange(SceneChangeParser),
    ServerConnection(ServerConnectionParser),
    CharacterLevel(CharacterLevelParser),
    CharacterDeath(CharacterDeathParser),
}

impl ParserType {
    pub fn parser_name(&self) -> &'static str {
        match self {
            ParserType::SceneChange(parser) => parser.parser_name(),
            ParserType::ServerConnection(parser) => parser.parser_name(),
            ParserType::CharacterLevel(parser) => parser.parser_name(),
            ParserType::CharacterDeath(parser) => parser.parser_name(),
        }
    }

    pub fn should_parse(&self, line: &str) -> bool {
        match self {
            ParserType::SceneChange(parser) => parser.should_parse(line),
            ParserType::ServerConnection(parser) => parser.should_parse(line),
            ParserType::CharacterLevel(parser) => parser.should_parse(line),
            ParserType::CharacterDeath(parser) => parser.should_parse(line),
        }
    }

    pub fn parse_line(&self, line: &str) -> Result<ParserResult, ParseError> {
        match self {
            ParserType::SceneChange(parser) => parser
                .parse_line(line)
                .map(ParserResult::SceneChange),
            ParserType::ServerConnection(parser) => parser
                .parse_line(line)
                .map(ParserResult::ServerConnection),
            ParserType::CharacterLevel(parser) => parser
                .parse_line(line)
                .map(ParserResult::CharacterLevel),
            ParserType::CharacterDeath(parser) => parser
                .parse_line(line)
                .map(ParserResult::CharacterDeath),
        }
    }
}

#[derive(Debug)]
pub enum ParserResult {
    SceneChange(String), // Now returns raw content instead of SceneChangeEvent
    ServerConnection(ServerConnectionEvent),
    CharacterLevel((String, CharacterClass, u32)), // (character_name, character_class, level)
    CharacterDeath(String), // character_name
}

#[derive(Clone)]
pub struct LogParserManager {
    parsers: Vec<ParserType>,
    config: ParsersConfig,
}

impl LogParserManager {
    pub fn new() -> Self {
        let config = ParsersConfig::default();
        Self::with_config(config)
    }

    pub fn with_config(config: ParsersConfig) -> Self {
        let parsers = ParserFactory::create_all_parsers(&config);
        Self { parsers, config }
    }

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

                        match &result {
                            ParserResult::SceneChange(content) => {
                                debug!("Scene change content parsed successfully: {}", content);
                            }
                            ParserResult::ServerConnection(event) => {
                                debug!("Server connection event detected: {:?}", event);
                            }
                            ParserResult::CharacterLevel((name, class, level)) => {
                                debug!("Character level-up detected: {} ({}) -> level {}", name, class, level);
                            }
                            ParserResult::CharacterDeath(name) => {
                                debug!("Character death detected: {} has been slain", name);
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

    pub fn get_active_parser_names(&self) -> Vec<&str> {
        self.parsers
            .iter()
            .map(|parser| parser.parser_name())
            .collect()
    }

    pub fn get_parser_by_name(&self, parser_name: &str) -> Option<ParserType> {
        ParserFactory::create_parser(parser_name, &self.config)
    }
}

impl Default for LogParserManager {
    fn default() -> Self {
        Self::new()
    }
}
