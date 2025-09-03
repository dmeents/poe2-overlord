use serde::{Deserialize, Serialize};

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

impl Default for SceneTypeConfig {
    fn default() -> Self {
        Self {
            hideout: vec![
                "hideout".to_string(),
                "personal hideout".to_string(),
                "ph".to_string(),
            ],
            act: vec![
                "act".to_string(),
                "chapter".to_string(),
            ],
            zone: vec![
                "zone".to_string(),
                "area".to_string(),
                "map".to_string(),
            ],
        }
    }
}
