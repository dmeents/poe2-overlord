use crate::models::events::ServerConnectionEvent;
use crate::parsers::config::ParsersConfig;
use crate::infrastructure::parsing::{LogParser, ParseError};
use crate::infrastructure::network::parse_ip_port;
use log::debug;

/// Server connection parser for detecting server connection patterns
#[derive(Clone)]
pub struct ServerConnectionParser {
    config: ParsersConfig,
}

impl ServerConnectionParser {
    /// Create a new server connection parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
        }
    }

    /// Create a new server connection parser with custom configuration
    pub fn with_config(config: ParsersConfig) -> Self {
        Self { config }
    }

    /// Extract server connection information from the log line
    fn extract_server_info(&self, line: &str) -> Result<(String, u16), ParseError> {
        debug!("Extracting server info from line: {}", line.trim());

        // Find the pattern and extract everything after it until end of line
        for pattern in &self.config.server_connection.patterns {
            if let Some(start) = line.find(pattern) {
                let content_start = start + pattern.len();
                let server_info = line[content_start..].trim();

                if !server_info.is_empty() {
                    debug!("Extracted server info: '{}'", server_info);
                    return parse_ip_port(server_info);
                }
            }
        }

        Err(ParseError::content_extraction_failed(line))
    }
}

impl LogParser for ServerConnectionParser {
    type Event = ServerConnectionEvent;

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("server_connection", line)
            .unwrap_or(false)
    }

    /// Parse a log line and return a server connection event if valid
    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Server connection parser attempting to parse line: {}",
            line.trim()
        );

        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            debug!("Line does not match server connection patterns");
            return Err(ParseError::no_pattern_match("server_connection"));
        }

        // Extract server information
        let (ip_address, port) = self.extract_server_info(line)?;

        let event = ServerConnectionEvent {
            ip_address,
            port,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        debug!("Successfully created server connection event: {:?}", event);
        Ok(event)
    }

    fn parser_name(&self) -> &'static str {
        "server_connection"
    }
}

impl Default for ServerConnectionParser {
    fn default() -> Self {
        Self::new()
    }
}
