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

export function joinTables(tables, statDescriptions, options = {}) {
  const enums = options.enums ?? {};
  const modDomain = enums.ModDomains ?? (() => null);
  const flaskTypeEnum = enums.FlaskType ?? (() => null);

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

  // SoulCoreStatCategories: keyed by _index (FK from SoulCoreStats.StatCategory).
  // Each category carries a human-readable Display string + the list of item
  // classes the stat group applies to when the rune/soul core is socketed.
  const soulCoreStatCatByIndex = buildIndexByCol(tables.SoulCoreStatCategories ?? [], '_index');

  // SoulCoreLimits: keyed by _index (FK from SoulCores.Limit). Text is the
  // user-facing rule ("Only one per item" etc.).
  const soulCoreLimitByIndex = buildIndexByCol(tables.SoulCoreLimits ?? [], '_index');

  // FlavourText: keyed by _index (FK from BaseItemTypes.FlavourText).
  const flavourTextByIndex = buildIndexByCol(tables.FlavourText ?? [], '_index');

  // Tags: keyed by _index (FK from BaseItemTypes.Tags[] array).
  const tagByIndex = buildIndexByCol(tables.Tags ?? [], '_index');

  // BaseItemTypes: keyed by _index so FK resolutions that end in a
  // BaseItemType (e.g. essence UpgradeResult chain) can fetch the display
  // name + metadata path.
  const baseItemByIndex = buildIndexByCol(tables.BaseItemTypes ?? [], '_index');

  // Essences: keyed by BaseItemType FK integer. Secondary index by _index
  // so UpgradeResult (a row FK into Essences itself) can resolve into the
  // upgraded essence's BaseItemType and then its display name.
  const essenceByBase     = buildFKIndex(tables.Essences ?? [], 'BaseItemType');
  const essenceByIndex    = buildIndexByCol(tables.Essences ?? [], '_index');
  const essenceModsByEss  = buildMultiFKIndex(tables.EssenceMods ?? [], 'Essence');
  const essenceCatByIndex = buildIndexByCol(tables.EssenceTargetItemCategories ?? [], '_index');

  // ------------------------------------------------------------------
  // 2. Build mod display text lookup
  // ------------------------------------------------------------------

  /** @type {Map<number, {id: string, text: string, domain: string | null, slot: string | null, target_item_classes: string[]}>} */
  const modDisplayByIndex = new Map();

  for (const mod of (tables.Mods ?? [])) {
    const idx = mod._index;
    if (idx == null) continue;
    const text = buildModText(mod, statDescriptions, statByIndex);
    const modId = strCol(mod, 'Id') ?? '';
    const domain = modDomain(intCol(mod, 'Domain'));
    if (text) {
      modDisplayByIndex.set(idx, {
        id: modId,
        text,
        domain,
        slot: null,
        target_item_classes: [],
      });
    }
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

    // Soul-core-specific info (null for non-socketable items).
    let soulCoreInfo = null;

    // For soul core items (runes, idols, etc.) that have no implicit mods from the Mods table,
    // derive stat descriptions from SoulCoreStats instead.
    if (implicitMods.length === 0 && baseIdx != null) {
      const soulCore = soulCoresByBase.get(baseIdx);
      if (soulCore != null) {
        const scStatsRows = soulCoreStatsBySC.get(soulCore._index) ?? [];
        implicitMods = buildSoulCoreMods(scStatsRows, statByIndex, classById, soulCoreStatCatByIndex);
        soulCoreInfo = buildSoulCoreInfo(soulCore, soulCoreLimitByIndex);
      }
    }

    // For gem items with no implicit mods, resolve SupportText from GemEffects.
    if (implicitMods.length === 0 && baseIdx != null) {
      const gemRow = gemByBase.get(baseIdx);
      if (gemRow) {
        const gemEffectFks = rowArray(gemRow, 'GemEffects');
        const gemDesc = buildGemDescription(gemEffectFks, gemEffectByIndex);
        if (gemDesc) {
          implicitMods = [{ id: 'gem_support_text', text: gemDesc, domain: null, slot: null, target_item_classes: [] }];
        }
      }
    }

    // Sub-type data — all keyed by BaseItemTypes row index (base._index)
    const armour       = buildArmour(baseIdx != null ? armourByBase.get(baseIdx) : null);
    const weapon       = buildWeapon(baseIdx != null ? weaponByBase.get(baseIdx) : null);
    const shield       = buildShield(baseIdx != null ? shieldByBase.get(baseIdx) : null);
    const gem          = buildGem(baseIdx != null ? gemByBase.get(baseIdx) : null);
    const currency     = buildCurrency(baseIdx != null ? currencyByBase.get(baseIdx) : null);
    const flask        = buildFlask(baseIdx != null ? flaskByBase.get(baseIdx) : null, flaskTypeEnum);
    const essence      = buildEssence(
      baseIdx != null ? essenceByBase.get(baseIdx) : null,
      {
        essenceByIndex,
        essenceModsByEss,
        essenceCatByIndex,
        classByIndex: classById,
        baseItemByIndex,
        modDisplayByIndex,
      },
    );

    // Requirements — keyed by BaseItemTypes.Id string (metadata path)
    const requirements = buildRequirements(reqByBaseId.get(baseId));

    // Human-readable flavour text + tag list (both null/empty on base items
    // that don't set them).
    const flavourIdx  = intCol(base, 'FlavourText');
    const flavourRow  = flavourIdx != null ? flavourTextByIndex.get(flavourIdx) : null;
    const flavourText = strCol(flavourRow, 'Text');

    const tagFks = rowArray(base, 'Tags');
    const tags = tagFks
      .map((fk) => typeof fk === 'number' ? tagByIndex.get(fk) : null)
      .map((row) => strCol(row, 'DisplayString') ?? strCol(row, 'Id'))
      .filter(Boolean);

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
      flavour_text: flavourText,
      tags,
      is_corrupted: Boolean(col(base, 'IsCorrupted')),
      unmodifiable: Boolean(col(base, 'Unmodifiable')),
      requirements,
      defences: armour,
      weapon,
      shield,
      gem,
      currency,
      flask,
      soul_core: soulCoreInfo,
      essence,
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
    armour:         intCol(row, 'Armour') ?? 0,
    evasion:        intCol(row, 'Evasion') ?? 0,
    energy_shield:  intCol(row, 'EnergyShield') ?? 0,
    ward:           intCol(row, 'Ward') ?? 0,
    movement_speed: intCol(row, 'IncreasedMovementSpeed') ?? 0,
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
    reload_time:  intCol(row, 'ReloadTime') ?? 0, // ms; only meaningful on crossbows
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
    gem_type:        gemType   != null ? String(gemType)   : null,
    gem_colour:      gemColour != null ? String(gemColour) : null,
    gem_min_level:   1,
    gem_tier:        intCol(row, 'Tier') ?? null,
    str_req_percent: intCol(row, 'StrengthRequirementPercent') ?? 0,
    dex_req_percent: intCol(row, 'DexterityRequirementPercent') ?? 0,
    int_req_percent: intCol(row, 'IntelligenceRequirementPercent') ?? 0,
  };
}

