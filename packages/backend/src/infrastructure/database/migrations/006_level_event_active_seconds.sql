-- Add active grinding seconds to level events so XP/hr can use grinding-only time
-- instead of wall-clock time between level-up timestamps.
ALTER TABLE level_events ADD COLUMN active_seconds INTEGER NOT NULL DEFAULT 0;
