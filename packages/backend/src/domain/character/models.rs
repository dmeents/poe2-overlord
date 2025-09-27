use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Consolidated character data containing all character information in a single structure.
///
/// This struct combines character metadata (name, class, level, etc.) with tracking data
/// (location, time tracking, zone statistics) into one unified data model. Each character
/// will have its own `character_data_{character_id}.json` file containing this structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterData {
    /// Unique identifier for the character, generated using UUID v4
    pub id: String,
    /// Display name of the character, must be unique across all characters
    pub name: String,
    /// The base character class (e.g., Warrior, Sorceress, etc.)
    pub class: CharacterClass,
    /// The specialized ascendency class chosen for this character
    pub ascendency: Ascendency,
    /// The league/game mode this character belongs to
    pub league: League,
    /// Whether this character is in hardcore mode (permadeath)
    pub hardcore: bool,
    /// Whether this character is in solo self-found mode (no trading)
    pub solo_self_found: bool,
    /// Current level of the character (defaults to 1)
    #[serde(default = "default_level")]
    pub level: u32,
    /// Timestamp when the character was first created
    pub created_at: DateTime<Utc>,
    /// Timestamp of the last time this character was played
    pub last_played: Option<DateTime<Utc>>,

    // Tracking data (consolidated from character_tracking domain)
    /// Current location state
    pub current_location: Option<LocationState>,
    /// Summary statistics
    pub summary: TrackingSummary,
    /// Zone statistics (aggregated data per location)
    pub zones: Vec<ZoneStats>,
    /// Last updated timestamp for tracking data
    pub last_updated: DateTime<Utc>,
}

/// Simple index structure for managing character IDs and active character.
///
/// This struct represents the `characters.json` file which only contains
/// the list of character IDs and which character is currently active.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CharactersIndex {
    /// Collection of all character IDs in the system
    pub character_ids: Vec<String>,
    /// ID of the currently active character (if any)
    pub active_character_id: Option<String>,
}

/// Parameters for updating an existing character.
///
/// This struct contains all the mutable fields that can be updated
/// when modifying a character's properties. It excludes immutable
/// fields like ID, creation timestamp, and progression data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterUpdateParams {
    /// New display name for the character
    pub name: String,
    /// New base character class
    pub class: CharacterClass,
    /// New ascendency class (must be valid for the chosen class)
    pub ascendency: Ascendency,
    /// New league/game mode
    pub league: League,
    /// New hardcore mode setting
    pub hardcore: bool,
    /// New solo self-found mode setting
    pub solo_self_found: bool,
    /// New level
    pub level: u32,
}

impl Default for CharacterData {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            class: CharacterClass::Warrior,
            ascendency: Ascendency::Titan,
            league: League::Standard,
            hardcore: false,
            solo_self_found: false,
            level: 1,
            created_at: Utc::now(),
            last_played: None,
            current_location: None,
            summary: TrackingSummary::new(String::new()),
            zones: Vec::new(),
            last_updated: Utc::now(),
        }
    }
}

