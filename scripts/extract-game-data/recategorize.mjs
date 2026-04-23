#!/usr/bin/env node
/**
 * recategorize.mjs — rewrite the `category` field of every record in
 * packages/backend/data/game_data/items.json using the current
 * deriveCategory() logic in lib/table-joiner.mjs.
 *
 * Use this after tweaking category buckets so you don't have to re-run the
 * full CDN extraction. Also bumps version.json.extracted_at so the Rust
 * backend will re-import on next launch.
 *
 *   node scripts/extract-game-data/recategorize.mjs
 */

import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { deriveCategory } from './lib/table-joiner.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', '..');
const DATA_DIR = resolve(ROOT, 'packages', 'backend', 'data', 'game_data');

const itemsPath = resolve(DATA_DIR, 'items.json');
const versionPath = resolve(DATA_DIR, 'version.json');

const items = JSON.parse(await readFile(itemsPath, 'utf8'));

const before = new Map();
const after = new Map();
const moved = new Map();

for (const item of items) {
  const prev = item.category;
  const next = deriveCategory(item.item_class_id);
  before.set(prev, (before.get(prev) ?? 0) + 1);
  after.set(next, (after.get(next) ?? 0) + 1);
  if (prev !== next) {
    const key = `${prev} -> ${next}`;
    moved.set(key, (moved.get(key) ?? 0) + 1);
  }
  item.category = next;
}

await writeFile(itemsPath, JSON.stringify(items, null, 2), 'utf8');

const version = JSON.parse(await readFile(versionPath, 'utf8'));
version.extracted_at = new Date().toISOString();
await writeFile(versionPath, JSON.stringify(version, null, 2), 'utf8');

const fmt = (m) => [...m.entries()].sort((a, b) => b[1] - a[1]).map(([k, v]) => `  ${String(k).padEnd(28)} ${v}`).join('\n');

console.log('Before:');
console.log(fmt(before));
console.log('\nAfter:');
console.log(fmt(after));
console.log('\nMoved:');
console.log(fmt(moved));
console.log(`\nUpdated ${items.length} items; bumped extracted_at → ${version.extracted_at}`);
