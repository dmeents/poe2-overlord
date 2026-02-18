-- Initial schema for POE2 Overlord SQLite database
-- Migrating from JSON file-based persistence to relational storage

-- ============================================================================
-- APP CONFIG (single-row configuration table)
-- ============================================================================
CREATE TABLE app_config (
    id                    INTEGER PRIMARY KEY CHECK (id = 1) DEFAULT 1,
    config_version        INTEGER NOT NULL DEFAULT 1,
    poe_client_log_path   TEXT    NOT NULL,
    log_level             TEXT    NOT NULL DEFAULT 'info',
    zone_refresh_interval TEXT    NOT NULL DEFAULT 'SevenDays',
    updated_at            TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================================
-- ZONE METADATA (zone information registry with integer surrogate keys)
-- ============================================================================
CREATE TABLE zone_metadata (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    zone_name            TEXT    NOT NULL UNIQUE,
    area_id              TEXT,
    act                  INTEGER NOT NULL DEFAULT 0,
    area_level           INTEGER,
    is_town              INTEGER NOT NULL DEFAULT 0,
    has_waypoint         INTEGER NOT NULL DEFAULT 0,
    bosses               TEXT    NOT NULL DEFAULT '[]',
    monsters             TEXT    NOT NULL DEFAULT '[]',
    npcs                 TEXT    NOT NULL DEFAULT '[]',
    connected_zones      TEXT    NOT NULL DEFAULT '[]',
    description          TEXT,
    points_of_interest   TEXT    NOT NULL DEFAULT '[]',
    image_url            TEXT,
    wiki_url             TEXT,
    first_discovered     TEXT    NOT NULL,
    last_updated         TEXT    NOT NULL
);

CREATE INDEX idx_zone_metadata_name ON zone_metadata (zone_name);
CREATE INDEX idx_zone_metadata_act ON zone_metadata (act);

-- ============================================================================
-- CHARACTERS (player character profiles)
-- ============================================================================
CREATE TABLE characters (
    id               TEXT    PRIMARY KEY,  -- UUID, kept as TEXT for external identity
    name             TEXT    NOT NULL,
    class            TEXT    NOT NULL,
    ascendency       TEXT    NOT NULL,
    league           TEXT    NOT NULL,
    hardcore         INTEGER NOT NULL DEFAULT 0,
    solo_self_found  INTEGER NOT NULL DEFAULT 0,
    level            INTEGER NOT NULL DEFAULT 1,
    is_active        INTEGER NOT NULL DEFAULT 0,
    created_at       TEXT    NOT NULL,
    last_played      TEXT,
    last_updated     TEXT    NOT NULL,
    current_zone_id         INTEGER REFERENCES zone_metadata(id) ON DELETE SET NULL,
    current_zone_updated_at TEXT
);

CREATE UNIQUE INDEX idx_characters_active ON characters (is_active) WHERE is_active = 1;
CREATE UNIQUE INDEX idx_characters_name ON characters (name);
CREATE INDEX idx_characters_last_played ON characters (last_played DESC);
CREATE INDEX idx_characters_league ON characters (league);

-- ============================================================================
-- ZONE STATS (character zone tracking data)
-- ============================================================================
CREATE TABLE zone_stats (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id    TEXT    NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    zone_id         INTEGER NOT NULL REFERENCES zone_metadata(id) ON DELETE RESTRICT,
    duration        INTEGER NOT NULL DEFAULT 0,
    deaths          INTEGER NOT NULL DEFAULT 0,
    visits          INTEGER NOT NULL DEFAULT 0,
    first_visited   TEXT    NOT NULL,
    last_visited    TEXT    NOT NULL,
    is_active       INTEGER NOT NULL DEFAULT 0,
    entry_timestamp TEXT,
    UNIQUE(character_id, zone_id)
);

CREATE INDEX idx_zone_stats_character ON zone_stats (character_id);
CREATE INDEX idx_zone_stats_active ON zone_stats (character_id, is_active) WHERE is_active = 1;

-- ============================================================================
-- WALKTHROUGH PROGRESS (campaign progress tracking per character)
-- ============================================================================
CREATE TABLE walkthrough_progress (
    character_id     TEXT    PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    current_step_id  TEXT,
    is_completed     INTEGER NOT NULL DEFAULT 0,
    last_updated     TEXT    NOT NULL
);

-- ============================================================================
-- SERVER STATUS (single-row server monitoring state)
-- ============================================================================
CREATE TABLE server_status (
    id          INTEGER PRIMARY KEY CHECK (id = 1) DEFAULT 1,
    ip_address  TEXT    NOT NULL DEFAULT '127.0.0.1',
    port        INTEGER NOT NULL DEFAULT 6112,
    is_online   INTEGER NOT NULL DEFAULT 0,
    latency_ms  INTEGER,
    timestamp   TEXT    NOT NULL
);
