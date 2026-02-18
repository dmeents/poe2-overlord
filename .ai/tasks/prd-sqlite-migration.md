# PRD: Migrate from JSON Files to SQLite

**Created:** 2026-02-17
**Status:** Planning
**Priority:** High
**Estimated effort:** 6 phases, ~6-8 sessions

## Problem Statement

All backend data persistence uses JSON files via `FileService`. While functional, this approach has significant limitations:

- **No relational queries** — can't query zone stats by act, filter characters by league, etc. without loading entire blobs
- **No referential integrity** — orphan files possible (manual `reconcile_character_storage` exists as a workaround)
- **No transactions** — create_character has manual rollback logic that could fail partially
- **Monolithic blobs** — CharacterData packs profile, zone stats, walkthrough progress, and tracking summary into one JSON file. Any mutation requires deserializing/serializing the entire blob
- **No concurrent access safety** — multiple services writing to the same file could corrupt data (mitigated by atomic writes but not eliminated)

Since the app hasn't shipped, now is the time to establish a proper data layer before the JSON structure calcifies.

## Solution: SQLite via sqlx

SQLite with the `sqlx` crate provides:
- Embedded, zero-config database (single file, ships with the app)
- ACID transactions (atomic create_character, cascading deletes)
- Relational normalization (zone_stats as a proper table, not a Vec in a blob)
- Compile-time checked queries (optional, via sqlx macros)
- WAL mode for concurrent reads + writes
- Trivial backup (copy the .db file)

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Zone metadata lists (bosses, monsters, etc.) | JSON arrays in TEXT columns | Never queried individually, always loaded/saved as whole |
| TrackingSummary | Computed on demand from zone_stats | Derived data, eliminates sync issues |
| Economy cache | Keep as JSON files | TTL cache, deeply nested, transient data |
| JSON data migration | Standalone script outside codebase | Run manually, clean up after use |
| Walkthrough guide | Keep as bundled JSON | Read-only static data, loaded once |

## Schema Design

### `app_config` (single-row)
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | CHECK (id = 1) |
| config_version | INTEGER | Default 1 |
| poe_client_log_path | TEXT | Required |
| log_level | TEXT | Default 'info' |
| zone_refresh_interval | TEXT | Enum as string |
| updated_at | TEXT | ISO 8601 |

### `characters`
| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | UUID |
| name | TEXT | UNIQUE |
| class | TEXT | Enum as string |
| ascendency | TEXT | Enum as string |
| league | TEXT | Enum as string |
| hardcore | INTEGER | Boolean |
| solo_self_found | INTEGER | Boolean |
| level | INTEGER | Default 1 |
| is_active | INTEGER | Partial unique index (at most one) |
| created_at | TEXT | ISO 8601 |
| last_played | TEXT | Nullable |
| last_updated | TEXT | ISO 8601 |
| current_zone_id | INTEGER FK | → zone_metadata(id) ON DELETE SET NULL |
| current_zone_updated_at | TEXT | Nullable |

**Indexes:** `idx_characters_name` (UNIQUE), `idx_characters_active` (partial UNIQUE), `idx_characters_last_played`, `idx_characters_league`

**Eliminates** `CharactersIndex` file entirely. The DB IS the index.

### `zone_stats`
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | AUTOINCREMENT |
| character_id | TEXT FK | → characters(id) ON DELETE CASCADE |
| zone_id | INTEGER FK | → zone_metadata(id) ON DELETE RESTRICT |
| duration | INTEGER | Seconds |
| deaths | INTEGER | |
| visits | INTEGER | |
| first_visited | TEXT | ISO 8601 |
| last_visited | TEXT | ISO 8601 |
| is_active | INTEGER | Boolean |
| entry_timestamp | TEXT | Nullable |

`TrackingSummary` computed from this table via `TrackingSummary::from_zones()` — not stored.

**Note:** Zone properties (act, is_town) are looked up via JOIN to zone_metadata, not stored redundantly.

### `walkthrough_progress`
| Column | Type | Notes |
|--------|------|-------|
| character_id | TEXT PK FK | → characters(id) ON DELETE CASCADE |
| current_step_id | TEXT | Nullable |
| is_completed | INTEGER | Boolean |
| last_updated | TEXT | ISO 8601 |

### `zone_metadata`
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | AUTOINCREMENT |
| zone_name | TEXT | UNIQUE (not PK for better join performance) |
| area_id | TEXT | Nullable |
| act | INTEGER | Default 0 |
| area_level | INTEGER | Nullable |
| is_town | INTEGER | Boolean |
| has_waypoint | INTEGER | Boolean |
| bosses | TEXT | JSON array |
| monsters | TEXT | JSON array |
| npcs | TEXT | JSON array |
| connected_zones | TEXT | JSON array |
| description | TEXT | Nullable |
| points_of_interest | TEXT | JSON array |
| image_url | TEXT | Nullable |
| wiki_url | TEXT | Nullable |
| first_discovered | TEXT | ISO 8601 |
| last_updated | TEXT | ISO 8601 |

**Indexes:** `idx_zone_metadata_name` on zone_name (for lookups), `idx_zone_metadata_act` on act

