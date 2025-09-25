use serde::{Deserialize, Serialize};

/// Represents the different types of scenes that can be detected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SceneType {
    Hideout,
    Act,
    Zone,
}
