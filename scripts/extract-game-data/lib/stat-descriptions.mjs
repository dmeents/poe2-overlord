/**
 * Parser for POE2's stat_descriptions.txt (and its skill/gem/passive variants).
 *
 * File format (per block):
 *
 *   description stat1_id [stat2_id ...]
 *   lang "English"
 *       <N>                         ← number of rules
 *       <cond1> [cond2 ...] "text {0} … {N-1}" [transform1 transform2 ...]
 *       …                           ← repeated N times
 *
 * Conditions (one per stat ID):
 *   #           – always matches; display the raw integer value
 *   !0          – matches when value ≠ 0; same display
 *   >0          – matches when value > 0
 *   <0          – matches when value < 0
 *   N|#         – matches when value ≥ N; display raw
 *   N           – matches only the exact integer N
 *
 * Transforms (applied to the matched value before substitution):
 *   negate                      → multiply by -1
 *   divide_by_one_hundred       → divide by 100
 *   divide_by_one_hundred_2dp   → divide by 100, 2 decimal places
 *   milliseconds_to_seconds     → divide by 1 000
 *   per_minute_to_per_second    → divide by 60
 *   (unknown transforms are ignored; raw value used)
 *
 * Template placeholders: {0}, {1}, … correspond to each stat in order.
 */

// ---------------------------------------------------------------------------
// Types (JSDoc only – no TypeScript)
// ---------------------------------------------------------------------------

/**
 * @typedef {{ conditions: string[], template: string, transforms: string[] }} StatRule
 * @typedef {{ statIds: string[], rules: StatRule[] }} StatDescription
 */

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Parse the raw text of stat_descriptions.txt.
 *
 * @param {string} text
 * @returns {Map<string, StatDescription>}   Keyed by "stat1,stat2,…" join of stat IDs.
 *                                           Single-stat descriptions are also indexed by just that ID.
 */
export function parseStatDescriptions(text) {
  /** @type {Map<string, StatDescription>} */
  const descriptions = new Map();

  const lines = text.split(/\r?\n/);
  let i = 0;

  while (i < lines.length) {
    const line = lines[i].trim();
    i++;

    if (!line.startsWith('description ')) continue;

    const statIds = line.slice('description '.length).trim().split(/\s+/).filter(Boolean);
    if (statIds.length === 0) continue;

    // Advance to "lang "English"" block; skip other lang blocks
    let found = false;
    while (i < lines.length) {
      const langLine = lines[i].trim();
      i++;
      if (langLine === 'lang "English"') { found = true; break; }
      // If we hit another description block, back up and let the outer loop handle it
      if (langLine.startsWith('description ')) { i--; break; }
    }
    if (!found) continue;

    // Next non-blank line is the rule count
    while (i < lines.length && lines[i].trim() === '') i++;
    if (i >= lines.length) continue;

    const countLine = lines[i].trim();
    i++;
    const count = parseInt(countLine, 10);
    if (isNaN(count) || count <= 0) continue;

    /** @type {StatRule[]} */
    const rules = [];

    for (let r = 0; r < count; r++) {
      while (i < lines.length && lines[i].trim() === '') i++;
      if (i >= lines.length) break;

      const ruleLine = lines[i].trim();
      i++;

      const rule = parseRule(ruleLine, statIds.length);
      if (rule) rules.push(rule);
    }

    const desc = { statIds, rules };
    const key = statIds.join(',');
    descriptions.set(key, desc);

    // Also index single-stat descriptions by just the stat ID for fast lookup
    if (statIds.length === 1) {
      descriptions.set(statIds[0], desc);
    }
  }

  return descriptions;
}

/**
 * Merge multiple parsed stat_descriptions files into one map.
 * Later entries (e.g. skill_stat_descriptions) override earlier ones if keys collide.
 *
 * @param  {...Map<string, StatDescription>} maps
 * @returns {Map<string, StatDescription>}
 */
export function mergeDescriptions(...maps) {
  const merged = new Map();
  for (const m of maps) {
    for (const [k, v] of m) {
      merged.set(k, v);
    }
  }
  return merged;
}

/**
 * Format a set of stat IDs + value ranges into human-readable display text.
 *
 * @param {Map<string, StatDescription>} descriptions
 * @param {string[]} statIds
 * @param {number[]} minValues
 * @param {number[]} maxValues
 * @returns {string | null}   null if no matching description found
 */
