CREATE TABLE level_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id    TEXT    NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    level           INTEGER NOT NULL,
    reached_at      TEXT    NOT NULL,
    deaths_at_level INTEGER NOT NULL DEFAULT 0,
    UNIQUE(character_id, level)
);
CREATE INDEX idx_level_events_character ON level_events(character_id);
CREATE INDEX idx_level_events_reached ON level_events(character_id, reached_at DESC);

ALTER TABLE characters ADD COLUMN deaths_at_current_level INTEGER NOT NULL DEFAULT 0;
