# Character List Bug Investigation

**Date**: 2026-01-13
**Issue**: Character list not appearing on app start

## Symptoms

- Application starts successfully (no compilation errors)
- Character list appears empty
- Logs show warnings when trying to load characters:
  ```
  [WARN] Failed to load character ...: missing field `play_time_act5` at line 27 column 3
  ```
- All 4 characters fail to deserialize with the same error

## Investigation Steps

1. Started app with `yarn dev` and captured startup logs
2. Analyzed logs for errors and warnings
3. Found repeated warning pattern for all characters: `missing field 'play_time_act5'`
4. Searched codebase for `play_time_act5` to understand the schema change
5. Found the field was added to `TrackingSummary` struct in `zone_tracking/models.rs`
6. Identified the field lacked `#[serde(default)]` attribute

## Root Cause

The `play_time_act5` field was added to the `TrackingSummary` struct as part of a recent refactoring (see `.ai/sessions/2026-01-11-refactors-3-batch.md`), but the existing character data JSON files on disk did not have this field.

When serde tried to deserialize the JSON without the new field, it failed with:
```
missing field `play_time_act5`
```

The other `play_time_act*` fields (act1-act4) were added at struct creation, so all existing data already had them. Only `play_time_act5` was added later without backwards compatibility.

## Fix Applied

Added `#[serde(default)]` attribute to the `play_time_act5` field in `packages/backend/src/domain/zone_tracking/models.rs:120`:

```rust
    pub play_time_act4: u64,
    #[serde(default)]
    pub play_time_act5: u64,
    pub play_time_interlude: u64,
```

This tells serde to use the default value (0 for u64) when the field is missing during deserialization.

## Verification

1. **Backend tests**: All 512 tests pass
2. **Frontend tests**: All 568 tests pass
3. **Runtime verification**:
   - Started app with `yarn dev`
   - Logs show clean startup with no warnings
   - `finalize_all_active_zones called - found 4 characters to process` (was 0 before fix)
   - All 4 characters load and process successfully
   - `character-updated` events published for all characters
   - No `WARN` or `ERROR` messages in startup logs

## Lessons Learned

When adding new fields to serializable structs that are persisted to disk:
1. Always add `#[serde(default)]` to new fields for backwards compatibility
2. Consider migration strategy if default value is not appropriate
3. Test with existing data files, not just fresh data

## Related Commits

- Fix commit: (to be created)
- Original change that introduced the issue: Batch 3 refactoring (2026-01-11)
