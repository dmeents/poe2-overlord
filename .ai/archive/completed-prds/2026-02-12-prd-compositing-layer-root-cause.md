# PRD: Compositing Layer Root Cause Investigation

## Status
**Completed** - Root cause identified and fixed (2026-02-12)

## Summary

**Root Cause:** WebKitGTK (Tauri's rendering engine on Linux) has a compositor bug when mixing `box-shadow` (paint phase) with `filter: drop-shadow()` (composite phase). When both shadow types coexist, especially when `box-shadow` is added inside an element with `filter: drop-shadow()` on a parent, the compositor incorrectly handles layer ordering, causing shadows to disappear or render inconsistently.

**Solution:** Standardize all shadows to use `filter: drop-shadow()` with GPU hints. Never mix shadow types.

---

## Background

This PRD follows from the DOM Structure & Compositing Layer Audit (2026-02-12). The original audit identified that `box-shadow` is invisible on card components while `filter: drop-shadow()` with GPU hints works.

## The Discovery

While attempting to add `shadow-sm` (a `box-shadow`) to the character card footer, we observed:
1. Shadows on sidebar, titlebar, and statusbar would disappear
2. The behavior was intermittent - sometimes shadows loaded, sometimes not
3. The footer shadow itself was not visible

## Investigation Process

### Experiments Tried (All Failed)

1. **Remove `backdrop-blur-sm` from chrome elements** - No improvement
2. **Remove `background-attachment: fixed`** - No improvement
3. **Add `transform-gpu` to chrome elements** - No improvement
4. **Use `filter: drop-shadow()` on footer only** - No improvement

### The Breakthrough

The key insight was that we were **mixing shadow types**:
- Cards used `filter: drop-shadow()` (composite phase)
- Chrome elements used `box-shadow` via Tailwind's `shadow-*` classes (paint phase)

When we unified ALL shadows to use `filter: drop-shadow()`, everything worked consistently.

## Technical Explanation

### Why Mixing Shadow Types Fails in WebKitGTK

1. **`box-shadow`** is rendered during the **paint phase** - it's painted onto the element's layer
2. **`filter: drop-shadow()`** is rendered during the **composite phase** - it's applied after painting

When both types coexist in the render tree, WebKitGTK's compositor makes inconsistent decisions about layer ordering and shadow rendering. Adding a `box-shadow` to an element inside a `filter` parent triggers a compositor recalculation that can break `box-shadow` on completely unrelated elements.

### The Pattern

| Setup | Result |
|-------|--------|
| Only `filter: drop-shadow()` everywhere | Works |
| Only `box-shadow` everywhere | Untested (probably works) |
| Mixed shadow types | Broken/inconsistent |

## Solution Implemented

### New Shadow Classes (`globals.css`)

```css
/* Card shadows */
.card-shadow {
  filter: drop-shadow(var(--shadow-md));
  will-change: filter;
  transform: translateZ(0);
}

/* Chrome shadows - filter-based for WebKitGTK compatibility */
.chrome-shadow-top {
  filter: drop-shadow(0 -4px 6px rgba(0, 0, 0, 0.7));
  will-change: filter;
  transform: translateZ(0);
}

.chrome-shadow-right {
  filter: drop-shadow(4px 0 6px rgba(0, 0, 0, 0.7));
  will-change: filter;
  transform: translateZ(0);
}

.chrome-shadow-bottom {
  filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.7));
  will-change: filter;
  transform: translateZ(0);
}
```

### Updated Components

| Component | Old Shadow | New Shadow |
|-----------|-----------|------------|
| Sidebar | `shadow-right` (box-shadow) | `chrome-shadow-right` (filter) |
| Titlebar | `shadow-bottom` (box-shadow) | `chrome-shadow-bottom` (filter) |
| StatusBar | `shadow-top` (box-shadow) | `chrome-shadow-top` (filter) |
| Card Footer | None | `chrome-shadow-top` (filter) |

## Key Takeaways

1. **Never mix `box-shadow` with `filter: drop-shadow()`** in a Tauri/WebKitGTK application
2. **Always include GPU hints** (`will-change: filter` + `transform: translateZ(0)`) with filter shadows
3. **Tailwind's `shadow-*` utilities use `box-shadow`** - don't use them alongside filter shadows
4. **This is a WebKitGTK-specific issue** - may not reproduce in Chromium-based browsers

## Related Files

- `.ai/memory/decisions.md` - ADR-002 documents this decision
- `.ai/memory/patterns.md` - Updated shadow guidelines
- `.ai/sessions/2026-02-12-dom-compositing-layer-audit.md` - Initial audit
- `packages/frontend/src/globals.css` - Shadow class definitions
