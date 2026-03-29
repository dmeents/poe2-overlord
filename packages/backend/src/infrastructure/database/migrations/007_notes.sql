CREATE TABLE notes (
    id           TEXT    PRIMARY KEY,
    title        TEXT    NOT NULL,
    content      TEXT    NOT NULL DEFAULT '',
    is_pinned    INTEGER NOT NULL DEFAULT 0,
    character_id TEXT    REFERENCES characters(id) ON DELETE SET NULL,
    created_at   TEXT    NOT NULL,
    updated_at   TEXT    NOT NULL
);

CREATE INDEX idx_notes_character ON notes(character_id);
CREATE INDEX idx_notes_pinned ON notes(is_pinned);
CREATE INDEX idx_notes_updated ON notes(updated_at DESC);
