# Economy Domain SQLite Cache Migration

**Date:** 2026-02-17
**Type:** Infrastructure Migration
**Status:** ✅ Complete

## Objective

Migrate the economy domain from JSON file caching to SQLite, bringing it in line with all other domains in the codebase. The economy domain was the last holdout using `FileService` for persistence.

## Context

The economy domain caches currency exchange data from poe.ninja API with 10-minute TTL. It had unique requirements:
- Ephemeral data (rebuilds from API, no need to preserve old data)
- TTL-based freshness checking
- Cross-economy aggregation (top currencies, search across all types)
- Lifecycle management (`is_active` flag for manual deactivations)

## Implementation

### Files Created

1. **`packages/backend/src/infrastructure/database/migrations/002_economy_cache.sql`**
   - `economy_exchange_rates` - Parent table (league+type context)
   - `currency_items` - Child table with FK (ON DELETE CASCADE)
   - Indexes for optimal queries

2. **`packages/backend/src/domain/economy/traits.rs`**
   - `EconomyRepository` trait with 5 methods
   - TTL-aware and TTL-agnostic load methods
   - Cross-economy query methods (top/search)

3. **`packages/backend/src/domain/economy/repository.rs`**
   - `EconomyRepositoryImpl` with full SQLite implementation
   - TTL checking in Rust (not SQL)
   - Transaction-based upserts preserving `is_active`

### Files Modified

4. **`models.rs`** - Removed deprecated cache structs (`LeagueEconomyCache`, `CachedEconomyData`, `LeagueTopCurrenciesCache`)

5. **`mod.rs`** - Added repository/traits modules and re-exports

6. **`service.rs`** - Replaced `FileService` with repository DI, removed file I/O logic

7. **`service_registry.rs`** - Wired repository with DI

8. **`service_test.rs`** - Added `MockEconomyRepository`, updated tests

## Key Design Decisions

### 1. Parent-Child FK Relationship

```sql
CREATE TABLE economy_exchange_rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    league TEXT NOT NULL,
    is_hardcore INTEGER NOT NULL,
    economy_type TEXT NOT NULL,
    -- ... metadata ...
    UNIQUE(league, is_hardcore, economy_type)
);

CREATE TABLE currency_items (
    exchange_rate_id INTEGER NOT NULL
        REFERENCES economy_exchange_rates(id) ON DELETE CASCADE,
    currency_id TEXT NOT NULL,
    -- ... item data ...
    is_active INTEGER NOT NULL DEFAULT 1,
    UNIQUE(exchange_rate_id, currency_id)
);
```

**Rationale:** Single integer JOIN, referential integrity, automatic cleanup with CASCADE.

### 2. Direct Upserts (Not Snapshot Replacement)

Each currency item is individually upserted via `ON CONFLICT DO UPDATE`. No delete-and-reinsert.

**Benefit:** Preserves `is_active` flag on existing items (manual deactivations survive API refreshes).

### 3. `is_active` Omitted from UPDATE Clause

```sql
INSERT INTO currency_items (exchange_rate_id, currency_id, ..., is_active)
VALUES (?, ?, ..., 1)
ON CONFLICT(exchange_rate_id, currency_id) DO UPDATE SET
    name = excluded.name,
    primary_value = excluded.primary_value
    -- is_active intentionally omitted
```

**Critical:** If a user manually deactivates a currency, the next API refresh updates all fields BUT preserves the `is_active=0` state.

### 4. TTL Check in Rust (Not SQL)

```rust
fn is_fresh(last_updated: &str, ttl_seconds: u64) -> bool {
    chrono::DateTime::parse_from_rfc3339(last_updated)
        .ok()
        .map(|time| {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(time.with_timezone(&Utc));
            elapsed.num_seconds() < ttl_i64
        })
        .unwrap_or(false)
}
```

**Rationale:** SQLite datetime functions are quirky with timezones. Rust chrono handles RFC3339 correctly. TTL logic stays in application layer.

### 5. No Data Migration

JSON cache is ephemeral - DB starts empty and rebuilds on first API fetch. Old JSON files can be manually deleted.

## Service Pattern

