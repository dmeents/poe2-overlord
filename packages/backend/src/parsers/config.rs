use crate::parsers::errors::ParseError;
use serde::{Deserialize, Serialize};

/// Configuration for a specific parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Patterns to match for this parser
    pub patterns: Vec<String>,
    /// Configuration for scene type detection
    pub scene_types: Option<SceneTypeConfig>,
    /// Whether this parser is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

/// Configuration for detecting different types of scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTypeConfig {
    /// Keywords that indicate a hideout
    pub hideout: Vec<String>,
    /// Keywords that indicate an act
    pub act: Vec<String>,
    /// Keywords that indicate a zone (default fallback)
    pub zone: Vec<String>,
}

/// Main configuration for all parsers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsersConfig {
    /// Scene change parser configuration
    pub scene_change: ParserConfig,
    /// Server connection parser configuration
    pub server_connection: ParserConfig,
    // Future parsers can be added here:
    // pub combat_event: ParserConfig,
    // pub trade_event: ParserConfig,
}

fn default_enabled() -> bool {
    true
}

impl Default for ParsersConfig {
    fn default() -> Self {
        Self {
            scene_change: ParserConfig {
                patterns: vec![
                    "[SCENE] Set Source [".to_string(),
                    "[SCENE] Load Source [".to_string(),
                ],
                scene_types: Some(SceneTypeConfig {
                    hideout: vec!["hideout".to_string(), "sanctuary".to_string()],
                    act: vec![
                        "act ".to_string(),
                        "atlas".to_string(),
                        "interlude".to_string(),
                    ],
                    zone: vec!["*".to_string()], // Wildcard for any content
                }),
                enabled: true,
            },
            server_connection: ParserConfig {
                patterns: vec!["Connecting to instance server at ".to_string()],
                scene_types: None, // Not needed for server connections
                enabled: true,
            },
        }
    }
}

impl ParsersConfig {
    /// Check if a line matches any patterns for a specific parser
    pub fn matches_patterns(&self, parser_name: &str, line: &str) -> Result<bool, ParseError> {
        let config = self.get_parser_config(parser_name)?;

        if !config.enabled {
            return Ok(false);
        }

        Ok(config.patterns.iter().any(|pattern| line.contains(pattern)))
    }

    /// Get scene type configuration for a parser
    pub fn get_scene_type_config(&self, parser_name: &str) -> Result<&SceneTypeConfig, ParseError> {
        let config = self.get_parser_config(parser_name)?;
        config.scene_types.as_ref().ok_or_else(|| {
            ParseError::configuration_error(&format!(
                "No scene type config for parser: {}",
                parser_name
            ))
        })
    }

    /// Get parser configuration by name
    pub fn get_parser_config(&self, parser_name: &str) -> Result<&ParserConfig, ParseError> {
        match parser_name {
            "scene_change" => Ok(&self.scene_change),
            "server_connection" => Ok(&self.server_connection),
            _ => Err(ParseError::unsupported_parser_type(parser_name)),
        }
    }

    /// Check if a parser is enabled
    pub fn is_parser_enabled(&self, parser_name: &str) -> Result<bool, ParseError> {
        let config = self.get_parser_config(parser_name)?;
        Ok(config.enabled)
    }

    /// Get all enabled parser names
    pub fn get_enabled_parsers(&self) -> Vec<&str> {
        let mut parsers = Vec::new();

        if self.scene_change.enabled {
            parsers.push("scene_change");
        }

        if self.server_connection.enabled {
            parsers.push("server_connection");
        }

        parsers
    }
}
