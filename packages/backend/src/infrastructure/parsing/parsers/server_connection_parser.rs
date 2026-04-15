use crate::domain::log_analysis::models::ServerConnectionEvent;
use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::{LogParser, ParseError};
use log::debug;

#[derive(Clone)]
pub struct ServerConnectionParser {
    pattern: String,
}

impl ServerConnectionParser {
    pub fn new() -> Self {
        Self {
            pattern: "Connecting to instance server at ".to_string(),
        }
    }

    fn extract_server_info(&self, line: &str) -> Result<(String, u16), ParseError> {
        debug!("Extracting server info from line: {}", line.trim());

        if let Some(start) = line.find(&self.pattern) {
            let content_start = start + self.pattern.len();
            let server_info = line[content_start..].trim();

            if !server_info.is_empty() {
                debug!("Extracted server info: '{server_info}'");
                return self.parse_ip_port(server_info);
            }
        }

        Err(ParseError::content_extraction_failed(line))
    }

    /// Parses "IP:PORT" format and validates both components
    fn parse_ip_port(&self, server_info: &str) -> Result<(String, u16), ParseError> {
        if let Some(colon_pos) = server_info.rfind(':') {
            let ip_part = server_info[..colon_pos].trim();
            let port_part = server_info[colon_pos + 1..].trim();

            if ip_part.is_empty() || port_part.is_empty() {
                return Err(ParseError::server_info_parse_failed("Empty IP or port"));
            }

            let port = port_part.parse::<u16>().map_err(|_| {
                ParseError::server_info_parse_failed(&format!("Invalid port: {port_part}"))
            })?;

            if !ip_part.chars().all(|c| c.is_alphanumeric() || c == '.') {
                return Err(ParseError::server_info_parse_failed(&format!(
                    "Invalid IP format: {ip_part}"
                )));
            }

            Ok((ip_part.to_string(), port))
        } else {
            Err(ParseError::server_info_parse_failed(
                "No colon found in server info",
            ))
        }
    }
}

impl LogParser for ServerConnectionParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Server connection parser attempting to parse line: {}",
            line.trim()
        );

        if !self.should_parse(line) {
            debug!("Line does not match server connection patterns");
            return Err(ParseError::no_pattern_match("server_connection"));
        }

        let (ip_address, port) = self.extract_server_info(line)?;

        let event = ServerConnectionEvent {
            ip_address,
            port,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        debug!("Successfully created server connection event: {event:?}");
        Ok(ParserResult::ServerConnection(event))
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
