use crate::infrastructure::parsing::ParseError;

/// Parses a server connection string into IP address and port
///
/// Expects format "IP:PORT" and validates both components.
/// Returns a tuple of (ip_address, port) or a parse error if the format is invalid.
///
pub fn parse_ip_port(server_info: &str) -> Result<(String, u16), ParseError> {
    if let Some(colon_pos) = server_info.rfind(':') {
        let ip_part = server_info[..colon_pos].trim();
        let port_part = server_info[colon_pos + 1..].trim();

        // Validate that both parts are non-empty
        if ip_part.is_empty() || port_part.is_empty() {
            return Err(ParseError::server_info_parse_failed("Empty IP or port"));
        }

        // Parse port as u16
        let port = port_part.parse::<u16>().map_err(|_| {
            ParseError::server_info_parse_failed(&format!("Invalid port: {}", port_part))
        })?;

        // Basic IP format validation (allows alphanumeric chars and dots)
        if !ip_part.chars().all(|c| c.is_alphanumeric() || c == '.') {
            return Err(ParseError::server_info_parse_failed(&format!(
                "Invalid IP format: {}",
                ip_part
            )));
        }

        Ok((ip_part.to_string(), port))
    } else {
        Err(ParseError::server_info_parse_failed(
            "No colon found in server info",
        ))
    }
}
