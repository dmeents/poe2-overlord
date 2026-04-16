/**
 * Joins raw pathofexile-dat table rows into denormalized item records.
 *
 * POE2 schema differences from POE1:
 * - Foreign-key columns export as integer row indices (_index), not row objects.
 * - BaseItemTypes: ItemClass, ItemVisualIdentity, Implicit_Mods (not the POE1 names)
 * - Sub-tables: BaseItemType FK column (not BaseItemTypesKey)
 * - WeaponTypes: Speed (not AttackSpeed)
 * - Mods: Stat1..6 + Stat1Value..6Value (no Min/Max range — single value per stat)
 * - IVI has no back-reference to BaseItemTypes; the FK lives on BaseItemTypes.ItemVisualIdentity
 * - UniqueStashLayout has no BaseItemTypes FK in POE2 — uniques not linked here
 * - ComponentAttributeRequirements.BaseItemTypesKey is a string (metadata path), not an FK int
 */

import { artPathToImageUrl } from './image-urls.mjs';
import { formatStatDisplay } from './stat-descriptions.mjs';

// ---------------------------------------------------------------------------
// Column helpers
// ---------------------------------------------------------------------------

function col(row, ...names) {
  if (row == null) return null;
  for (const n of names) {
    if (row[n] !== undefined && row[n] !== null) return row[n];
    const cc = n.charAt(0).toLowerCase() + n.slice(1);
    if (row[cc] !== undefined && row[cc] !== null) return row[cc];
  }
  return null;
}

function intCol(row, ...names) {
  const v = col(row, ...names);
  if (v == null) return null;
  const n = parseInt(v, 10);
  return isNaN(n) ? null : n;
}

function strCol(row, ...names) {
  const v = col(row, ...names);
  return v == null ? null : String(v);
}

function rowArray(row, ...names) {
  const v = col(row, ...names);
  return Array.isArray(v) ? v : [];
}

// ---------------------------------------------------------------------------
// Index builders
// ---------------------------------------------------------------------------

/** Build a map keyed by a column value (any type). */
function buildIndexByCol(rows, keyColumn) {
  const map = new Map();
  for (const row of (rows ?? [])) {
    const v = col(row, keyColumn);
    if (v != null) map.set(v, row);
  }
  return map;
}

/**
 * Build a map from the integer value of a FK column → row.
 * FK columns in POE2 export as plain integers (row indices into the referenced table).
 */
