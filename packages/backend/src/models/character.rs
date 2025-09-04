use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Core character structure for Path of Exile 2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
    /// UUID for unique identification
    pub id: String,
    /// Character name (user-defined)
    pub name: String,
    /// POE2 class
    pub class: CharacterClass,
    /// Character ascendency (subclass)
    pub ascendency: Ascendency,
    /// League the character is in
    pub league: League,
    /// Hardcore mode flag
    pub hardcore: bool,
    /// Solo Self Found mode flag
    pub solo_self_found: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last time this character was active
    pub last_played: Option<DateTime<Utc>>,
    /// Currently selected character
    pub is_active: bool,
}

/// POE2 character classes with proper display names
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    #[serde(rename = "Warrior")]
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
}

/// POE2 ascendencies with proper display names
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Ascendency {
    // Warrior ascendencies
    #[serde(rename = "Titan")]
    Titan,
    #[serde(rename = "Warbringer")]
    Warbringer,
    #[serde(rename = "Smith of Katava")]
    SmithOfKatava,

    // Sorceress ascendencies
    #[serde(rename = "Stormweaver")]
    Stormweaver,
    #[serde(rename = "Chronomancer")]
    Chronomancer,

    // Ranger ascendencies
    #[serde(rename = "Deadeye")]
    Deadeye,
    #[serde(rename = "Pathfinder")]
    Pathfinder,

    // Huntress ascendencies
    #[serde(rename = "Ritualist")]
    Ritualist,
    #[serde(rename = "Amazon")]
    Amazon,

    // Monk ascendencies
    #[serde(rename = "Invoker")]
    Invoker,
    #[serde(rename = "Acolyte of Chayula")]
    AcolyteOfChayula,

    // Mercenary ascendencies
    #[serde(rename = "Gemling Legionnaire")]
    GemlingLegionnaire,
    #[serde(rename = "Tactitian")]
    Tactitian,
    #[serde(rename = "Witchhunter")]
    Witchhunter,

    // Witch ascendencies
    #[serde(rename = "Blood Mage")]
    BloodMage,
    #[serde(rename = "Infernalist")]
    Infernalist,
    #[serde(rename = "Lich")]
    Lich,
}

/// League options with proper display names
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum League {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "Third Edict")]
    ThirdEdict,
}

/// Container for character data (used by services)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    /// All characters
    pub characters: Vec<Character>,
    /// Currently active character ID
    pub active_character_id: Option<String>,
}

/// Check if an ascendency is valid for a given class
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

/// Get all valid ascendencies for a given class
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

impl Default for CharacterData {
    fn default() -> Self {
        Self {
            characters: Vec::new(),
            active_character_id: None,
        }
    }
}

/// Helper function to get all available character classes
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

/// Helper function to get all available leagues
pub fn get_all_leagues() -> Vec<League> {
    vec![League::Standard, League::ThirdEdict]
}