function buildCurrency(row) {
  if (!row) return null;
  return {
    stack_size:  intCol(row, 'StackSize') ?? 1,
    description: strCol(row, 'Description') ?? null,
  };
}

function buildFlask(row, flaskTypeEnum) {
  if (!row) return null;
  return {
    flask_type:          flaskTypeEnum(intCol(row, 'Type')),   // LIFE | MANA | HYBRID | UTILITY | null
    flask_name:          strCol(row, 'Name') ?? null,
    flask_life:          intCol(row, 'LifePerUse') ?? 0,
    flask_mana:          intCol(row, 'ManaPerUse') ?? 0,
    flask_recovery_time: intCol(row, 'RecoveryTime') ?? 0,
  };
}

/**
 * Build a soul-core descriptor from the SoulCores row + resolved limit row.
 * Returns null for rune/soul-core bases that don't set either a required
 * level or a socket limit.
 */
/**
 * Build an essence descriptor: tier, Perfect flag, upgrade-to target (via
 * UpgradeResult → essence → BaseItemType.Name), and the list of per-item-
 * class guaranteed modifiers the essence grants.
 *
 * Returns null when the base item isn't an essence (no Essences row).
 */
function buildEssence(essenceRow, lookups) {
  if (!essenceRow) return null;

  const { essenceByIndex, essenceModsByEss, essenceCatByIndex, classByIndex,
          baseItemByIndex, modDisplayByIndex } = lookups;

  const essenceIdx = essenceRow._index;
  const tier       = intCol(essenceRow, 'Tier') ?? 0;
  const isPerfect  = Boolean(col(essenceRow, 'Perfect'));

  // UpgradeResult is a row FK back into Essences itself. Walk to the
  // upgraded essence's BaseItemType to surface the display name.
  const upgradeIdx  = intCol(essenceRow, 'UpgradeResult');
  const upgradeRow  = upgradeIdx != null ? essenceByIndex.get(upgradeIdx) : null;
  const upgradeBaseIdx = intCol(upgradeRow, 'BaseItemType');
  const upgradeBase = upgradeBaseIdx != null ? baseItemByIndex.get(upgradeBaseIdx) : null;
  const upgradeBaseId = strCol(upgradeBase, 'Id');
  const upgradeName  = strCol(upgradeBase, 'Name');

  // Resolve per-category modifiers. Prefer DisplayMod (human-facing
  // template) over Mod (internal) — the Text column is occasionally set to
  // an override that overrides both.
  const modifiers = [];
  for (const emod of essenceModsByEss.get(essenceIdx) ?? []) {
    const catIdx = intCol(emod, 'TargetItemCategory');
    const catRow = catIdx != null ? essenceCatByIndex.get(catIdx) : null;
    const categoryRaw = strCol(catRow, 'Text') ?? strCol(catRow, 'Id') ?? null;
    // Strip wiki markup like `[EquipArmour|Armour] or Belt` → `Armour or Belt`.
    const category = categoryRaw ? cleanGemMarkup(categoryRaw) : null;
    const classFks = rowArray(catRow, 'ItemClasses');
    const targetItemClasses = classFks
      .map((fk) => typeof fk === 'number' ? strCol(classByIndex.get(fk), 'Id') : null)
      .filter(Boolean);

    const overrideText = strCol(emod, 'Text');
    const displayIdx   = intCol(emod, 'DisplayMod');
    const modIdx       = intCol(emod, 'Mod');
    const displayMod   = displayIdx != null ? modDisplayByIndex.get(displayIdx) : null;
    const baseMod      = modIdx != null ? modDisplayByIndex.get(modIdx) : null;
    const modText      = overrideText || displayMod?.text || baseMod?.text;
    const modId        = displayMod?.id || baseMod?.id || '';

    if (!modText) continue;
    modifiers.push({
      target_category: category,
      target_item_classes: targetItemClasses,
      mod_id: modId,
      mod_text: modText,
    });
  }

  return {
    tier,
    is_perfect: isPerfect,
    upgrade_to_id:   upgradeBaseId ? `base/${upgradeBaseId}` : null,
    upgrade_to_name: upgradeName,
    modifiers,
  };
}

