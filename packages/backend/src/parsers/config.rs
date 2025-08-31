use serde::{Deserialize, Serialize};

/// Configuration for a specific parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Patterns to match for this parser
    pub patterns: Vec<String>,
    /// Configuration for scene type detection
    pub scene_types: Option<SceneTypeConfig>,
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
    // Future parsers can be added here:
    // pub combat_event: ParserConfig,
    // pub trade_event: ParserConfig,
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
            },
        }
    }
}

impl ParsersConfig {
    /// Check if a line matches any patterns for a specific parser
    pub fn matches_patterns(&self, parser_name: &str, line: &str) -> bool {
        match parser_name {
            "scene_change" => self
                .scene_change
                .patterns
                .iter()
                .any(|pattern| line.contains(pattern)),
            _ => false,
        }
    }

    /// Get scene type configuration for a parser
    pub fn get_scene_type_config(&self, parser_name: &str) -> Option<&SceneTypeConfig> {
        match parser_name {
            "scene_change" => self.scene_change.scene_types.as_ref(),
            _ => None,
        }
    }
}
