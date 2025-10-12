use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main zone configuration containing all zone metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConfiguration {
    /// Map of area_id to their metadata
    pub zones: HashMap<String, ZoneMetadata>,
}

/// Comprehensive zone metadata from wiki and game data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneMetadata {
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
    /// When this zone was first discovered by the player
    pub first_discovered: DateTime<Utc>,
    /// When this zone data was last updated from wiki
    pub last_updated: DateTime<Utc>,
    /// URL to the wiki page
    pub wiki_url: Option<String>,
}

impl ZoneConfiguration {
    /// Creates a new empty zone configuration
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
        }
    }

    /// Adds or updates a zone in the configuration
    pub fn add_zone(&mut self, zone: ZoneMetadata) {
        if let Some(area_id) = &zone.area_id {
            self.zones.insert(area_id.clone(), zone);
        }
    }

    /// Gets zone metadata by area_id
    pub fn get_zone(&self, area_id: &str) -> Option<&ZoneMetadata> {
        self.zones.get(area_id)
    }

    /// Gets zone metadata by area_id (mutable)
    pub fn get_zone_mut(&mut self, area_id: &str) -> Option<&mut ZoneMetadata> {
        self.zones.get_mut(area_id)
    }

    /// Checks if a zone exists by area_id
    pub fn has_zone(&self, area_id: &str) -> bool {
        self.zones.contains_key(area_id)
    }

    /// Gets zone metadata by zone name (searches through all zones)
    pub fn get_zone_by_name(&self, zone_name: &str) -> Option<&ZoneMetadata> {
        self.zones.values().find(|zone| zone.zone_name == zone_name)
    }

    /// Gets all zones for a specific act
    pub fn get_act_zones(&self, act: u32) -> Vec<&ZoneMetadata> {
        self.zones
            .values()
            .filter(|zone| zone.act == act)
            .collect()
    }

    /// Gets the act for a specific zone by area_id
    pub fn get_act_for_zone(&self, area_id: &str) -> Option<u32> {
        self.zones.get(area_id).map(|zone| zone.act)
    }

    /// Gets the act for a specific zone by zone name
    pub fn get_act_for_zone_by_name(&self, zone_name: &str) -> Option<u32> {
        self.get_zone_by_name(zone_name).map(|zone| zone.act)
    }

    /// Checks if a zone is a town by area_id
    pub fn is_town_zone(&self, area_id: &str) -> bool {
        self.zones
            .get(area_id)
            .map(|zone| zone.is_town)
            .unwrap_or(false)
    }

    /// Checks if a zone is a town by zone name
    pub fn is_town_zone_by_name(&self, zone_name: &str) -> bool {
        self.get_zone_by_name(zone_name)
            .map(|zone| zone.is_town)
            .unwrap_or(false)
    }

    /// Gets all area_ids
    pub fn get_all_area_ids(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }

    /// Gets all zone names
    pub fn get_all_zone_names(&self) -> Vec<String> {
        self.zones.values().map(|zone| zone.zone_name.clone()).collect()
    }

    /// Gets zones that need refresh (older than 1 week)
    pub fn get_zones_needing_refresh(&self) -> Vec<String> {
        let week_ago = Utc::now() - chrono::Duration::weeks(1);
        self.zones
            .iter()
            .filter(|(_, zone)| zone.last_updated < week_ago)
            .map(|(area_id, _)| area_id.clone())
            .collect()
    }
}

impl ZoneMetadata {
    /// Creates a new zone metadata with minimal data
    pub fn new(zone_name: String) -> Self {
        let now = Utc::now();
        Self {
            zone_name: zone_name.clone(),
            area_id: Some(Self::generate_area_id(&zone_name)),
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
            first_discovered: now,
            last_updated: now,
            wiki_url: None,
        }
    }

    /// Generates a consistent area_id from zone name
    /// Format: lowercase with underscores instead of spaces
    /// Examples: "Clearfell" -> "clearfell", "Felled Hideout" -> "felled_hideout"
    pub fn generate_area_id(zone_name: &str) -> String {
        zone_name
            .to_lowercase()
            .replace(' ', "_")
            .replace('-', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .trim_matches('_')
            .to_string()
    }

    /// Creates a placeholder zone metadata for unknown zones
    pub fn placeholder(zone_name: String) -> Self {
        Self::new(zone_name)
    }

    /// Updates zone metadata from wiki data
    pub fn update_from_wiki_data(&mut self, wiki_data: &crate::domain::wiki_scraping::models::WikiZoneData) {
        self.area_id = wiki_data.area_id.clone();
        self.act = wiki_data.act;
        self.area_level = wiki_data.area_level;
        self.is_town = wiki_data.is_town;
        self.has_waypoint = wiki_data.has_waypoint;
        self.bosses = wiki_data.bosses.clone();
        self.monsters = wiki_data.monsters.clone();
        self.tags = wiki_data.tags.clone();
        self.connected_zones = wiki_data.connected_zones.clone();
        self.description = wiki_data.description.clone();
        self.points_of_interest = wiki_data.points_of_interest.clone();
        self.wiki_url = Some(wiki_data.wiki_url.clone());
        self.last_updated = Utc::now();
    }

    /// Checks if this zone needs refresh (older than 1 week)
    pub fn needs_refresh(&self) -> bool {
        let week_ago = Utc::now() - chrono::Duration::weeks(1);
        self.last_updated < week_ago
    }
}

impl Default for ZoneConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