function buildSoulCoreInfo(soulCoreRow, soulCoreLimitByIndex) {
  const requiredLevel = intCol(soulCoreRow, 'RequiredLevel') ?? 0;
  const limitIdx = intCol(soulCoreRow, 'Limit');
  const limitRow = limitIdx != null ? soulCoreLimitByIndex.get(limitIdx) : null;
  const limitCount = intCol(limitRow, 'Limit');
  const limitTextRaw = strCol(limitRow, 'Text');
  // Templated strings like `{0} [Ancient|Ancient Augment]` aren't user-ready;
  // fall back to a generic "Only N per item" in the frontend when null.
  const limitText = limitTextRaw && !limitTextRaw.includes('{')
    ? cleanGemMarkup(limitTextRaw)
    : null;
  if (!requiredLevel && limitCount == null && !limitText) return null;
  return {
    required_level: requiredLevel,
    limit_count:    limitCount,
    limit_text:     limitText,
  };
}

// ---------------------------------------------------------------------------
// Mod text builder
// POE2 Mods: Stat1..6 are FK ints to Stats, Stat1Value..6Value are single values (no Min/Max range)
// ---------------------------------------------------------------------------

function buildModText(mod, statDescriptions, statByIndex) {
  const resolvedIds = [];
  const synthIds    = [];
  const values      = [];

  for (let i = 1; i <= 6; i++) {
    const statKeyIdx = intCol(mod, `Stat${i}`);
    if (statKeyIdx == null) continue;

    const value = intCol(mod, `Stat${i}Value`) ?? 0;
    const statRow = statByIndex ? statByIndex.get(statKeyIdx) : null;
    const realId  = strCol(statRow, 'Id');

    synthIds.push(`stat_${statKeyIdx}`);        // for formatStatDisplay keying
    resolvedIds.push(realId ?? `stat_${statKeyIdx}`);
    values.push(value);
  }

  if (synthIds.length === 0) {
    return strCol(mod, 'Name') ?? null;
  }

  // Prefer real stat-description formatting when stat_descriptions.txt is
  // loaded (POE1-style extraction). POE2 bundles don't expose those files,
  // so we synthesize display text from the resolved stat IDs + values —
  // same path the soul-core implicits use.
  const text = formatStatDisplay(statDescriptions, synthIds, values, values);
  if (text) return text;

  const synthesized = formatSoulCoreStatLine(resolvedIds, values);
  return synthesized ?? strCol(mod, 'Name') ?? null;
}

