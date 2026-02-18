# Architecture Decisions

## ADR-007: SQLite Migration for Data Persistence

**Date:** 2026-02-17

**Status:** Accepted

**Context:**
The application originally used JSON files for all data persistence (character data, configuration, zone metadata, server status). This approach worked but had limitations:
- No relational queries or joins
- No referential integrity constraints
- No transactions (atomicity issues)
- Character data stored as monolithic blobs (profile + zone stats + walkthrough progress in one file)
- Manual index management with orphan detection/cleanup logic

Since the app hasn't shipped yet, now was the ideal time to migrate to SQLite before the data model calcified around JSON limitations.

**Decision:**
Migrated all backend data persistence from JSON files to SQLite using sqlx 0.8:

**Migrated Domains:**
1. **Configuration** (`app_config` table) - Single-row configuration with CHECK(id=1) constraint
2. **Server Monitoring** (`server_status` table) - Single-row status tracking
3. **Zone Configuration** (`zone_metadata` table) - Integer surrogate keys, JSON TEXT columns for lists
4. **Character Domain** (normalized into 3 tables):
   - `characters` - Profile + timestamps + current_location reference
   - `zone_stats` - Zone tracking data (references `zone_metadata` by ID)
   - `walkthrough_progress` - Campaign progress

**NOT Migrated:**
- **Economy cache** - Remains as JSON files (TTL cache, not relational data)
- **Walkthrough guide** - Remains as bundled read-only JSON (`config/walkthrough_guide.json`)
- **Game monitoring** - Runtime state only, no persistence
- **Log analysis** - Reads game log file, no own persistence

**Schema Design Patterns:**
- **Integer surrogate keys** for `zone_metadata` (better join performance than zone_name)
- **Foreign keys** with appropriate cascade behaviors (DELETE CASCADE, RESTRICT, SET NULL)
- **Single-row tables** use `CHECK(id=1)` + `INSERT OR REPLACE` pattern
- **JSON TEXT columns** for Vec<String> fields in zone_metadata (bosses, monsters, npcs, etc.)
- **TrackingSummary** computed on demand from `zone_stats` (not stored)
- **Partial unique indexes** for business constraints (e.g., at-most-one active character)

**Database Configuration:**
- WAL journal mode (concurrent reads + writes)
- `synchronous = Normal`
- `foreign_keys = ON`
- `max_connections = 5`
- `busy_timeout = 5s`
- DB file: `AppPaths::data_dir().join("poe2-overlord.db")`

**Shared Helpers:**
Created `infrastructure/database/helpers.rs` with zone ID lookup functions:
- `get_or_create_zone_id()` - Generic executor (read-only)
- `get_or_create_zone_id_tx()` - Transaction-aware (creates stub zones)
- `get_or_create_zone_id_pool()` - Pool-aware (creates stubs)

Pattern: When a character enters an unknown zone, auto-create a stub `zone_metadata` row with just zone_name + timestamps. WikiScrapingService fills in metadata later.

**Transaction Boundaries:**
All multi-step operations wrapped in SQLite transactions:
- Character creation: INSERT character + INSERT walkthrough_progress (atomic)
- Character data save: UPDATE character + UPSERT zone_stats + UPSERT walkthrough_progress
- Set active character: UPDATE all is_active=0 + UPDATE target is_active=1
- Zone configuration save: DELETE all + INSERT all (replaces entire config atomically)

**Repository Pattern:**
Each SQLite repository implements the existing trait, maintaining the abstraction boundary:
- `CharacterSqliteRepository` implements `CharacterRepository`
- `ConfigurationSqliteRepository` implements `ConfigurationRepository`
- `ZoneConfigurationSqliteRepository` implements `ZoneConfigurationRepository`
- `ServerStatusSqliteRepository` implements `ServerStatusRepository`

Services use dependency injection (no changes to service layer needed).

**Error Handling:**
Added `From<sqlx::Error>` impl to `AppError` with SQLite error code mapping:
- Code 2067/1555 (UNIQUE constraint) → `AppError::Validation`
- Code 787 (Foreign key constraint) → `AppError::Validation`
- All other database errors → `AppError::Internal`

Also added `From<chrono::ParseError>` for RFC3339 timestamp parsing.

**Consequences:**

**Benefits:**
- **Relational queries**: Can JOIN zone_stats with zone_metadata for enriched data
- **Referential integrity**: Foreign keys prevent orphaned zone_stats
- **Transactions**: Atomic multi-step operations (no manual rollback logic needed)
- **Simplified services**: Eliminated manual index management, orphan cleanup, and reconciliation logic
- **Better performance**: Batch queries avoid N+1, indexes optimize lookups
- **Future opportunities**: Can add pagination, filtering, server-side sorting pushed to SQL

**Trade-offs:**
- SQLite dependency (acceptable for desktop app, already using Tauri)
- Migration script needed for existing dev data (Phase 6)
- Slightly more complex repository implementations (but simpler service layer)

**Migration Strategy:**
Phase 6 will create a standalone migration script (`scripts/migrate-json-to-sqlite/`) to port existing JSON data. The script runs in one transaction for atomicity and is idempotent (checks if data exists before migrating). After use, the script can be deleted.

**Related Files:**
- `infrastructure/database/` - Pool, migrations, helpers
- `domain/*/sqlite_repository.rs` - SQLite implementations
- `infrastructure/database/migrations/001_initial_schema.sql` - Complete schema
- `.ai/tasks/prd-sqlite-migration.md` - Implementation PRD
- `.ai/memory/patterns.md` - sqlx repository pattern

---

## ADR-006: React Portals for Fixed-Position Dropdowns

**Date:** 2026-02-16

**Status:** Accepted

