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
