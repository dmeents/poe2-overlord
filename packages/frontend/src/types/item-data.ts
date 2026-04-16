/**
 * Types for item data from the backend item_data domain.
 * These mirror the Rust structs in backend/src/domain/item_data/models.rs.
 */

export interface ModDisplay {
  id: string;
  text: string;
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
}

export interface WeaponValues {
  damage_min: number;
  damage_max: number;
  /** Stored x100 (e.g. 500 = 5.00%) */
  critical: number;
  /** Stored x100 (e.g. 120 = 1.20 aps) */
  attack_speed: number;
  range_max: number;
}

export interface ShieldValues {
  block: number;
}

export interface GemData {
  gem_type: string | null;
  gem_colour: string | null;
  gem_min_level: number;
  gem_tier: number | null;
}

export interface CurrencyItemData {
  stack_size: number;
  description: string | null;
}

export interface FlaskData {
  flask_type: string | null;
  flask_life: number;
  flask_mana: number;
  /** Duration in milliseconds */
  flask_recovery_time: number;
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
  requirements: AttributeRequirements;
  defences: DefenceValues | null;
  weapon: WeaponValues | null;
  shield: ShieldValues | null;
  gem: GemData | null;
  currency: CurrencyItemData | null;
  flask: FlaskData | null;
  implicit_mods: ModDisplay[];
  explicit_mods: ModDisplay[];
}