impl CharacterData {
    /// Creates new empty character data for a character
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
            name,
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
            level: 1,
            created_at: now,
            last_played: None,
            current_location: None,
            summary: TrackingSummary::new(id),
            zones: Vec::new(),
            last_updated: now,
        }
    }

    /// Updates the last_updated timestamp to current time
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Recalculates summary statistics from zone data
    pub fn update_summary(&mut self) {
        self.summary = TrackingSummary::from_zones(&self.id, &self.zones);
        self.touch();
    }

    /// Finds a zone by location ID
    pub fn find_zone(&self, location_id: &str) -> Option<&ZoneStats> {
        self.zones
            .iter()
            .find(|zone| zone.location_id == location_id)
    }

    /// Finds a zone by location ID (mutable)
    pub fn find_zone_mut(&mut self, location_id: &str) -> Option<&mut ZoneStats> {
        self.zones
            .iter_mut()
            .find(|zone| zone.location_id == location_id)
    }

    /// Adds or updates a zone with the given statistics
    pub fn upsert_zone(&mut self, zone: ZoneStats) {
        if let Some(existing_zone) = self.find_zone_mut(&zone.location_id) {
            *existing_zone = zone;
        } else {
            self.zones.push(zone);
        }
        self.update_summary();
    }

    /// Gets the current active zone (only one can be active at a time)
    pub fn get_active_zone(&self) -> Option<&ZoneStats> {
        self.zones.iter().find(|zone| zone.is_active)
    }

    /// Gets zones sorted by total time spent (descending)
    pub fn get_zones_by_time(&self) -> Vec<&ZoneStats> {
        let mut zones: Vec<&ZoneStats> = self.zones.iter().collect();
        zones.sort_by(|a, b| b.duration.cmp(&a.duration));
        zones
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

/// Base character classes available in Path of Exile 2.
///
/// Each class represents a different archetype with unique starting attributes
/// and access to specific ascendency specializations. The serde rename attributes
/// ensure proper serialization to match the game's naming conventions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    /// Melee-focused class with high strength and durability
    #[serde(rename = "Warrior")]
    Warrior,
    /// Magic-focused class with high intelligence and spellcasting abilities
    #[serde(rename = "Sorceress")]
    Sorceress,
    /// Ranged combat class with high dexterity and bow skills
    #[serde(rename = "Ranger")]
    Ranger,
    /// Hybrid class combining ranged and melee combat with nature magic
    #[serde(rename = "Huntress")]
    Huntress,
    /// Martial arts class with high dexterity and unarmed combat
    #[serde(rename = "Monk")]
    Monk,
    /// Versatile class with balanced attributes and weapon mastery
    #[serde(rename = "Mercenary")]
    Mercenary,
    /// Dark magic class with necromancy and curse abilities
    #[serde(rename = "Witch")]
    Witch,
}

/// Ascendency specializations available for each character class.
///
/// Ascendencies provide specialized passive skill trees and unique abilities
/// that further define a character's playstyle. Each base class has access
/// to 2-3 specific ascendencies that complement their core archetype.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Ascendency {
    // Warrior ascendencies
    /// Tank-focused ascendency with defensive bonuses and shield skills
    #[serde(rename = "Titan")]
    Titan,
    /// Berserker ascendency with damage bonuses and rage mechanics
    #[serde(rename = "Warbringer")]
    Warbringer,
    /// Crafting-focused ascendency with item enhancement abilities
    #[serde(rename = "Smith of Katava")]
    SmithOfKatava,

    // Sorceress ascendencies
    /// Lightning and storm magic specialist
    #[serde(rename = "Stormweaver")]
    Stormweaver,
    /// Time manipulation and temporal magic specialist
    #[serde(rename = "Chronomancer")]
    Chronomancer,

    // Ranger ascendencies
    /// Precision and critical hit specialist
    #[serde(rename = "Deadeye")]
    Deadeye,
    /// Nature magic and elemental arrow specialist
    #[serde(rename = "Pathfinder")]
    Pathfinder,

    // Huntress ascendencies
    /// Ritual magic and totem specialist
    #[serde(rename = "Ritualist")]
    Ritualist,
    /// Physical prowess and spear specialist
    #[serde(rename = "Amazon")]
    Amazon,

    // Monk ascendencies
    /// Elemental invocation and spell casting specialist
    #[serde(rename = "Invoker")]
    Invoker,
    /// Dark magic and chaos specialist
    #[serde(rename = "Acolyte of Chayula")]
    AcolyteOfChayula,

    // Mercenary ascendencies
    /// Gem-based abilities and magical enhancement specialist
    #[serde(rename = "Gemling Legionnaire")]
    GemlingLegionnaire,
    /// Strategic combat and battlefield control specialist
    #[serde(rename = "Tactitian")]
    Tactitian,
    /// Anti-magic and spell disruption specialist
    #[serde(rename = "Witchhunter")]
    Witchhunter,

    // Witch ascendencies
    /// Blood magic and life force manipulation specialist
    #[serde(rename = "Blood Mage")]
    BloodMage,
    /// Fire and destruction magic specialist
    #[serde(rename = "Infernalist")]
    Infernalist,
    /// Undead mastery and necromancy specialist
    #[serde(rename = "Lich")]
    Lich,
}

