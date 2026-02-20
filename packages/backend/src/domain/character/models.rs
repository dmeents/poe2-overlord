use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::domain::walkthrough::models::WalkthroughProgress;
use crate::domain::zone_tracking::{TrackingSummary, ZoneStats};

/// Character profile information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterProfile {
    pub name: String,
    pub class: CharacterClass,
    pub ascendency: Ascendency,
    pub league: League,
    pub hardcore: bool,
    pub solo_self_found: bool,
    #[serde(default = "default_level")]
    pub level: u32,
}

/// Character timestamps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterTimestamps {
    pub created_at: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
}

/// Consolidated character data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterData {
    pub id: String,
    #[serde(flatten)]
    pub profile: CharacterProfile,
    #[serde(flatten)]
    pub timestamps: CharacterTimestamps,
    pub current_location: Option<LocationState>,
    pub summary: TrackingSummary,
    pub zones: Vec<ZoneStats>,
    #[serde(default)]
    pub walkthrough_progress: WalkthroughProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CharactersIndex {
    pub character_ids: Vec<String>,
    pub active_character_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterUpdateParams {
    pub name: String,
    pub class: CharacterClass,
    pub ascendency: Ascendency,
    pub league: League,
    pub hardcore: bool,
    pub solo_self_found: bool,
    pub level: u32,
}

impl Default for CharacterData {
    fn default() -> Self {
        let now = Utc::now();
        let id = uuid::Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            profile: CharacterProfile {
                name: String::new(),
                class: CharacterClass::Warrior,
                ascendency: Ascendency::Titan,
                league: League::Standard,
                hardcore: false,
                solo_self_found: false,
                level: 1,
            },
            timestamps: CharacterTimestamps {
                created_at: now,
                last_played: None,
                last_updated: now,
            },
            current_location: None,
            summary: TrackingSummary::new(id),
            zones: Vec::new(),
            walkthrough_progress: WalkthroughProgress::new(),
        }
    }
}

impl CharacterData {
    pub fn new(
        id: String,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.clone(),
            profile: CharacterProfile {
                name,
                class,
                ascendency,
                league,
                hardcore,
                solo_self_found,
                level: 1,
            },
            timestamps: CharacterTimestamps {
                created_at: now,
                last_played: None,
                last_updated: now,
            },
            current_location: None,
            summary: TrackingSummary::new(id),
            zones: Vec::new(),
            walkthrough_progress: WalkthroughProgress::new(),
        }
    }

    pub fn touch(&mut self) {
        self.timestamps.last_updated = Utc::now();
    }

    pub fn update_walkthrough_progress(&mut self, progress: WalkthroughProgress) {
        self.walkthrough_progress = progress;
        self.touch();
    }

    pub fn get_walkthrough_progress(&self) -> &WalkthroughProgress {
        &self.walkthrough_progress
    }
}

impl CharactersIndex {
    /// Creates a new empty characters index
    pub fn new() -> Self {
        Self {
            character_ids: Vec::new(),
            active_character_id: None,
        }
    }

    /// Adds a character ID to the index
    pub fn add_character(&mut self, character_id: String) {
        if !self.character_ids.contains(&character_id) {
            self.character_ids.push(character_id);
        }
    }

    /// Removes a character ID from the index
    pub fn remove_character(&mut self, character_id: &str) {
        self.character_ids.retain(|id| id != character_id);
        if self.active_character_id.as_ref() == Some(&character_id.to_string()) {
            self.active_character_id = None;
        }
    }

    /// Sets the active character ID
    pub fn set_active_character(&mut self, character_id: Option<String>) {
        self.active_character_id = character_id;
    }

    /// Checks if a character ID exists in the index
    pub fn has_character(&self, character_id: &str) -> bool {
        self.character_ids.contains(&character_id.to_string())
    }
}

