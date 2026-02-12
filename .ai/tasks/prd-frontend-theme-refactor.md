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

## Ralph Loop Structure

Each component iteration follows this pattern:

```
<ralph-loop>
  <phase name="research">
    1. Read the component's .tsx file
    2. Read the component's .styles.ts file (if exists)
    3. Identify hardcoded colors (zinc, blue, green, red, etc.)
    4. Identify styling patterns that don't match theme
    5. Note any missing .styles.ts file
  </phase>

  <phase name="implement">
    1. Create/update .styles.ts with theme-compliant styles
    2. Update .tsx to use styles from .styles.ts
    3. Replace hardcoded colors with theme tokens
    4. Ensure card-shadow class used where appropriate
    5. Verify z-index follows scale (see patterns.md)
  </phase>

  <phase name="verify">
    1. Run `pnpm lint` to check for errors
    2. Run `pnpm build` to verify build succeeds
    3. Mark component as complete in this PRD
  </phase>

  <phase name="continue">
    If more components remain: "Continuing to next component: [name]"
    If all complete: "All components refactored. PRD complete."
  </phase>
</ralph-loop>
```

---

## Component Inventory

### UI Components (Priority: High - used everywhere)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 1 | `ui/accordion` | `[ ]` | Uses `zinc-*`, needs `.styles.ts` |
| 2 | `ui/data-item` | `[ ]` | Uses `zinc-*`, needs `.styles.ts` |
| 3 | `ui/empty-state` | `[ ]` | Uses `zinc-*`, needs `.styles.ts` |
| 4 | `ui/error-state` | `[ ]` | Uses `red-*`, `zinc-*`, needs `.styles.ts` |
| 5 | `ui/loading-spinner` | `[ ]` | Check theme compliance |
| 6 | `ui/modal` | `[ ]` | Already has `.styles.ts`, verify compliance |
| 7 | `ui/section-header` | `[ ]` | Uses `zinc-*`, needs `.styles.ts` |
| 8 | `ui/time-display` | `[ ]` | Check theme compliance |
| 9 | `ui/tooltip` | `[ ]` | Has `.styles.ts`, uses arbitrary z-index `z-[9999]` |

### Form Components (Priority: High - core interactions)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 10 | `forms/form-alert-message` | `[ ]` | Check theme compliance |
| 11 | `forms/form-checkbox-input` | `[ ]` | Check theme compliance |
| 12 | `forms/form-field` | `[ ]` | Check theme compliance |
| 13 | `forms/form-filter-toggle` | `[ ]` | Has `.styles.ts`, verify compliance |
| 14 | `forms/form-input` | `[ ]` | Has `.styles.ts`, verify compliance |
| 15 | `forms/form-select` | `[ ]` | Has `.styles.ts`, verify compliance |
| 16 | `forms/form-sort-select` | `[ ]` | Check theme compliance |
| 17 | `forms/settings-form` | `[ ]` | Check theme compliance |

### Character Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 18 | `character/character-form-modal` | `[ ]` | Check theme compliance |
| 19 | `character/character-list` | `[ ]` | Check theme compliance |
| 20 | `character/character-list-controls-form` | `[ ]` | Check theme compliance |
| 21 | `character/character-status-card` | `[ ]` | Uses `zinc-*`, needs update |
| 22 | `character/delete-character-modal` | `[ ]` | Check theme compliance |

### Economy Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 23 | `economy/currency-list-controls-form` | `[ ]` | Check theme compliance |
| 24 | `economy/economy-list` | `[ ]` | Check theme compliance |
| 25 | `economy/economy-row` | `[ ]` | Uses `zinc-*`, `emerald-*`, `red-*` |
| 26 | `economy/exchange-rates-card` | `[ ]` | Check theme compliance |
| 27 | `economy/top-items-card` | `[ ]` | Check theme compliance |

