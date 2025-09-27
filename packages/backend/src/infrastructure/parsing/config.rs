use crate::infrastructure::parsing::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub patterns: Vec<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsersConfig {
    pub scene_change: ParserConfig,

    pub server_connection: ParserConfig,

    pub character_level: ParserConfig,

    pub character_death: ParserConfig,

    pub zone_level: ParserConfig,

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
            character_level: ParserConfig {
                patterns: vec!["is now level".to_string()],
                enabled: true,
            },
            character_death: ParserConfig {
                patterns: vec!["has been slain".to_string()],
                enabled: true,
            },
            zone_level: ParserConfig {
                patterns: vec!["Generating level".to_string()],
                enabled: true,
            },

            hideout_keywords: vec!["hideout".to_string()],

            zone_keywords: vec!["*".to_string()], // Wildcard for any content

            act_keywords: vec![
                "act".to_string(),
                "endgame".to_string(),
                "interlude".to_string(),
            ],
        }
    }
}

impl ParsersConfig {
    pub fn matches_patterns(&self, parser_name: &str, line: &str) -> Result<bool, ParseError> {
        let config = self.get_parser_config(parser_name)?;

        if !config.enabled {
            return Ok(false);
        }

        Ok(config.patterns.iter().any(|pattern| line.contains(pattern)))
    }

    pub fn hideout_keywords(&self) -> &Vec<String> {
        &self.hideout_keywords
    }

    pub fn act_keywords(&self) -> &Vec<String> {
        &self.act_keywords
    }

    pub fn zone_keywords(&self) -> &Vec<String> {
        &self.zone_keywords
    }

    pub fn get_parser_config(&self, parser_name: &str) -> Result<&ParserConfig, ParseError> {
        match parser_name {
            "scene_change" => Ok(&self.scene_change),
            "server_connection" => Ok(&self.server_connection),
            "character_level" => Ok(&self.character_level),
            "character_death" => Ok(&self.character_death),
            "zone_level" => Ok(&self.zone_level),
            _ => Err(ParseError::unsupported_parser_type(parser_name)),
        }
    }

    pub fn is_parser_enabled(&self, parser_name: &str) -> Result<bool, ParseError> {
        let config = self.get_parser_config(parser_name)?;
        Ok(config.enabled)
    }

    pub fn get_enabled_parsers(&self) -> Vec<&str> {
        let mut parsers = Vec::new();

        if self.scene_change.enabled {
            parsers.push("scene_change");
        }

        if self.server_connection.enabled {
            parsers.push("server_connection");
        }

        if self.character_level.enabled {
            parsers.push("character_level");
        }

        if self.character_death.enabled {
            parsers.push("character_death");
        }

        if self.zone_level.enabled {
            parsers.push("zone_level");
        }

        parsers
    }
}