/// Game leagues/modes available in Path of Exile 2.
///
/// Leagues represent different game modes with varying rules, mechanics,
/// and economies. Each league provides a fresh start for characters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum League {
    /// The permanent league with standard game rules and persistent economy
    #[serde(rename = "Standard")]
    Standard,
    /// A temporary league with unique mechanics and modifiers
    #[serde(rename = "Third Edict")]
    ThirdEdict,
}

// Re-export tracking-related types (these will be moved here in a future step)
// For now, we'll define them here to avoid circular dependencies

/// Current location state for a character
/// Simplified to focus on current location without complex session management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationState {
    /// Current scene/zone name (hideout, zone, etc.)
    pub scene: Option<String>,
    /// Current act name (Act 1, Act 2, etc.)
    pub act: Option<String>,
    /// Whether the current location is a town
    #[serde(default = "default_is_town")]
    pub is_town: bool,
    /// Type of location (Zone, Act, Hideout)
    pub location_type: LocationType,
    /// Timestamp of the last location update
    pub last_updated: DateTime<Utc>,
}

/// Default value for is_town field during deserialization
fn default_is_town() -> bool {
    false
}

impl LocationState {
    /// Creates a new location state with current timestamp
    pub fn new() -> Self {
        Self {
            scene: None,
            act: None,
            is_town: false,
            location_type: LocationType::Zone,
            last_updated: Utc::now(),
        }
    }

    /// Creates a new location state for a specific location
    pub fn new_for_location(
        scene: Option<String>,
        act: Option<String>,
        is_town: bool,
        location_type: LocationType,
    ) -> Self {
        Self {
            scene,
            act,
            is_town,
            location_type,
            last_updated: Utc::now(),
        }
    }

    /// Updates the current scene and returns true if it actually changed
    /// Returns false if the scene is the same as the current one
    pub fn update_scene(&mut self, new_scene: String, location_type: LocationType) -> bool {
        if self.scene.as_ref() != Some(&new_scene) || self.location_type != location_type {
            self.scene = Some(new_scene);
            self.location_type = location_type;
            self.last_updated = Utc::now();
            true
        } else {
            false
        }
    }

    /// Updates the current act and returns true if it actually changed
    /// Returns false if the act is the same as the current one
    pub fn update_act(&mut self, new_act: String) -> bool {
        if self.act.as_ref() != Some(&new_act) {
            self.act = Some(new_act);
            self.last_updated = Utc::now();
            true
        } else {
            false
        }
    }

    /// Resets the location state to initial values
    /// Clears scene and act, updates timestamps
    pub fn reset(&mut self) {
        self.scene = None;
        self.act = None;
        self.location_type = LocationType::Zone;
        self.last_updated = Utc::now();
    }

    /// Gets a reference to the current scene name
    pub fn get_current_scene(&self) -> Option<&String> {
        self.scene.as_ref()
    }

    /// Gets a reference to the current act name
    pub fn get_current_act(&self) -> Option<&String> {
        self.act.as_ref()
    }
}

/// Summary statistics for character tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TrackingSummary {
    /// Character this summary belongs to
    pub character_id: String,
    /// Total play time across all zones (seconds)
    pub total_play_time: u64,
    /// Total time spent in hideouts (seconds)
    pub total_hideout_time: u64,
    /// Total number of unique zones visited
    pub total_zones_visited: usize,
    /// Total number of deaths across all zones
    pub total_deaths: u32,
}

