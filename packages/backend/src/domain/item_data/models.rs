use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Core item types
// ---------------------------------------------------------------------------

/// A single human-readable mod line (implicit or explicit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDisplay {
    pub id: String,
    pub text: String,
    /// Decoded `ModDomains` enum value (e.g. `ITEM`, `SANCTUM_RELIC`, `IDOL`).
    /// Also used to flag synthesized entries (e.g. `BONDED` for the secondary
    /// stat group on a soul-core row).
    #[serde(default)]
    pub domain: Option<String>,
    /// For rune / soul-core implicits, a human-readable label for which gear
    /// slot the stats apply to when socketed (e.g. `Weapon`, `Armour`). Null
    /// for normal item mods.
    #[serde(default)]
    pub slot: Option<String>,
    /// For rune / soul-core implicits, the list of ItemClasses.Id values the
    /// stats apply to (e.g. `["Body Armour", "Helmet", "Gloves", "Boots"]`).
    /// Empty for normal item mods.
    #[serde(default)]
    pub target_item_classes: Vec<String>,
}

/// Attribute requirements for equipping an item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttributeRequirements {
    pub str_req: i64,
    pub dex_req: i64,
    pub int_req: i64,
}

/// Defence values for armour pieces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceValues {
    pub armour: i64,
    pub evasion: i64,
    pub energy_shield: i64,
    pub ward: i64,
    /// Increased movement speed % bonus (boots only; 0 elsewhere).
    #[serde(default)]
    pub movement_speed: i64,
}

/// Weapon stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponValues {
    pub damage_min: i64,
    pub damage_max: i64,
    /// Stored x100 (e.g. 500 = 5.00%)
    pub critical: i64,
    /// Stored x100 (e.g. 120 = 1.20 aps)
    pub attack_speed: i64,
    pub range_max: i64,
    /// Reload time in ms (crossbows only; 0 elsewhere).
    #[serde(default)]
    pub reload_time: i64,
}

/// Shield stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldValues {
    pub block: i64,
}

/// Skill gem data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemData {
    pub gem_type: Option<String>,
    pub gem_colour: Option<String>,
    pub gem_min_level: i64,
    pub gem_tier: Option<i64>,
    /// Percent of the per-level attribute requirement this gem scales with.
    /// Zero means the attribute is not required for this gem.
    #[serde(default)]
    pub str_req_percent: i64,
    #[serde(default)]
    pub dex_req_percent: i64,
    #[serde(default)]
    pub int_req_percent: i64,
}

/// Currency item data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyData {
    pub stack_size: i64,
    pub description: Option<String>,
}

/// Flask data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlaskData {
    /// Decoded `FlaskType` enum: `LIFE`, `MANA`, `HYBRID`, or `UTILITY`.
    pub flask_type: Option<String>,
    /// In-game name from the Flasks table (may differ from the BaseItemTypes
    /// name for buff variants — e.g. "Granite Flask of Iron Skin").
    #[serde(default)]
    pub flask_name: Option<String>,
    pub flask_life: i64,
    pub flask_mana: i64,
    /// Duration in milliseconds
    pub flask_recovery_time: i64,
}

/// Rune / soul-core socket data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulCoreInfo {
    /// Character level required to equip an item with this rune socketed.
    pub required_level: i64,
    /// Max number of this rune allowed per item (null if no numeric cap).
    #[serde(default)]
    pub limit_count: Option<i64>,
    /// Display text of the socket-limit rule (e.g. "Only one per item").
    #[serde(default)]
    pub limit_text: Option<String>,
}

/// One guaranteed modifier granted by an essence when used on a specific
/// item category (Weapon, Body Armour, Jewellery, ...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssenceModifier {
    /// Human-readable category label (e.g. "Body Armour", "Jewellery").
    #[serde(default)]
    pub target_category: Option<String>,
    /// Raw ItemClasses.Id values this category expands to.
    #[serde(default)]
    pub target_item_classes: Vec<String>,
    /// Mods.Id for the resolved modifier (debug / filtering).
    #[serde(default)]
    pub mod_id: String,
    /// User-facing modifier text (from Mods stat descriptions, or the
    /// EssenceMods.Text override).
    pub mod_text: String,
}

