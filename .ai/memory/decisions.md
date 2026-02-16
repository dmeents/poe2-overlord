# Architecture Decisions

## ADR-004: AppEvent Domain-Infrastructure Coupling

**Date:** 2026-02-15

**Status:** Accepted

**Context:**
During the backend infrastructure audit, we identified that domain services directly construct `AppEvent` variants (an infrastructure type defined in `infrastructure/events/types.rs`). This creates a bidirectional dependency between the domain and infrastructure layers, which violates typical clean architecture principles.

**Trade-offs Considered:**

**Option 1: Move AppEvent to a shared types location**
- Pros: Clear separation, no circular dependencies
- Cons: Large refactor, event bus and domain both import from shared layer, breaks current layering

**Option 2: Create domain event types and map to AppEvent**
- Pros: Clean separation, domain layer independent
- Cons: Significant boilerplate, double the event definitions, mapping logic in infrastructure

**Option 3: Accept the coupling with factory methods**
- Pros: Minimal code, pragmatic for application-level architecture
- Cons: Domain depends on infrastructure (but indirectly, via factory methods)

**Decision:**
Accept the domain-to-infrastructure coupling for `AppEvent` with the following constraints:
- Domain services call `event_bus.publish(AppEvent::factory_method(...))` using factory methods
- All `AppEvent` variants have factory methods (e.g., `character_updated()`, `server_status_changed()`)
- Domain services never construct `AppEvent` directly with struct literals
- The coupling is one-directional: domain → infrastructure (not bidirectional)
- This pattern is already established and working well in practice

**Rationale:**
1. This is an application, not a library - the pragmatic benefits outweigh theoretical purity
2. The factory methods already exist and provide a clean API
3. The coupling is minimal and explicit
4. Alternative approaches would add significant complexity with little practical benefit
5. The event bus is a cross-cutting concern that both layers need to interact with

**Consequences:**
- Domain layer imports `infrastructure::events::AppEvent`
- Domain layer imports `infrastructure::events::EventBus`
- The pattern is explicit and well-documented
- Future refactoring to separate layers is possible if needed, but not prioritized

**Related Code:**
- `infrastructure/events/types.rs` - AppEvent definition and factory methods
- `domain/*/service.rs` - Domain services that publish events

---

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

## ADR-003: Shared Theme Package Architecture

**Date:** 2026-02-15

**Status:** Accepted

**Context:**
The POE2 Overlord project needed a marketing website for downloads and documentation. To maintain visual consistency between the desktop app (Tauri + React) and the website (Next.js), we needed to share design tokens and theme utilities without duplicating code.

**Decision:**
Created `@poe2-overlord/theme` as a shared package containing:
- **CSS tokens** (`tokens.css`) - `@theme` block extracted from frontend's `globals.css`
- **JS utilities** - `cn()` (classname merging) and `getThemeHexColor()` (CSS variable reader)
- **No build step** - Consumed as TypeScript source via Vite (frontend) and `transpilePackages` (website)

**Architecture:**
```
@poe2-overlord/theme        (no workspace deps)
    |
    +--- @poe2-overlord/frontend   (Tauri app)
    |        |
    |        +--- @poe2-overlord/backend
    |
    +--- @poe2-overlord/website    (Next.js)
```

**Font Loading Per Platform:**
- **Desktop (frontend)**: Google Fonts CSS imports in `globals.css` (already bundled in Tauri)
- **Website**: `next/font/google` for automatic optimization and self-hosting

**WebKitGTK Workarounds:**
Shadow utility classes (`.card-shadow`, `.chrome-shadow-*`) remain in frontend's `globals.css` since they're only needed for WebKitGTK compositor bugs (ADR-002). The website uses standard `box-shadow` without issues.

**What Moved to Theme:**
- All color scales, font families, shadow definitions, layout spacing
- Generic utilities: `cn()` and `getThemeHexColor()`

**What Stayed in Frontend:**
- Game-domain utilities: `class-colors.ts`, `league-colors.ts`, `act-colors.ts`
- WebKitGTK shadow utility classes
- App-specific styles (scrollbar, select focus, `.app-background`)

**Consequences:**
- Visual consistency between desktop and web
- Single source of truth for design tokens
- No build step - faster dev experience
- Website can use standard CSS features without WebKitGTK constraints

**Related Files:**
- `packages/theme/` - Shared theme package
- `packages/frontend/src/globals.css` - Now imports tokens from theme
- `packages/website/` - Next.js site using shared theme
- `.ai/tasks/prd-website-and-shared-theme.md` - Implementation PRD

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