### Before (FileService)
```rust
// Fast path cache check
let cache = FileService::read_json_optional(&cache_path).await?;
if let Some(cached) = cache.get_economy_type(type) {
    if cached.is_fresh() { return Ok(cached.data); }
}

// ... fetch from API ...

// Save to disk
cache.update_economy_type(type, data, TTL);
FileService::write_json(&cache_path, &cache).await?;
```

### After (Repository)
```rust
// Fast path cache check
if let Some(data) = repository.load_fresh_exchange_data(..., TTL).await? {
    return Ok(data);
}

// ... fetch from API ...

// Save to DB
repository.save_exchange_data(..., &data).await?;
```

**Benefits:**
- No file path logic
- No cache file management
- SQL handles filtering/sorting
- Cross-economy queries via JOINs

## Cross-Economy Queries

**Top Currencies (across all economy types):**
```rust
sqlx::query(
    "SELECT ci.currency_id, ci.name, er.economy_type, ci.primary_value
     FROM currency_items ci
     JOIN economy_exchange_rates er ON ci.exchange_rate_id = er.id
     WHERE er.league = ? AND er.is_hardcore = ? AND ci.is_active = 1
     ORDER BY ci.primary_value DESC
     LIMIT 10"
)
```

**Search Currencies:**
```rust
sqlx::query(
    "SELECT ci.name, er.economy_type, ci.primary_value, ci.display_value
     FROM currency_items ci
     JOIN economy_exchange_rates er ON ci.exchange_rate_id = er.id
     WHERE er.league = ? AND er.is_hardcore = ?
       AND ci.is_active = 1
       AND ci.name LIKE ? COLLATE NOCASE
     ORDER BY ci.primary_value DESC"
)
```

**Before:** In-memory filtering across all cached files
**After:** Single SQL query with JOIN, LIKE, ORDER BY

## Test Results

```
✅ All 526 backend tests passing
✅ Backend compiles successfully (cargo check)
✅ No frontend changes required
```

## Verification Steps

1. ✅ `pnpm test:backend` - All tests pass
2. ✅ `pnpm check:backend` - Compiles without errors
3. ✅ Migration file in correct location (`002_economy_cache.sql`)
4. ✅ Repository trait matches existing service interface
5. ✅ Service refactored to use repository (no FileService)
6. ✅ Service registry wires repository with DI
7. ✅ Tests use mock repository

## Deployment

When the app starts:
1. SQLite applies `002_economy_cache.sql` migration automatically
2. Tables created with proper schema and indexes
3. DB is empty initially
4. First economy data fetch populates DB from poe.ninja
5. Subsequent fetches hit DB cache (if within TTL)

**Manual cleanup (optional):**
- Delete old JSON cache files in `~/.local/share/poe2-overlord/economy_cache/`
- These files are no longer read by the app

## Documentation Updates

1. **`decisions.md`** - Updated ADR-007 to reflect economy domain migration
2. **`patterns.md`** - Added comprehensive "SQLite Cache Migration Pattern" section
3. **This session log** - Documents the implementation for future reference

## Learnings

### Pattern: Ephemeral Cache Migration

For TTL-based caches that rebuild from external APIs:
- No data migration needed (cache is ephemeral)
- Design schema with parent-child FK for referential integrity
- Preserve lifecycle flags (`is_active`) on upserts
- TTL checking in Rust (not SQL) for timezone correctness
- Cross-cache queries via JOINs unlock new features

### Benefits Over JSON Files

- **SQL queries**: Filter, sort, join across caches
- **Referential integrity**: FK constraints prevent orphans
- **Transactions**: Atomic upserts
- **Performance**: Indexes, batch queries
- **Consistency**: Same pattern as all other domains

### When to Keep JSON

Don't migrate if:
- Data is bundled/read-only (walkthrough guide)
- Cache is pure runtime (game monitoring state)
- File-based approach is simpler and fits the use case

## Next Steps

✅ Economy domain now follows established architecture
✅ All domains consistently use SQLite except bundled read-only data
✅ Pattern documented for future reference

This completes the backend data persistence migration. All user-generated and cached data now lives in `poe2-overlord.db` with proper relational structure.