function buildFKIndex(rows, fkColumn) {
  const map = new Map();
  for (const row of (rows ?? [])) {
    const fkVal = col(row, fkColumn);
    if (fkVal != null) map.set(fkVal, row);
  }
  return map;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

export function joinTables(tables, statDescriptions) {
  // ------------------------------------------------------------------
  // 1. Build lookup indexes
  // ------------------------------------------------------------------

  // ItemClasses: keyed by _index (integer FK from BaseItemTypes.ItemClass)
  const classById = buildIndexByCol(tables.ItemClasses ?? [], '_index');

  // ItemVisualIdentity: keyed by _index (integer FK from BaseItemTypes.ItemVisualIdentity)
  // (IVI has no back-reference to BaseItemTypes in POE2)
  const visualByIndex = buildIndexByCol(tables.ItemVisualIdentity ?? [], '_index');

  // Sub-type tables: keyed by BaseItemType FK integer (= BaseItemTypes._index)
  const armourByBase   = buildFKIndex(tables.ArmourTypes   ?? [], 'BaseItemType');
  const weaponByBase   = buildFKIndex(tables.WeaponTypes   ?? [], 'BaseItemType');
  const shieldByBase   = buildFKIndex(tables.ShieldTypes   ?? [], 'BaseItemType');
  const gemByBase      = buildFKIndex(tables.SkillGems     ?? [], 'BaseItemType');
  const currencyByBase = buildFKIndex(tables.CurrencyItems ?? [], 'BaseItemType');
  const flaskByBase    = buildFKIndex(tables.Flasks        ?? [], 'BaseItemType');

  // ComponentAttributeRequirements: BaseItemTypesKey is a STRING (metadata path), not an FK int
  const reqByBaseId = buildIndexByCol(tables.ComponentAttributeRequirements ?? [], 'BaseItemTypesKey');

  // Words: keyed by _index (FK from UniqueStashLayout.WordsKey)
  const wordsById = buildIndexByCol(tables.Words ?? [], '_index');

  // Mods: keyed by _index (FK from BaseItemTypes.Implicit_Mods array)
  const modByIndex = buildIndexByCol(tables.Mods ?? [], '_index');

  // ------------------------------------------------------------------
  // 2. Build mod display text lookup
  // ------------------------------------------------------------------

  /** @type {Map<number, {id: string, text: string}>} */
  const modDisplayByIndex = new Map();

  for (const mod of (tables.Mods ?? [])) {
    const idx = mod._index;
    if (idx == null) continue;
    const text = buildModText(mod, statDescriptions);
    const modId = strCol(mod, 'Id') ?? '';
    if (text) modDisplayByIndex.set(idx, { id: modId, text });
  }

  // ------------------------------------------------------------------
  // 3. Build categories list
  // ------------------------------------------------------------------

  const categories = [];
  for (const cls of (tables.ItemClasses ?? [])) {
    const id   = strCol(cls, 'Id') ?? '';
    const name = strCol(cls, 'Name') ?? id;
    if (id) categories.push({ id, name });
  }

  // ------------------------------------------------------------------
  // 4. Join BaseItemTypes → denormalized item records
  // ------------------------------------------------------------------

  const items = [];

  for (const base of (tables.BaseItemTypes ?? [])) {
    const baseId = strCol(base, 'Id') ?? '';
    if (!baseId) continue;

    const baseIdx   = base._index;         // integer row index
    const name      = strCol(base, 'Name') ?? '';
    const dropLevel = intCol(base, 'DropLevel') ?? 0;
    const width     = intCol(base, 'Width') ?? 1;
    const height    = intCol(base, 'Height') ?? 1;

    // Item class — FK integer into ItemClasses table
    const classIdx     = intCol(base, 'ItemClass');
    const itemClassRow = classIdx != null ? classById.get(classIdx) : null;
    const itemClassId  = strCol(itemClassRow, 'Id') ?? '';
    const itemClassName = strCol(itemClassRow, 'Name') ?? itemClassId;
    const category     = deriveCategory(itemClassId);

    // Visual identity — FK integer into ItemVisualIdentity table
    const visualIdx = intCol(base, 'ItemVisualIdentity');
    const visualRow = visualIdx != null ? visualByIndex.get(visualIdx) : null;
    const imageUrl  = artPathToImageUrl(strCol(visualRow, 'DDSFile'));

    // Implicit mods — array of FK integers into Mods table
    const implicitModIndices = rowArray(base, 'Implicit_Mods');
    const implicitMods = implicitModIndices
      .map((idx) => (typeof idx === 'number' ? modDisplayByIndex.get(idx) : null))
      .filter(Boolean);

    // Sub-type data — all keyed by BaseItemTypes row index (base._index)
    const armour       = buildArmour(baseIdx != null ? armourByBase.get(baseIdx) : null);
    const weapon       = buildWeapon(baseIdx != null ? weaponByBase.get(baseIdx) : null);
    const shield       = buildShield(baseIdx != null ? shieldByBase.get(baseIdx) : null);
    const gem          = buildGem(baseIdx != null ? gemByBase.get(baseIdx) : null);
    const currency     = buildCurrency(baseIdx != null ? currencyByBase.get(baseIdx) : null);
    const flask        = buildFlask(baseIdx != null ? flaskByBase.get(baseIdx) : null);

    // Requirements — keyed by BaseItemTypes.Id string (metadata path)
    const requirements = buildRequirements(reqByBaseId.get(baseId));

    items.push({
      id: `base/${baseId}`,
      name,
      is_unique: false,
      unique_name: null,
      base_type: null,
      item_class_id: itemClassId,
      item_class_name: itemClassName,
      category,
      rarity_frame: 0,
      width,
      height,
      drop_level: dropLevel,
      image_url: imageUrl,
      flavour_text: null,
      tags: [],
      requirements,
      defences: armour,
      weapon,
      shield,
      gem,
      currency,
      flask,
      implicit_mods: implicitMods,
      explicit_mods: [],
    });
  }

  return { categories, items };
}

// ---------------------------------------------------------------------------
// Sub-type builders
// ---------------------------------------------------------------------------

function buildArmour(row) {
  if (!row) return null;
  return {
    armour:        intCol(row, 'Armour') ?? 0,
    evasion:       intCol(row, 'Evasion') ?? 0,
    energy_shield: intCol(row, 'EnergyShield') ?? 0,
    ward:          intCol(row, 'Ward') ?? 0,
  };
}

function buildWeapon(row) {
  if (!row) return null;
  return {
    damage_min:   intCol(row, 'DamageMin') ?? 0,
    damage_max:   intCol(row, 'DamageMax') ?? 0,
    critical:     intCol(row, 'Critical')  ?? 0,  // stored x100
    attack_speed: intCol(row, 'Speed')     ?? 0,  // stored x100, POE2 column is "Speed"
    range_max:    intCol(row, 'RangeMax')  ?? 0,
  };
}

function buildShield(row) {
  if (!row) return null;
  return { block: intCol(row, 'Block') ?? 0 };
}

function buildRequirements(row) {
  if (!row) return { str: 0, dex: 0, int: 0 };
  return {
    str: intCol(row, 'ReqStr') ?? 0,
    dex: intCol(row, 'ReqDex') ?? 0,
    int: intCol(row, 'ReqInt') ?? 0,
  };
}

function buildGem(row) {
  if (!row) return null;
  // gem_type and gem_colour are enumrow/i32 integers in the game data.
  // ImportedGem on the Rust side expects Option<String> for these fields,
  // so we stringify them. gem_min_level is expected by ImportedGem (defaults to 1).
  const gemType   = intCol(row, 'GemType');
  const gemColour = intCol(row, 'GemColour');
  return {
    gem_type:      gemType   != null ? String(gemType)   : null,
    gem_colour:    gemColour != null ? String(gemColour) : null,
    gem_min_level: 1,
    gem_tier:      intCol(row, 'Tier') ?? null,
  };
}

function buildCurrency(row) {
  if (!row) return null;
  return {
    stack_size:  intCol(row, 'StackSize') ?? 1,
    description: strCol(row, 'Description') ?? null,
  };
}

function buildFlask(row) {
  if (!row) return null;
  return {
    flask_life:          intCol(row, 'LifePerUse') ?? 0,
    flask_mana:          intCol(row, 'ManaPerUse') ?? 0,
    flask_recovery_time: intCol(row, 'RecoveryTime') ?? 0,
  };
}

// ---------------------------------------------------------------------------
// Mod text builder
// POE2 Mods: Stat1..6 are FK ints to Stats, Stat1Value..6Value are single values (no Min/Max range)
// ---------------------------------------------------------------------------

function buildModText(mod, statDescriptions) {
  const statIds   = [];
  const values    = [];

  for (let i = 1; i <= 6; i++) {
    const statKeyIdx = intCol(mod, `Stat${i}`);
    if (statKeyIdx == null) continue;

    const value = intCol(mod, `Stat${i}Value`) ?? 0;
    statIds.push(`stat_${statKeyIdx}`);
    values.push(value);
  }

  if (statIds.length === 0) {
    return strCol(mod, 'Name') ?? null;
  }

  // If stat descriptions were loaded, try to format them; otherwise fall back to mod Name.
  const text = formatStatDisplay(statDescriptions, statIds, values, values);
  return text ?? strCol(mod, 'Name') ?? null;
}

// ---------------------------------------------------------------------------
// Category derivation
// ---------------------------------------------------------------------------

function deriveCategory(classId) {
  if (!classId) return 'Other';
  const id = classId.toLowerCase();

  if (/helmet|glove|boot|body|shield/.test(id)) return 'Armour';
  if (/sword|axe|mace|bow|staff|wand|dagger|claw|spear|flail|crossbow/.test(id)) return 'Weapon';
  if (/gem|skill|support/.test(id)) return 'Gem';
  if (/currency|stackable/.test(id)) return 'Currency';
  if (/flask/.test(id)) return 'Flask';
  if (/map|fragment|piece|scarab/.test(id)) return 'Map';
  if (/jewel/.test(id)) return 'Jewel';
  if (/amulet|ring|belt/.test(id)) return 'Accessory';
  return 'Other';
}
