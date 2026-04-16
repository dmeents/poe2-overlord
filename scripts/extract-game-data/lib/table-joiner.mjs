/**
 * Joins raw pathofexile-dat table rows into denormalized item records.
 *
 * Input tables (as arrays of plain objects from pathofexile-dat):
 *   BaseItemTypes, ItemClasses, ItemVisualIdentity,
 *   ArmourTypes, WeaponTypes, ShieldTypes,
 *   ComponentAttributeRequirements,
 *   Mods, Stats,
 *   SkillGems, CurrencyItems, Flasks,
 *   UniqueStashLayout, Words
 *
 * Output:
 *   { categories: ItemCategory[], items: Item[] }
 *
 * Where items includes both base items and unique items.
 */

import { artPathToImageUrl } from './image-urls.mjs';
import { formatStatDisplay } from './stat-descriptions.mjs';

// ---------------------------------------------------------------------------
// Column name helpers
// ---------------------------------------------------------------------------
// pathofexile-dat exports columns using the names from dat-schema. Some are
// PascalCase from the schema, others may be serialised differently. We try
// both forms.

function col(row, ...names) {
  for (const n of names) {
    if (row[n] !== undefined && row[n] !== null) return row[n];
    // camelCase fallback
    const cc = n.charAt(0).toLowerCase() + n.slice(1);
    if (row[cc] !== undefined && row[cc] !== null) return row[cc];
  }
  return null;
}

function intCol(row, ...names) {
  const v = col(row, ...names);
  return v == null ? null : parseInt(v, 10) || 0;
}

function strCol(row, ...names) {
  const v = col(row, ...names);
  return v == null ? null : String(v);
}

function rowRef(row, name) {
  // pathofexile-dat resolves foreign-key references inline; the value may be
  // the referenced row object, an index, or a key string depending on schema.
  const v = col(row, name);
  if (v && typeof v === 'object' && !Array.isArray(v)) return v;
  return null;
}

function rowArray(row, name) {
  const v = col(row, name);
  return Array.isArray(v) ? v : [];
}

// ---------------------------------------------------------------------------
// Mod generation types (from game data constants)
// ---------------------------------------------------------------------------
const GEN_TYPE_UNIQUE = 3; // GenerationType = Unique

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * @param {object} tables        – keyed by table name, values are row arrays
 * @param {Map<string, *>} statDescriptions – parsed from stat_descriptions.mjs
 * @returns {{ categories: object[], items: object[] }}
 */
