#!/usr/bin/env node
/**
 * extract.mjs — POE2 game data extraction script
 *
 * Downloads POE2 game bundles from GGG CDN via pathofexile-dat, extracts
 * the relevant .datc64 tables, joins them into denormalized item records, and
 * writes JSON output files to packages/backend/data/game_data/.
 *
 * Usage (first time):
 *   cd scripts/extract-game-data && pnpm install
 *   node extract.mjs --patch-version 4.1.0.1
 *
 * Or from repo root:
 *   pnpm extract:gamedata -- --patch-version 4.1.0.1
 *
 * The script caches downloaded bundles in scripts/extract-game-data/.cache/
 * so subsequent runs for the same patch version are much faster.
 *
 * Output files (relative to repo root):
 *   packages/backend/data/game_data/version.json
 *   packages/backend/data/game_data/items.json
 *   packages/backend/data/game_data/item_categories.json
 */

import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import minimist from 'minimist';

import { parseStatDescriptions, mergeDescriptions } from './lib/stat-descriptions.mjs';
import { joinTables } from './lib/table-joiner.mjs';
import { TABLES, STAT_DESC_FILES, buildEnumLookups, buildGetHeaders } from './lib/tables.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', '..');
const OUTPUT_DIR = resolve(ROOT, 'packages', 'backend', 'data', 'game_data');
const CACHE_DIR = resolve(__dirname, '.cache');

// ---------------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------------

const argv = minimist(process.argv.slice(2), {
  string: ['patch-version'],
  boolean: ['help', 'dry-run'],
  alias: { h: 'help', v: 'patch-version', n: 'dry-run' },
});

if (argv.help) {
  console.log(`
Usage:
  node extract.mjs --patch-version 4.x.x.x

Options:
  --patch-version, -v   POE2 CDN patch version (e.g. 4.1.0.1). Must start with "4.".
                        Find the current version with: node probe-version.mjs
  --dry-run, -n         Extract and join data but do not write output files.
  --help, -h            Show this help.
`);
  process.exit(0);
}

const patchVersion = argv['patch-version'];

if (!patchVersion) {
  console.error('Error: --patch-version is required (e.g. --patch-version 4.1.0.1).');
  console.error('Run with --help for usage.');
  process.exit(1);
}

if (!patchVersion.startsWith('4.')) {
  console.error(`Error: patch version must start with "4." for POE2 (e.g. 4.1.0.1), got "${patchVersion}"`);
  console.error('Tip: the in-game display version ("0.4.0i") is not the CDN version.');
  console.error('Run node probe-version.mjs to discover the current CDN version string.');
  process.exit(1);
}

// Table/column definitions + stat-description file list live in lib/tables.mjs
// so extract.mjs and extract-local.mjs stay in sync when new fields are added.

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  console.log(`POE2 Game Data Extractor`);
  console.log(`Mode: CDN download`);
  console.log(`Patch version: ${patchVersion}`);
  console.log('');

  // ------------------------------------------------------------------
  // 1. Import pathofexile-dat CLI internals
  // ------------------------------------------------------------------
  // These are internal module paths — not in the package's exports map,
  // but stable within a major version.

  const { CdnBundleLoader, FileLoader } = await import(
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
  if (!schemaRes.ok) {
    throw new Error(`Failed to download schema: HTTP ${schemaRes.status}`);
  }
  const schema = await schemaRes.json();
  if (schema.version !== SCHEMA_VERSION) {
    console.warn(`Warning: schema version mismatch (expected ${SCHEMA_VERSION}, got ${schema.version}). Column access may fail.`);
  }
  console.log(`Schema loaded (${schema.tables.length} tables, version ${schema.version})`);

  // ------------------------------------------------------------------
  // 3. Create bundle loader (CDN with local cache)
  // ------------------------------------------------------------------
  console.log('');
  console.log('Initialising bundle loader...');
  await mkdir(CACHE_DIR, { recursive: true });
  const bundleLoader = await CdnBundleLoader.create(CACHE_DIR, patchVersion);
  console.log(`Downloading from CDN (patch ${patchVersion})`);
  const loader = await FileLoader.create(bundleLoader);
  console.log('Bundle index loaded.');

  // ------------------------------------------------------------------
  // 4. Load schema headers helper
  // ------------------------------------------------------------------
  const getHeaders = buildGetHeaders(schema, ValidFor, getHeaderLength);

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
  // 6. Extract stat description text files
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
      // Fetch once, try UTF-16LE first (standard POE encoding), fall back to
      // UTF-8. Re-fetching in the catch would race against clearBundleCache().
      let text;
      let usedFallback = false;
      try {
        text = new TextDecoder('utf-16le').decode(contents);
      } catch {
        text = new TextDecoder('utf-8').decode(contents);
        usedFallback = true;
      }
      const map = parseStatDescriptions(text);
      descMaps.push(map);
      const label = usedFallback ? ' (utf-8 fallback)' : '';
      console.log(`${map.size} descriptions${label}`);
    } catch (e) {
      console.log(`ERROR: ${e.message}`);
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
  const versionData = { patch_version: patchVersion, extracted_at: now };

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
  console.log(`Done! Game data for patch ${patchVersion} extracted successfully.`);
  console.log('Run `pnpm dev` or `pnpm build` to bundle the updated data into the app.');
}

main().catch((err) => {
  console.error('');
  console.error('Extraction failed:', err.message ?? err);
  if (err.stack) console.error(err.stack);
  process.exit(1);
});