// ---------------------------------------------------------------------------
// SoulCore stat description builder
// Stat description txt files are not accessible via the bundle system in POE2,
// so we derive approximate human-readable text directly from stat IDs + values.
// ---------------------------------------------------------------------------

/**
 * Build implicit-mod-like entries from SoulCoreStats rows.
 * Each row holds parallel Stats[] (FK int array) and StatsValues[] (int array),
 * plus a StatCategory FK that identifies which item classes the stats apply to
 * when the rune/soul-core is socketed, plus optional BondedStats[] that kick
 * in when two runes from different item classes are bonded.
 */
function buildSoulCoreMods(scStatsRows, statByIndex, classByIndex, statCatByIndex) {
  const mods = [];

  for (const row of scStatsRows) {
    const statFks  = rowArray(row, 'Stats');
    const values   = rowArray(row, 'StatsValues');

    // Resolve the stat-category context: the Display label + the list of
    // ItemClasses.Id strings this row's stats apply to. Null for older data
    // without the join. The Display string occasionally contains wiki markup
    // (e.g. `[MartialWeapon|Martial Weapon]`) — strip it for user display.
    const catIdx = intCol(row, 'StatCategory');
    const catRow = catIdx != null ? statCatByIndex.get(catIdx) : null;
    const slotRaw = strCol(catRow, 'Display');
    const targetClassFks = rowArray(catRow, 'TargetItemClasses');
    const targetItemClasses = targetClassFks
      .map((fk) => typeof fk === 'number' ? strCol(classByIndex.get(fk), 'Id') : null)
      .filter(Boolean);
    // Prefer the designer-authored Display label; fall back to the list of
    // target classes (e.g. "Helmet") when Display is empty.
    const slot = slotRaw
      ? cleanGemMarkup(slotRaw)
      : (targetItemClasses.length ? targetItemClasses.join(', ') : null);

    if (statFks.length > 0) {
      const statIds = statFks
        .map((fk) => (typeof fk === 'number' ? strCol(statByIndex.get(fk), 'Id') : null))
        .filter(Boolean);

      if (statIds.length > 0) {
        const text = formatSoulCoreStatLine(statIds, values);
        if (text) {
          mods.push({
            id: statIds.join(','),
            text,
            domain: null,
            slot,
            target_item_classes: targetItemClasses,
          });
        }
      }
    }

    // Bonded stats — extra mods that apply only when paired with a second rune
    // from a different slot. Tag the display text so the tooltip can call it
    // out; same slot/class context as the primary stats for this row.
    const bondedFks    = rowArray(row, 'BondedStats');
    const bondedValues = rowArray(row, 'BondedStatsValues');
    if (bondedFks.length > 0) {
      const bondedStatIds = bondedFks
        .map((fk) => (typeof fk === 'number' ? strCol(statByIndex.get(fk), 'Id') : null))
        .filter(Boolean);

      if (bondedStatIds.length > 0) {
        const bondedText = formatSoulCoreStatLine(bondedStatIds, bondedValues);
        if (bondedText) {
          mods.push({
            id: `bonded:${bondedStatIds.join(',')}`,
            text: bondedText,
            domain: 'BONDED',
            slot,
            target_item_classes: targetItemClasses,
          });
        }
      }
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
 *   additional_strength, 6                                  → "+6 to Strength"
 *   base_fire_damage_resistance_%, 12                       → "+12% Fire Damage Resistance"
 *   attack_speed_+%, 15                                     → "+15% Attack Speed"
 *   base_maximum_life, 50                                   → "+50 Maximum Life"
 *   energy_generated_+%, 10                                 → "+10% Energy Generated"
 *   non_skill_base_all_damage_%_to_gain_as_fire, 8          → "+8% of Damage Gained as Fire"
 *   enemies_damage_taken_+%_while_cursed, 6                 → "Enemies Damage Taken +6% While Cursed"
 *   local_requirements_%_to_convert_to_strength, 20         → "Requirements +20% To Convert To Strength"
 *   stun_threshold_+, 60                                    → "+60 Stun Threshold"
 */
export function formatSingleSoulCoreStat(id, value) {
  const sign = value >= 0 ? '+' : '';

  // "damage_%_to_gain_as_X" pattern (e.g. non_skill_base_all_damage_%_to_gain_as_fire)
  const gainAs = id.match(/damage_%_to_gain_as_(\w+)$/);
  if (gainAs) {
    const element = humanizeStat(gainAs[1]);
    return `${sign}${value}% of Damage Gained as ${element}`;
  }

  // "additional_X" → "+V to X"
  if (id.startsWith('additional_')) {
    const attr = humanizeStat(id.slice('additional_'.length));
    return `${sign}${value} to ${attr}`;
  }

  const stripped = id.replace(/^(non_skill_base_|base_|local_|global_)/, '');

  // Find the value placeholder: `_+`, `_%`, `_+%`, `_%+` at an underscore
  // boundary (or end of string). POE stat IDs often put this marker mid-ID
  // (e.g. `enemies_damage_taken_+%_while_cursed`) rather than at the end;
  // the formatted value must slot into that position, not at the start.
  const markerRe = /_([+%]+)(?=_|$)/;
  const m = stripped.match(markerRe);

  if (m) {
    const pctStr = m[1].includes('%') ? '%' : '';
    const valueText = `${sign}${value}${pctStr}`;
    const before = humanizeStat(stripped.slice(0, m.index));
    const tailRaw = stripped.slice(m.index + m[0].length).replace(/^_+/, '');

    if (!tailRaw) {
      return `${valueText} ${before}`.trim();
    }

    // A tail with another orphan `_+`/`_%` marker means a baked-in constant
    // we don't have a value for (e.g. `stun_threshold_+_from_%_maximum_es`).
    // Drop the orphan marker so it doesn't appear as a stray "%" in output.
    const tail = humanizeStat(tailRaw.replace(/_[+%]+(?=_|$)/g, ''));
    return `${before} ${valueText} ${tail}`.trim();
  }

  return `${sign}${value} ${humanizeStat(stripped)}`;
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
//
// High-level buckets used by the UI and for filtering. The source of truth
// for category membership is the explicit per-class map below. A regex
// fallback only catches unexpected weapon-class variants (POE has many of
// the form "Thrown One Hand Axe", "Two Hand Sword", etc.) so new classes
// don't silently end up as "Other".
// ---------------------------------------------------------------------------

const CATEGORY_BY_CLASS_ID = {
  // Accessories
  Amulet: 'Accessory',
  Ring: 'Accessory',
  Belt: 'Accessory',
  Talisman: 'Accessory',
  Quiver: 'Accessory',
  Trinket: 'Accessory',

  // Armour (including offhands that are defensive)
  'Body Armour': 'Armour',
  Helmet: 'Armour',
  Gloves: 'Armour',
  Boots: 'Armour',
  Shield: 'Armour',
  Buckler: 'Armour',
  Focus: 'Armour',

  // Weapons — explicit entries for classes whose ID doesn't obviously
  // read as a weapon (the regex fallback below catches "Sword", "Axe",
  // "Mace", "Bow", "Staff", "Warstaff", "Wand", "Dagger", "Claw",
  // "Spear", "Flail", "Crossbow").
  Sceptre: 'Weapon',
  FishingRod: 'Weapon',

  // Flasks
  LifeFlask: 'Flask',
  ManaFlask: 'Flask',
  UtilityFlask: 'Flask',

  // Gems
  'Active Skill Gem': 'Gem',
  'Support Skill Gem': 'Gem',
  'Meta Skill Gem': 'Gem',
  UncutSkillGemStackable: 'Gem',
  UncutSupportGemStackable: 'Gem',
  UncutReservationGemStackable: 'Gem',
  UncutSkillGem_OLD: 'Gem',
  UncutSupportGem_OLD: 'Gem',
  UncutReservationGem_OLD: 'Gem',

  // Currency and currency-adjacent consumables
  Currency: 'Currency',
  StackableCurrency: 'Currency',
  DelveSocketableCurrency: 'Currency',
  DelveStackableSocketableCurrency: 'Currency',
  SkillGemToken: 'Currency',
  Omen: 'Currency',
  Incubator: 'Currency',
  IncubatorStackable: 'Currency',
  SoulCore: 'Currency',
  ArchnemesisMod: 'Currency',
  DivinationCard: 'Currency',

  // Jewels
  Jewel: 'Jewel',
  AbyssJewel: 'Jewel',

  // Maps, fragments, keys, tablets, logbooks — anything consumed to open
  // or modify endgame content.
  Map: 'Map',
  MapFragment: 'Map',
  MiscMapItem: 'Map',
  UniqueFragment: 'Map',
  VaultKey: 'Map',
  Breachstone: 'Map',
  PinnacleKey: 'Map',
  UltimatumKey: 'Map',
  MemoryLine: 'Map',
  ItemisedSanctum: 'Map',
  TowerAugmentation: 'Map',
  AtlasUpgradeItem: 'Map',
  ExpeditionLogbook: 'Map',

  // Sanctum relics
  Relic: 'Relic',
  SanctumSpecialRelic: 'Relic',
  SmallRelic: 'Relic',
  MediumRelic: 'Relic',
  LargeRelic: 'Relic',

  // Quest
  QuestItem: 'Quest',

  // Heist (POE1 mechanic still present in data tables; bucket separately
  // so it doesn't pollute the main currency/map categories).
  HeistObjective: 'Heist',
  HeistContract: 'Heist',
  HeistBlueprint: 'Heist',
  HeistEquipmentWeapon: 'Heist',
  HeistEquipmentTool: 'Heist',
  HeistEquipmentUtility: 'Heist',
  HeistEquipmentReward: 'Heist',
};

export function deriveCategory(classId) {
  if (!classId) return 'Other';

  const explicit = CATEGORY_BY_CLASS_ID[classId];
  if (explicit) return explicit;

  // Fallback: catch weapon-class variants not listed explicitly.
  const id = classId.toLowerCase();
  if (/sword|axe|mace|bow|staff|wand|dagger|claw|spear|flail|crossbow/.test(id)) {
    return 'Weapon';
  }

  return 'Other';
}
