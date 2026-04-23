-- Per-item-class enrichment: map tier, talisman tier, breachstone info, quest descriptions.
-- All columns are nullable with safe defaults; the bundled JSON re-import populates them.

ALTER TABLE items ADD COLUMN map_tier INTEGER;
ALTER TABLE items ADD COLUMN talisman_tier INTEGER;
-- Breachstone stored as JSON (tier + upgrade chain — variable-length)
ALTER TABLE items ADD COLUMN breachstone TEXT;
ALTER TABLE items ADD COLUMN quest_description TEXT;
