-- Economy cache migration
-- Migrating economy domain from JSON file caching to SQLite

-- ============================================================================
-- ECONOMY EXCHANGE RATES (exchange rate context per league+type)
-- ============================================================================
CREATE TABLE economy_exchange_rates (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    league                    TEXT    NOT NULL,
    is_hardcore               INTEGER NOT NULL DEFAULT 0,
    economy_type              TEXT    NOT NULL,
    primary_currency_id       TEXT    NOT NULL,
    primary_currency_name     TEXT    NOT NULL,
    primary_currency_image    TEXT    NOT NULL,
    secondary_currency_id     TEXT    NOT NULL,
    secondary_currency_name   TEXT    NOT NULL,
    secondary_currency_image  TEXT    NOT NULL,
    tertiary_currency_id      TEXT,
    tertiary_currency_name    TEXT,
    tertiary_currency_image   TEXT,
    secondary_rate            REAL    NOT NULL,
    tertiary_rate             REAL,
    fetched_at                TEXT    NOT NULL,
    last_updated              TEXT    NOT NULL,
    UNIQUE(league, is_hardcore, economy_type)
);

CREATE INDEX idx_exchange_rates_lookup
    ON economy_exchange_rates (league, is_hardcore, economy_type);

-- ============================================================================
-- CURRENCY ITEMS (individual currency data, FK to exchange rates)
-- ============================================================================
CREATE TABLE currency_items (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    exchange_rate_id          INTEGER NOT NULL REFERENCES economy_exchange_rates(id) ON DELETE CASCADE,
    currency_id               TEXT    NOT NULL,
    name                      TEXT    NOT NULL,
    image_url                 TEXT    NOT NULL,
    primary_value             REAL    NOT NULL,
    secondary_value           REAL    NOT NULL,
    tertiary_value            REAL    NOT NULL DEFAULT 0.0,
    volume                    REAL,
    change_percent            REAL,
    display_tier              TEXT    NOT NULL,
    display_value             REAL    NOT NULL,
    display_inverted          INTEGER NOT NULL DEFAULT 0,
    display_currency_id       TEXT    NOT NULL,
    display_currency_name     TEXT    NOT NULL,
    display_currency_image    TEXT    NOT NULL,
    price_history             TEXT    NOT NULL DEFAULT '[]',
    is_active                 INTEGER NOT NULL DEFAULT 1,
    last_updated              TEXT    NOT NULL,
    UNIQUE(exchange_rate_id, currency_id)
);

CREATE INDEX idx_currency_items_exchange_rate
    ON currency_items (exchange_rate_id);
CREATE INDEX idx_currency_items_name
    ON currency_items (name);
CREATE INDEX idx_currency_items_value
    ON currency_items (primary_value DESC);
