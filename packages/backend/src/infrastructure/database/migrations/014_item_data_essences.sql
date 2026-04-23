-- Essence-specific fields. Stored as a single JSON column because the
-- modifier list is variable-length (one entry per item-class category)
-- and only ~94 rows have it populated — not worth its own normalized table.
ALTER TABLE items ADD COLUMN essence TEXT;