**Context:**
After refactoring the filter/sort components, dropdowns were not positioning correctly. Investigation revealed the root cause: CSS transforms create a new containing block for `position: fixed` descendants.

The `.card-shadow` class (used on Card components) includes `transform: translateZ(0)` for GPU layer promotion to work around WebKitGTK compositor bugs. This transform causes any `position: fixed` dropdown inside the Card to be positioned relative to the Card instead of the viewport, breaking the positioning logic.

**Problem:**
- `useDropdownPosition` hook calculates viewport-relative coordinates using `getBoundingClientRect()`
- These coordinates are correct for `position: fixed` in a normal context
- But CSS transforms on parent elements change the containing block for fixed-position descendants
- Result: Dropdowns appear in incorrect locations

**Decision:**
All dropdowns with `position: fixed` must be rendered using React portals to `document.body`. This ensures they escape the Card's transformed container and position correctly relative to the viewport.

A shared `DropdownPortal` component (`components/ui/dropdown-portal`) was created to eliminate code duplication and centralize portal logic.

**Changes:**
1. **DropdownPortal component**: Created shared component handling portal rendering, SSR checks, and positioning
2. **ListControlBar**: Filters dropdown uses `DropdownPortal`
3. **Select**: Dropdown variant uses `DropdownPortal`
4. **SortSelect**: Sort options dropdown uses `DropdownPortal`
5. **Select positioning**: Removed incorrect `window.scrollY/scrollX` offsets (not needed for fixed positioning)
6. **Select click-outside**: Updated to check both `triggerRef` and `dropdownRef` (since dropdown is now outside DOM hierarchy)

**Consequences:**

**Benefits:**
- Dropdowns position correctly regardless of parent transforms
- Consistent with existing pattern (Tooltip component already uses portals)
- Dropdowns can never be clipped by parent `overflow: hidden`
- Z-index stacking is simpler (portals at body level)

**Trade-offs:**
- Slightly more complex component code (portal + typeof document check for SSR)
- Click-outside detection requires checking trigger separately
- Dropdown ref must be on the portal element, not container

**See Also:**
- ADR-002 (WebKitGTK shadow bug that necessitated the transforms in the first place)
- CSS Spec: Transforms create containing blocks for fixed-position descendants

## ADR-005: Walkthrough Guide Array-Based Navigation

**Date:** 2026-02-16

**Status:** Accepted

**Context:**
The walkthrough guide system used HashMap-based structures (`HashMap<String, WalkthroughAct>`, `HashMap<String, WalkthroughStep>`) with explicit linked-list navigation (`next_step_id`, `previous_step_id`). This structure had three major issues:

1. **Fragile navigation**: Manually wiring step IDs created opportunities for broken references (e.g., `act_4_step_18` key with `act_4_step_22` ID)
2. **Field redundancy**: Objectives had both `notes` and `details` fields serving the same purpose
3. **Hardcoded URLs**: `wiki_items` were plain strings requiring hardcoded URL construction in frontend utils

This structure was also problematic for future custom guide creator UIs, as editors would need to manually maintain fragile reference chains.

**Decision:**
Refactored walkthrough guide to use ordered arrays with implicit navigation:

**Data Structure Changes:**
- `acts: HashMap<String, WalkthroughAct>` → `acts: Vec<WalkthroughAct>`
- `steps: HashMap<String, WalkthroughStep>` → `steps: Vec<WalkthroughStep>`
- Removed `next_step_id`, `previous_step_id` from `WalkthroughStep`
- Removed `act_number` from `WalkthroughAct` (implicit in array position)
- Removed `notes` from `Objective` (merged into `details`)
- Replaced `wiki_items: Vec<String>` with `links: Vec<StepLink>` where `StepLink { text: String, url: String }`

**Navigation Helpers:**
Added methods to `WalkthroughGuide` (Rust) and utility functions (TypeScript):
- `find_step(step_id)` - Locates step by ID across all acts
- `next_step_id(step_id)` - Computes next step (handles cross-act boundaries)
- `previous_step_id(step_id)` - Computes previous step (handles cross-act boundaries)
- `step_exists(step_id)` - Validates step existence
- `first_step_id()` - Gets first step in guide

**Consequences:**

**Benefits:**
- **Impossible to misconfigure**: Array ordering prevents broken references
- **Simpler structure**: No manual ID wiring, no sorting needed in UI
- **Explicit links**: URLs are part of the data, not hardcoded logic
- **Future-proof**: Guide creator UIs can work with simple ordered lists
- **Type safety**: `StepLink` provides structure for external resources

**Trade-offs:**
- Navigation helpers do O(n) linear search instead of O(1) HashMap lookup
  - Acceptable: ~87 steps total, lookups happen on user navigation (not hot path)
  - Cache-friendly: Acts/steps are small and sequential in memory
- Step IDs still exist but are only used for saved progress references
- One-time migration required for existing JSON data

**Migration:**
- Created Node.js migration script to transform existing `walkthrough_guide.json`
- Merged 9 non-empty `notes` into `details` fields
- Converted 330 `wiki_items` to structured `links`
- Fixed `act_4_step_18` ID mismatch bug during migration

**Related Files:**
- Backend: `domain/walkthrough/models.rs`, `domain/walkthrough/service.rs`
- Frontend: `types/walkthrough.ts`, `utils/walkthrough.ts`, `contexts/WalkthroughContext.tsx`
- Components: All walkthrough components updated to use array iteration
- Data: `backend/config/walkthrough_guide.json` (transformed structure)
- Migration: `migrate-walkthrough.js` (one-time script, backed up original)

**Related Documentation:**
- `.ai/sessions/2026-02-16-walkthrough-refactor.md` - Implementation session

---

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
