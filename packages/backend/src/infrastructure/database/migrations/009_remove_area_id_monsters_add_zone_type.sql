-- Remove area_id and monsters columns, add zone_type column.
--
-- Uses ALTER TABLE DROP COLUMN instead of table recreation to avoid a
-- FOREIGN KEY constraint failure: zone_stats.zone_id REFERENCES
-- zone_metadata(id) ON DELETE RESTRICT, which prevents DROP TABLE while
-- rows exist. PRAGMA foreign_keys cannot be toggled inside a transaction,
-- and sqlx runs each migration in a transaction.
--
-- SQLite 3.35+ (bundled in Tauri's sqlcipher, and available on all
-- target platforms since 2021) supports ALTER TABLE DROP COLUMN provided
-- the column is not part of an index, FK, or view — which area_id and
-- monsters are not.

ALTER TABLE zone_metadata DROP COLUMN area_id;
ALTER TABLE zone_metadata DROP COLUMN monsters;
ALTER TABLE zone_metadata ADD COLUMN zone_type TEXT NOT NULL DEFAULT 'Unknown';