/// Essence-specific metadata: tier, Perfect flag, upgrade path, and the
/// per-item-class guaranteed modifiers the essence grants when used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssenceInfo {
    pub tier: i64,
    #[serde(default)]
    pub is_perfect: bool,
    /// Item-id (`base/<path>`) of the essence this upgrades into, if any.
    #[serde(default)]
    pub upgrade_to_id: Option<String>,
    /// Display name of the upgrade target (e.g. "Essence of the Body").
    #[serde(default)]
    pub upgrade_to_name: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<EssenceModifier>,
}

// ---------------------------------------------------------------------------
// Main item type
// ---------------------------------------------------------------------------

/// A fully denormalized item record (base item or unique item).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub is_unique: bool,
    /// Display name for unique items (e.g. "Mjolner"); None for base items.
    pub unique_name: Option<String>,
    /// Base type name for unique items (e.g. "Gavel"); None for base items.
    pub base_type: Option<String>,
    pub item_class_id: String,
    pub category: String,
    /// 0=normal, 1=magic, 2=rare, 3=unique
    pub rarity_frame: i64,
    pub width: i64,
    pub height: i64,
    pub drop_level: i64,
    pub image_url: Option<String>,
    pub flavour_text: Option<String>,
    pub tags: Vec<String>,
    /// Base item is permanently corrupted (never drops in a non-corrupted form).
    #[serde(default)]
    pub is_corrupted: bool,
    /// Base cannot have mods rolled onto it.
    #[serde(default)]
    pub unmodifiable: bool,
    pub requirements: AttributeRequirements,
    pub defences: Option<DefenceValues>,
    pub weapon: Option<WeaponValues>,
    pub shield: Option<ShieldValues>,
    pub gem: Option<GemData>,
    pub currency: Option<CurrencyData>,
    pub flask: Option<FlaskData>,
    /// Populated for runes / soul cores / idols only.
    #[serde(default)]
    pub soul_core: Option<SoulCoreInfo>,
    /// Populated for essence currency items only.
    #[serde(default)]
    pub essence: Option<EssenceInfo>,
    pub implicit_mods: Vec<ModDisplay>,
    pub explicit_mods: Vec<ModDisplay>,
}

// ---------------------------------------------------------------------------
// Item category
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: String,
    pub name: String,
}

// ---------------------------------------------------------------------------
// Game data version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataVersion {
    pub patch_version: String,
    pub extracted_at: String,
    pub imported_at: String,
}

// ---------------------------------------------------------------------------
// Search / query types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemSearchParams {
    pub query: Option<String>,
    pub category: Option<String>,
    pub is_unique: Option<bool>,
    pub min_level: Option<i64>,
    pub max_level: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSearchResult {
    pub items: Vec<Item>,
    pub total_count: i64,
}