export function formatStatDisplay(descriptions, statIds, minValues, maxValues) {
  // Try multi-stat key first, then fall back to individual single-stat lookups
  const keys = [statIds.join(','), ...statIds];

  for (const key of keys) {
    const desc = descriptions.get(key);
    if (!desc) continue;

    const rule = findMatchingRule(desc.rules, minValues, maxValues);
    if (!rule) continue;

    return substituteValues(rule.template, rule.transforms, minValues, maxValues);
  }

  return null;
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/**
 * @param {string} line
 * @param {number} numStats
 * @returns {StatRule | null}
 */
function parseRule(line, numStats) {
  // Extract the quoted template (may contain escaped quotes – rare in practice)
  const templateMatch = line.match(/"((?:[^"\\]|\\.)*)"/);
  if (!templateMatch) return null;

  const template = templateMatch[1];
  const beforeTemplate = line.slice(0, templateMatch.index).trim();
  const afterTemplate = line.slice(templateMatch.index + templateMatch[0].length).trim();

  const condParts = beforeTemplate.split(/\s+/).filter(Boolean);
  // Take exactly numStats conditions; extras are transforms that sneak in before the string
  const conditions = condParts.slice(0, numStats);
  const transforms = afterTemplate.split(/\s+/).filter(Boolean);

  return { conditions, template, transforms };
}

/**
 * Pick the first rule whose conditions all match the provided min values.
 * We use minValues for condition testing (representative value for range display).
 *
 * @param {StatRule[]} rules
 * @param {number[]} minValues
 * @param {number[]} maxValues
 * @returns {StatRule | null}
 */
function findMatchingRule(rules, minValues, maxValues) {
  for (const rule of rules) {
    if (ruleMatches(rule, minValues, maxValues)) return rule;
  }
  return null;
}

/**
 * @param {StatRule} rule
 * @param {number[]} minValues
 * @param {number[]} maxValues
 * @returns {boolean}
 */
function ruleMatches(rule, minValues, maxValues) {
  const values = minValues.length > 0 ? minValues : maxValues;

  for (let i = 0; i < rule.conditions.length; i++) {
    const cond = rule.conditions[i];
    const val = values[i] ?? 0;

    if (!conditionMatches(cond, val)) return false;
  }
  return true;
}

/**
 * @param {string} condition
 * @param {number} value
 * @returns {boolean}
 */
function conditionMatches(condition, value) {
  if (condition === '#') return true;
  if (condition === '!0') return value !== 0;
  if (condition === '>0') return value > 0;
  if (condition === '<0') return value < 0;

  // "N|#" — value must be >= N (N is lower bound)
  const rangeMatch = condition.match(/^(-?\d+)\|#$/);
  if (rangeMatch) {
    return value >= parseInt(rangeMatch[1], 10);
  }

  // Exact integer match
  const exact = parseInt(condition, 10);
  if (!isNaN(exact)) return value === exact;

  // Unknown condition — assume matches
  return true;
}

/**
 * Apply transforms to values and substitute into the template.
 *
 * @param {string} template
 * @param {string[]} transforms
 * @param {number[]} minValues
 * @param {number[]} maxValues
 * @returns {string}
 */
function substituteValues(template, transforms, minValues, maxValues) {
  let result = template;
  const count = Math.max(minValues.length, maxValues.length);

  for (let i = 0; i < count; i++) {
    let min = minValues[i] ?? 0;
    let max = maxValues[i] ?? 0;
    const transform = transforms[i] ?? '';

    [min, max] = applyTransform(transform, min, max);

    // Format display value: show range if min ≠ max, otherwise just the value
    const display = formatValue(min, max, transform);

    // Replace positional placeholder {i}; also handle bare # for single-stat descs
    result = result.replace(new RegExp(`\\{${i}\\}`, 'g'), display);
    if (i === 0) {
      result = result.replace(/#/, display);
    }
  }

  return result;
}

/**
 * @param {string} transform
 * @param {number} min
 * @param {number} max
 * @returns {[number, number]}
 */
function applyTransform(transform, min, max) {
  switch (transform) {
    case 'negate': {
      const newMin = -max;
      const newMax = -min;
      return [newMin, newMax];
    }
    case 'divide_by_one_hundred':
    case 'divide_by_one_hundred_2dp':
      return [min / 100, max / 100];
    case 'milliseconds_to_seconds':
      return [min / 1000, max / 1000];
    case 'per_minute_to_per_second':
      return [min / 60, max / 60];
    default:
      return [min, max];
  }
}

/**
 * @param {number} min
 * @param {number} max
 * @param {string} transform
 * @returns {string}
 */
function formatValue(min, max, transform) {
  const decimals =
    transform === 'divide_by_one_hundred_2dp' ? 2
    : transform === 'divide_by_one_hundred' ? (Number.isInteger(min) && Number.isInteger(max) ? 0 : 1)
    : transform === 'milliseconds_to_seconds' || transform === 'per_minute_to_per_second' ? 2
    : 0;

  const fmt = (n) => decimals > 0 ? n.toFixed(decimals) : String(Math.round(n));

  if (fmt(min) === fmt(max)) return fmt(min);
  return `(${fmt(min)}-${fmt(max)})`;
}