impl TrackingSummary {
    /// Creates new empty summary for a character
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            total_play_time: 0,
            total_hideout_time: 0,
            total_zones_visited: 0,
            total_deaths: 0,
        }
    }

    /// Creates summary from zone data
    pub fn from_zones(character_id: &str, zones: &[ZoneStats]) -> Self {
        let total_play_time = zones.iter().map(|zone| zone.duration).sum();
        let total_hideout_time = zones
            .iter()
            .filter(|zone| zone.location_type == LocationType::Hideout)
            .map(|zone| zone.duration)
            .sum();
        let total_deaths = zones.iter().map(|zone| zone.deaths).sum();

        Self {
            character_id: character_id.to_string(),
            total_play_time,
            total_hideout_time,
            total_zones_visited: zones.len(),
            total_deaths,
        }
    }
}

/// Aggregated statistics for a specific zone/location
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ZoneStats {
    /// Unique identifier for the location
    pub location_id: String,
    /// Human-readable name of the location
    pub location_name: String,
    /// Type of location (Zone, Act, Hideout)
    pub location_type: LocationType,
    /// Act this zone belongs to (1, 2, 3, 4, interlude, endgame)
    pub act: Option<String>,
    /// Whether this zone is a town
    #[serde(default = "default_is_town")]
    pub is_town: bool,
    /// Total time spent in this zone (seconds)
    pub duration: u64,
    /// Number of deaths in this zone
    pub deaths: u32,
    /// Number of visits to this zone
    pub visits: u32,
    /// Timestamp of first visit
    pub first_visited: DateTime<Utc>,
    /// Timestamp of most recent visit
    pub last_visited: DateTime<Utc>,
    /// Whether this zone is currently active
    pub is_active: bool,
    /// Timestamp when character entered this zone (for time tracking)
    pub entry_timestamp: Option<DateTime<Utc>>,
    /// Level of the zone (extracted from game logs)
    pub zone_level: Option<u32>,
}

impl ZoneStats {
    /// Creates new zone stats for a location
    pub fn new(
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
        is_town: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            location_id,
            location_name,
            location_type,
            act,
            is_town,
            duration: 0,
            deaths: 0,
            visits: 0,
            first_visited: now,
            last_visited: now,
            is_active: true,
            entry_timestamp: Some(now),
            zone_level: None,
        }
    }

    /// Adds time to the zone duration
    pub fn add_time(&mut self, seconds: u64) {
        self.duration += seconds;
        self.last_visited = Utc::now();
    }

    /// Records a death in this zone
    pub fn record_death(&mut self) {
        self.deaths += 1;
        self.last_visited = Utc::now();
    }

    /// Records a visit to this zone
    pub fn record_visit(&mut self) {
        self.visits += 1;
        self.last_visited = Utc::now();
    }

    /// Activates the zone (character enters)
    pub fn activate(&mut self) {
        self.is_active = true;
        self.record_visit();
    }

    /// Deactivates the zone (character leaves)
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.last_visited = Utc::now();
    }

    /// Starts the timer for this zone (character enters)
    pub fn start_timer(&mut self) {
        self.entry_timestamp = Some(Utc::now());
    }

    /// Stops the timer and adds the elapsed time to duration
    /// Returns the time spent in seconds
    pub fn stop_timer_and_add_time(&mut self) -> u64 {
        if let Some(entry_time) = self.entry_timestamp {
            let time_spent = Utc::now()
                .signed_duration_since(entry_time)
                .num_seconds()
                .max(0) as u64;
            self.add_time(time_spent);
            self.entry_timestamp = None;
            time_spent
        } else {
            0
        }
    }

    /// Gets the current time spent in this zone (if active)
    pub fn get_current_time_spent(&self) -> u64 {
        if let Some(entry_time) = self.entry_timestamp {
            Utc::now()
                .signed_duration_since(entry_time)
                .num_seconds()
                .max(0) as u64
        } else {
            0
        }
    }

    /// Updates the zone level
    pub fn update_zone_level(&mut self, level: u32) {
        self.zone_level = Some(level);
    }
}

