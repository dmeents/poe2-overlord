/**
 * Types for item data from the backend item_data domain.
 * These mirror the Rust structs in backend/src/domain/item_data/models.rs.
 */

export interface ItemCategory {
  id: string;
  name: string;
}

export interface GameDataVersion {
  patch_version: string;
  extracted_at: string;
  imported_at: string;
}

export interface ItemSearchParams {
  query?: string | null;
  category?: string | null;
  is_unique?: boolean | null;
  min_level?: number | null;
  max_level?: number | null;
  /** Clamped to 1–500 by the backend. Default: 50. */
  limit?: number | null;
  offset?: number | null;
}

export interface ItemSearchResult {
  items: ItemData[];
  total_count: number;
}

export interface ModDisplay {
  id: string;
  text: string;
  /** Decoded ModDomains enum (ITEM, SANCTUM_RELIC, IDOL, …) or 'BONDED' for
   * the secondary stat group on a soul-core row. */
  domain: string | null;
  /** Human-readable slot label for rune / soul-core implicits
   * (e.g. 'Weapon', 'Armour'). Null for normal item mods. */
  slot: string | null;
  /** ItemClasses.Id values the stats apply to when socketed. Empty for
   * normal item mods. */
  target_item_classes: string[];
}

export interface AttributeRequirements {
  str_req: number;
  dex_req: number;
  int_req: number;
}

export interface DefenceValues {
  armour: number;
  evasion: number;
  energy_shield: number;
  ward: number;
  /** % movement speed bonus; non-zero on boots only. */
  movement_speed: number;
}

export interface WeaponValues {
  damage_min: number;
  damage_max: number;
  /** Stored x100 (e.g. 500 = 5.00%) */
  critical: number;
  /** Stored x100 (e.g. 120 = 1.20 aps) */
  attack_speed: number;
  range_max: number;
  /** Reload time in ms; non-zero on crossbows only. */
  reload_time: number;
}

export interface ShieldValues {
  block: number;
}

export interface GemData {
  gem_type: string | null;
  gem_colour: string | null;
  gem_min_level: number;
  gem_tier: number | null;
  /** % of the per-level attribute requirement this gem scales with. */
  str_req_percent: number;
  dex_req_percent: number;
  int_req_percent: number;
}

export interface CurrencyItemData {
  stack_size: number;
  description: string | null;
}

export interface FlaskData {
  /** Decoded FlaskType: LIFE | MANA | HYBRID | UTILITY */
  flask_type: string | null;
  flask_name: string | null;
  flask_life: number;
  flask_mana: number;
  /** Duration in milliseconds */
  flask_recovery_time: number;
}

export interface SoulCoreInfo {
  required_level: number;
  limit_count: number | null;
  limit_text: string | null;
}

export interface EssenceModifier {
  target_category: string | null;
  target_item_classes: string[];
  mod_id: string;
  mod_text: string;
}

export interface EssenceInfo {
  tier: number;
  is_perfect: boolean;
  upgrade_to_id: string | null;
  upgrade_to_name: string | null;
  modifiers: EssenceModifier[];
}

export interface ItemData {
  id: string;
  name: string;
  is_unique: boolean;
  unique_name: string | null;
  base_type: string | null;
  item_class_id: string;
  category: string;
  /** 0=normal, 1=magic, 2=rare, 3=unique */
  rarity_frame: number;
  width: number;
  height: number;
  drop_level: number;
  image_url: string | null;
  flavour_text: string | null;
  tags: string[];
  is_corrupted: boolean;
  unmodifiable: boolean;
  requirements: AttributeRequirements;
  defences: DefenceValues | null;
  weapon: WeaponValues | null;
  shield: ShieldValues | null;
  gem: GemData | null;
  currency: CurrencyItemData | null;
  flask: FlaskData | null;
  soul_core: SoulCoreInfo | null;
  essence: EssenceInfo | null;
  implicit_mods: ModDisplay[];
  explicit_mods: ModDisplay[];
}
