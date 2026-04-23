#!/usr/bin/env node
/**
 * refix-soul-core-mods.mjs — re-format the display text of every soul-core
 * implicit mod in packages/backend/data/game_data/items.json, using the
 * current formatSingleSoulCoreStat() logic in lib/table-joiner.mjs.
 *
 * The stat IDs are preserved in each mod's `id` field (single ID, or
 * comma-joined for multi-stat rows). Numeric values are recovered by parsing
 * them from the existing generated text (the leading signed integer on each
 * line). Min/max damage pair mods ("Adds X to Y Z Damage") are left alone —
 * their formatting is already correct.
 *
 * Bumps version.json.extracted_at so the Rust backend will re-import.
 *
 *   node scripts/extract-game-data/refix-soul-core-mods.mjs
 */

import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { formatSingleSoulCoreStat } from './lib/table-joiner.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', '..');
const DATA_DIR = resolve(ROOT, 'packages', 'backend', 'data', 'game_data');

const itemsPath = resolve(DATA_DIR, 'items.json');
const versionPath = resolve(DATA_DIR, 'version.json');

const items = JSON.parse(await readFile(itemsPath, 'utf8'));

/** Parse the leading signed integer from a single line of mod text. */
function parseLeadingValue(line) {
  const m = line.match(/^([+-]?\d+)/);
  return m ? parseInt(m[1], 10) : null;
}

let reformatted = 0;
let skippedPair = 0;
let skippedUnparsable = 0;
const changes = [];

for (const item of items) {
  if (item.item_class_id !== 'SoulCore') continue;

  for (const mod of item.implicit_mods) {
    const statIds = mod.id.split(',');
    const lines = mod.text.split('\n');

    if (statIds.length === 1) {
      const value = parseLeadingValue(mod.text);
      if (value == null) { skippedUnparsable++; continue; }
      const next = formatSingleSoulCoreStat(statIds[0], value);
      if (next && next !== mod.text) {
        changes.push([mod.text, next]);
        mod.text = next;
        reformatted++;
      }
      continue;
    }

    // Min/max damage pair — already handled correctly by formatSoulCoreStatLine.
    const isMinMaxPair =
      statIds.length === 2 &&
      /minimum(?:_added|_base)?_\w+_damage$/.test(statIds[0]) &&
      /maximum(?:_added|_base)?_\w+_damage$/.test(statIds[1]);
    if (isMinMaxPair) { skippedPair++; continue; }

    // Multi-stat, non-pair row → each line is one stat.
    if (statIds.length !== lines.length) { skippedUnparsable++; continue; }

    let anyChange = false;
    const nextLines = lines.map((line, i) => {
      const value = parseLeadingValue(line);
      if (value == null) return line;
      const next = formatSingleSoulCoreStat(statIds[i], value);
      if (next && next !== line) {
        anyChange = true;
        return next;
      }
      return line;
    });

    if (anyChange) {
      const nextText = nextLines.join('\n');
      changes.push([mod.text, nextText]);
      mod.text = nextText;
      reformatted++;
    }
  }
}

await writeFile(itemsPath, JSON.stringify(items, null, 2), 'utf8');

const version = JSON.parse(await readFile(versionPath, 'utf8'));
version.extracted_at = new Date().toISOString();
await writeFile(versionPath, JSON.stringify(version, null, 2), 'utf8');

console.log(`Reformatted ${reformatted} soul-core mod texts.`);
console.log(`Skipped ${skippedPair} min/max damage pairs (already correct).`);
if (skippedUnparsable) console.log(`Skipped ${skippedUnparsable} unparsable entries.`);
console.log(`\nSample changes (first 10 unique):`);
const seen = new Set();
for (const [before, after] of changes) {
  const key = `${before}||${after}`;
  if (seen.has(key)) continue;
  seen.add(key);
  if (seen.size > 10) break;
  console.log(`  - ${JSON.stringify(before)}`);
  console.log(`  + ${JSON.stringify(after)}`);
}
console.log(`\nBumped extracted_at → ${version.extracted_at}`);
