use crate::parsers::core::ParseError;

/// Parse IP address and port from a string in format "IP:PORT"
pub fn parse_ip_port(server_info: &str) -> Result<(String, u16), ParseError> {
    if let Some(colon_pos) = server_info.rfind(':') {
        let ip_part = server_info[..colon_pos].trim();
        let port_part = server_info[colon_pos + 1..].trim();

        if ip_part.is_empty() || port_part.is_empty() {
            return Err(ParseError::server_info_parse_failed("Empty IP or port"));
        }

        let port = port_part.parse::<u16>().map_err(|_| {
            ParseError::server_info_parse_failed(&format!("Invalid port: {}", port_part))
        })?;

        // Basic IP validation (could be enhanced with proper IP parsing)
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
