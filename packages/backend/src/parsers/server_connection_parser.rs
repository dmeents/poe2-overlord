use crate::models::events::ServerConnectionEvent;
use crate::parsers::config::ParsersConfig;
use crate::parsers::traits::LogParser;

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

    /// Check if a line should be parsed by this parser
    pub fn should_parse(&self, line: &str) -> bool {
        self.config.matches_patterns("server_connection", line)
    }

    /// Extract server connection information from the log line
    fn extract_server_info(&self, line: &str) -> Option<(String, u16)> {
        // Look for the pattern "Connecting to instance server at IP:PORT"
        for pattern in &self.config.server_connection.patterns {
            if let Some(start) = line.find(pattern) {
                let server_info_start = start + pattern.len();
                let server_info = line[server_info_start..].trim();

                // Parse IP:PORT format
                if let Some(colon_pos) = server_info.rfind(':') {
                    let ip_part = server_info[..colon_pos].trim();
                    let port_part = server_info[colon_pos + 1..].trim();

                    // Validate IP address (basic validation)
                    if !ip_part.is_empty() && !port_part.is_empty() {
                        if let Ok(port) = port_part.parse::<u16>() {
                            return Some((ip_part.to_string(), port));
                        }
                    }
                }
            }
        }
        None
    }
}

impl LogParser for ServerConnectionParser {
    type Event = ServerConnectionEvent;

    /// Parse a log line and return a server connection event if valid
    fn parse_line(&self, line: &str) -> Option<ServerConnectionEvent> {
        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            return None;
        }

        // Extract server information
        let (ip_address, port) = self.extract_server_info(line)?;

        Some(ServerConnectionEvent {
            ip_address,
            port,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}

impl Default for ServerConnectionParser {
    fn default() -> Self {
        Self::new()
    }
}
