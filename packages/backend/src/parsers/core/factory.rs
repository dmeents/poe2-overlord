use crate::parsers::config::ParsersConfig;
use crate::parsers::core::ParserType;
use crate::parsers::parsers::{SceneChangeParser, ServerConnectionParser};

/// Factory for creating parser instances
pub struct ParserFactory;

impl ParserFactory {
    /// Create a parser by name with the given configuration
    pub fn create_parser(parser_name: &str, config: &ParsersConfig) -> Option<ParserType> {
        match parser_name {
            "scene_change" if config.scene_change.enabled => {
                Some(ParserType::SceneChange(SceneChangeParser::with_config(config.clone())))
            }
            "server_connection" if config.server_connection.enabled => {
                Some(ParserType::ServerConnection(ServerConnectionParser::with_config(config.clone())))
            }
            _ => None,
        }
    }

    /// Create all enabled parsers from the configuration
    pub fn create_all_parsers(config: &ParsersConfig) -> Vec<ParserType> {
        let mut parsers = Vec::new();

        if config.scene_change.enabled {
            parsers.push(ParserType::SceneChange(SceneChangeParser::with_config(config.clone())));
        }

        if config.server_connection.enabled {
            parsers.push(ParserType::ServerConnection(ServerConnectionParser::with_config(config.clone())));
        }

        parsers
    }
}
