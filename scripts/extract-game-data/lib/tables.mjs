/**
 * Shared table/column definitions and schema helpers for the POE2 data
 * extractors (extract.mjs, extract-local.mjs). Kept in one place so
 * adding a column doesn't require editing two files.
 *
 * POE2 schema notes:
 * - Foreign-key columns export as plain integer row indices (_index), not
 *   row objects. Extra columns we don't extract simply get dropped by the
 *   schema header filter.
 * - enumrow columns are exported as integer values indexing into the
 *   corresponding enum in schema.enumerations[].enumerators (1-based if
 *   enum.indexing === 1).
 */

export const TABLES = [
  {
    name: 'BaseItemTypes',
    columns: [
      'Id', 'Name', 'ItemClass', 'DropLevel', 'Width', 'Height',
      'Implicit_Mods', 'ItemVisualIdentity',
      'FlavourText', 'Tags', 'IsCorrupted', 'Unmodifiable',
    ],
  },
  { name: 'ItemClasses',        columns: ['Id', 'Name'] },
  { name: 'ItemVisualIdentity', columns: ['Id', 'DDSFile'] },
  {
    name: 'ArmourTypes',
    columns: ['BaseItemType', 'Armour', 'Evasion', 'EnergyShield', 'Ward', 'IncreasedMovementSpeed'],
  },
  {
    name: 'WeaponTypes',
    columns: ['BaseItemType', 'DamageMin', 'DamageMax', 'Critical', 'Speed', 'RangeMax', 'ReloadTime'],
  },
  { name: 'ShieldTypes', columns: ['BaseItemType', 'Block'] },
  {
    name: 'ComponentAttributeRequirements',
    columns: ['BaseItemTypesKey', 'ReqStr', 'ReqDex', 'ReqInt'],
  },
  {
    name: 'Mods',
    columns: [
      'Id', 'Name', 'GenerationType', 'Domain',
      'Stat1', 'Stat2', 'Stat3', 'Stat4', 'Stat5', 'Stat6',
      'Stat1Value', 'Stat2Value', 'Stat3Value', 'Stat4Value', 'Stat5Value', 'Stat6Value',
    ],
  },
  { name: 'Stats', columns: ['Id', 'IsLocal', 'IsWeaponLocal'] },
  {
    name: 'SkillGems',
    columns: [
      'BaseItemType', 'Tier', 'GemType', 'GemColour', 'GemEffects',
      'StrengthRequirementPercent', 'DexterityRequirementPercent', 'IntelligenceRequirementPercent',
    ],
  },
  { name: 'GemEffects', columns: ['Id', 'SupportText', 'SupportName'] },
  { name: 'CurrencyItems', columns: ['BaseItemType', 'StackSize', 'Description'] },
  {
    name: 'Flasks',
    columns: ['BaseItemType', 'Name', 'Type', 'LifePerUse', 'ManaPerUse', 'RecoveryTime'],
  },
  // SoulCores (runes, soul cores, idols): stats come from SoulCoreStats, not BaseItemTypes.Implicit_Mods
  { name: 'SoulCores', columns: ['BaseItemType', 'Type', 'RequiredLevel', 'Limit'] },
  {
    name: 'SoulCoreStats',
    columns: ['SoulCore', 'Stats', 'StatsValues', 'StatCategory', 'BondedStats', 'BondedStatsValues'],
  },
  // Each StatCategory tells us which item classes (weapon, body armour, etc.)
  // the stats in that row apply to when the rune/soul core is socketed.
  { name: 'SoulCoreStatCategories', columns: ['Id', 'TargetItemClasses', 'Display'] },
  { name: 'SoulCoreLimits',         columns: ['Id', 'Limit', 'Text'] },
  // Human-readable lookups: BaseItemTypes.FlavourText → FlavourText.Text,
  //                         BaseItemTypes.Tags → Tags.DisplayString
  { name: 'FlavourText', columns: ['Id', 'Text'] },
  { name: 'Tags',        columns: ['Id', 'DisplayString'] },
  // Essences: Essences.BaseItemType → BaseItemTypes; the per-item-class
  // guaranteed-modifier rows live in EssenceMods (Essence FK + Mod FK).
  { name: 'Essences',                    columns: ['BaseItemType', 'Perfect', 'UpgradeResult', 'Tier'] },
  { name: 'EssenceMods',                 columns: ['Essence', 'TargetItemCategory', 'Mod', 'DisplayMod', 'Text'] },
  { name: 'EssenceTargetItemCategories', columns: ['Id', 'ItemClasses', 'Text'] },
  // UniqueStashLayout has no BaseItemTypes FK in POE2 — unique linking skipped for now
  { name: 'UniqueStashLayout', columns: ['WordsKey', 'ItemVisualIdentityKey'] },
  { name: 'Words',             columns: ['Text'] },
];

