-- Enrichment columns for the items table (see table-joiner.mjs + models.rs).
-- All columns are nullable or have a safe default so existing rows survive
-- the migration; the bundled JSON re-import populates them on next launch.

-- Base item flags
ALTER TABLE items ADD COLUMN is_corrupted INTEGER NOT NULL DEFAULT 0;
ALTER TABLE items ADD COLUMN unmodifiable INTEGER NOT NULL DEFAULT 0;

-- Extra stats on specific item classes
ALTER TABLE items ADD COLUMN movement_speed INTEGER;        -- % bonus on boots
ALTER TABLE items ADD COLUMN reload_time INTEGER;           -- ms on crossbows

-- Flask metadata
ALTER TABLE items ADD COLUMN flask_name TEXT;

-- Gem attribute-requirement scaling (% of the per-level base requirement)
ALTER TABLE items ADD COLUMN gem_str_req_percent INTEGER;
ALTER TABLE items ADD COLUMN gem_dex_req_percent INTEGER;
ALTER TABLE items ADD COLUMN gem_int_req_percent INTEGER;

-- Rune / soul-core socket data
ALTER TABLE items ADD COLUMN soul_core_required_level INTEGER;
ALTER TABLE items ADD COLUMN soul_core_limit_count INTEGER;
ALTER TABLE items ADD COLUMN soul_core_limit_text TEXT;
