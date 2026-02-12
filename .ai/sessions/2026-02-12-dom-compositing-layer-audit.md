# DOM Structure & Compositing Layer Audit Report

**Date:** 2026-02-12
**Status:** Complete - Structural fixes implemented, root cause investigation deferred

**Follow-up:** `.ai/tasks/prd-compositing-layer-root-cause.md` - Experiments to identify why `box-shadow` requires compositing layer

---

## Executive Summary

This audit examined the application's DOM structure, stacking contexts, and compositing layers to understand why `box-shadow` was invisible on card components while `filter: drop-shadow()` worked.

**Root Cause: UNCONFIRMED** - Initial hypothesis that `overflow-auto` was the clipping culprit was tested and did NOT resolve the issue. The actual root cause remains unknown and may involve:
- Backdrop-filter stacking context interactions
- Tauri WebView-specific compositing behavior
- Hardware acceleration quirks (AMD GPU)
- Complex stacking context layering from multiple sources

### Key Findings

| Category | Issues Found | Severity |
|----------|-------------|----------|
| Stacking Contexts | 8 components creating contexts | Medium |
| Overflow Clipping | 4 locations, 1 problematic | **High** |
| Z-Index Consistency | StatusBar missing z-50 | Medium |
| Theme Consistency | 50+ zinc-* color usages | Low |
| GPU Compositing | 1 workaround in place | Medium |

---

## Phase 1: DOM Structure Audit - Stacking Context Map

### Stacking Context Hierarchy

```
<html>
└── <body>
    └── .app-background
        ├── WindowTitle [STACKING CONTEXT: fixed z-50, backdrop-filter]
        ├── SidebarNavigation [STACKING CONTEXT: fixed z-50, backdrop-filter]
        ├── <div class="overflow-auto"> [CLIPPING BOUNDARY]
        │   └── <Outlet>
        │       └── Page Content
        │           └── Card [STACKING CONTEXT: filter, transform, will-change]
        │               └── box-shadow ← CLIPPED BY PARENT
        ├── StatusBar [STACKING CONTEXT: fixed, backdrop-filter] (missing z-50!)
        └── ZoneDetailsModal
            └── Modal [STACKING CONTEXT: fixed z-50]
                └── backdrop [STACKING CONTEXT: backdrop-filter]
```

### Elements Creating Stacking Contexts

| Component | File:Line | Stacking Context Triggers | Necessary? |
|-----------|-----------|--------------------------|------------|
| WindowTitle | `window-title.styles.ts:6` | `fixed z-50`, `backdrop-blur-sm` | Yes |
| SidebarNavigation | `sidebar-navigation.styles.ts:6` | `fixed z-50`, `backdrop-blur-sm` | Yes |
| StatusBar | `status-bar.styles.ts:6` | `fixed`, `backdrop-blur-sm` | Yes (but missing z-50) |
| Modal overlay | `modal.styles.ts:6` | `fixed z-50` | Yes |
| Modal backdrop | `modal.styles.ts:8` | `fixed`, `backdrop-blur-sm` | Yes |
| Card (via .card-shadow) | `globals.css:248-252` | `filter`, `transform`, `will-change` | **Workaround** |
| CharacterCard | `character-card.styles.ts:113` | `relative z-10`, `overflow-hidden` | Review needed |
| Form Dropdowns | Multiple styles.ts | `fixed z-50` | Yes |
| Chart Tooltips | chart components | `relative z-50` | Yes |

### Stacking Context Triggers Reference

Properties that create stacking contexts (per MDN):
- `position: fixed/absolute/relative` with `z-index`
- `opacity` < 1 (NOT `rgba` alpha or `/95` notation)
- `transform` (any value except none)
- `filter` (any value except none)
- `backdrop-filter` (any value except none)
- `will-change` (specifying stacking context properties)
- `isolation: isolate`

---

## Phase 2: Overflow & Clipping Audit

### Clipping Boundaries Found

| Location | File:Line | Property | Impact |
|----------|-----------|----------|--------|
| **Main Content Wrapper** | `__root.tsx:13` | `overflow-auto` | **CLIPS ALL CHILD SHADOWS** |
| CharacterCard | `character-card.styles.ts:113` | `overflow-hidden` | Clips card content, uses filter workaround |
| ZoneDetailsModal image | `zone-details-modal.tsx:88` | `overflow-hidden` | Intentional for image cropping |
| ZoneList container | `zone-list.tsx:76` | `overflow-hidden` | Clips list boundaries |

