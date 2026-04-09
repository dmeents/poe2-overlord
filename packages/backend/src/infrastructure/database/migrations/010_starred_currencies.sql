-- Add is_starred column to currency_items for user-pinned currencies
ALTER TABLE currency_items ADD COLUMN is_starred INTEGER NOT NULL DEFAULT 0;

CREATE INDEX idx_currency_items_starred
    ON currency_items (is_starred)
    WHERE is_starred = 1;
