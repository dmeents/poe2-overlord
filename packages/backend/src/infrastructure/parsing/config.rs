use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsersConfig {
    pub hideout_keywords: Vec<String>,
    pub act_keywords: Vec<String>,
    pub zone_keywords: Vec<String>,
}

impl Default for ParsersConfig {
    fn default() -> Self {
        Self {
            hideout_keywords: vec!["hideout".to_string()],
            zone_keywords: vec!["*".to_string()],
            act_keywords: vec![
                "act".to_string(),
                "endgame".to_string(),
                "interlude".to_string(),
            ],
        }
    }
}

impl ParsersConfig {
    pub fn hideout_keywords(&self) -> &Vec<String> {
        &self.hideout_keywords
    }

    pub fn act_keywords(&self) -> &Vec<String> {
        &self.act_keywords
    }

    pub fn zone_keywords(&self) -> &Vec<String> {
        &self.zone_keywords
    }
}