/// Types of locations that can be tracked in the game
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

/// Validates whether a given ascendency is compatible with a character class.
///
/// This function enforces the game's business rules by ensuring that only
/// valid ascendency-class combinations are allowed. Each class has a specific
/// set of ascendencies they can choose from.
///
/// # Arguments
/// * `ascendency` - The ascendency to validate
/// * `class` - The character class to check compatibility against
///
/// # Returns
/// * `true` if the ascendency is valid for the given class
/// * `false` if the combination is not allowed
pub fn is_valid_ascendency_for_class(ascendency: &Ascendency, class: &CharacterClass) -> bool {
    match class {
        CharacterClass::Warrior => matches!(
            ascendency,
            Ascendency::Titan | Ascendency::Warbringer | Ascendency::SmithOfKatava
        ),
        CharacterClass::Sorceress => matches!(
            ascendency,
            Ascendency::Stormweaver | Ascendency::Chronomancer
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
    }
}

/// Returns all available ascendencies for a given character class.
///
/// This utility function provides a convenient way to get all valid
/// ascendency options when creating or updating a character of a specific class.
/// Used primarily by the UI layer to populate dropdown menus and validate selections.
///
/// # Arguments
/// * `class` - The character class to get ascendencies for
///
/// # Returns
/// A vector containing all ascendencies available to the specified class
pub fn get_ascendencies_for_class(class: &CharacterClass) -> Vec<Ascendency> {
    match class {
        CharacterClass::Warrior => vec![
            Ascendency::Titan,
            Ascendency::Warbringer,
            Ascendency::SmithOfKatava,
        ],
        CharacterClass::Sorceress => vec![Ascendency::Stormweaver, Ascendency::Chronomancer],
        CharacterClass::Ranger => vec![Ascendency::Deadeye, Ascendency::Pathfinder],
        CharacterClass::Huntress => vec![Ascendency::Ritualist, Ascendency::Amazon],
        CharacterClass::Monk => vec![Ascendency::Invoker, Ascendency::AcolyteOfChayula],
        CharacterClass::Mercenary => vec![
            Ascendency::GemlingLegionnaire,
            Ascendency::Tactitian,
            Ascendency::Witchhunter,
        ],
        CharacterClass::Witch => vec![
            Ascendency::BloodMage,
            Ascendency::Infernalist,
            Ascendency::Lich,
        ],
    }
}

/// Returns all available character classes in the game.
///
/// This utility function provides a complete list of all character classes
/// that can be selected when creating a new character. Used by the UI layer
/// to populate class selection dropdowns.
///
/// # Returns
/// A vector containing all available character classes
pub fn get_all_character_classes() -> Vec<CharacterClass> {
    vec![
        CharacterClass::Warrior,
        CharacterClass::Sorceress,
        CharacterClass::Ranger,
        CharacterClass::Huntress,
        CharacterClass::Monk,
        CharacterClass::Mercenary,
        CharacterClass::Witch,
    ]
}

/// Returns all available leagues in the game.
///
/// This utility function provides a complete list of all leagues
/// that can be selected when creating a new character. Used by the UI layer
/// to populate league selection dropdowns.
///
/// # Returns
/// A vector containing all available leagues
pub fn get_all_leagues() -> Vec<League> {
    vec![League::Standard, League::ThirdEdict]
}

/// Default level for new characters.
///
/// All characters start at level 1 when created.
fn default_level() -> u32 {
    1
}

/// Implementation of Display trait for CharacterClass.
///
/// Provides human-readable string representation of character classes
/// for display purposes in the UI and logging.
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
        }
    }
}

impl Default for CharacterClass {
    fn default() -> Self {
        CharacterClass::Warrior
    }
}

impl Default for Ascendency {
    fn default() -> Self {
        Ascendency::Titan
    }
}

impl Default for League {
    fn default() -> Self {
        League::Standard
    }
}

