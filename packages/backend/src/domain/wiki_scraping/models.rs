use crate::domain::zone_configuration::models::ZoneType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiZoneData {
    pub zone_name: String,
    pub act: u32,
    pub area_level: Option<u32>,
    pub is_town: bool,
    pub has_waypoint: bool,
    pub zone_type: ZoneType,
    pub bosses: Vec<String>,
    #[serde(default)]
    pub npcs: Vec<String>,
    pub connected_zones: Vec<String>,
    pub description: Option<String>,
    pub points_of_interest: Vec<String>,
    pub image_url: Option<String>,
    pub wiki_url: String,
    pub scraped_at: DateTime<Utc>,
}

impl WikiZoneData {
    pub fn new(zone_name: String, wiki_url: String) -> Self {
        Self {
            zone_name,
            act: 0,
            area_level: None,
            is_town: false,
            has_waypoint: false,
            zone_type: ZoneType::Unknown,
            bosses: Vec::new(),
            npcs: Vec::new(),
            connected_zones: Vec::new(),
            description: None,
            points_of_interest: Vec::new(),
            image_url: None,
            wiki_url,
            scraped_at: Utc::now(),
        }
    }
}
