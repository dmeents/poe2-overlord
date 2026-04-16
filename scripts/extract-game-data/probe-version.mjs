#!/usr/bin/env node
/**
 * probe-version.mjs — Discover the current POE2 CDN patch version string
 *
 * Fetches the version from the community-maintained tracker:
 *   https://raw.githubusercontent.com/poe-tool-dev/latest-patch-version/main/latest.txt
 *   https://raw.githubusercontent.com/poe-tool-dev/latest-patch-version/main/poe2/latest.txt
 *
 * Falls back to probing the CDN directly if the tracker is unavailable.
 *
 * Usage:
 *   node probe-version.mjs
 *
 * Output:
 *   Prints only the version string to stdout on success, everything else to stderr.
 *   Use with command substitution:
 *     node extract.mjs --patch-version $(node probe-version.mjs)
 */

const TRACKER_URLS = [
  'https://raw.githubusercontent.com/poe-tool-dev/latest-patch-version/main/poe2/latest.txt',
  'https://raw.githubusercontent.com/poe-tool-dev/latest-patch-version/main/latest.txt',
];

const CDN_BASE = 'https://patch-poe2.poecdn.com';
const INDEX_PATH = 'Bundles2/_.index.bin';

// ---------------------------------------------------------------------------
// Strategy 1: community tracker
// ---------------------------------------------------------------------------

async function fetchFromTracker() {
  for (const url of TRACKER_URLS) {
    process.stderr.write(`Trying tracker: ${url}\n`);
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(8000) });
      if (!res.ok) {
        process.stderr.write(`  → HTTP ${res.status}, skipping\n`);
        continue;
      }
      const text = (await res.text()).trim();
      process.stderr.write(`  → ${text}\n`);
      if (text.startsWith('4.')) {
        return text;
      }
      if (text.startsWith('3.')) {
        // POE1 version — this URL tracks POE1 only; try next
        process.stderr.write(`  → POE1 version, skipping\n`);
        continue;
      }
    } catch (err) {
      process.stderr.write(`  → ${err.message}\n`);
    }
  }
  return null;
}

// ---------------------------------------------------------------------------
// Strategy 2: CDN probe (targeted — only probes .0 of each minor)
// ---------------------------------------------------------------------------

async function probeCdn() {
  process.stderr.write('\nFalling back to CDN probing...\n');

  // First pass: find which minor version (4.0, 4.1, ...) has any content.
  let activeMinor = null;
  for (let minor = 0; minor <= 9; minor++) {
    const version = `4.${minor}.0.0`;
    process.stderr.write(`  HEAD ${CDN_BASE}/${version}/${INDEX_PATH} ... `);
    try {
      const res = await fetch(`${CDN_BASE}/${version}/${INDEX_PATH}`, {
        method: 'HEAD',
        signal: AbortSignal.timeout(6000),
      });
      process.stderr.write(`${res.status}\n`);
      if (res.ok) activeMinor = minor;
    } catch {
      process.stderr.write('timeout\n');
    }
  }

  if (activeMinor === null) {
    return null;
  }

  // Second pass: walk patch numbers within the active minor to find the latest.
  process.stderr.write(`\nScanning 4.${activeMinor}.x.x for latest patch...\n`);
  let latest = null;
  for (let build = 0; build <= 9; build++) {
    let foundInBuild = false;
    for (let patch = 0; patch <= 50; patch++) {
      const version = `4.${activeMinor}.${build}.${patch}`;
      process.stderr.write(`  HEAD ${CDN_BASE}/${version}/${INDEX_PATH} ... `);
      try {
        const res = await fetch(`${CDN_BASE}/${version}/${INDEX_PATH}`, {
          method: 'HEAD',
          signal: AbortSignal.timeout(6000),
        });
        process.stderr.write(`${res.status}\n`);
        if (res.ok) {
          latest = version;
          foundInBuild = true;
        } else if (foundInBuild) {
          break; // first miss after a hit → end of this build series
        } else if (patch >= 5) {
          break; // 5 misses with no hit → skip this build number
        }
      } catch {
        process.stderr.write('timeout\n');
        break;
      }
    }
    if (!foundInBuild && latest !== null) break; // past the active build range
  }

  return latest;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  process.stderr.write('Discovering current POE2 CDN patch version...\n\n');

  let version = await fetchFromTracker();

  if (!version) {
    version = await probeCdn();
  }

  if (!version) {
    process.stderr.write('\nCould not determine the current CDN patch version.\n');
    process.stderr.write('Check https://github.com/poe-tool-dev/latest-patch-version for the current version,\n');
    process.stderr.write('then run: node extract.mjs --patch-version <version>\n');
    process.exit(1);
  }

  process.stderr.write(`\nCurrent version: ${version}\n`);
  console.log(version); // stdout only — for $(command substitution)
}

main().catch((err) => {
  process.stderr.write(`\nFailed: ${err.message ?? err}\n`);
  process.exit(1);
});
