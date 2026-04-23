#!/usr/bin/env node
/**
 * extract-local.mjs — Extract POE2 game data from a local Steam installation
 *
 * Reads directly from your POE2 Bundles2/ directory — no CDN access needed.
 * Use this for local development. For CI/CD, commit the output JSON files to
 * the repo and the build uses whatever is committed.
 *
 * Usage:
 *   node extract-local.mjs --version 0.4.0i
 *
 * Or from repo root:
 *   pnpm extract:gamedata:local -- --version 0.4.0i
 *
 * The --version flag is a label written to version.json for tracking purposes.
 * Use the in-game display version (lower-left corner of the game screen).
 *
 * Default game directory (Linux/Steam):
 *   ~/.local/share/Steam/steamapps/common/Path of Exile 2
 *
 * Override with --game-dir if your installation is elsewhere:
 *   node extract-local.mjs --version 0.4.0i --game-dir "/path/to/Path of Exile 2"
 *
 * Output files (relative to repo root):
 *   packages/backend/data/game_data/version.json
 *   packages/backend/data/game_data/items.json
 *   packages/backend/data/game_data/item_categories.json
 */

import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { homedir } from 'node:os';
import minimist from 'minimist';

import { parseStatDescriptions, mergeDescriptions } from './lib/stat-descriptions.mjs';
import { joinTables } from './lib/table-joiner.mjs';
import { TABLES, STAT_DESC_FILES, buildEnumLookups } from './lib/tables.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', '..');
const OUTPUT_DIR = resolve(ROOT, 'packages', 'backend', 'data', 'game_data');

const DEFAULT_GAME_DIR = resolve(
  homedir(),
  '.local/share/Steam/steamapps/common/Path of Exile 2',
);

// ---------------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------------

const argv = minimist(process.argv.slice(2), {
  string: ['version', 'game-dir'],
  boolean: ['help', 'dry-run'],
  alias: { h: 'help', v: 'version', g: 'game-dir', n: 'dry-run' },
});

if (argv.help) {
  console.log(`
Usage:
  node extract-local.mjs --version 0.4.0i [--game-dir "/path/to/poe2"]

Options:
  --version, -v     In-game display version label (e.g. 0.4.0i). Written to
                    version.json for tracking. Find it in the bottom-left of
                    the game screen.
  --game-dir, -g    Path to your POE2 installation (the folder containing
                    Bundles2/). Defaults to the standard Linux/Steam path:
                    ~/.local/share/Steam/steamapps/common/Path of Exile 2
  --dry-run, -n     Extract and join data but do not write output files.
  --help, -h        Show this help.
`);
  process.exit(0);
}

const versionLabel = argv['version'] ?? 'local';
const gameDir = argv['game-dir'] ?? DEFAULT_GAME_DIR;