export function joinTables(tables, statDescriptions) {
  // ------------------------------------------------------------------
  // 1. Build lookup indexes from sub-tables
  // ------------------------------------------------------------------

  /** @type {Map<string, object>} id (rowid/index) → ItemClass row */
  const classById = buildIndex(tables.ItemClasses, 'Id', 'id');

  /** @type {Map<string, object>} BaseItemType id → ArmourTypes row */
  const armourByBase = buildFKIndex(tables.ArmourTypes, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → WeaponTypes row */
  const weaponByBase = buildFKIndex(tables.WeaponTypes, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → ShieldTypes row */
  const shieldByBase = buildFKIndex(tables.ShieldTypes, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → ComponentAttributeRequirements row */
  const reqByBase = buildFKIndex(
    tables.ComponentAttributeRequirements,
    'BaseItemTypesKey',
    'BaseItemType',
  );

  /** @type {Map<string, object>} BaseItemType id → SkillGems row */
  const gemByBase = buildFKIndex(tables.SkillGems, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → CurrencyItems row */
  const currencyByBase = buildFKIndex(tables.CurrencyItems, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → Flasks row */
  const flaskByBase = buildFKIndex(tables.Flasks, 'BaseItemTypesKey', 'BaseItemType');

  /** @type {Map<string, object>} BaseItemType id → ItemVisualIdentity row */
  const visualByBase = buildFKIndex(tables.ItemVisualIdentity, 'BaseItemTypesKey', 'BaseItemType');

  // Also index ItemVisualIdentity by its own Id for unique art lookups
  const visualById = buildIndex(tables.ItemVisualIdentity, 'Id', 'id');

  // ------------------------------------------------------------------
  // 2. Build Mods lookup: mod id → formatted display text per stat combo
  //    Also separate unique mods (GenerationType = 3) grouped by item name word
  // ------------------------------------------------------------------

  /** @type {Map<string, string>} mod Id → display text */
  const modTextById = new Map();

  /** @type {Map<string, object[]>} base-item Id → list of unique mod display objects */
  const uniqueModsByBaseId = new Map();

  for (const mod of (tables.Mods ?? [])) {
    const modId = strCol(mod, 'Id') ?? '';
    const text = buildModText(mod, statDescriptions);
    if (text) modTextById.set(modId, text);

    // Collect unique-specific mods
    if (intCol(mod, 'GenerationType') === GEN_TYPE_UNIQUE) {
      const domain = strCol(mod, 'Domain') ?? '';
      // Mods table doesn't directly reference BaseItemTypes for uniques;
      // we'll associate them via UniqueStashLayout later.
    }
  }

  // ------------------------------------------------------------------
  // 3. Build Words index for unique names (UniqueStashLayout → Words)
  // ------------------------------------------------------------------

  /** @type {Map<string, object>} words id → Words row */
  const wordsById = buildIndex(tables.Words, 'Id', 'id');

  /**
   * UniqueStashLayout has:
   *   BaseItemTypesKey → BaseItemTypes row
   *   WordsKey → Words row (the unique name)
   *   ItemVisualIdentityKey → unique-specific art
   *
   * Build: baseItemId → [{ uniqueName, imageUrl, flavourText, words }]
   */
  /** @type {Map<string, object[]>} */
  const uniquesByBaseId = new Map();

  for (const layout of (tables.UniqueStashLayout ?? [])) {
    const baseRef = rowRef(layout, 'BaseItemTypesKey') ?? rowRef(layout, 'BaseItemType');
    const wordsRef = rowRef(layout, 'WordsKey') ?? rowRef(layout, 'Words');
    const visualRef = rowRef(layout, 'ItemVisualIdentityKey') ?? rowRef(layout, 'ItemVisualIdentity');

    if (!baseRef) continue;

    const baseId = strCol(baseRef, 'Id') ?? '';
    const uniqueName = strCol(wordsRef, 'Text') ?? strCol(wordsRef, 'Word') ?? '';
    const flavourText = strCol(layout, 'FlavourText') ?? null;

    const imageUrl = visualRef
      ? artPathToImageUrl(strCol(visualRef, 'DDSFile', 'dds_file'))
      : artPathToImageUrl(
          strCol(visualByBase.get(baseId), 'DDSFile', 'dds_file'),
        );

    if (!uniquesByBaseId.has(baseId)) uniquesByBaseId.set(baseId, []);
    uniquesByBaseId.get(baseId).push({ uniqueName, imageUrl, flavourText });
  }

  // ------------------------------------------------------------------
  // 4. Build categories list
  // ------------------------------------------------------------------

  /** @type {object[]} */
  const categories = [];
  for (const cls of (tables.ItemClasses ?? [])) {
    const id = strCol(cls, 'Id') ?? '';
    const name = strCol(cls, 'Name') ?? id;
    if (id) categories.push({ id, name });
  }

  // ------------------------------------------------------------------
  // 5. Join BaseItemTypes → denormalized item records
  // ------------------------------------------------------------------

  /** @type {object[]} */
  const items = [];

  for (const base of (tables.BaseItemTypes ?? [])) {
    const baseId = strCol(base, 'Id') ?? '';
    if (!baseId) continue;

    const name = strCol(base, 'Name') ?? '';
    const dropLevel = intCol(base, 'DropLevel') ?? 0;
    const width = intCol(base, 'Width') ?? 1;
    const height = intCol(base, 'Height') ?? 1;

    // Item class
    const itemClassRef = rowRef(base, 'ItemClassesKey', 'ItemClass');
    const itemClassId = strCol(itemClassRef, 'Id') ?? strCol(base, 'ItemClassId') ?? '';
    const itemClassName = strCol(itemClassRef, 'Name') ?? itemClassId;
    const category = deriveCategory(itemClassId);

    // Implicit mods (array of mod refs on BaseItemTypes)
    const implicitModRefs = rowArray(base, 'ImplicitMods', 'Implicits');
    const implicitMods = implicitModRefs
      .map((modRef) => {
        const modId = typeof modRef === 'object' ? strCol(modRef, 'Id') : String(modRef);
        if (!modId) return null;
        const text = modTextById.get(modId) ?? null;
        return text ? { id: modId, text } : null;
      })
      .filter(Boolean);

    // Tags
    const tagRefs = rowArray(base, 'Tags');
    const tags = tagRefs
      .map((t) => (typeof t === 'object' ? strCol(t, 'Id') : String(t)))
      .filter(Boolean);

    // Base item art
    const visualRow = visualByBase.get(baseId);
    const baseImageUrl = artPathToImageUrl(strCol(visualRow, 'DDSFile', 'dds_file'));

    // Sub-type data
    const armour = buildArmour(armourByBase.get(baseId));
    const weapon = buildWeapon(weaponByBase.get(baseId));
    const shield = buildShield(shieldByBase.get(baseId));
    const requirements = buildRequirements(reqByBase.get(baseId));
    const gem = buildGem(gemByBase.get(baseId));
    const currency = buildCurrency(currencyByBase.get(baseId));
    const flask = buildFlask(flaskByBase.get(baseId));

    // --- Base item record ---
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
      image_url: baseImageUrl,
      flavour_text: null,
      tags,
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

    // --- Unique item records (one per unique sharing this base type) ---
    for (const unique of (uniquesByBaseId.get(baseId) ?? [])) {
      if (!unique.uniqueName) continue;

      const uniqueId = `unique/${unique.uniqueName.replace(/\s+/g, '_')}`;

      // Unique items share the base's implicit mods; explicit mods are unique-specific
      // (we collect those from the Mods table filtered by name match — approximation)
      items.push({
        id: uniqueId,
        name: unique.uniqueName,
        is_unique: true,
        unique_name: unique.uniqueName,
        base_type: name,
        item_class_id: itemClassId,
        item_class_name: itemClassName,
        category,
        rarity_frame: 3,
        width,
        height,
        drop_level: dropLevel,
        image_url: unique.imageUrl ?? baseImageUrl,
        flavour_text: unique.flavourText ?? null,
        tags: [...tags, 'unique'],
        requirements,
        defences: armour,
        weapon,
        shield,
        gem,
        currency,
        flask,
        implicit_mods: implicitMods,
        explicit_mods: [], // populated separately when Mods table links unique names
      });
    }
  }

  return { categories, items };
}

// ---------------------------------------------------------------------------
// Sub-type builders
// ---------------------------------------------------------------------------

function buildArmour(row) {
  if (!row) return null;
  return {
    armour: intCol(row, 'Armour') ?? 0,
    evasion: intCol(row, 'Evasion') ?? 0,
    energy_shield: intCol(row, 'EnergyShield') ?? 0,
    ward: intCol(row, 'Ward') ?? 0,
  };
}

function buildWeapon(row) {
  if (!row) return null;
  return {
    damage_min: intCol(row, 'DamageMin', 'MinDamage') ?? 0,
    damage_max: intCol(row, 'DamageMax', 'MaxDamage') ?? 0,
    critical: intCol(row, 'Critical', 'CriticalStrikeChance') ?? 0,   // stored x100
    attack_speed: intCol(row, 'AttackSpeed', 'AttackTimeMs') ?? 0,    // stored x100
    range_max: intCol(row, 'RangeMax') ?? 0,
  };
}

function buildShield(row) {
  if (!row) return null;
  return {
    block: intCol(row, 'Block') ?? 0,
  };
}

function buildRequirements(row) {
  if (!row) return { str: 0, dex: 0, int: 0 };
  return {
    str: intCol(row, 'ReqStr', 'Str') ?? 0,
    dex: intCol(row, 'ReqDex', 'Dex') ?? 0,
    int: intCol(row, 'ReqInt', 'Int') ?? 0,
  };
}

function buildGem(row) {
  if (!row) return null;
  return {
    gem_type: strCol(row, 'GemTagsKey', 'Type') ?? null,
    gem_colour: strCol(row, 'Colour') ?? null,
    gem_min_level: intCol(row, 'MinLevel') ?? 1,
    gem_tier: intCol(row, 'Tier') ?? null,
  };
}

function buildCurrency(row) {
  if (!row) return null;
  return {
    stack_size: intCol(row, 'StackSize') ?? 1,
    description: strCol(row, 'Description') ?? null,
  };
}

function buildFlask(row) {
  if (!row) return null;
  return {
    flask_type: strCol(row, 'FlaskType', 'Type') ?? null,
    flask_life: intCol(row, 'LifePerUse', 'Life') ?? 0,
    flask_mana: intCol(row, 'ManaPerUse', 'Mana') ?? 0,
    flask_recovery_time: intCol(row, 'RecoveryTime', 'Duration') ?? 0,
  };
}

// ---------------------------------------------------------------------------
// Mod text builder
// ---------------------------------------------------------------------------

function buildModText(mod, statDescriptions) {
  const statIds = [];
  const minValues = [];
  const maxValues = [];

  for (let i = 1; i <= 6; i++) {
    const statRef = rowRef(mod, `Stat${i}`, `stats${i}`);
    const statId = typeof statRef === 'object'
      ? strCol(statRef, 'Id')
      : strCol(mod, `Stat${i}Id`);

    if (!statId) continue;

    const min = intCol(mod, `Stat${i}Min`, `stat${i}Min`) ?? 0;
    const max = intCol(mod, `Stat${i}Max`, `stat${i}Max`) ?? 0;

    statIds.push(statId);
    minValues.push(min);
    maxValues.push(max);
  }

  if (statIds.length === 0) {
    // No stats; try Name column as display fallback
    return strCol(mod, 'Name') ?? null;
  }

  const text = formatStatDisplay(statDescriptions, statIds, minValues, maxValues);
  return text ?? `[${statIds.join(', ')}]`;
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/**
 * Build a map keyed by a column value from a row array.
 *
 * @param {object[] | undefined} rows
 * @param {...string} keyColumns – tried in order
 * @returns {Map<string, object>}
 */
function buildIndex(rows, ...keyColumns) {
  const map = new Map();
  for (const row of (rows ?? [])) {
    for (const key of keyColumns) {
      const v = col(row, key);
      if (v != null) { map.set(String(v), row); break; }
    }
  }
  return map;
}

/**
 * Build a foreign-key index: given rows that have a reference to BaseItemTypes,
 * key the rows by the base item's Id.
 *
 * @param {object[] | undefined} rows
 * @param {...string} fkColumns – column names to check (in priority order) for the FK ref
 * @returns {Map<string, object>}
 */
function buildFKIndex(rows, ...fkColumns) {
  const map = new Map();
  for (const row of (rows ?? [])) {
    let baseId = null;

    for (const fkCol of fkColumns) {
      const ref = rowRef(row, fkCol);
      if (ref) {
        baseId = strCol(ref, 'Id');
        break;
      }
      // Also try direct string ID column (e.g. BaseItemTypesKey stored as plain string)
      const direct = strCol(row, fkCol);
      if (direct) { baseId = direct; break; }
    }

    if (baseId) map.set(baseId, row);
  }
  return map;
}

/**
 * Derive a broad category string from an ItemClasses Id.
 *
 * @param {string} classId
 * @returns {string}
 */
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
