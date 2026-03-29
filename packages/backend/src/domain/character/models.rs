use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::domain::walkthrough::models::WalkthroughProgress;
use crate::domain::zone_tracking::TrackingSummary;

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
    #[serde(default)]
    pub walkthrough_progress: WalkthroughProgress,
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
            walkthrough_progress: WalkthroughProgress::new(),
        }
    }

    pub fn touch(&mut self) {
        self.timestamps.last_updated = Utc::now();
    }
}

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
    #[serde(rename = "Tactician")]
    Tactician,
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

}

/// Enriched location state with zone metadata for API responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnrichedLocationState {
    pub zone_name: String,

    pub act: u32,

    pub is_town: bool,

    pub location_type: LocationType,

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
    pub act: Option<u32>,
    pub area_level: Option<u32>,
    pub is_town: bool,
    pub has_waypoint: bool,
    pub zone_type: String,
    pub bosses: Vec<String>,
    pub npcs: Vec<String>,
    pub connected_zones: Vec<String>,
    pub description: Option<String>,
    pub points_of_interest: Vec<String>,
    pub image_url: Option<String>,
    pub wiki_url: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum LocationType {
    /// A playable game zone/area
    #[default]
    Zone,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationType::Zone => write!(f, "Zone"),
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
            Ascendency::GemlingLegionnaire | Ascendency::Tactician | Ascendency::Witchhunter
        ),
        CharacterClass::Witch => matches!(
            ascendency,
            Ascendency::BloodMage | Ascendency::Infernalist | Ascendency::Lich
        ),
        CharacterClass::Druid => matches!(ascendency, Ascendency::Shaman | Ascendency::Oracle),
    }
}

/// Lean character response for list views — no zones array, SQL-aggregated summary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterSummaryResponse {
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
    pub last_updated: DateTime<Utc>,
    pub current_location: Option<EnrichedLocationState>,
    pub summary: TrackingSummary,
    pub walkthrough_progress: WalkthroughProgress,
    pub is_active: bool,
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

impl FromStr for CharacterClass {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Warrior" => Ok(Self::Warrior),
            "Sorceress" => Ok(Self::Sorceress),
            "Ranger" => Ok(Self::Ranger),
            "Huntress" => Ok(Self::Huntress),
            "Monk" => Ok(Self::Monk),
            "Mercenary" => Ok(Self::Mercenary),
            "Witch" => Ok(Self::Witch),
            "Druid" => Ok(Self::Druid),
            _ => Err(format!("Unknown CharacterClass: '{}'", s)),
        }
    }
}

impl fmt::Display for Ascendency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ascendency::Titan => write!(f, "Titan"),
            Ascendency::Warbringer => write!(f, "Warbringer"),
            Ascendency::SmithOfKatava => write!(f, "Smith of Katava"),
            Ascendency::Stormweaver => write!(f, "Stormweaver"),
            Ascendency::Chronomancer => write!(f, "Chronomancer"),
            Ascendency::DiscipleOfVarashta => write!(f, "Disciple of Varashta"),
            Ascendency::Deadeye => write!(f, "Deadeye"),
            Ascendency::Pathfinder => write!(f, "Pathfinder"),
            Ascendency::Ritualist => write!(f, "Ritualist"),
            Ascendency::Amazon => write!(f, "Amazon"),
            Ascendency::Invoker => write!(f, "Invoker"),
            Ascendency::AcolyteOfChayula => write!(f, "Acolyte of Chayula"),
            Ascendency::GemlingLegionnaire => write!(f, "Gemling Legionnaire"),
            Ascendency::Tactician => write!(f, "Tactician"),
            Ascendency::Witchhunter => write!(f, "Witchhunter"),
            Ascendency::BloodMage => write!(f, "Blood Mage"),
            Ascendency::Infernalist => write!(f, "Infernalist"),
            Ascendency::Lich => write!(f, "Lich"),
            Ascendency::Shaman => write!(f, "Shaman"),
            Ascendency::Oracle => write!(f, "Oracle"),
        }
    }
}

impl FromStr for Ascendency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Titan" => Ok(Self::Titan),
            "Warbringer" => Ok(Self::Warbringer),
            "Smith of Katava" => Ok(Self::SmithOfKatava),
            "Stormweaver" => Ok(Self::Stormweaver),
            "Chronomancer" => Ok(Self::Chronomancer),
            "Disciple of Varashta" => Ok(Self::DiscipleOfVarashta),
            "Deadeye" => Ok(Self::Deadeye),
            "Pathfinder" => Ok(Self::Pathfinder),
            "Ritualist" => Ok(Self::Ritualist),
            "Amazon" => Ok(Self::Amazon),
            "Invoker" => Ok(Self::Invoker),
            "Acolyte of Chayula" => Ok(Self::AcolyteOfChayula),
            "Gemling Legionnaire" => Ok(Self::GemlingLegionnaire),
            "Tactician" => Ok(Self::Tactician),
            "Witchhunter" => Ok(Self::Witchhunter),
            "Blood Mage" => Ok(Self::BloodMage),
            "Infernalist" => Ok(Self::Infernalist),
            "Lich" => Ok(Self::Lich),
            "Shaman" => Ok(Self::Shaman),
            "Oracle" => Ok(Self::Oracle),
            _ => Err(format!("Unknown Ascendency: '{}'", s)),
        }
    }
}

impl fmt::Display for League {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            League::Standard => write!(f, "Standard"),
            League::RiseOfTheAbyssal => write!(f, "Rise of the Abyssal"),
            League::TheFateOfTheVaal => write!(f, "The Fate of the Vaal"),
        }
    }
}

impl FromStr for League {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(Self::Standard),
            "Rise of the Abyssal" => Ok(Self::RiseOfTheAbyssal),
            "The Fate of the Vaal" => Ok(Self::TheFateOfTheVaal),
            _ => Err(format!("Unknown League: '{}'", s)),
        }
    }
}
