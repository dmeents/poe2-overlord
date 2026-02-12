# PRD: DOM Structure & Compositing Layer Audit

## Status

**Completed** - Audit performed 2026-02-12. Structural fixes implemented. Root cause investigation deferred.

See:
- `.ai/sessions/2026-02-12-dom-compositing-layer-audit.md` - Detailed findings
- `.ai/tasks/prd-compositing-layer-root-cause.md` - Follow-up investigation (deferred)

---

## Problem Statement

During implementation of card shadows, we discovered that `box-shadow` CSS properties were completely invisible on card components, while `filter: drop-shadow()` worked correctly. Further investigation revealed that shadows were intermittently appearing/disappearing during route navigation until GPU compositing hints (`will-change`, `transform: translateZ(0)`) were added.

This indicates fundamental issues with the application's DOM structure, stacking contexts, and compositing layers that are causing unpredictable rendering behavior.

## Background & Findings

### The Shadow Investigation

**Symptoms:**
- `box-shadow` with inline styles showed in DevTools but was completely invisible on screen
- Even extreme test values (solid blue, large spread) produced no visible shadow
- `filter: drop-shadow()` worked immediately
- After switching to `drop-shadow()`, shadows flickered intermittently during route navigation

**Root Cause Analysis:**

The difference between `box-shadow` and `filter: drop-shadow()` is *when* they render in the browser pipeline:

| Property | Render Phase | Behavior |
|----------|--------------|----------|
| `box-shadow` | Paint phase | Subject to clipping by ancestor stacking contexts and overflow |
| `filter: drop-shadow()` | Composite phase | Applied after painting, can "escape" clipping constraints |

**Current DOM Structure (simplified):**
```
<div class="app-background">                          // Background image + gradient
  <div class="fixed z-50 ... backdrop-blur-sm">       // WindowTitle - creates stacking context
  <div class="fixed z-50 ... backdrop-blur-sm shadow-right">  // Sidebar - creates stacking context
  <div class="overflow-auto">                         // Main content - CLIPPING BOUNDARY
    <div class="min-h-screen">                        // PageLayout
      <div class="grid">                              // Grid layout
        <Card />                                      // box-shadow invisible here
```

**Identified Issues:**

1. **`backdrop-blur-sm` creates stacking contexts** - Both the title bar and sidebar use this, creating isolated compositing layers

2. **`overflow-auto` clips box-shadows** - The main content wrapper clips any box-shadows that extend beyond its bounds

3. **Fixed positioning with high z-index** - Creates layer separation between fixed UI elements and scrollable content

4. **Intermittent compositing** - Without explicit GPU layer hints, the browser inconsistently promotes elements to compositing layers during navigation

### Workaround Applied

```css
.card-shadow {
  filter: drop-shadow(var(--shadow-md));
  will-change: filter;
  transform: translateZ(0);
}
```

This works but is a band-aid, not a fix. It forces GPU compositing which has performance implications and doesn't address the underlying structural issues.

## Scope of Investigation

### Phase 1: DOM Structure Audit

**Objective:** Map all stacking contexts and compositing layers in the application

**Tasks:**
1. Document every element that creates a new stacking context:
   - `position: fixed/absolute/relative` with `z-index`
   - `opacity` < 1
   - `transform` (any value except none)
   - `filter` (any value except none)
   - `backdrop-filter`
   - `perspective`
   - `clip-path`
   - `mask`/`mask-image`
   - `mix-blend-mode`
   - `isolation: isolate`
   - `will-change` (specifying any property that creates stacking context)
   - `contain: layout/paint/strict/content`

2. Create visual diagram of stacking context hierarchy

3. Identify which contexts are necessary vs. accidental

### Phase 2: Overflow & Clipping Audit

**Objective:** Identify all clipping boundaries and their impact

**Tasks:**
1. Find all elements with `overflow: hidden/auto/scroll`
2. Document what content is being clipped (intentionally or not)
3. Identify alternatives that don't create clipping (e.g., `overflow: visible` with scrollbar styling)

### Phase 3: Fixed/Absolute Positioning Audit

**Objective:** Review all fixed and absolutely positioned elements

**Tasks:**
1. Catalog all fixed elements (titlebar, sidebar, statusbar, modals)
2. Review z-index values for consistency and necessity
3. Identify if any fixed elements could be `sticky` instead
4. Check for z-index "arms race" anti-patterns

### Phase 4: Compositing Layer Analysis

**Objective:** Understand GPU layer promotion and its performance impact

**Tasks:**
1. Use Chrome DevTools "Layers" panel to visualize compositing layers
2. Identify elements being promoted to their own layers
3. Check for "layer explosion" (too many layers)
4. Measure paint/composite performance before and after changes

### Phase 5: CSS Architecture Review

**Objective:** Ensure styling patterns are consistent and maintainable

**Tasks:**
1. Audit use of `backdrop-filter` - is it necessary everywhere?
2. Review shadow implementation across all components
3. Check for hardcoded values vs. theme variables
4. Identify any remaining `zinc-*` colors (should be `stone-*`)
5. Review transition/animation properties that might force compositing

## Specific Areas to Investigate

### Root Layout (`__root.tsx`)
- Is `overflow-auto` on the content wrapper necessary?
- Could scrolling be handled differently?
- What's the z-index strategy for fixed elements?

### Sidebar Navigation
- `backdrop-blur-sm` creates stacking context - is blur necessary?
- `shadow-right` uses `box-shadow` and works - why? (fixed + z-50?)
- Could this be simplified?

### Window Title Bar
- `backdrop-blur-sm` here too - consistent with sidebar or redundant?
- `fixed top-0` with `z-50` - appropriate?

### Card Components
- Current workaround uses `filter` + `will-change` + `transform`
- What's the performance cost?
- Is there a cleaner solution?

### Modal System
- How do modals interact with the stacking contexts?
- Are there z-index conflicts?

## Success Criteria

1. **Shadows work with `box-shadow`** - No need for `filter` workaround
2. **No intermittent rendering** - Consistent appearance without GPU hints
3. **Documented stacking context strategy** - Clear rationale for each context
4. **Minimal compositing layers** - Only what's necessary for performance
5. **Clean overflow handling** - No unintended clipping
6. **Consistent z-index scale** - Documented and enforced

## Deliverables

1. **Stacking Context Map** - Visual diagram of all contexts
2. **Recommendations Document** - Proposed structural changes
3. **Refactored Root Layout** - Cleaner DOM structure
4. **Updated patterns.md** - Document the "why" for future reference
5. **Performance Baseline** - Before/after metrics for paint/composite

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking existing layouts | High | Incremental changes with visual regression testing |
| Performance regression | Medium | Measure before/after with DevTools |
| Browser compatibility | Low | Test in Tauri's WebView (Chromium-based) |
| Scope creep | Medium | Strict phase boundaries |

## Open Questions

1. Is the `backdrop-blur` effect on titlebar/sidebar a design requirement or nice-to-have?
2. What's the minimum z-index needed for fixed elements in a Tauri app?
3. Should we consider a CSS-in-JS solution that handles stacking automatically?
4. Are there Tauri-specific WebView quirks affecting compositing?

## References

- [CSS Stacking Context (MDN)](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_positioned_layout/Understanding_z-index/Stacking_context)
- [Compositor Layers (Chrome DevTools)](https://developer.chrome.com/docs/devtools/rendering/performance#layers)
- [CSS Triggers](https://csstriggers.com/) - What CSS properties trigger layout/paint/composite
- [GPU Compositing in Chrome](https://www.chromium.org/developers/design-documents/gpu-accelerated-compositing-in-chrome/)
