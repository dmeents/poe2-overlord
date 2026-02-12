# PRD: Compositing Layer Root Cause Investigation

## Status
**Deferred** - Documented for future investigation

## Background

This PRD follows from the DOM Structure & Compositing Layer Audit (2026-02-12). The original audit identified that `box-shadow` is invisible on card components while `filter: drop-shadow()` with GPU hints works. The audit also revealed that the GPU hints (`will-change`, `transform: translateZ(0)`) are **required** to prevent shadow flickering during route navigation.

See: `.ai/sessions/2026-02-12-dom-compositing-layer-audit.md` for full audit findings.

## Problem Statement

The application requires GPU compositing hints on every card component to render shadows consistently. While this workaround is functional, it:
1. Increases GPU memory usage (each card gets its own compositing layer)
2. May impact performance on lower-end devices
3. Masks an underlying architectural issue that could cause other rendering problems

## Key Discovery

The sidebar uses `box-shadow` (via `shadow-right` class) and **it works correctly**. This revealed a pattern:

| Element | Has Own Compositing Layer? | Shadow Rendering |
|---------|---------------------------|------------------|
| Sidebar | Yes (via `backdrop-filter`) | `box-shadow` works |
| Titlebar | Yes (via `backdrop-filter`) | `box-shadow` works |
| Cards (no hints) | No | `box-shadow` invisible |
| Cards (filter only) | Intermittent | `drop-shadow` flickers |
| Cards (filter + GPU hints) | Yes (stable) | `drop-shadow` stable |

**Pattern**: Elements with their own compositing layer render shadows correctly. Elements without their own layer do not.

## Current Workaround

```css
/* globals.css */
.card-shadow {
  filter: drop-shadow(var(--shadow-md));
  will-change: filter;      /* Pre-allocate compositing layer */
  transform: translateZ(0); /* Guarantee layer persistence */
}
```

### Why All Three Properties Are Required

1. **`filter: drop-shadow()`** - Renders shadow during composite phase (avoids paint-phase issues)
2. **`will-change: filter`** - Hints browser to pre-allocate a compositing layer
3. **`transform: translateZ(0)`** - Forces GPU layer promotion to prevent intermittent demotion

Without `will-change` + `transform`, the browser inconsistently promotes/demotes layers during React re-renders and route navigation, causing shadows to flicker.

## Root Cause Hypotheses

### Hypothesis 1: `background-attachment: fixed` (HIGH probability)

```css
/* globals.css */
.app-background {
  background:
    linear-gradient(...),
    url("/background.png") center / cover no-repeat fixed; /* <- This */
}
```

`background-attachment: fixed` positions the background relative to the viewport, not the element. This creates unusual compositing behavior:
- The background may be rendered on a separate layer
- Paint operations on child elements may not composite correctly with the fixed background
- This is a known source of rendering quirks in Chromium

### Hypothesis 2: `backdrop-filter` on Fixed Elements (MEDIUM probability)

```ts
// window-title.styles.ts, sidebar-navigation.styles.ts, status-bar.styles.ts
'... backdrop-blur-sm ...'
```

`backdrop-filter` creates compositing layers that sample from layers "behind" them. The compositing order might cause:
- Content layer paint to be lost or overwritten
- Incorrect layer stacking during composition
- Race conditions between backdrop sampling and content rendering

### Hypothesis 3: Content Wrapper Lacks Compositing Layer (MEDIUM probability)

The content wrapper (`<main>` in `__root.tsx`) doesn't have its own compositing layer. This means:
- Card `box-shadow` is painted onto the parent's layer
- If that parent layer has compositing issues (due to H1 or H2), the paint is lost
- Giving the content wrapper its own layer might "shield" it from parent issues

## Experiments to Conduct

### Experiment A: Content Wrapper Compositing Layer

**Goal**: Test if giving the content wrapper its own compositing layer allows `box-shadow` on cards.

**Change**:
```tsx
// __root.tsx
<main className="relative h-[calc(100vh-52px)] mt-[28px] ml-12 overflow-auto font-sans transform-gpu">
```
Or in CSS:
```css
main {
  transform: translateZ(0);
}
```