export const STAT_DESC_FILES = [
  'Metadata/StatDescriptions/stat_descriptions.txt',
  'Metadata/StatDescriptions/skill_stat_descriptions.txt',
  'Metadata/StatDescriptions/gem_stat_descriptions.txt',
  'Metadata/StatDescriptions/passive_skill_stat_descriptions.txt',
  'Metadata/StatDescriptions/advanced_mod_stat_descriptions.txt',
];

/**
 * Build the `getHeaders(tableName, datFile, columnFilter?)` function bound to
 * a specific schema + runtime dependencies. Kept here so both extract.mjs and
 * extract-local.mjs share the same logic without copy-pasting.
 *
 * @param {object} schema       Parsed schema JSON from pathofexile-dat-schema
 * @param {object} ValidFor     ValidFor flags (from pathofexile-dat-schema)
 * @param {Function} getHeaderLength  From pathofexile-dat dist/dat/header.js
 */
export function buildGetHeaders(schema, ValidFor, getHeaderLength) {
  return function getHeaders(tableName, datFile, columnFilter) {
    const foundByName = schema.tables.filter((s) => s.name === tableName);
    const sch =
      foundByName.find((s) => s.validFor & ValidFor.PoE2) ?? foundByName.at(0);
    if (!sch) throw new Error(`No schema found for table "${tableName}"`);

    let offset = 0;
    const headers = [];
    for (const column of sch.columns) {
      const h = {
        name: column.name ?? '',
        offset,
        type: {
          array: column.array,
          interval: column.interval,
          integer:
            column.type === 'u16' ? { unsigned: true,  size: 2 }
            : column.type === 'u32' ? { unsigned: true,  size: 4 }
            : column.type === 'i16' ? { unsigned: false, size: 2 }
            : column.type === 'i32' ? { unsigned: false, size: 4 }
            : column.type === 'enumrow' ? { unsigned: false, size: 4 }
            : undefined,
          decimal: column.type === 'f32' ? { size: 4 } : undefined,
          string: column.type === 'string' ? {} : undefined,
          boolean: column.type === 'bool' ? {} : undefined,
          key: column.type === 'row' || column.type === 'foreignrow'
            ? { foreign: column.type === 'foreignrow' }
            : undefined,
        },
      };
      headers.push(h);
      offset += getHeaderLength(h, datFile);
    }

    return columnFilter
      ? headers.filter((h) => !h.name || columnFilter.includes(h.name))
      : headers;
  };
}

/**
 * Build an `{ enumName: (index) => string | null }` map from the schema's
 * `enumerations` list. Respects `indexing` (0- or 1-based). Missing/null
 * positions return null so callers can distinguish "unknown index" from
 * "intentionally blank slot".
 */
export function buildEnumLookups(enumerations, names) {
  const out = {};
  for (const name of names) {
    const e = enumerations.find((en) => en.name === name);
    if (!e) continue;
    const offset = e.indexing ?? 0;
    const values = e.enumerators ?? [];
    out[name] = (idx) => {
      if (idx == null) return null;
      const n = Number(idx);
      if (!Number.isFinite(n)) return null;
      return values[n - offset] ?? null;
    };
  }
  return out;
}
