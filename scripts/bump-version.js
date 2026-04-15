#!/usr/bin/env node
// Bumps the version in Cargo.toml and the root package.json atomically.
// Tauri reads its version from Cargo.toml, so those two are the only sources of truth.
//
// Primary usage is via the Release workflow (Actions tab -> Release -> Run workflow).
// Can also be run locally:
//   node scripts/bump-version.js 0.2.0
//   pnpm bump-version 0.2.0

'use strict';

const { readFileSync, writeFileSync } = require('node:fs');
const { resolve } = require('node:path');

const root = resolve(__dirname, '..');
const version = process.argv[2];

if (!version) {
  console.error('Usage: node scripts/bump-version.js <version>');
  console.error('Example: node scripts/bump-version.js 0.2.0');
  process.exit(1);
}

if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error(`Invalid version format: "${version}". Expected semver like "1.2.3".`);
  process.exit(1);
}

// Update packages/backend/Cargo.toml
const cargoPath = resolve(root, 'packages/backend/Cargo.toml');
const cargo = readFileSync(cargoPath, 'utf8');
// Only replace the version in the [package] section (first occurrence)
const updatedCargo = cargo.replace(/^(version\s*=\s*)"[^"]*"/m, `$1"${version}"`);
if (updatedCargo === cargo) {
  console.error('Could not find version field in Cargo.toml');
  process.exit(1);
}
writeFileSync(cargoPath, updatedCargo);
console.log(`  packages/backend/Cargo.toml -> ${version}`);

// Update root package.json
const pkgPath = resolve(root, 'package.json');
const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
const prev = pkg.version;
pkg.version = version;
writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
console.log(`  package.json -> ${version}`);

console.log(`\nBumped ${prev} -> ${version}`);