// Re-export all the enums and types from the old character domain
// These will be moved here in a future step, but for now we'll import them

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    #[serde(rename = "Warrior")]
    #[default]
    Warrior,
    #[serde(rename = "Sorceress")]
    Sorceress,
    #[serde(rename = "Ranger")]
    Ranger,
    #[serde(rename = "Huntress")]
    Huntress,
    #[serde(rename = "Monk")]
    Monk,
    #[serde(rename = "Mercenary")]
    Mercenary,
    #[serde(rename = "Witch")]
    Witch,
    #[serde(rename = "Druid")]
    Druid,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Ascendency {
    #[serde(rename = "Titan")]
    #[default]
    Titan,
    #[serde(rename = "Warbringer")]
    Warbringer,
    #[serde(rename = "Smith of Katava")]
    SmithOfKatava,
    #[serde(rename = "Stormweaver")]
    Stormweaver,
    #[serde(rename = "Chronomancer")]
    Chronomancer,
    #[serde(rename = "Disciple of Varashta")]
    DiscipleOfVarashta,
    #[serde(rename = "Deadeye")]
    Deadeye,
    #[serde(rename = "Pathfinder")]
    Pathfinder,
    #[serde(rename = "Ritualist")]
    Ritualist,
    #[serde(rename = "Amazon")]
    Amazon,
    #[serde(rename = "Invoker")]
    Invoker,
    #[serde(rename = "Acolyte of Chayula")]
    AcolyteOfChayula,
    #[serde(rename = "Gemling Legionnaire")]
    GemlingLegionnaire,
    #[serde(rename = "Tactitian")]
    Tactitian,
    #[serde(rename = "Witchhunter")]
    Witchhunter,
    #[serde(rename = "Blood Mage")]
    BloodMage,
    #[serde(rename = "Infernalist")]
    Infernalist,
    #[serde(rename = "Lich")]
    Lich,
    #[serde(rename = "Shaman")]
    Shaman,
    #[serde(rename = "Oracle")]
    Oracle,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum League {
    #[serde(rename = "Standard")]
    #[default]
    Standard,
    #[serde(rename = "Rise of the Abyssal")]
    RiseOfTheAbyssal,
    #[serde(rename = "The Fate of the Vaal")]
    TheFateOfTheVaal,
}

// Re-export tracking-related types (these will be moved here in a future step)
// For now, we'll define them here to avoid circular dependencies

/// Current location state for a character.
/// Stores only a reference to the current zone - full zone metadata comes from zone configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationState {
    /// Current zone name
    pub zone_name: String,

    pub last_updated: DateTime<Utc>,
}

impl LocationState {
    pub fn new(zone_name: String) -> Self {
        Self {
            zone_name,
            last_updated: Utc::now(),
        }
    }

    /// Updates the current zone and returns true if it actually changed
    /// Returns false if the zone is the same as the current one
    pub fn update_zone(&mut self, new_zone_name: String) -> bool {
        if self.zone_name != new_zone_name {
            self.zone_name = new_zone_name;
            self.last_updated = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn get_zone_name(&self) -> &String {
        &self.zone_name
    }
}

/// Enriched location state with zone metadata for API responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnrichedLocationState {
    pub zone_name: String,

    pub act: u32,

    pub is_town: bool,

    pub location_type: LocationType,

    pub area_id: Option<String>,

    pub area_level: Option<u32>,

    pub has_waypoint: bool,

    pub last_updated: DateTime<Utc>,
}

impl EnrichedLocationState {
    pub fn from_location_and_metadata(
        location: &LocationState,
        metadata: &crate::domain::zone_configuration::models::ZoneMetadata,
    ) -> Self {
        Self {
            zone_name: location.zone_name.clone(),
            act: metadata.act,
            is_town: metadata.is_town,
            location_type: LocationType::Zone, // All locations are zones (including towns)
            area_id: metadata.area_id.clone(),
            area_level: metadata.area_level,
            has_waypoint: metadata.has_waypoint,
            last_updated: location.last_updated,
        }
    }

    pub fn from_location_minimal(location: &LocationState) -> Self {
        Self {
            zone_name: location.zone_name.clone(),
            act: 0, // Unknown act
            is_town: false,
            location_type: LocationType::Zone,
            area_id: None,
            area_level: None,
            has_waypoint: false,
            last_updated: location.last_updated,
        }
    }
}

/// Character data response with enriched zone information for API responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterDataResponse {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
    pub ascendency: Ascendency,
    pub league: League,
    pub hardcore: bool,
    pub solo_self_found: bool,
    pub level: u32,
    pub created_at: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>,
    pub current_location: Option<EnrichedLocationState>,
    pub summary: TrackingSummary,
    pub zones: Vec<EnrichedZoneStats>,
    pub walkthrough_progress: WalkthroughProgress,
    pub last_updated: DateTime<Utc>,
}

impl From<CharacterData> for CharacterDataResponse {
    fn from(character: CharacterData) -> Self {
        Self {
            id: character.id,
            name: character.profile.name,
            class: character.profile.class,
            ascendency: character.profile.ascendency,
            league: character.profile.league,
            hardcore: character.profile.hardcore,
            solo_self_found: character.profile.solo_self_found,
            level: character.profile.level,
            created_at: character.timestamps.created_at,
            last_played: character.timestamps.last_played,
            current_location: None,
            summary: character.summary,
            zones: character
                .zones
                .into_iter()
                .map(|zone| EnrichedZoneStats {
                    zone_name: zone.zone_name,
                    duration: zone.duration,
                    deaths: zone.deaths,
                    visits: zone.visits,
                    first_visited: zone.first_visited,
                    last_visited: zone.last_visited,
                    is_active: zone.is_active,
                    entry_timestamp: zone.entry_timestamp,
                    area_id: None,
                    act: None,
                    area_level: None,
                    is_town: false,
                    has_waypoint: false,
                    bosses: Vec::new(),
                    monsters: Vec::new(),
                    npcs: Vec::new(),
                    connected_zones: Vec::new(),
                    description: None,
                    points_of_interest: Vec::new(),
                    image_url: None,
                    wiki_url: None,
                    last_updated: None,
                })
                .collect(),
            walkthrough_progress: character.walkthrough_progress,
            last_updated: character.timestamps.last_updated,
        }
    }
}