### The Core Problem

```tsx
// __root.tsx:13
<div className="mt-7.5 ml-12 overflow-auto font-sans">
```

This `overflow-auto`:
1. Creates a scrollable container for the main content
2. Establishes a **clipping boundary** for any child elements
3. Any `box-shadow` that extends beyond this element's bounds is clipped
4. The `filter: drop-shadow()` workaround works because filters apply at composite time, after clipping

### Why Sidebar/Titlebar Shadows Work

The sidebar uses `box-shadow` via `shadow-right` and it works because:
1. It's `fixed` positioned at `z-50`
2. It exists **outside** the `overflow-auto` container
3. It's in its own stacking context, not subject to the content wrapper's clipping

---

## Phase 3: Fixed/Absolute Positioning Audit

### All Fixed Elements

| Element | Position | Z-Index | Backdrop | Notes |
|---------|----------|---------|----------|-------|
| WindowTitle | `fixed top-0 left-0 right-0` | `z-50` | `backdrop-blur-sm` | Correct |
| SidebarNavigation | `fixed left-0 top-7 bottom-6` | `z-50` | `backdrop-blur-sm` | Correct |
| StatusBar | `fixed bottom-0 w-full` | **NONE** | `backdrop-blur-sm` | **Missing z-50!** |
| Modal overlay | `fixed inset-0` | `z-50` | None | Correct |
| Modal backdrop | `fixed inset-0` | None (inherits) | `backdrop-blur-sm` | Correct |
| Form dropdowns | `fixed` | `z-50` | None | Correct |

### Z-Index Scale Analysis

Current usage:
- `z-50` (50): WindowTitle, Sidebar, Modal, Dropdowns, Chart Tooltips
- `z-10` (10): CharacterCard (for hover states)

**Issues:**
1. StatusBar is missing `z-50` - could render behind other elements
2. All major UI elements share `z-50` - no layering strategy
3. CharacterCard uses `z-10` without clear rationale

**Recommended Z-Index Scale:**
```
z-0   : Base content
z-10  : Elevated cards, hover states
z-20  : Dropdowns, popovers
z-30  : Fixed UI (sidebar, titlebar, statusbar)
z-40  : Notifications, toasts
z-50  : Modals, dialogs
```

---

## Phase 4: Compositing Layer Analysis

### GPU Layer Promotion

The `.card-shadow` class in `globals.css:248-252`:

```css
.card-shadow {
  filter: drop-shadow(var(--shadow-md));  /* Creates compositing layer */
  will-change: filter;                      /* Hints GPU layer needed */
  transform: translateZ(0);                /* Forces GPU layer */
}
```

**This is a triple-force:**
1. `filter` alone creates a compositing layer
2. `will-change: filter` tells browser to pre-allocate layer
3. `transform: translateZ(0)` is a classic GPU-forcing hack

**Performance Implications:**
- Each element with `.card-shadow` gets its own GPU layer
- Increases GPU memory usage
- May cause "layer explosion" with many cards
- Overkill - `filter` alone would suffice

**Used By:**
- `card.styles.ts:2` - Base Card component
- `character-card.styles.ts:113` - CharacterCard component

---

## Phase 5: CSS Architecture Review

### Backdrop-Filter Usage

| Component | Purpose | Necessary? |
|-----------|---------|------------|
| WindowTitle | Blur content behind title bar | Design choice - acceptable |
| SidebarNavigation | Blur content behind sidebar | Design choice - acceptable |
| StatusBar | Blur content behind status bar | Design choice - acceptable |
| Modal backdrop | Blur behind modal | Standard pattern - acceptable |
| CharacterCard action buttons | Blur behind hover buttons | **Questionable** - adds complexity |

### Theme Consistency: zinc-* vs stone-*

Per `patterns.md`, all colors should use theme tokens (stone, ember, blood, etc.), not zinc.

**Files with zinc-* colors (should be migrated to stone-*):**

| File | Count | Priority |
|------|-------|----------|
| `zone-details-modal.tsx` | 30+ | High |
| `economy.tsx` | 10+ | Medium |
| `zone-list.tsx` | 4+ | Medium |
| `form-sort-select.styles.ts` | 10+ | Medium |
| `form-filter-toggle.styles.ts` | 4+ | Medium |
| `top-items-card.tsx` | 4+ | Low |
| `text-parser.tsx` | 2 | Low |
| Chart components | 6+ | Low |
| `settings.tsx` | 2 | Low |