// ---------------------------------------------------------------------------
// Favourites
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFavorite {
    pub item_id: String,
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// JSON import shapes (from bundled game_data/ JSON files)
// These match the output of scripts/extract-game-data/extract.mjs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ImportedVersion {
    pub patch_version: String,
    pub extracted_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportedCategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportedModDisplay {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub slot: Option<String>,
    #[serde(default)]
    pub target_item_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportedRequirements {
    #[serde(default)]
    pub str_req: i64,
    #[serde(default)]
    pub dex_req: i64,
    #[serde(default)]
    pub int_req: i64,
    // Accept alternative field names from the JS extractor
    #[serde(default, rename = "str")]
    pub str_alt: i64,
    #[serde(default, rename = "dex")]
    pub dex_alt: i64,
    #[serde(default, rename = "int")]
    pub int_alt: i64,
}

impl ImportedRequirements {
    pub fn str_val(&self) -> i64 {
        if self.str_req != 0 {
            self.str_req
        } else {
            self.str_alt
        }
    }
    pub fn dex_val(&self) -> i64 {
        if self.dex_req != 0 {
            self.dex_req
        } else {
            self.dex_alt
        }
    }
    pub fn int_val(&self) -> i64 {
        if self.int_req != 0 {
            self.int_req
        } else {
            self.int_alt
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImportedDefences {
    #[serde(default)]
    pub armour: i64,
    #[serde(default)]
    pub evasion: i64,
    #[serde(default)]
    pub energy_shield: i64,
    #[serde(default)]
    pub ward: i64,
    #[serde(default)]
    pub movement_speed: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportedWeapon {
    #[serde(default)]
    pub damage_min: i64,
    #[serde(default)]
    pub damage_max: i64,
    #[serde(default)]
    pub critical: i64,
    #[serde(default)]
    pub attack_speed: i64,
    #[serde(default)]
    pub range_max: i64,
    #[serde(default)]
    pub reload_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportedShield {
    #[serde(default)]
    pub block: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportedGem {
    pub gem_type: Option<String>,
    pub gem_colour: Option<String>,
    #[serde(default = "default_one")]
    pub gem_min_level: i64,
    pub gem_tier: Option<i64>,
    #[serde(default)]
    pub str_req_percent: i64,
    #[serde(default)]
    pub dex_req_percent: i64,
    #[serde(default)]
    pub int_req_percent: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportedCurrency {
    #[serde(default = "default_one")]
    pub stack_size: i64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportedFlask {
    pub flask_type: Option<String>,
    #[serde(default)]
    pub flask_name: Option<String>,
    #[serde(default)]
    pub flask_life: i64,
    #[serde(default)]
    pub flask_mana: i64,
    #[serde(default)]
    pub flask_recovery_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportedSoulCore {
    #[serde(default)]
    pub required_level: i64,
    #[serde(default)]
    pub limit_count: Option<i64>,
    #[serde(default)]
    pub limit_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportedEssenceModifier {
    #[serde(default)]
    pub target_category: Option<String>,
    #[serde(default)]
    pub target_item_classes: Vec<String>,
    #[serde(default)]
    pub mod_id: String,
    pub mod_text: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportedEssence {
    #[serde(default)]
    pub tier: i64,
    #[serde(default)]
    pub is_perfect: bool,
    #[serde(default)]
    pub upgrade_to_id: Option<String>,
    #[serde(default)]
    pub upgrade_to_name: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<ImportedEssenceModifier>,
}

fn default_one() -> i64 {
    1
}

/// Full item as emitted by the extraction script.
#[derive(Debug, Deserialize)]
pub struct ImportedItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub is_unique: bool,
    pub unique_name: Option<String>,
    pub base_type: Option<String>,
    pub item_class_id: String,
    pub category: String,
    #[serde(default)]
    pub rarity_frame: i64,
    #[serde(default = "default_one")]
    pub width: i64,
    #[serde(default = "default_one")]
    pub height: i64,
    #[serde(default)]
    pub drop_level: i64,
    pub image_url: Option<String>,
    pub flavour_text: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub is_corrupted: bool,
    #[serde(default)]
    pub unmodifiable: bool,
    #[serde(default)]
    pub requirements: Option<ImportedRequirements>,
    #[serde(default, rename = "defences")]
    pub defences: Option<ImportedDefences>,
    pub weapon: Option<ImportedWeapon>,
    pub shield: Option<ImportedShield>,
    pub gem: Option<ImportedGem>,
    pub currency: Option<ImportedCurrency>,
    pub flask: Option<ImportedFlask>,
    #[serde(default)]
    pub soul_core: Option<ImportedSoulCore>,
    #[serde(default)]
    pub essence: Option<ImportedEssence>,
    #[serde(default)]
    pub implicit_mods: Vec<ImportedModDisplay>,
    #[serde(default)]
    pub explicit_mods: Vec<ImportedModDisplay>,
}
