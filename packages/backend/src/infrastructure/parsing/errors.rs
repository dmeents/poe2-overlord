use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("No patterns matched for parser '{parser_name}'")]
    NoPatternMatch { parser_name: String },

    #[error("Failed to extract content from line: {line}")]
    ContentExtractionFailed { line: String },

    #[error("Invalid content: {content}")]
    InvalidContent { content: String },

    #[error("Failed to parse server information: {details}")]
    ServerInfoParseFailed { details: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

impl ParseError {
    pub fn no_pattern_match(parser_name: &str) -> Self {
        Self::NoPatternMatch {
            parser_name: parser_name.to_string(),
        }
    }

    pub fn content_extraction_failed(line: &str) -> Self {
        Self::ContentExtractionFailed {
            line: line.to_string(),
        }
    }

    pub fn invalid_content(content: &str) -> Self {
        Self::InvalidContent {
            content: content.to_string(),
        }
    }

    pub fn server_info_parse_failed(details: &str) -> Self {
        Self::ServerInfoParseFailed {
            details: details.to_string(),
        }
    }

    pub fn configuration_error(message: &str) -> Self {
        Self::ConfigurationError {
            message: message.to_string(),
        }
    }
}
