# PRD: Frontend Theme Refactor

## Overview

Systematic refactor and redesign of all frontend components to align with the established dark, fantasy, fiery theme. This PRD uses ralph-loop syntax for iterative, trackable progress.

## Design System Reference

### Established Patterns (from patterns.md)

**Color Palette:**
| Token | Purpose |
|-------|---------|
| `ember` | Primary accent (volcanic orange) |
| `molten` | Secondary accent (gold/amber) |
| `blood` | Danger states, hardcore mode |
| `bone` | Muted text, subtle highlights |
| `stone` | Neutral backgrounds (warm gray) |
| `ash` | Disabled/muted states (cool gray) |

**Class Colors:** `blood` (Warrior), `arcane` (Sorceress), `verdant` (Ranger), `molten` (Huntress), `spirit` (Monk), `ember` (Mercenary), `hex` (Witch), `primal` (Druid)

**Key Rules:**
- Never hardcode colors - use theme tokens (`stone-*`, `ember-*`, etc.)
- Replace `zinc-*` with `stone-*` throughout
- Replace `red-*` with `blood-*` for danger states
- Replace `green-*`/`emerald-*` with `verdant-*` for success states
- Replace `blue-*` with `arcane-*` or `ember-*` depending on context
- Replace `purple-*` with `spirit-*` or `hex-*`
- Use `card-shadow` class for card shadows (GPU-composited drop-shadow)
- Use theme shadows (`shadow-md`, `shadow-right`, etc.) instead of arbitrary values

**Already Completed (Reference Examples):**
- `character-card` - Hero-style card with class-specific theming
- `card` (ui) - Base card component with accent variants
- `sidebar-navigation` - Fixed nav with ember accent
- `status-bar` - Bottom status bar
- `window-title` - Top titlebar
- `button` - Full theme integration

---

## Component Inventory

### UI Components (Priority: High - used everywhere)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 1 | `ui/accordion` | `[x]` | Converted zinc-* to stone-*, added .styles.ts |
| 2 | `ui/data-item` | `[x]` | Converted zinc-* to stone-*, added .styles.ts |
| 3 | `ui/empty-state` | `[x]` | Converted zinc-* to stone/bone-*, added .styles.ts |
| 4 | `ui/error-state` | `[x]` | Converted red-* to blood-*, zinc-* to stone-*, added .styles.ts |
| 5 | `ui/loading-spinner` | `[x]` | Already theme compliant (stone/ember, has .styles.ts) |
| 6 | `ui/modal` | `[x]` | Already theme compliant (stone/ember, z-50, has .styles.ts) |
| 7 | `ui/section-header` | `[x]` | Converted zinc-* to stone/bone-*, added .styles.ts |
| 8 | `ui/time-display` | `[x]` | Already theme compliant (utility component, no hardcoded colors) |
| 9 | `ui/tooltip` | `[x]` | Fixed z-[9999] to z-20 (tooltips z-index) |

### Form Components (Priority: High - core interactions)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 10 | `forms/form-alert-message` | `[x]` | Converted red-* to blood-*, green-* to verdant-* |
| 11 | `forms/form-checkbox-input` | `[x]` | Converted zinc-* to stone-*, blue-* to ember-* |
| 12 | `forms/form-field` | `[x]` | Converted zinc-* to stone/bone-* |
| 13 | `forms/form-filter-toggle` | `[x]` | Already theme compliant (stone/ember, z-20) |
| 14 | `forms/form-input` | `[x]` | Already theme compliant (stone/ember/blood) |
| 15 | `forms/form-select` | `[x]` | Already theme compliant (stone/ember/blood, z-20) |
| 16 | `forms/form-sort-select` | `[x]` | Already theme compliant (stone/ember, z-20) |
| 17 | `forms/settings-form` | `[x]` | Converted zinc-* to stone-* |

### Character Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 18 | `character/character-form-modal` | `[x]` | Converted zinc-* to stone-*, red-* to blood-* |
| 19 | `character/character-list` | `[x]` | Already theme compliant (no hardcoded colors) |
| 20 | `character/character-list-controls-form` | `[x]` | Converted zinc-* to stone-*, blue-* to ember-* |
| 21 | `character/character-status-card` | `[x]` | Converted zinc-* to stone-* |
| 22 | `character/delete-character-modal` | `[x]` | Converted zinc-* to stone-*, red-* to blood-* |

### Economy Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 23 | `economy/currency-list-controls-form` | `[x]` | Converted zinc-* to stone-* |
| 24 | `economy/economy-list` | `[x]` | Added .styles.ts, converted zinc-* to stone-* |
| 25 | `economy/economy-row` | `[x]` | Added .styles.ts, zinc->stone, emerald->verdant, red->blood |
| 26 | `economy/exchange-rates-card` | `[x]` | Already theme compliant (stone-*) |
| 27 | `economy/top-items-card` | `[x]` | Added .styles.ts, zinc->stone, emerald->verdant, red->blood |

