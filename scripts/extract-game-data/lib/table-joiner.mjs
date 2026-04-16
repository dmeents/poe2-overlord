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

/** Build a map from a FK column value → array of rows (one-to-many). */
function buildMultiFKIndex(rows, fkColumn) {
  const map = new Map();
  for (const row of (rows ?? [])) {
    const fkVal = col(row, fkColumn);
    if (fkVal != null) {
      if (!map.has(fkVal)) map.set(fkVal, []);
      map.get(fkVal).push(row);
    }
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

  // Stats: keyed by _index (used for SoulCoreStats stat lookups)
  const statByIndex = buildIndexByCol(tables.Stats ?? [], '_index');

  // GemEffects: keyed by _index (FK from SkillGems.GemEffects array)
  const gemEffectByIndex = buildIndexByCol(tables.GemEffects ?? [], '_index');

  // SoulCores: keyed by BaseItemType FK integer (→ base._index)
  const soulCoresByBase = buildFKIndex(tables.SoulCores ?? [], 'BaseItemType');

  // SoulCoreStats: one-to-many by SoulCore FK integer (→ soulCore._index)
  const soulCoreStatsBySC = buildMultiFKIndex(tables.SoulCoreStats ?? [], 'SoulCore');

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
    let implicitMods = implicitModIndices
      .map((idx) => (typeof idx === 'number' ? modDisplayByIndex.get(idx) : null))
      .filter(Boolean);

    // For soul core items (runes, idols, etc.) that have no implicit mods from the Mods table,
    // derive stat descriptions from SoulCoreStats instead.
    if (implicitMods.length === 0 && baseIdx != null) {
      const soulCore = soulCoresByBase.get(baseIdx);
      if (soulCore != null) {
        const scStatsRows = soulCoreStatsBySC.get(soulCore._index) ?? [];
        implicitMods = buildSoulCoreMods(scStatsRows, statByIndex);
      }
    }

    // For gem items with no implicit mods, resolve SupportText from GemEffects.
    if (implicitMods.length === 0 && baseIdx != null) {
      const gemRow = gemByBase.get(baseIdx);
      if (gemRow) {
        const gemEffectFks = rowArray(gemRow, 'GemEffects');
        const gemDesc = buildGemDescription(gemEffectFks, gemEffectByIndex);
        if (gemDesc) {
          implicitMods = [{ id: 'gem_support_text', text: gemDesc }];
        }
      }
    }

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
// SoulCore stat description builder
// Stat description txt files are not accessible via the bundle system in POE2,
// so we derive approximate human-readable text directly from stat IDs + values.
// ---------------------------------------------------------------------------

/**
 * Build implicit-mod-like entries from SoulCoreStats rows.
 * Each row holds parallel Stats[] (FK int array) and StatsValues[] (int array).
 */
function buildSoulCoreMods(scStatsRows, statByIndex) {
  const mods = [];

  for (const row of scStatsRows) {
    const statFks  = rowArray(row, 'Stats');
    const values   = rowArray(row, 'StatsValues');

    if (statFks.length === 0) continue;

    // Resolve FK ints → stat ID strings
    const statIds = statFks
      .map((fk) => (typeof fk === 'number' ? strCol(statByIndex.get(fk), 'Id') : null))
      .filter(Boolean);

    if (statIds.length === 0) continue;

    const text = formatSoulCoreStatLine(statIds, values);
    if (text) {
      mods.push({ id: statIds.join(','), text });
    }
  }

  return mods;
}

/**
 * Format one SoulCoreStats row (arrays of stat IDs + values) into display text.
 * Handles min/max damage pairs specially; falls back to per-stat formatting.
 */
function formatSoulCoreStatLine(statIds, values) {
  // Min/max damage pair detection (2-stat rows only).
  // Handles: local_minimum_added_X_damage, thorns_minimum_base_X_damage,
  //          attack_minimum_added_X_damage, allies_in_presence_attack_minimum_added_X_damage
  if (statIds.length === 2) {
    const m1 = statIds[0].match(/minimum(?:_added|_base)?_(\w+)_damage$/);
    const m2 = statIds[1].match(/maximum(?:_added|_base)?_(\w+)_damage$/);
    if (m1 && m2 && m1[1] === m2[1]) {
      const dmgType = m1[1].charAt(0).toUpperCase() + m1[1].slice(1);
      const verb    = statIds[0].startsWith('thorns_') ? 'Deals' : 'Adds';
      const suffix  = statIds[0].startsWith('thorns_') ? ' to Attackers' : '';
      return `${verb} ${values[0] ?? 0} to ${values[1] ?? 0} ${dmgType} Damage${suffix}`;
    }
  }

  // Format each stat individually and join with newline
  const parts = statIds
    .map((id, i) => formatSingleSoulCoreStat(id, values[i] ?? 0))
    .filter(Boolean);
  return parts.join('\n') || null;
}

/**
 * Convert a single stat ID + integer value to approximate display text.
 *
 * Examples:
 *   additional_strength, 6                             → "+6 to Strength"
 *   base_fire_damage_resistance_%, 12                  → "+12% Fire Damage Resistance"
 *   attack_speed_+%, 15                                → "+15% Attack Speed"
 *   base_maximum_life, 50                              → "+50 Maximum Life"
 *   energy_generated_+%, 10                            → "+10% Energy Generated"
 *   non_skill_base_all_damage_%_to_gain_as_fire, 8     → "+8% of Damage Gained as Fire"
 */
function formatSingleSoulCoreStat(id, value) {
  const sign = value >= 0 ? '+' : '';

  // "damage_%_to_gain_as_X" pattern (e.g. non_skill_base_all_damage_%_to_gain_as_fire)
  const gainAs = id.match(/damage_%_to_gain_as_(\w+)$/);
  if (gainAs) {
    const element = gainAs[1].charAt(0).toUpperCase() + gainAs[1].slice(1);
    return `${sign}${value}% of Damage Gained as ${element}`;
  }

  // "additional_X" → "+V to X"
  if (id.startsWith('additional_')) {
    const attr = humanizeStat(id.slice('additional_'.length));
    return `${sign}${value} to ${attr}`;
  }

  // Detect percentage stat: id ends with %, _%,  _+%, or +%
  const isPct = /[_%+]%$/.test(id) || id.endsWith('%');
  const pctStr = isPct ? '%' : '';

  // Strip common prefixes, trailing underscores, and percent/plus markers
  const name = humanizeStat(
    id
      .replace(/^(non_skill_base_|base_|local_|global_)/, '')
      .replace(/[_+]*%$/, '')   // remove trailing _+%  (handles energy_generated_+%)
      .replace(/_+$/, '')       // remove any remaining trailing underscores
  );

  return `${sign}${value}${pctStr} ${name}`;
}

/** Convert snake_case to Title Case words, ignoring empty segments. */
function humanizeStat(str) {
  return str
    .split('_')
    .filter(Boolean)
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(' ');
}

// ---------------------------------------------------------------------------
// Gem description builder
// Support gem descriptions live in GemEffects.SupportText (not in Mods or implicit_mods).
// ---------------------------------------------------------------------------

/**
 * Resolve a SkillGems.GemEffects FK array → first non-empty SupportText, cleaned of markup.
 */
function buildGemDescription(gemEffectFks, gemEffectByIndex) {
  for (const fk of gemEffectFks) {
    if (typeof fk !== 'number') continue;
    const ge = gemEffectByIndex.get(fk);
    if (!ge) continue;
    const text = strCol(ge, 'SupportText');
    if (text) return cleanGemMarkup(text);
  }
  return null;
}

/**
 * Strip POE2 rich-text markup from gem description strings.
 *   [Attack|Attacks] → "Attacks"  (second value is the display form)
 *   [Gain]           → "Gain"     (single-value tags just drop brackets)
 */
function cleanGemMarkup(text) {
  return text
    .replace(/\[([^|\]]+)\|([^\]]+)\]/g, '$2')
    .replace(/\[([^\]]+)\]/g, '$1')
    .replace(/^\s*DNT\s+/i, '')   // strip "DNT " dev prefix (Do Not Translate placeholder)
    .trim();
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
