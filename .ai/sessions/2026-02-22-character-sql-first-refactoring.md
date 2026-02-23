# Session: Character Domain SQL-First Refactoring

**Date:** 2026-02-22
**PRD:** `.ai/archive/completed-prds/2026-02-22-prd-character-sql-first-refactoring.md`
**ADR:** ADR-008 in `.ai/memory/decisions.md`

---

## Summary

Completed a full SQL-first refactoring of the character domain across two
sessions (ran out of context, continued from summary). All three phases
implemented and verified.

## Key Technical Decisions Made

### `transition_zone` as atomic transaction
The core insight: deactivate old zone + activate new zone + update
`last_played` in a single SQLite transaction. This was the highest-impact
change — every zone change went from 100+ statements to 4.

```rust
async fn transition_zone(&self, character_id: &str, zone_name: &str, ...) {
    let mut tx = self.pool.begin().await?;
    // 1. Compute elapsed for active zone, deactivate it
    // 2. Get or create zone_id (single lookup via get_or_create_zone_id_tx)
    // 3. Upsert new zone_stats row (visits++, is_active=1, entry_timestamp=now)
    // 4. UPDATE characters SET current_zone_id, last_played, last_updated
    tx.commit().await?;
}
```

### ZoneTrackingService deletion
Removed entirely. `zone_tracking/service.rs` and `zone_tracking/traits.rs`
deleted. `zone_tracking/models.rs` kept (ZoneStats, TrackingSummary,
is_hideout_zone — still referenced).

Service registry no longer creates `ZoneTrackingServiceImpl`. `CharacterServiceImpl`
no longer holds `zone_tracking: Arc<dyn ZoneTrackingService>`.

### `process_scene_change` triple-publish fix
The log analysis service was calling:
1. `load_character_data` (find active zone)
2. `leave_zone` (load + mutate + save + publish CharacterUpdated)
3. `enter_zone` (load + mutate + save + publish CharacterUpdated)
4. Load again, update last_played, save again
5. `get_character` again, publish CharacterUpdated again

Now it just calls `character_service.enter_zone()` which internally runs
`transition_zone` (handles leave+enter atomically) + loads for event + publishes
once. The `_event_bus` parameter in `process_scene_change_with_error_handling`
is now unused (kept for signature compatibility).

### CharacterSummaryData type design
`CharacterData` has `zones: ZoneStats[]` (full type, used for playtime/detail pages)
`CharacterSummaryData` has `is_active: boolean`, no zones (used for list views)

They're intentionally separate types, not a union. `CharacterCard` accepts
`CharacterSummaryData`. `CharacterStatusCard` spreads `activeCharacter` with
`is_active: true` since `activeCharacter` from context is always the active char.

### Event-driven cache updates pattern
```tsx
// CharacterContext.tsx
queryClient.setQueryData(characterQueryKeys.active(), prev =>
    prev?.id === character_id ? characterData : prev
);
queryClient.setQueryData(characterQueryKeys.lists(), prev =>
    prev?.map(c => c.id === character_id ? toSummary(characterData, c.is_active) : c)
);
```

The `toSummary(data, is_active)` helper converts `CharacterData` to
`CharacterSummaryData` while preserving the `is_active` flag from the existing
list entry (the event doesn't change who is active).

## Errors Encountered

1. **`unresolved imports ZoneTrackingService, ZoneTrackingServiceImpl`** in
   `domain/mod.rs` — fixed by updating `pub use zone_tracking::{...}` exports

2. **`unresolved import CharactersIndex`** in `domain/mod.rs` — fixed by
   replacing with `CharacterSummaryResponse`

3. **`WalkthroughProgressUpdated variant not found`** in `bridge.rs` — fixed by
   removing the match arm

4. **TypeScript spec files** — all specs using `CharacterData` mock fixtures
   needed `zones: []` removed and `is_active: false` added. Updated 5 spec
   files total.

5. **`character-status-card.tsx` type mismatch** — `activeCharacter` is
   `CharacterData` (no `is_active`), `CharacterCard` expects `CharacterSummaryData`
   (has `is_active`). Fixed with `{ ...activeCharacter, is_active: true }` spread.

6. **Biome formatter** — several files needed `pnpm biome check --write` after
   edits.

7. **Unused import warning** — `use crate::domain::walkthrough::models::WalkthroughProgress`
   left in `models_test.rs` after model cleanup. Removed.

## Files Changed

**Backend (Rust):**
- `domain/character/models.rs` — FromStr/Display for enums, CharacterSummaryResponse
- `domain/character/models_test.rs` — removed unused WalkthroughProgress import
- `domain/character/traits.rs` — new granular methods, removed load/save/index
- `domain/character/repository.rs` — implemented all granular SQL methods
- `domain/character/service.rs` — use granular methods, removed ZoneTrackingService
- `domain/character/commands.rs` — added get_all_characters_summary command
- `domain/walkthrough/service.rs` — use character_service.update_walkthrough_progress
- `domain/walkthrough/service_test.rs` — updated mock to new trait
- `domain/log_analysis/service.rs` — fixed triple-publish in process_scene_change
- `domain/zone_tracking/mod.rs` — removed service/traits exports
- `domain/mod.rs` — updated exports
- `infrastructure/events/types.rs` — removed WalkthroughProgressUpdated
- `infrastructure/events/bridge.rs` — removed WalkthroughProgressUpdated arm
- `application/service_registry.rs` — removed ZoneTrackingServiceImpl
- `lib.rs` — added get_all_characters_summary to invoke_handler

**Frontend (TypeScript/React):**
- `types/character.ts` — added CharacterSummaryData interface
- `queries/characters.ts` — useCharacters → get_all_characters_summary, export characterQueryKeys
- `contexts/CharacterContext.tsx` — event-driven cache updates, no useState mirrors
- `hooks/configs/character-list.config.ts` — CharacterData → CharacterSummaryData
- `components/character/character-card/character-card.tsx` — CharacterSummaryData prop
- `components/character/character-card/character-card.spec.tsx` — updated fixtures
- `components/character/character-list/character-list.tsx` — CharacterSummaryData
- `components/character/character-list/character-list.spec.tsx` — updated fixtures
- `components/character/character-form-modal/character-form-modal.tsx` — CharacterSummaryData prop
- `components/character/character-form-modal/character-form-modal.spec.tsx` — updated fixtures
- `components/character/delete-character-modal/delete-character-modal.tsx` — CharacterSummaryData prop
- `components/character/delete-character-modal/delete-character-modal.spec.tsx` — updated fixtures
- `components/character/character-status-card/character-status-card.tsx` — is_active spread
- `components/insights/character-insights/character-insights.tsx` — CharacterSummaryData
- `components/insights/character-insights/character-insights.spec.tsx` — updated fixtures
- `routes/characters.tsx` — CharacterSummaryData types, useMemo for distributions