### Zone Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 28 | `zones/current-zone-card` | `[x]` | Already theme compliant (stone/ember) |
| 29 | `zones/zone-card` | `[x]` | Full theme update: zinc->stone, blue->arcane, purple->spirit, green->verdant, yellow->molten, emerald->verdant |
| 30 | `zones/zone-details-modal` | `[x]` | Added .styles.ts, full theme update: zinc->stone, blue->arcane, red->blood, purple->spirit, emerald->verdant |
| 31 | `zones/zone-list` | `[x]` | Already theme compliant (no hardcoded colors) |
| 32 | `zones/zone-list-controls-form` | `[x]` | Converted zinc-* to stone-*, blue-* to ember-* |

### Walkthrough Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 33 | `walkthrough/walkthrough-act-accordion` | `[x]` | Already theme compliant (uses themed sub-components) |
| 34 | `walkthrough/walkthrough-guide` | `[x]` | Already theme compliant (uses themed sub-components) |
| 35 | `walkthrough/walkthrough-step-card` | `[x]` | Added .styles.ts, full update: zinc->stone, blue->arcane, green->verdant, purple->spirit, orange->ember |

### Chart Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 36 | `charts/act-distribution-chart` | `[x]` | Fixed JSX syntax error, converted zinc-* to stone-* |
| 37 | `charts/class-distribution-chart` | `[x]` | Fixed JSX syntax error, converted zinc-* to stone-* |

### Insight Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 38 | `insights/campaign-insights` | `[x]` | Already theme compliant (no hardcoded colors) |
| 39 | `insights/character-insights` | `[x]` | Already theme compliant (no hardcoded colors) |
| 40 | `insights/playtime-insights` | `[x]` | Already theme compliant (no hardcoded colors) |

### Layout Components (Priority: Low - already updated)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 41 | `layout/page-layout` | `[x]` | Already theme compliant (no hardcoded colors) |

### Status Components (Priority: Low - already updated)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 42 | `status/status-indicator` | `[x]` | Converted green-500 to verdant-500 |

### Icon Components (Priority: Low)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 43 | `icons/mars-icon` | `[x]` | Already theme compliant (no hardcoded colors) |
| 44 | `icons/venus-icon` | `[x]` | Already theme compliant (no hardcoded colors) |

### Routes/Pages (Priority: Low - inline styles only)

| # | Route | Status | Notes |
|---|-------|--------|-------|
| 45 | `routes/index.tsx` | `[x]` | Already theme compliant (no hardcoded colors) |
| 46 | `routes/characters.tsx` | `[x]` | Already theme compliant (no hardcoded colors) |
| 47 | `routes/economy.tsx` | `[x]` | Converted zinc-* to stone-* |
| 48 | `routes/playtime.tsx` | `[x]` | Already theme compliant (no hardcoded colors) |
| 49 | `routes/settings.tsx` | `[x]` | Converted zinc-* to stone-* |
| 50 | `routes/walkthrough.tsx` | `[x]` | Already theme compliant (no hardcoded colors) |
| 51 | `routes/__root.tsx` | `[x]` | Already theme compliant (uses app-background) |

---

## Common Transformations

### Color Replacements

zinc-* → stone-*
red-* → blood-*
green-*/emerald-* → verdant-*
blue-* → arcane-* (for mystical) or ember-* (for action)
purple-* → spirit-* or hex-*
yellow-*/amber-* → molten-*
orange-* → ember-*

### Shadow Replacements

shadow-lg → shadow-lg (theme version)
shadow-xl → shadow-xl (theme version)
arbitrary shadows → use theme shadows

### Z-Index Standardization

z-10 → Elevated cards
z-20 → Dropdowns, tooltips
z-30 → Fixed chrome (titlebar, sidebar, statusbar)
z-40 → Notifications
z-50 → Modals
z-[9999] → z-50 (modals)

### Utility Classes

Add `card-shadow` class to cards that need drop-shadow effects.

---

## Success Criteria

- [x] All `zinc-*` references replaced with `stone-*`
- [x] All semantic colors use appropriate theme tokens
- [x] All components have co-located `.styles.ts` files
- [x] All z-indexes follow the established scale
- [x] `pnpm lint` passes
- [x] `pnpm build` succeeds
- [x] Visual consistency with already-completed components

---

## Session Log

_Updates will be logged here as components are completed._

| Date | Components Completed | Notes |
|------|---------------------|-------|
| 2026-02-12 | #1-9 (UI Components) | Created .styles.ts files, zinc→stone conversions |
| 2026-02-12 | #10-17 (Form Components) | red→blood, green→verdant, zinc→stone |
| 2026-02-12 | #18-22 (Character Components) | Full theme integration |
| 2026-02-12 | #23-27 (Economy Components) | emerald→verdant, red→blood |
| 2026-02-12 | #28-32 (Zone Components) | blue→arcane, purple→spirit |
| 2026-02-12 | #33-35 (Walkthrough Components) | Full theme integration |
| 2026-02-12 | #36-37 (Chart Components) | Fixed JSX syntax errors, zinc→stone |
| 2026-02-12 | #38-44 (Insights/Layout/Status/Icons) | All theme compliant |
| 2026-02-12 | #45-51 (Routes) | Converted remaining zinc-* references |
| 2026-02-12 | Final cleanup | Fixed all remaining zinc refs in components, utils, and tests |
