use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Raw data scraped from the PoE2 wiki for a zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiZoneData {
    /// Name of the zone as it appears in the game
    pub zone_name: String,
    /// Area ID from the wiki (e.g., "G1_2")
    pub area_id: Option<String>,
    /// Act number
    pub act: u32,
    /// Area level
    pub area_level: Option<u32>,
    /// Whether this zone is a town
    pub is_town: bool,
    /// Whether this zone has a waypoint
    pub has_waypoint: bool,
    /// List of bosses in this zone
    pub bosses: Vec<String>,
    /// List of monsters in this zone
    pub monsters: Vec<String>,
    /// Tags associated with this zone
    pub tags: Vec<String>,
    /// Connected zones
    pub connected_zones: Vec<String>,
    /// Zone description
    pub description: Option<String>,
    /// Points of interest in the zone
    pub points_of_interest: Vec<String>,
    /// URL to the wiki page
    pub wiki_url: String,
    /// When this data was scraped
    pub scraped_at: DateTime<Utc>,
}

impl WikiZoneData {
    /// Creates a new WikiZoneData instance
    pub fn new(zone_name: String, wiki_url: String) -> Self {
        Self {
            zone_name,
            area_id: None,
            act: 0,
            area_level: None,
            is_town: false,
            has_waypoint: false,
            bosses: Vec::new(),
            monsters: Vec::new(),
            tags: Vec::new(),
            connected_zones: Vec::new(),
            description: None,
            points_of_interest: Vec::new(),
            wiki_url,
            scraped_at: Utc::now(),
        }
    }
}
