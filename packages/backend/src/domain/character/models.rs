use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Core character entity representing a Path of Exile 2 character.
/// 
/// This struct encapsulates all the essential data for a character including
/// identity, class information, game mode settings, and progression tracking.
/// The character serves as the primary entity in the character management domain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
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
    /// Timestamp when the character was first created
    pub created_at: DateTime<Utc>,
    /// Timestamp of the last time this character was played
    pub last_played: Option<DateTime<Utc>>,
    /// Whether this character is currently the active/selected character
    pub is_active: bool,
    /// Current level of the character (defaults to 1)
    #[serde(default = "default_level")]
    pub level: u32,
    /// Number of times this character has died (defaults to 0)
    #[serde(default = "default_death_count")]
    pub death_count: u32,
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
}

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

/// Container for all character-related data in the application.
/// 
/// This struct serves as the root data structure for persistence,
/// containing all characters and tracking which one is currently active.
/// It's used by the repository layer for data serialization/deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterData {
    /// Collection of all characters in the system
    pub characters: Vec<Character>,
    /// ID of the currently active character (if any)
    pub active_character_id: Option<String>,
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

/// Default death count for new characters.
/// 
/// All characters start with 0 deaths when created.
fn default_death_count() -> u32 {
    0
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
