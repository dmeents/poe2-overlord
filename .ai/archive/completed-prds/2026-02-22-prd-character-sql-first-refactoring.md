# PRD: Character Domain SQL-First Refactoring

**Date:** 2026-02-22
**Status:** Completed
**Session:** `.ai/sessions/2026-02-22-character-sql-first-refactoring.md`

---

## Problem

The character domain was migrated from JSON to SQLite but retained the old
"load-everything, mutate-in-memory, save-everything" pattern. Every mutation
(zone change, death, level-up) loaded the full character aggregate (profile +
ALL zone stats + walkthrough progress), changed 1-2 fields, then wrote
everything back. For a character with 100 zones, a single death triggered 102+
SQL statements. The frontend compounded this by over-fetching full character
data (zones with 20+ fields each) for list views that only needed summaries.

## Solution

Shifted to targeted SQL operations for mutations. Full loads are kept only for
event publishing. Eliminated `ZoneTrackingService` entirely — its logic maps
cleanly to SQL. Added a lean `get_all_characters_summary` endpoint for list
views. Removed dual-state management from the frontend context.

## Phases Completed

### Phase 1: Granular Repository Methods (Backend)

Added targeted SQL methods to `CharacterRepository` trait and implemented them
in `CharacterSqliteRepository`:

- `transition_zone` — single atomic transaction: deactivates old zone (with
  elapsed time calc), activates new zone, updates `last_played`. Replaces
  5+ round trips + 102 UPSERTs per zone change.
- `record_death_in_active_zone` — single UPDATE on `zone_stats WHERE is_active=1`
- `update_character_level` — single UPDATE on `characters`
- `update_character_profile` — single UPDATE on `characters`
- `leave_active_zone` — compute elapsed time + UPDATE zone_stats
- `enter_zone` — UPSERT zone_stats via zone_id lookup
- `finalize_character_active_zones` — deactivates all active zones on shutdown
- `update_walkthrough_progress` — INSERT OR REPLACE on `walkthrough_progress`
- `get_active_zone_name` — single SELECT
- `add_zone_time` / `update_zone_metadata` — targeted zone_stats UPDATEs
- `load_all_characters_summary` — SQL `SUM()/COUNT()` aggregates, no zones array
- Added proper `FromStr`/`Display` impls for `CharacterClass`, `Ascendency`,
  `League` enums (replaced fragile `serde_json::from_str(&format!(...))` pattern)
- Removed unused `character_exists()` method

### Phase 2: Service Layer Refactoring

- **`CharacterServiceImpl`**: Each mutation now calls a single targeted repo
  method, then does one SELECT for event publishing. No more ZoneTrackingService.
- **`ZoneTrackingService` removed**: Deleted `zone_tracking/service.rs` and
  `zone_tracking/traits.rs`. Kept `zone_tracking/models.rs` (ZoneStats,
  TrackingSummary, is_hideout_zone) — still used by repository and enrichment.
- **`process_scene_change` triple-publish fix**: Was doing 5 loads + 3 saves +
  3 `CharacterUpdated` events per scene change. Now just calls
  `character_service.enter_zone()` which handles everything in one atomic
  transaction + one event. Removed ~75 lines of redundant load/leave/save/publish.
- **Walkthrough service**: Replaced `load_character_data` + mutate +
  `save_character_data` with `character_service.update_walkthrough_progress()`.
- **`CharacterService` trait**: Removed `load_character_data`, `save_character_data`,
  `get_characters_index`. Added `update_walkthrough_progress` and
  `get_all_characters_summary`.
- **Dead code removed**: `CharactersIndex`, `WalkthroughProgressUpdated` event
  variant, `LocationType::Act/Hideout` (only `Zone` was used),
  `LocationState::get_zone_name()`, `CharacterData::update_walkthrough_progress()`,
  `CharacterData::default()`.
- **`service_registry.rs`**: Removed `ZoneTrackingServiceImpl` creation and wiring.
- **`CharacterSummaryResponse` DTO**: New lean backend type returned by
  `get_all_characters_summary` — no zones array, has `is_active` bool.

### Phase 3: Frontend State Management Cleanup

- **`CharacterSummaryData` type** added to `types/character.ts` — mirrors
  `CharacterSummaryResponse` from backend. Same shape as `CharacterData` but
  without `zones: ZoneStats[]`, with `is_active: boolean` added.
- **`useCharacters()`** now calls `get_all_characters_summary` → returns
  `CharacterSummaryData[]`. No more zones array in list views.
- **`characterQueryKeys` exported** from `queries/characters.ts` so context
  can import it.
- **`CharacterContext.tsx` rewritten**: Removed `useState` dual-state mirrors and
  `useEffect` syncing. Events now update React Query cache directly:
  - `CharacterUpdated` → `setQueryData` on active char + `setQueryData` on list
    (converts full `CharacterData` to `CharacterSummaryData` via `toSummary()`,
    preserving `is_active` from existing list entry)
  - `CharacterDeleted` → `setQueryData` on list (filter) + clear active char
- **Type propagation**: Updated all components, specs, and hooks that used
  `CharacterData` for list-view characters to use `CharacterSummaryData`:
  `character-card.tsx`, `character-list.tsx`, `character-form-modal.tsx`,
  `delete-character-modal.tsx`, `character-insights.tsx`, `character-list.config.ts`,
  `routes/characters.tsx`, and all their `.spec.tsx` test files.
- **`character-status-card.tsx`**: Passes `{ ...activeCharacter, is_active: true }`
  to `CharacterCard` since `activeCharacter` is `CharacterData` (full type) but
  `CharacterCard` expects `CharacterSummaryData`.
- **`useMemo`** added for `classDistribution` and `leagueDistribution` in
  `routes/characters.tsx`.

## Performance Impact

| Operation | Before | After |
|-----------|--------|-------|
| Single zone change | 6 SELECTs + 102+ UPSERTs | 1 transaction (4 stmts) + 1 SELECT |
| Record a death | 3 SELECTs + 102 UPSERTs | 1 UPDATE + 1 SELECT |
| Level up | 3 SELECTs + 102 UPSERTs | 1 UPDATE + 1 SELECT |
| Scene change (log parser) | 5 loads + 3 saves + 3 events | 1 tx + 1 SELECT + 1 event |
| Character list fetch | Full CharacterData[] with zones | CharacterSummaryData[] (no zones) |
| Frontend event handling | useState mirror + useEffect sync | Direct queryClient.setQueryData |

## Verification

- Backend: 495/495 tests pass, no warnings
- Frontend: typecheck clean, same test baseline as pre-refactor
- Lint: only pre-existing issues in unmodified files
