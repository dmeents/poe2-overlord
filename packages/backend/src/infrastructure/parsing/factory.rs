use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::parsers::{
    CharacterDeathParser, CharacterLevelParser, SceneChangeParser, ServerConnectionParser,
    ZoneLevelParser,
};
use crate::infrastructure::parsing::LogParser;

pub struct ParserFactory;

impl ParserFactory {
    pub fn create_parser(
        parser_name: &str,
    ) -> Option<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> {
        match parser_name {
            "scene_change" => Some(Box::new(SceneChangeParser::new())),
            "server_connection" => Some(Box::new(ServerConnectionParser::new())),
            "character_level" => Some(Box::new(CharacterLevelParser::new())),
            "character_death" => Some(Box::new(CharacterDeathParser::new())),
            "zone_level" => Some(Box::new(ZoneLevelParser::new())),
            _ => None,
        }
    }

    pub fn create_all_parsers() -> Vec<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> {
        vec![
            Box::new(SceneChangeParser::new()),
            Box::new(ServerConnectionParser::new()),
            Box::new(CharacterLevelParser::new()),
            Box::new(CharacterDeathParser::new()),
            Box::new(ZoneLevelParser::new()),
        ]
    }
}