### `server_status` (single-row)
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | CHECK (id = 1) |
| ip_address | TEXT | Default '127.0.0.1' |
| port | INTEGER | Default 6112 |
| is_online | INTEGER | Boolean |
| latency_ms | INTEGER | Nullable |
| timestamp | TEXT | ISO 8601 |

## Implementation Phases

### Phase 0: Database Infrastructure
- Add `sqlx` to Cargo.toml
- Create `infrastructure/database/` module (pool.rs, mod.rs)
- Create migration file with all CREATE TABLE statements
- Add `From<sqlx::Error>` to `AppError`
- Initialize pool in `ServiceInitializer` before repositories

### Phase 1: Configuration Domain
- New `ConfigurationSqliteRepository` implementing existing trait
- Eliminate debounce write task (SQLite writes are fast enough)
- Keep in-memory cache for reads

### Phase 2: Server Monitoring Domain
- New `ServerStatusSqliteRepository`
- Simple single-row INSERT OR REPLACE / SELECT

### Phase 3: Zone Configuration Domain
- New `ZoneConfigurationSqliteRepository`
- Deserialize JSON TEXT columns for Vec<String> fields
- Create shared `infrastructure/database/helpers.rs` with `get_or_create_zone_id(pool, zone_name)` helper
- Keep in-memory cache

### Phase 4: Character Domain (largest)
- New `CharacterSqliteRepository`
- Normalize CharacterData blob into characters + zone_stats + walkthrough_progress tables
- TrackingSummary computed from zone_stats (not stored)
- Use shared `get_or_create_zone_id()` before inserting zone_stats (auto-creates stub zone_metadata rows)
- `load_all_characters()` uses 3 batch queries + Rust assembly (avoids N+1)
- All multi-step operations wrapped in transactions
- JOINs to zone_metadata for zone properties when loading character data
- Simplify CharacterService: transactions replace manual rollback, cascading deletes replace manual cleanup
- `reconcile_character_storage()` becomes no-op
- `enter_zone()` workflow: lookup zone by name → get/create id → use id in zone_stats

### Phase 5: Cleanup and Testing
- Delete old JSON repository files
- Remove dead imports
- Write unit tests (in-memory SQLite)
- Write transaction tests (verify rollback on failure)
- Write constraint tests (UNIQUE, FK violations)
- Update `.ai/memory/` with ADR and patterns
- Full clippy + test pass

### Phase 6: Migration Script
- **⚠️ Backup data directory first!**
- Standalone Rust binary in `scripts/migrate-json-to-sqlite/`
- Reads existing JSON files, inserts into SQLite in single transaction
- **Idempotent:** checks if DB has data, skips if already migrated
- Correct order: zone_metadata BEFORE zone_stats (FK constraint)
- Uses `get_or_create_zone_id()` to handle zone references
- Run manually, delete after use

## Frontend Impact

**Zero changes required.** Service traits (the Tauri IPC boundary) are unchanged. Same `CharacterDataResponse`, `AppConfig`, etc. cross the wire.

**Future opportunities** (separate PRD):
- Paginated/filtered queries pushed to SQL
- Server-side sorting
- Lazy loading of zone data
- New query patterns not possible with JSON

## Key Design Decisions

### Integer Surrogate Keys for zone_metadata
- `zone_metadata.id` (INTEGER AUTOINCREMENT) is the primary key
- `zone_name` remains UNIQUE but is not the primary key
- `zone_stats` and `characters` reference zones by integer ID (faster joins, smaller storage)
- Workflow: lookup zone by name, if not found insert stub and get ID back
- Zone properties (act, is_town) accessed via JOIN, never duplicated in zone_stats

### UUID for characters
- `characters.id` stays as TEXT UUID primary key (external identity for IPC/frontend)
- No integer surrogate key added (UUIDs are stable and reasonably sized)

### Stub Zone Metadata Pattern
Use shared helper `get_or_create_zone_id(pool, zone_name)` from `infrastructure/database/helpers.rs`:
1. Looks up `SELECT id FROM zone_metadata WHERE zone_name = ?`
2. If NULL: `INSERT INTO zone_metadata (zone_name, first_discovered, last_updated) VALUES (?, NOW(), NOW())`
3. Returns AUTOINCREMENT id
4. Both ZoneConfigurationRepository and CharacterRepository use this helper
5. Later, WikiScrapingService fills in metadata via `UPDATE zone_metadata WHERE zone_name = ?`

### Transaction Boundaries
All multi-step operations must be atomic:
- `create_character()`, `save_character_data()`, `set_active_character()` - wrap all writes in transaction
- `save_configuration()` - DELETE all + INSERT all in transaction
- Migration script - entire migration in one transaction (all-or-nothing)

### Error Mapping
- UNIQUE constraint violation (code 2067/1555) → `AppError::Validation`
- Foreign key violation (code 787) → `AppError::Validation`
- `RowNotFound` → `AppError::Validation`
- Other errors → `AppError::Internal`

## Out of Scope
- Frontend changes
- Economy cache migration (stays JSON)
- Walkthrough guide migration (stays bundled JSON)
- Schema versioning for production migrations (app not shipped yet)
