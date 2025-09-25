use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
    pub ascendency: Ascendency,
    pub league: League,
    pub hardcore: bool,
    pub solo_self_found: bool,
    pub created_at: DateTime<Utc>,
    pub last_played: Option<DateTime<Utc>>,
    pub is_active: bool,
    #[serde(default = "default_level")]
    pub level: u32,
    #[serde(default = "default_death_count")]
    pub death_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterUpdateParams {
    pub name: String,
    pub class: CharacterClass,
    pub ascendency: Ascendency,
    pub league: League,
    pub hardcore: bool,
    pub solo_self_found: bool,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Ascendency {
    #[serde(rename = "Titan")]
    Titan,
    #[serde(rename = "Warbringer")]
    Warbringer,
    #[serde(rename = "Smith of Katava")]
    SmithOfKatava,

    #[serde(rename = "Stormweaver")]
    Stormweaver,
    #[serde(rename = "Chronomancer")]
    Chronomancer,

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum League {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "Third Edict")]
    ThirdEdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterData {
    pub characters: Vec<Character>,
    pub active_character_id: Option<String>,
}

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

pub fn get_all_leagues() -> Vec<League> {
    vec![League::Standard, League::ThirdEdict]
}

fn default_level() -> u32 {
    1
}

fn default_death_count() -> u32 {
    0
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
        }
    }
}