### Shadow Implementation Review

**Working (box-shadow):**
- Sidebar: `shadow-right` (fixed element, own stacking context)
- Titlebar: `shadow-bottom` (fixed element, own stacking context)
- StatusBar: `shadow-top` (fixed element, own stacking context)
- Modal: `shadow-xl` (fixed element, own stacking context)

**Requires Workaround (drop-shadow filter):**
- Card components in scrollable content area

---

## Root Cause Summary

**STATUS: PARTIALLY UNDERSTOOD**

Initial hypothesis (overflow-auto clipping) was disproven. Re-evaluation with the context that GPU hints are REQUIRED revealed a deeper pattern.

### Key Evidence

| Element | Has Compositing Layer? | box-shadow works? |
|---------|----------------------|-------------------|
| Sidebar | Yes (backdrop-filter) | **YES** |
| Titlebar | Yes (backdrop-filter) | **YES** |
| Cards (no hints) | No | **NO** (invisible) |
| Cards (with filter only) | Yes (intermittent) | **FLICKERS** |
| Cards (with filter + GPU hints) | Yes (stable) | **YES** |

**The pattern**: Elements with their own compositing layer can render shadows. Elements without cannot.

### Why GPU Hints Are Required

1. `filter: drop-shadow()` creates a compositing layer, but the browser may REMOVE it during React re-renders as an optimization
2. `will-change: filter` tells browser to pre-allocate and keep the layer
3. `transform: translateZ(0)` forces layer promotion as a fallback guarantee
4. Without both hints, the browser inconsistently promotes/demotes layers during route navigation → flickering

### Why box-shadow Is Completely Invisible (Not Flickering)

`box-shadow` is painted during the **paint phase** onto whatever layer contains the element. If that element doesn't have its own compositing layer, the shadow is painted onto a parent layer. Something about the current structure causes that paint to be lost or obscured during compositing.

`drop-shadow()` is applied during the **composite phase** to the element's own layer. Since filter forces layer creation, the shadow survives compositing.

### Prime Suspects

**1. `background-attachment: fixed` on .app-background (HIGH)**
```css
.app-background {
  background: ..., url("/background.png") ... fixed;
}
```
`background-attachment: fixed` creates unusual compositing behavior - the background is positioned relative to viewport, potentially causing paint/composite ordering issues.

**2. `backdrop-filter` on fixed elements (MEDIUM)**
The sidebar/titlebar's `backdrop-blur-sm` creates compositing layers that sample from "behind" them. The compositing order might be causing content layer paint to be lost.

