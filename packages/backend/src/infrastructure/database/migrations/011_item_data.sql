-- Game data version tracking (single-row table, id always = 1)
CREATE TABLE IF NOT EXISTS game_data_version (
    id INTEGER PRIMARY KEY CHECK (id = 1) DEFAULT 1,
    patch_version TEXT NOT NULL,
    extracted_at TEXT NOT NULL,
    imported_at TEXT NOT NULL
);

-- Item categories (ItemClasses from game data)
CREATE TABLE IF NOT EXISTS item_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- Items: denormalized, one row per base item or unique item
CREATE TABLE IF NOT EXISTS items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    is_unique INTEGER NOT NULL DEFAULT 0,
    unique_name TEXT,                           -- display name for uniques (e.g. "Mjolner")
    base_type TEXT,                             -- base item name for uniques (e.g. "Gavel")
    item_class_id TEXT NOT NULL REFERENCES item_categories(id),
    category TEXT NOT NULL,
    rarity_frame INTEGER NOT NULL DEFAULT 0,    -- 0=normal, 1=magic, 2=rare, 3=unique
    width INTEGER NOT NULL DEFAULT 1,
    height INTEGER NOT NULL DEFAULT 1,
    drop_level INTEGER NOT NULL DEFAULT 0,
    image_url TEXT,
    flavour_text TEXT,
    tags TEXT NOT NULL DEFAULT '[]',            -- JSON array of strings
    -- Attribute requirements
    req_str INTEGER NOT NULL DEFAULT 0,
    req_dex INTEGER NOT NULL DEFAULT 0,
    req_int INTEGER NOT NULL DEFAULT 0,
    -- Defence values (NULL if not an armour piece)
    armour INTEGER,
    evasion INTEGER,
    energy_shield INTEGER,
    ward INTEGER,
    -- Weapon values (NULL if not a weapon)
    damage_min INTEGER,
    damage_max INTEGER,
    critical INTEGER,                           -- stored x100 (e.g. 5.00% → 500)
    attack_speed INTEGER,                       -- stored x100 (e.g. 1.20 aps → 120)
    range_max INTEGER,
    -- Shield value (NULL if not a shield)
    block INTEGER,
    -- Gem data (NULL if not a gem)
    gem_type TEXT,
    gem_colour TEXT,
    gem_min_level INTEGER,
    gem_tier INTEGER,
    -- Currency data (NULL if not currency)
    stack_size INTEGER,
    currency_description TEXT,
    -- Flask data (NULL if not a flask)
    flask_type TEXT,
    flask_life INTEGER,
    flask_mana INTEGER,
    flask_recovery_time INTEGER,                -- milliseconds
    -- Mod lists as JSON arrays of {id, text} objects
    implicit_mods TEXT NOT NULL DEFAULT '[]',
    explicit_mods TEXT NOT NULL DEFAULT '[]'
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_items_name ON items (name);
CREATE INDEX IF NOT EXISTS idx_items_class ON items (item_class_id);
CREATE INDEX IF NOT EXISTS idx_items_category ON items (category);
CREATE INDEX IF NOT EXISTS idx_items_drop_level ON items (drop_level);
CREATE INDEX IF NOT EXISTS idx_items_is_unique ON items (is_unique);
CREATE INDEX IF NOT EXISTS idx_items_base_type ON items (base_type);

-- User favourites: separate from imported game data, cascades on item re-import
CREATE TABLE IF NOT EXISTS item_favorites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id TEXT NOT NULL UNIQUE REFERENCES items(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL
);
