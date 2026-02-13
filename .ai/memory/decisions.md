# Architecture Decisions

## ADR-002: Filter-Based Shadows for WebKitGTK Compatibility

**Date:** 2026-02-12

**Status:** Accepted

**Context:**
Tauri on Linux uses WebKitGTK as its rendering engine. We discovered a compositor bug where mixing `box-shadow` (paint phase) with `filter: drop-shadow()` (composite phase) causes shadows to render inconsistently or disappear entirely. The specific trigger was adding `box-shadow` to an element inside a parent that uses `filter: drop-shadow()`, which would cause unrelated elements using `box-shadow` to lose their shadows.

**Investigation:**
- Cards used `filter: drop-shadow()` with GPU hints
- Chrome elements (sidebar, titlebar, statusbar) used `box-shadow` via `shadow-*` classes
- Adding `box-shadow` to a child of a `filter` parent caused all `box-shadow` elements to break
- The issue was intermittent/inconsistent, suggesting compositor timing issues
- Removing `backdrop-blur-sm` and `background-attachment: fixed` did not fix the issue
- Unifying all shadows to `filter: drop-shadow()` resolved the issue

**Decision:**
- Use `filter: drop-shadow()` exclusively for all shadows in the application
- Never mix `box-shadow` with `filter: drop-shadow()` in the same render tree
- All shadow-casting elements must include GPU layer hints (`will-change: filter` + `transform: translateZ(0)`)
- Created `chrome-shadow-*` utility classes in `globals.css` for directional shadows on fixed chrome

**Shadow Classes:**
- `.card-shadow` - General card shadows (existing)
- `.chrome-shadow-top` - Upward shadow (statusbar, footers)
- `.chrome-shadow-right` - Rightward shadow (sidebar)
- `.chrome-shadow-bottom` - Downward shadow (titlebar)

**Consequences:**
- Consistent shadow rendering across the application
- Slightly higher GPU memory usage (each shadow element gets its own compositing layer)
- Cannot use Tailwind's built-in `shadow-*` utilities for actual shadows (they use `box-shadow`)
- The `shadow-top`, `shadow-right`, etc. theme variables are now only useful for non-WebKitGTK contexts

**Related Files:**
- `globals.css` - Shadow class definitions
- `.ai/sessions/2026-02-12-dom-compositing-layer-audit.md` - Full investigation
- `.ai/archive/completed-prds/2026-02-12-prd-compositing-layer-root-cause.md` - PRD

---

## ADR-001: Centralized Theme System

**Date:** 2025-02-12

**Status:** Accepted

**Context:**
The application needed a consistent design language with colors extracted from the logo and background assets (volcanic/infernal aesthetic).

**Decision:**
- Define all design tokens in `globals.css` `@theme` block (single source of truth)
- Use semantic color names (ember, molten, blood, bone, stone, ash) instead of generic Tailwind defaults
- Define custom shadows with high opacity for dark backgrounds
- Apply background image with gradient overlay using CSS multiple backgrounds (no z-index needed)

**Consequences:**
- All design token changes can be made in one place (`globals.css`)
- Consistent visual language across components
- Component `.styles.ts` files use Tailwind classes that reference theme tokens
- No need for a separate JS theme file - everything is CSS-native
