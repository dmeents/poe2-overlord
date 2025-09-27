use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::parsers::{
    CharacterDeathParser, CharacterLevelParser, SceneChangeParser, ServerConnectionParser, ZoneLevelParser,
};
use crate::infrastructure::parsing::{LogParser, ParsersConfig};

pub struct ParserFactory;

impl ParserFactory {
    pub fn create_parser(parser_name: &str, config: &ParsersConfig) -> Option<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> {
        match parser_name {
            "scene_change" if config.scene_change.enabled => Some(Box::new(
                SceneChangeParser::with_config(config.clone()),
            )),
            "server_connection" if config.server_connection.enabled => Some(Box::new(
                ServerConnectionParser::with_config(config.clone()),
            )),
            "character_level" if config.character_level.enabled => Some(Box::new(
                CharacterLevelParser::with_config(config.clone()),
            )),
            "character_death" if config.character_death.enabled => Some(Box::new(
                CharacterDeathParser::with_config(config.clone()),
            )),
            "zone_level" if config.zone_level.enabled => Some(Box::new(
                ZoneLevelParser::with_config(config.clone()),
            )),
            _ => None,
        }
    }

    pub fn create_all_parsers(config: &ParsersConfig) -> Vec<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> {
        let mut parsers: Vec<Box<dyn LogParser<Event = ParserResult> + Send + Sync>> = Vec::new();

        if config.scene_change.enabled {
            parsers.push(Box::new(SceneChangeParser::with_config(
                config.clone(),
            )));
        }

        if config.server_connection.enabled {
            parsers.push(Box::new(ServerConnectionParser::with_config(
                config.clone(),
            )));
        }

        if config.character_level.enabled {
            parsers.push(Box::new(CharacterLevelParser::with_config(
                config.clone(),
            )));
        }

        if config.character_death.enabled {
            parsers.push(Box::new(CharacterDeathParser::with_config(
                config.clone(),
            )));
        }

        if config.zone_level.enabled {
            parsers.push(Box::new(ZoneLevelParser::with_config(
                config.clone(),
            )));
        }

        parsers
    }
}