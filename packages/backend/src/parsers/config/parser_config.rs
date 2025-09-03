use crate::parsers::core::ParseError;
use serde::{Deserialize, Serialize};

/// Configuration for a specific parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Patterns to match for this parser
    pub patterns: Vec<String>,
    /// Whether this parser is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

/// Main configuration for all parsers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsersConfig {
    /// Scene change parser configuration
    pub scene_change: ParserConfig,

    /// Server connection parser configuration
    pub server_connection: ParserConfig,

    // Scene type detection keywords - single source of truth
    pub hideout_keywords: Vec<String>,
    pub act_keywords: Vec<String>,
    pub zone_keywords: Vec<String>,
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
                enabled: true,
            },
            server_connection: ParserConfig {
                patterns: vec!["Connecting to instance server at ".to_string()],
                enabled: true,
            },

            // Single source of truth for scene type keywords
            hideout_keywords: vec!["hideout".to_string()],

            zone_keywords: vec!["*".to_string()], // Wildcard for any content

            act_keywords: vec![
                "act".to_string(),
                "atlas".to_string(),
                "interlude".to_string(),
            ],
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

    /// Get hideout keywords
    pub fn hideout_keywords(&self) -> &Vec<String> {
        &self.hideout_keywords
    }

    /// Get act keywords
    pub fn act_keywords(&self) -> &Vec<String> {
        &self.act_keywords
    }

    /// Get zone keywords
    pub fn zone_keywords(&self) -> &Vec<String> {
        &self.zone_keywords
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
