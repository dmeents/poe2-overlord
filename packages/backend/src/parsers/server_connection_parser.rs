use crate::models::events::ServerConnectionEvent;
use crate::parsers::config::ParsersConfig;
use crate::parsers::traits::LogParser;
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

    /// Check if a line should be parsed by this parser
    pub fn should_parse(&self, line: &str) -> bool {
        let matches = self.config.matches_patterns("server_connection", line);
        if matches {
            debug!("Server connection parser matched line: {}", line.trim());
        }
        matches
    }

    /// Extract server connection information from the log line
    fn extract_server_info(&self, line: &str) -> Option<(String, u16)> {
        debug!("Extracting server info from line: {}", line.trim());

        // Look for the pattern "Connecting to instance server at IP:PORT"
        for pattern in &self.config.server_connection.patterns {
            debug!("Checking pattern: '{}'", pattern);
            if let Some(start) = line.find(pattern) {
                debug!("Pattern found at position: {}", start);
                let server_info_start = start + pattern.len();
                let server_info = line[server_info_start..].trim();
                debug!("Server info part: '{}'", server_info);

                // Parse IP:PORT format
                if let Some(colon_pos) = server_info.rfind(':') {
                    let ip_part = server_info[..colon_pos].trim();
                    let port_part = server_info[colon_pos + 1..].trim();
                    debug!("IP part: '{}', Port part: '{}'", ip_part, port_part);

                    // Validate IP address (basic validation)
                    if !ip_part.is_empty() && !port_part.is_empty() {
                        if let Ok(port) = port_part.parse::<u16>() {
                            debug!("Successfully parsed server info: {}:{}", ip_part, port);
                            return Some((ip_part.to_string(), port));
                        } else {
                            debug!("Failed to parse port: '{}'", port_part);
                        }
                    } else {
                        debug!("Empty IP or port part");
                    }
                } else {
                    debug!("No colon found in server info: '{}'", server_info);
                }
            }
        }
        debug!("No server info extracted from line");
        None
    }
}

impl LogParser for ServerConnectionParser {
    type Event = ServerConnectionEvent;

    /// Parse a log line and return a server connection event if valid
    fn parse_line(&self, line: &str) -> Option<ServerConnectionEvent> {
        debug!(
            "Server connection parser attempting to parse line: {}",
            line.trim()
        );

        // Check if this line should be parsed by this parser
        if !self.should_parse(line) {
            debug!("Line does not match server connection patterns");
            return None;
        }

        // Extract server information
        if let Some((ip_address, port)) = self.extract_server_info(line) {
            let event = ServerConnectionEvent {
                ip_address,
                port,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            debug!("Successfully created server connection event: {:?}", event);
            Some(event)
        } else {
            debug!("Failed to extract server info from line");
            None
        }
    }
}

impl Default for ServerConnectionParser {
    fn default() -> Self {
        Self::new()
    }
}