**3. Content wrapper has no compositing layer (MEDIUM)**
The content is painted onto its parent's layer. If that parent's layer has compositing issues (due to #1 or #2), the paint is lost.

### Recommended Experiments

To definitively identify the root cause:

| Experiment | Change | Tests |
|------------|--------|-------|
| A | Add `transform: translateZ(0)` to content wrapper (`main` element) | Does giving content its own layer allow box-shadow on cards? |
| B | Remove `fixed` from `background-attachment` | Does removing fixed background allow box-shadow? |
| C | Remove `backdrop-blur-sm` from ALL fixed elements | Does removing backdrop-filter allow box-shadow? |

**If Experiment A works**, it would mean we can use a single GPU hint on the content wrapper instead of every card - cleaner architecture.

---

## Revised Structural Analysis

After re-examining the code, I identified several structural issues beyond the shadow problem:

### Current DOM Structure

```
<div class="app-background">              ← NO height, NO position, NO stacking context
  <WindowTitle />                         ← fixed top-0 z-50, backdrop-blur-sm
  <SidebarNavigation />                   ← fixed left-0 z-50, backdrop-blur-sm
  <div class="mt-7.5 ml-12 overflow-auto">  ← NO height, NO position, NO z-index
    <div class="mb-16">
      <Outlet />                          ← Cards with card-shadow
    </div>
  </div>
  <StatusBar />                           ← fixed bottom-0, **NO Z-INDEX!**, backdrop-blur-sm
  <ZoneDetailsModal />                    ← fixed z-50 when open
</div>
```

### Structural Problems Identified

| Issue | Location | Impact |
|-------|----------|--------|
| Root has no height | `.app-background` | Relies on content to define size |
| Root has no position | `.app-background` | Doesn't establish containing block |
| Content wrapper has no height | `__root.tsx:13` | `overflow-auto` without height constraint |
| Content wrapper has no stacking context | `__root.tsx:13` | Cards in root stacking context |
| StatusBar missing z-index | `status-bar.styles.ts:6` | Could render behind content |
| CharacterCard has unexplained z-10 | `character-card.styles.ts:113` | Creates stacking context unnecessarily? |
| CharacterCard has overflow-hidden | `character-card.styles.ts:113` | Clips content and shadows |
| All chrome uses z-50 | Multiple files | No layering strategy |

---

## Action List

### CRITICAL - Must Fix (Bugs)

#### 1. Add z-50 to StatusBar
**File:** `status-bar.styles.ts:6`
**Current:**
```ts
'fixed bottom-0 w-full py-1 px-4 border-t bg-stone-950/95 backdrop-blur-sm border-stone-800/50 flex justify-between gap-2 shadow-top'
```
**Change:** Add `z-50` to match WindowTitle and Sidebar
**Risk:** None - this is clearly missing

#### 2. ~~Simplify .card-shadow class~~ **RETRACTED**
**Status:** These properties are REQUIRED, not over-engineering.

The `will-change` and `transform` are necessary to prevent shadow flickering during route navigation. Without them, the browser inconsistently promotes/demotes compositing layers during React re-renders.

**Current implementation is correct:**
```css
.card-shadow {
  filter: drop-shadow(var(--shadow-md));
  will-change: filter;    /* Pre-allocate layer */
  transform: translateZ(0); /* Guarantee layer persistence */
}
```

See "Why GPU Hints Are Required" section above for full explanation.

---

### HIGH - Structural Fixes

#### 3. Add explicit sizing to .app-background
**File:** `globals.css` (add to .app-background rule)
**Change:** Add `min-h-screen` to ensure root fills viewport
```css
.app-background {
  min-height: 100vh; /* or use Tailwind min-h-screen in __root.tsx */
  background: ...existing...
}
```
**Rationale:** Without explicit height, the root element only expands to fit in-flow content (the scroll wrapper). Fixed elements don't contribute.
**Risk:** Low - should have no visual change if layout is correct

#### 4. Add stacking context to content wrapper
**File:** `__root.tsx:13`
**Current:**
```tsx
<div className="mt-7.5 ml-12 overflow-auto font-sans">
```
**Change:** Add `relative` or `isolation-isolate` to create stacking context boundary
```tsx
<div className="relative mt-7.5 ml-12 overflow-auto font-sans">
```
**Rationale:** Creates a stacking context boundary so cards are isolated from fixed chrome. This is proper CSS architecture.
**Risk:** Low - cards will stack relative to content wrapper instead of root

#### 5. Add explicit height to content wrapper
**File:** `__root.tsx:13`
**Change:** Add height constraint so `overflow-auto` works predictably
```tsx
<div className="relative h-[calc(100vh-52px)] mt-7.5 ml-12 overflow-auto font-sans">
```
Where `52px` = `28px` (titlebar) + `24px` (statusbar)
**Rationale:** `overflow-auto` on an element without height constraint behaves unpredictably. Explicit height ensures proper scroll containment.
**Risk:** Medium - test scrolling behavior, may need adjustment

---

### MEDIUM - Architecture Improvements

#### 6. Establish z-index scale
**File:** `patterns.md` and `globals.css`
**Proposed scale:**
```
z-0   : Base content
z-10  : Elevated cards, hover states (if needed)
z-20  : Dropdowns, popovers, tooltips
z-30  : Fixed UI chrome (sidebar, titlebar, statusbar)
z-40  : Notifications, toasts
z-50  : Modals, dialogs (reserved for blocking UI)
```
**Changes needed:**
- WindowTitle: z-50 → z-30
- Sidebar: z-50 → z-30
- StatusBar: add z-30
- Form dropdowns: z-50 → z-20
- Chart tooltips: z-50 → z-20
- Modal: keep z-50 (correct)
**Risk:** Medium - requires testing all z-index interactions

#### 7. Review CharacterCard z-10
**File:** `character-card.styles.ts:113`
**Current:** `'group relative z-10 bg-stone-900 border border-stone-800 card-shadow overflow-hidden'`
**Question:** Why does CharacterCard need z-10 when Card doesn't?
**Options:**
- Remove z-10 if it's unnecessary
- Document why it's needed if there's a reason
**Risk:** Low - test CharacterCard hover/interaction states

#### 8. Review CharacterCard overflow-hidden
**File:** `character-card.styles.ts:113`
**Current:** Has `overflow-hidden` which clips the ascendancy background image
**Question:** Is this intentional for the background image, or was it added to "fix" shadow clipping?
**Options:**
- Keep if needed for background image clipping
- Remove if it was a shadow workaround (since card-shadow uses filter now)
**Risk:** Low - test visual appearance of CharacterCard

#### 9. Remove backdrop-blur from CharacterCard buttons
**File:** `character-card.styles.ts:161,163`
**Current:**
```ts
actionButton: 'bg-stone-800/80 backdrop-blur-sm',
deleteButton: '...bg-stone-800/80 backdrop-blur-sm',
```
**Change:** Remove `backdrop-blur-sm` - buttons are on an opaque card, blur adds no value
**Rationale:** `backdrop-blur` creates stacking contexts and has performance cost. These buttons are on an opaque `bg-stone-900` card background, so there's nothing to blur.
**Risk:** None - visual change should be imperceptible

---

### LOW - Theme Consistency

#### 10. Migrate zinc-* to stone-*
**Files:** See Appendix for full list
**Priority order:**
1. `zone-details-modal.tsx` (30+ usages, high visibility)
2. `form-sort-select.styles.ts` and `form-filter-toggle.styles.ts` (UI controls)
3. `economy.tsx` (page-level)
4. Remaining files
**Risk:** Low - visual change from zinc (cool gray) to stone (warm gray)

---

## Implementation Status

| # | Item | Status |
|---|------|--------|
| 1 | StatusBar z-50 | **DONE** - Added z-30 (per new scale) |
| 2 | .card-shadow GPU hints | **KEPT** - Required for stability (see docs) |
| 3 | .app-background min-h-screen | **DONE** |
| 4 | Content wrapper relative | **DONE** |
| 5 | Content wrapper height | **DONE** - h-[calc(100vh-52px)] |
| 6 | Z-index scale | **DONE** - Implemented and documented in patterns.md |
| 7 | CharacterCard z-10 | **DONE** - Removed (not needed) |
| 8 | CharacterCard overflow-hidden | **KEPT** - Required for ascendancy bg clipping |
| 9 | CharacterCard backdrop-blur | **DONE** - Removed from buttons |
| 10 | Theme migration (zinc→stone) | **PARTIAL** - Form styles migrated, more remain |

---

## Success Criteria Evaluation

| Criteria | Status | Notes |
|----------|--------|-------|
| Shadows work with box-shadow | **Blocked** | Requires compositing layer; see experiments above |
| No intermittent rendering | **Achieved** | GPU hints (will-change + transform) are REQUIRED, not over-engineering |
| Documented stacking context strategy | **Now Documented** | This audit |
| Minimal compositing layers | **Acceptable** | GPU hints are necessary; may optimize by moving to content wrapper |
| Clean overflow handling | **Resolved** | overflow-auto was not the cause; structural fixes applied |
| Consistent z-index scale | **Partial** | StatusBar fixed; full scale refactor still needed |

---

## Appendix: Full File References

### Stacking Context Files
- `/packages/frontend/src/routes/__root.tsx`
- `/packages/frontend/src/globals.css`
- `/packages/frontend/src/components/layout/window-title/window-title.styles.ts`
- `/packages/frontend/src/components/layout/sidebar-navigation/sidebar-navigation.styles.ts`
- `/packages/frontend/src/components/status/status-bar/status-bar.styles.ts`
- `/packages/frontend/src/components/ui/card/card.styles.ts`
- `/packages/frontend/src/components/ui/modal/modal.styles.ts`
- `/packages/frontend/src/components/character/character-card/character-card.styles.ts`

### Zinc Migration Files
- `/packages/frontend/src/routes/economy.tsx`
- `/packages/frontend/src/routes/settings.tsx`
- `/packages/frontend/src/components/zones/zone-details-modal/zone-details-modal.tsx`
- `/packages/frontend/src/components/zones/zone-list/zone-list.tsx`
- `/packages/frontend/src/components/forms/form-sort-select/form-sort-select.styles.ts`
- `/packages/frontend/src/components/forms/form-filter-toggle/form-filter-toggle.styles.ts`
- `/packages/frontend/src/components/economy/top-items-card/top-items-card.tsx`
- `/packages/frontend/src/components/charts/class-distribution-chart/class-distribution-chart.tsx`
- `/packages/frontend/src/components/charts/act-distribution-chart/act-distribution-chart.tsx`
- `/packages/frontend/src/utils/text-parser.tsx`