/// Response DTO combining zone stats with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnrichedZoneStats {
    pub zone_name: String,
    pub duration: u64,
    pub deaths: u32,
    pub visits: u32,
    pub first_visited: DateTime<Utc>,
    pub last_visited: DateTime<Utc>,
    pub is_active: bool,
    pub entry_timestamp: Option<DateTime<Utc>>,
    pub area_id: Option<String>,
    pub act: Option<u32>,
    pub area_level: Option<u32>,
    pub is_town: bool,
    pub has_waypoint: bool,
    pub bosses: Vec<String>,
    pub monsters: Vec<String>,
    pub npcs: Vec<String>,
    pub connected_zones: Vec<String>,
    pub description: Option<String>,
    pub points_of_interest: Vec<String>,
    pub image_url: Option<String>,
    pub wiki_url: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

impl EnrichedZoneStats {
    pub fn from_stats_and_metadata(
        stats: &ZoneStats,
        metadata: &crate::domain::zone_configuration::models::ZoneMetadata,
    ) -> Self {
        Self {
            zone_name: stats.zone_name.clone(),
            duration: stats.duration,
            deaths: stats.deaths,
            visits: stats.visits,
            first_visited: stats.first_visited,
            last_visited: stats.last_visited,
            is_active: stats.is_active,
            entry_timestamp: stats.entry_timestamp,
            area_id: metadata.area_id.clone(),
            act: Some(metadata.act),
            area_level: metadata.area_level,
            is_town: metadata.is_town,
            has_waypoint: metadata.has_waypoint,
            bosses: metadata.bosses.clone(),
            monsters: metadata.monsters.clone(),
            npcs: metadata.npcs.clone(),
            connected_zones: metadata.connected_zones.clone(),
            description: metadata.description.clone(),
            points_of_interest: metadata.points_of_interest.clone(),
            image_url: metadata.image_url.clone(),
            wiki_url: metadata.wiki_url.clone(),
            last_updated: Some(metadata.last_updated),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum LocationType {
    /// A playable game zone/area
    #[default]
    Zone,
    /// A major story act
    Act,
    /// Player's personal hideout
    Hideout,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationType::Zone => write!(f, "Zone"),
            LocationType::Act => write!(f, "Act"),
            LocationType::Hideout => write!(f, "Hideout"),
        }
    }
}

/// Validates whether an ascendency is compatible with a character class
pub fn is_valid_ascendency_for_class(ascendency: &Ascendency, class: &CharacterClass) -> bool {
    match class {
        CharacterClass::Warrior => matches!(
            ascendency,
            Ascendency::Titan | Ascendency::Warbringer | Ascendency::SmithOfKatava
        ),
        CharacterClass::Sorceress => matches!(
            ascendency,
            Ascendency::Stormweaver | Ascendency::Chronomancer | Ascendency::DiscipleOfVarashta
        ),
        CharacterClass::Ranger => {
            matches!(ascendency, Ascendency::Deadeye | Ascendency::Pathfinder)
        }
        CharacterClass::Huntress => {
            matches!(ascendency, Ascendency::Ritualist | Ascendency::Amazon)
        }
        CharacterClass::Monk => matches!(
            ascendency,
            Ascendency::Invoker | Ascendency::AcolyteOfChayula
        ),
        CharacterClass::Mercenary => matches!(
            ascendency,
            Ascendency::GemlingLegionnaire | Ascendency::Tactitian | Ascendency::Witchhunter
        ),
        CharacterClass::Witch => matches!(
            ascendency,
            Ascendency::BloodMage | Ascendency::Infernalist | Ascendency::Lich
        ),
        CharacterClass::Druid => matches!(ascendency, Ascendency::Shaman | Ascendency::Oracle),
    }
}

fn default_level() -> u32 {
    1
}

impl fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacterClass::Warrior => write!(f, "Warrior"),
            CharacterClass::Sorceress => write!(f, "Sorceress"),
            CharacterClass::Ranger => write!(f, "Ranger"),
            CharacterClass::Huntress => write!(f, "Huntress"),
            CharacterClass::Monk => write!(f, "Monk"),
            CharacterClass::Mercenary => write!(f, "Mercenary"),
            CharacterClass::Witch => write!(f, "Witch"),
            CharacterClass::Druid => write!(f, "Druid"),
        }
    }
}