### Zone Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 28 | `zones/current-zone-card` | `[ ]` | Check theme compliance |
| 29 | `zones/zone-card` | `[ ]` | Uses `zinc-*`, `blue-*`, `emerald-*` - major update needed |
| 30 | `zones/zone-details-modal` | `[ ]` | Check theme compliance |
| 31 | `zones/zone-list` | `[ ]` | Check theme compliance |
| 32 | `zones/zone-list-controls-form` | `[ ]` | Check theme compliance |

### Walkthrough Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 33 | `walkthrough/walkthrough-act-accordion` | `[ ]` | Check theme compliance |
| 34 | `walkthrough/walkthrough-guide` | `[ ]` | Check theme compliance |
| 35 | `walkthrough/walkthrough-step-card` | `[ ]` | Uses `zinc-*`, `blue-*`, `green-*`, `purple-*`, `orange-*` - major update |

### Chart Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 36 | `charts/act-distribution-chart` | `[ ]` | Check theme compliance |
| 37 | `charts/class-distribution-chart` | `[ ]` | Uses `zinc-*` |

### Insight Components (Priority: Medium)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 38 | `insights/campaign-insights` | `[ ]` | Check theme compliance |
| 39 | `insights/character-insights` | `[ ]` | Check theme compliance |
| 40 | `insights/playtime-insights` | `[ ]` | Check theme compliance |

### Layout Components (Priority: Low - already updated)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 41 | `layout/page-layout` | `[ ]` | Verify theme compliance |

### Status Components (Priority: Low - already updated)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 42 | `status/status-indicator` | `[ ]` | Check theme compliance |

### Icon Components (Priority: Low)

| # | Component | Status | Notes |
|---|-----------|--------|-------|
| 43 | `icons/mars-icon` | `[ ]` | Minimal styling |
| 44 | `icons/venus-icon` | `[ ]` | Minimal styling |

### Routes/Pages (Priority: Low - inline styles only)

| # | Route | Status | Notes |
|---|-------|--------|-------|
| 45 | `routes/index.tsx` | `[ ]` | Minimal inline styles |
| 46 | `routes/characters.tsx` | `[ ]` | Minimal inline styles |
| 47 | `routes/economy.tsx` | `[ ]` | Uses `zinc-*` inline |
| 48 | `routes/playtime.tsx` | `[ ]` | Check inline styles |
| 49 | `routes/settings.tsx` | `[ ]` | Uses `zinc-*` inline |
| 50 | `routes/walkthrough.tsx` | `[ ]` | Check inline styles |
| 51 | `routes/__root.tsx` | `[ ]` | App wrapper |

---

## Common Transformations

### Color Replacements

```
zinc-* → stone-*
red-* → blood-*
green-*/emerald-* → verdant-*
blue-* → arcane-* (for mystical) or ember-* (for action)
purple-* → spirit-* or hex-*
yellow-*/amber-* → molten-*
orange-* → ember-*
```

### Shadow Replacements

```
shadow-lg → shadow-lg (theme version)
shadow-xl → shadow-xl (theme version)
arbitrary shadows → use theme shadows
```

### Z-Index Standardization

```
z-10 → Elevated cards
z-20 → Dropdowns, tooltips
z-30 → Fixed chrome (titlebar, sidebar, statusbar)
z-40 → Notifications
z-50 → Modals
z-[9999] → z-50 (modals)
```

### Utility Classes

Add `card-shadow` class to cards that need drop-shadow effects.

---

## Success Criteria

- [ ] All `zinc-*` references replaced with `stone-*`
- [ ] All semantic colors use appropriate theme tokens
- [ ] All components have co-located `.styles.ts` files
- [ ] All z-indexes follow the established scale
- [ ] `pnpm lint` passes
- [ ] `pnpm build` succeeds
- [ ] Visual consistency with already-completed components

---

## Loop Execution Commands

To start the ralph-loop:
```
Begin the frontend theme refactor loop starting with component #1 (ui/accordion).
Follow the ralph-loop phases for each component.
Update this PRD after each component is complete.
```

To resume after interruption:
```
Resume the frontend theme refactor loop. Check this PRD for the last incomplete component.
```

---

## Session Log

_Updates will be logged here as components are completed._

| Date | Components Completed | Notes |
|------|---------------------|-------|
| | | |