**Test**:
1. Remove `card-shadow` class from a Card component
2. Add inline `box-shadow` style
3. Check if shadow is visible
4. Navigate between routes, check for flickering

**Success Criteria**: `box-shadow` renders consistently without per-card GPU hints.

**If Successful**: We can use a single GPU hint on the content wrapper instead of every card - cleaner architecture and less memory usage.

---

### Experiment B: Remove Fixed Background Attachment

**Goal**: Test if `background-attachment: fixed` is causing the compositing issue.

**Change**:
```css
/* globals.css - temporarily modify */
.app-background {
  background:
    linear-gradient(...),
    url("/background.png") center / cover no-repeat; /* Remove 'fixed' */
}
```

**Test**:
1. Remove `card-shadow` class from a Card component
2. Add inline `box-shadow` style
3. Check if shadow is visible

**Success Criteria**: `box-shadow` renders correctly.

**Trade-off**: The background will scroll with content instead of staying fixed. May need alternative solution if this is the cause (e.g., separate fixed background element).

---

### Experiment C: Remove Backdrop Blur from Fixed Elements

**Goal**: Test if `backdrop-filter` on fixed elements is causing the compositing issue.

**Change**:
```ts
// window-title.styles.ts
container: '... bg-stone-950 ...'  // Remove backdrop-blur-sm, use solid bg

// sidebar-navigation.styles.ts
container: '... bg-stone-950 ...'  // Remove backdrop-blur-sm, use solid bg

// status-bar.styles.ts
container: '... bg-stone-950 ...'  // Remove backdrop-blur-sm, use solid bg
```

**Test**:
1. Remove `card-shadow` class from a Card component
2. Add inline `box-shadow` style
3. Check if shadow is visible

**Success Criteria**: `box-shadow` renders correctly.

**Trade-off**: Loses the subtle blur effect on chrome elements. Visual change is minor since backgrounds are already 95% opaque.

---

### Experiment D: Combination Test

**Goal**: If individual experiments don't work, test combinations.

**Order**:
1. A alone → if fails...
2. A + B → if fails...
3. A + C → if fails...
4. A + B + C → if fails, root cause is elsewhere (Tauri WebView specific?)

## Implementation Notes

### If Experiment A Succeeds

Update `__root.tsx`:
```tsx
<main className="... transform-gpu">
```

Remove GPU hints from `.card-shadow`:
```css
.card-shadow {
  filter: drop-shadow(var(--shadow-md));
  /* will-change and transform no longer needed */
}
```

Or potentially switch back to `box-shadow`:
```css
.card-shadow {
  box-shadow: var(--shadow-md);
}
```

### If Experiment B Succeeds

Consider alternative fixed background approaches:
1. Separate `position: fixed` element for background
2. `::before` pseudo-element with fixed positioning
3. Accept scrolling background (may not match design intent)

### If Experiment C Succeeds

Options:
1. Remove backdrop-blur entirely (solid backgrounds)
2. Only use backdrop-blur on modal overlays
3. Investigate alternative blur implementations

## Success Criteria

1. **Primary**: `box-shadow` renders consistently on cards without per-card GPU hints
2. **Secondary**: Understand the exact technical cause for documentation
3. **Tertiary**: Reduce GPU memory usage from multiple compositing layers

## Resources

- [CSS Stacking Context (MDN)](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_positioned_layout/Understanding_z-index/Stacking_context)
- [Compositor Layers (Chrome DevTools)](https://developer.chrome.com/docs/devtools/rendering/performance#layers)
- [CSS Triggers](https://csstriggers.com/)
- [GPU Compositing in Chrome](https://www.chromium.org/developers/design-documents/gpu-accelerated-compositing-in-chrome/)
- [backdrop-filter and stacking contexts](https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter)

## Related Files

- `.ai/sessions/2026-02-12-dom-compositing-layer-audit.md` - Full audit report
- `packages/frontend/src/globals.css` - `.card-shadow` class, `.app-background`
- `packages/frontend/src/routes/__root.tsx` - Root layout structure
- `packages/frontend/src/components/layout/window-title/window-title.styles.ts`
- `packages/frontend/src/components/layout/sidebar-navigation/sidebar-navigation.styles.ts`
- `packages/frontend/src/components/status/status-bar/status-bar.styles.ts`
