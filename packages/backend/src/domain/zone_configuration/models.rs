use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Classification of a zone's role, derived from the wiki info-card subheading.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum ZoneType {
    Campaign,
    Town,
    Map,
    Hideout,
    #[default]
    Unknown,
}

impl fmt::Display for ZoneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZoneType::Campaign => write!(f, "Campaign"),
            ZoneType::Town => write!(f, "Town"),
            ZoneType::Map => write!(f, "Map"),
            ZoneType::Hideout => write!(f, "Hideout"),
            ZoneType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for ZoneType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Campaign" => Ok(ZoneType::Campaign),
            "Town" => Ok(ZoneType::Town),
            "Map" => Ok(ZoneType::Map),
            "Hideout" => Ok(ZoneType::Hideout),
            _ => Ok(ZoneType::Unknown),
        }
    }
}

/// Main zone configuration containing all zone metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConfiguration {
    /// Map of zone_name to their metadata
    pub zones: HashMap<String, ZoneMetadata>,
}

/// Comprehensive zone metadata from wiki and game data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneMetadata {
    /// Name of the zone as it appears in the game
    pub zone_name: String,
    /// Act number
    pub act: u32,
    /// Area level
    pub area_level: Option<u32>,
    /// Whether this zone is a town
    pub is_town: bool,
    /// Whether this zone has a waypoint
    pub has_waypoint: bool,
    /// Classification of the zone type (Campaign, Town, Map, Hideout, Unknown)
    pub zone_type: ZoneType,
    /// List of bosses in this zone
    pub bosses: Vec<String>,
    /// List of NPCs in this zone
    #[serde(default)]
    pub npcs: Vec<String>,
    /// Connected zones
    pub connected_zones: Vec<String>,
    /// Zone description
    pub description: Option<String>,
    /// Points of interest in the zone
    pub points_of_interest: Vec<String>,
    /// URL to the zone screenshot/image
    #[serde(default)]
    pub image_url: Option<String>,
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
        self.zones.insert(zone.zone_name.clone(), zone);
    }

    /// Gets zone metadata by zone name
    pub fn get_zone(&self, zone_name: &str) -> Option<&ZoneMetadata> {
        self.zones.get(zone_name)
    }

    /// Gets zone metadata by zone name
    pub fn get_zone_by_name(&self, zone_name: &str) -> Option<&ZoneMetadata> {
        self.get_zone(zone_name)
    }

    /// Gets all zones for a specific act
    pub fn get_act_zones(&self, act: u32) -> Vec<&ZoneMetadata> {
        self.zones.values().filter(|zone| zone.act == act).collect()
    }

    /// Gets the act for a specific zone by zone name
    pub fn get_act_for_zone(&self, zone_name: &str) -> Option<u32> {
        self.zones.get(zone_name).map(|zone| zone.act)
    }

    /// Checks if a zone is a town by zone name
    pub fn is_town_zone(&self, zone_name: &str) -> bool {
        self.zones
            .get(zone_name)
            .map(|zone| zone.is_town)
            .unwrap_or(false)
    }
}

impl ZoneMetadata {
    pub fn new(zone_name: String) -> Self {
        let now = Utc::now();
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
            first_discovered: now,
            last_updated: now,
            wiki_url: None,
        }
    }

    /// Creates a placeholder zone metadata for unknown zones
    pub fn placeholder(zone_name: String) -> Self {
        Self::new(zone_name)
    }

    /// Updates zone metadata from wiki data
    pub fn update_from_wiki_data(
        &mut self,
        wiki_data: &crate::domain::wiki_scraping::models::WikiZoneData,
    ) {
        self.act = wiki_data.act;
        self.area_level = wiki_data.area_level;
        self.is_town = wiki_data.is_town;
        self.has_waypoint = wiki_data.has_waypoint;
        self.zone_type = wiki_data.zone_type.clone();
        self.bosses = wiki_data.bosses.clone();
        self.npcs = wiki_data.npcs.clone();
        self.connected_zones = wiki_data.connected_zones.clone();
        self.description = wiki_data.description.clone();
        self.points_of_interest = wiki_data.points_of_interest.clone();
        self.image_url = wiki_data.image_url.clone();
        self.wiki_url = Some(wiki_data.wiki_url.clone());
        self.last_updated = Utc::now();
    }

    /// Checks if this zone needs refresh based on a custom duration
    ///
    /// # Arguments
    ///
    /// * `refresh_interval_seconds` - The refresh interval in seconds
    pub fn needs_refresh(&self, refresh_interval_seconds: i64) -> bool {
        let threshold = Utc::now() - chrono::Duration::seconds(refresh_interval_seconds);
        self.last_updated < threshold
    }
}

impl Default for ZoneConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