// Table/column definitions + stat-description file list live in lib/tables.mjs
// so extract.mjs and extract-local.mjs stay in sync when new fields are added.

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  console.log('POE2 Game Data Extractor (local)');
  console.log(`Game dir:  ${gameDir}`);
  console.log(`Version:   ${versionLabel}`);
  console.log('');

  // ------------------------------------------------------------------
  // 1. Import pathofexile-dat internals
  // ------------------------------------------------------------------
  const { SteamBundleLoader, FileLoader } = await import(
    './node_modules/pathofexile-dat/dist/cli/bundle-loaders.js'
  );
  const { exportAllRows } = await import(
    './node_modules/pathofexile-dat/dist/cli/export-tables.js'
  );
  const { readDatFile } = await import(
    './node_modules/pathofexile-dat/dist/dat/dat-file.js'
  );
  const { getHeaderLength } = await import(
    './node_modules/pathofexile-dat/dist/dat/header.js'
  );
  const { SCHEMA_URL, SCHEMA_VERSION, ValidFor } = await import('pathofexile-dat-schema');

  // ------------------------------------------------------------------
  // 2. Download schema
  // ------------------------------------------------------------------
  console.log(`Downloading dat-schema from ${SCHEMA_URL}...`);
  const schemaRes = await fetch(SCHEMA_URL);
  if (!schemaRes.ok) throw new Error(`Failed to download schema: HTTP ${schemaRes.status}`);
  const schema = await schemaRes.json();
  if (schema.version !== SCHEMA_VERSION) {
    console.warn(`Warning: schema version mismatch (expected ${SCHEMA_VERSION}, got ${schema.version}).`);
  }
  console.log(`Schema loaded (${schema.tables.length} tables, version ${schema.version})`);

  // ------------------------------------------------------------------
  // 3. Create bundle loader from local game files
  // ------------------------------------------------------------------
  console.log('');
  console.log(`Reading from local game files: ${gameDir}`);
  const bundleLoader = new SteamBundleLoader(gameDir);
  const loader = await FileLoader.create(bundleLoader);
  console.log('Bundle index loaded.');

  // ------------------------------------------------------------------
  // 4. Schema headers helper
  // ------------------------------------------------------------------
  function getHeaders(tableName, datFile, columnFilter) {
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
  }

  // ------------------------------------------------------------------
  // 5. Extract tables
  // ------------------------------------------------------------------
  console.log('');
  console.log('Extracting tables...');

  const tables = {};
  const POE2_PATH = 'Data/Balance';

  for (const { name, columns } of TABLES) {
    process.stdout.write(`  ${name}... `);
    try {
      const fileContents =
        (await loader.tryGetFileContents(`${POE2_PATH}/${name}.datc64`)) ??
        (await loader.tryGetFileContents(`Data/${name}.datc64`));

      if (!fileContents) {
        console.log('NOT FOUND (skip)');
        tables[name] = [];
        continue;
      }

      const datFile = readDatFile('.datc64', fileContents);
      const headers = getHeaders(name, datFile, columns);
      const rows = exportAllRows(headers, datFile);
      tables[name] = rows;
      console.log(`${rows.length} rows`);
    } catch (e) {
      console.log(`ERROR: ${e.message}`);
      tables[name] = [];
    }
    loader.clearBundleCache();
  }

  // ------------------------------------------------------------------
  // 6. Extract stat descriptions
  // ------------------------------------------------------------------
  console.log('');
  console.log('Extracting stat descriptions...');

  const descMaps = [];
  for (const filePath of STAT_DESC_FILES) {
    process.stdout.write(`  ${filePath}... `);
    try {
      const contents = await loader.tryGetFileContents(filePath);
      if (!contents) {
        console.log('NOT FOUND (skip)');
        continue;
      }
      const text = new TextDecoder('utf-16le').decode(contents);
      const map = parseStatDescriptions(text);
      descMaps.push(map);
      console.log(`${map.size} descriptions`);
    } catch (e) {
      try {
        const contents = await loader.tryGetFileContents(filePath);
        const text = new TextDecoder('utf-8').decode(contents);
        const map = parseStatDescriptions(text);
        descMaps.push(map);
        console.log(`${map.size} descriptions (utf-8 fallback)`);
      } catch {
        console.log(`ERROR: ${e.message}`);
      }
    }
    loader.clearBundleCache();
  }

  const statDescriptions = mergeDescriptions(...descMaps);
  console.log(`Merged ${statDescriptions.size} stat descriptions total.`);

  // ------------------------------------------------------------------
  // 7. Join tables
  // ------------------------------------------------------------------
  console.log('');
  console.log('Joining tables...');
  const enums = buildEnumLookups(schema.enumerations ?? [], ['ModDomains', 'FlaskType']);
  const { categories, items } = joinTables(tables, statDescriptions, { enums });

  const baseItems = items.filter((i) => !i.is_unique);
  const uniqueItems = items.filter((i) => i.is_unique);
  console.log(`  ${categories.length} categories`);
  console.log(`  ${baseItems.length} base items`);
  console.log(`  ${uniqueItems.length} unique items`);
  console.log(`  ${items.length} total items`);

  // ------------------------------------------------------------------
  // 8. Write output files
  // ------------------------------------------------------------------
  const now = new Date().toISOString();
  const versionData = { patch_version: versionLabel, extracted_at: now };

  if (argv['dry-run']) {
    console.log('');
    console.log('Dry run — skipping file writes.');
    return;
  }

  await mkdir(OUTPUT_DIR, { recursive: true });
  const toWrite = [
    { name: 'version.json',        data: versionData },
    { name: 'item_categories.json',data: categories },
    { name: 'items.json',          data: items },
  ];

  console.log('');
  console.log(`Writing output to ${OUTPUT_DIR}...`);
  for (const { name, data } of toWrite) {
    const outPath = resolve(OUTPUT_DIR, name);
    const text = JSON.stringify(data, null, 2);
    await writeFile(outPath, text, 'utf8');
    console.log(`  ${name} (${Math.round(text.length / 1024)} KB)`);
  }

  console.log('');
  console.log(`Done! Game data for version ${versionLabel} extracted successfully.`);
  console.log('Run `pnpm dev` or `pnpm build` to bundle the updated data into the app.');
}

main().catch((err) => {
  console.error('');
  console.error('Extraction failed:', err.message ?? err);
  if (err.stack) console.error(err.stack);
  process.exit(1);
});
