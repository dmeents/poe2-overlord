use crate::models::events::{SceneChangeEvent, ServerConnectionEvent};
use crate::parsers::{
    scene_change_parser::SceneChangeParser, server_connection_parser::ServerConnectionParser,
    traits::LogParser, errors::ParseError,
};
use crate::services::player_location_manager::PlayerLocationManager;
use log::debug;
use std::sync::Arc;

/// Manager for all log parsers
#[derive(Clone)]
pub struct LogParserManager {
    scene_parser: SceneChangeParser,
    server_parser: ServerConnectionParser,
    state_manager: Arc<PlayerLocationManager>,
}

impl LogParserManager {
    /// Create a new parser manager with default configuration
    pub fn new(state_manager: Arc<PlayerLocationManager>) -> Self {
        Self {
            scene_parser: SceneChangeParser::new(),
            server_parser: ServerConnectionParser::new(),
            state_manager,
        }
    }

    /// Create a new parser manager with custom configuration
    pub fn with_config(
        config: crate::parsers::config::ParsersConfig,
        state_manager: Arc<PlayerLocationManager>,
    ) -> Self {
        Self {
            scene_parser: SceneChangeParser::with_config(config.clone()),
            server_parser: ServerConnectionParser::with_config(config),
            state_manager,
        }
    }

    /// Parse a log line using all available parsers and return events for actual changes
    pub async fn parse_line(&self, line: &str) -> Result<Option<SceneChangeEvent>, ParseError> {
        debug!("Parsing log line: {}", line.trim());

        // Try scene change parser
        if self.scene_parser.should_parse(line) {
            debug!("Scene change parser matched line");

            match self.scene_parser.parse_line(line) {
                Ok(event) => {
                    // Check if this represents an actual change using the state manager
                    let should_return_event = match &event {
                        SceneChangeEvent::Hideout(hideout_event) => {
                            self.state_manager
                                .update_scene(&hideout_event.hideout_name)
                                .await
                        }
                        SceneChangeEvent::Zone(zone_event) => {
                            self.state_manager.update_scene(&zone_event.zone_name).await
                        }
                        SceneChangeEvent::Act(act_event) => {
                            self.state_manager.update_act(&act_event.act_name).await
                        }
                    };

                    if should_return_event {
                        debug!("Scene change parser successfully parsed event: {:?}", event);
                        return Ok(Some(event));
                    } else {
                        debug!("Scene change parser matched but no actual change detected");
                    }
                }
                Err(e) => {
                    debug!("Scene change parser matched but failed to parse line: {}", e);
                }
            }
        }

        // Future parsers can be added here:
        // if self.config.combat_event.enabled { ... }
        // if self.config.trade_event.enabled { ... }

        debug!("No parsers matched the line or no actual changes detected");
        Ok(None)
    }

    /// Parse a log line for server connection events
    pub fn parse_server_connection(&self, line: &str) -> Result<Option<ServerConnectionEvent>, ParseError> {
        debug!("Parsing log line for server connection: {}", line.trim());

        if self.server_parser.should_parse(line) {
            debug!("Server connection parser matched line");

            match self.server_parser.parse_line(line) {
                Ok(event) => {
                    debug!(
                        "Server connection parser successfully parsed event: {:?}",
                        event
                    );
                    return Ok(Some(event));
                }
                Err(e) => {
                    debug!("Server connection parser matched but failed to parse line: {}", e);
                }
            }
        }

        debug!("Server connection parser did not match the line");
        Ok(None)
    }

    /// Get a list of all active parser names
    pub fn get_active_parsers(&self) -> Vec<&str> {
        let mut parsers = Vec::new();

        // Add scene change parser if enabled
        if self.scene_parser.should_parse("") { // Check if parser is enabled
            parsers.push("scene_change");
        }

        // Add server connection parser if enabled
        if self.server_parser.should_parse("") { // Check if parser is enabled
            parsers.push("server_connection");
        }

        // Future parsers can be added here:
        // if self.config.combat_event.enabled { parsers.push("combat_event"); }
        // if self.config.trade_event.enabled { parsers.push("trade_event"); }

        parsers
    }
    
    /// Get a specific parser by name
    pub fn get_parser<T: LogParser>(&self, _parser_name: &str) -> Option<&T> {
        // This is a placeholder for future extensibility
        // In a more sophisticated implementation, we could store parsers in a HashMap
        None
    }
}

impl Default for LogParserManager {
    fn default() -> Self {
        // Note: This requires a StateManager, so we can't really have a default
        // This is just to satisfy the trait requirement
        unimplemented!(
            "LogParserManager requires a StateManager and cannot have a default implementation"
        )
    }
}
