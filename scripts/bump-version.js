#!/usr/bin/env node
// Bumps the version in Cargo.toml, the root package.json, and aur/PKGBUILD atomically.
// Tauri reads its version from Cargo.toml, so that is the primary source of truth.
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
const cargoVersionRe = /^(version\s*=\s*)"([^"]*)"/m;
const cargoMatch = cargoVersionRe.exec(cargo);
if (!cargoMatch) {
  console.error('Could not find version field in Cargo.toml');
  process.exit(1);
}
const prev = cargoMatch[2];
if (prev === version) {
  console.error(`Version is already ${version}. Nothing to bump.`);
  process.exit(1);
}
// Only replace the version in the [package] section (first occurrence)
const updatedCargo = cargo.replace(cargoVersionRe, `$1"${version}"`);
writeFileSync(cargoPath, updatedCargo);
console.log(`  packages/backend/Cargo.toml -> ${version}`);

// Update root package.json
const pkgPath = resolve(root, 'package.json');
const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
pkg.version = version;
writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
console.log(`  package.json -> ${version}`);

// Update aur/PKGBUILD
const pkgbuildPath = resolve(root, 'aur/PKGBUILD');
const pkgbuild = readFileSync(pkgbuildPath, 'utf8');
if (!/^pkgver=.*/m.test(pkgbuild)) {
  console.error('Could not find pkgver field in aur/PKGBUILD');
  process.exit(1);
}
const updatedPkgbuild = pkgbuild
  .replace(/^pkgver=.*/m, `pkgver=${version}`)
  .replace(/^pkgrel=.*/m, 'pkgrel=1');
writeFileSync(pkgbuildPath, updatedPkgbuild);
console.log(`  aur/PKGBUILD -> ${version}`);

// Update release workflow description so the dropdown shows the current version
const workflowPath = resolve(root, '.github/workflows/release.yml');
const workflow = readFileSync(workflowPath, 'utf8');
const updatedWorkflow = workflow.replace(
  /description: '.*'/,
  `description: 'Current: ${version}. Enter the next version (e.g. X.Y.Z)'`
);
writeFileSync(workflowPath, updatedWorkflow);
console.log(`  .github/workflows/release.yml description -> ${version}`);

console.log(`\nBumped ${prev} -> ${version}`);
