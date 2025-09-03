use std::borrow::Cow;
use crate::parsers::errors::ParseError;

/// Extract content from a line using a pattern and bracket delimiters
pub fn extract_bracketed_content<'a>(
    line: &'a str,
    pattern: &str,
    start_delimiter: char,
    end_delimiter: char,
) -> Result<Cow<'a, str>, ParseError> {
    if let Some(start) = line.find(pattern) {
        let content_start = start + pattern.len();
        let remaining = &line[content_start..];
        
        if let Some(start_bracket) = remaining.find(start_delimiter) {
            let content_start_pos = content_start + start_bracket + 1;
            let content_remaining = &line[content_start_pos..];
            
            if let Some(end_bracket) = content_remaining.find(end_delimiter) {
                let content = &line[content_start_pos..content_start_pos + end_bracket];
                let trimmed_content = content.trim();
                
                // Validate content
                if is_valid_content(trimmed_content) {
                    return Ok(Cow::Borrowed(trimmed_content));
                } else {
                    return Err(ParseError::invalid_content(trimmed_content));
                }
            }
        }
    }
    
    Err(ParseError::content_extraction_failed(line))
}

/// Check if content is valid (not null, undefined, or empty)
pub fn is_valid_content(content: &str) -> bool {
    !content.is_empty()
        && content != "(null)"
        && content != "(undefined)"
        && content != "undefined"
        && content.to_lowercase() != "null"
        && content.to_lowercase() != "undefined"
}

/// Parse IP address and port from a string in format "IP:PORT"
pub fn parse_ip_port(server_info: &str) -> Result<(String, u16), ParseError> {
    if let Some(colon_pos) = server_info.rfind(':') {
        let ip_part = server_info[..colon_pos].trim();
        let port_part = server_info[colon_pos + 1..].trim();
        
        if ip_part.is_empty() || port_part.is_empty() {
            return Err(ParseError::server_info_parse_failed("Empty IP or port"));
        }
        
        let port = port_part.parse::<u16>()
            .map_err(|_| ParseError::server_info_parse_failed(&format!("Invalid port: {}", port_part)))?;
        
        // Basic IP validation (could be enhanced with proper IP parsing)
        if !ip_part.chars().all(|c| c.is_alphanumeric() || c == '.') {
            return Err(ParseError::server_info_parse_failed(&format!("Invalid IP format: {}", ip_part)));
        }
        
        Ok((ip_part.to_string(), port))
    } else {
        Err(ParseError::server_info_parse_failed("No colon found in server info"))
    }
}

/// Check if a line matches any of the given patterns
pub fn matches_any_pattern(line: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|pattern| line.contains(pattern))
}

/// Extract content from a line using multiple patterns
pub fn extract_content_with_patterns<'a>(
    line: &'a str,
    patterns: &[String],
    start_delimiter: char,
    end_delimiter: char,
) -> Result<Cow<'a, str>, ParseError> {
    for pattern in patterns {
        if let Ok(content) = extract_bracketed_content(line, pattern, start_delimiter, end_delimiter) {
            return Ok(content);
        }
    }
    
    Err(ParseError::content_extraction_failed(line))
}
