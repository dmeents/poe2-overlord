/**
 * Types for economy data from the backend
 * These match the Rust structs in backend/src/domain/economy/models.rs
 * Using snake_case to match project convention
 */

/**
 * Types of economy data available from poe.ninja API
 */
export type EconomyType =
  | 'Currency'
  | 'Fragments'
  | 'Abyss'
  | 'UncutGems'
  | 'LineageSupportGems'
  | 'Essences'
  | 'Ultimatum'
  | 'Talismans'
  | 'Runes'
  | 'Ritual'
  | 'Expedition'
  | 'Delirium'
  | 'Breach';

/**
 * Represents which tier of currency should be used for display
 */
export type CurrencyTier = 'Primary' | 'Secondary' | 'Tertiary';

/**
 * Smart display value that automatically selects the most appropriate currency tier
 */
export interface DisplayValue {
  /** Which tier this value is displayed in */
  tier: CurrencyTier;
  /** The calculated value in the selected tier's currency */
  value: number;
  /** Whether the display is inverted (e.g., "X items per currency" vs "X currency per item") */
  inverted: boolean;
  /** ID of the currency used for display */
  currency_id: string;
  /** Name of the currency used for display */
  currency_name: string;
  /** Image URL of the currency used for display */
  currency_image_url: string;
}

/**
 * Basic currency information
 */
export interface CurrencyInfo {
  id: string;
  name: string;
  image_url: string;
}

/**
 * Enriched currency exchange rate with all relevant data
 */
export interface CurrencyExchangeRate {
  id: string;
  name: string;
  image_url: string;

  /** Smart display value (automatically selects best tier and direction) */
  display_value: DisplayValue;

  /** Value in primary currency (dynamic based on economy type) */
  primary_value: number;
  /** Value in secondary currency (dynamic based on economy type) */
  secondary_value: number;
  /** Value in tertiary currency (dynamic based on economy type) */
  tertiary_value: number;

  /** Trading volume in primary currency */
  volume: number | null;
  /** Price change percentage over time period */
  change_percent: number | null;
  /** Historical price data points (can contain null for missing data) */
  price_history: (number | null)[];
}

/**
 * Lightweight item for top currencies aggregation across all economy types
 */
export interface TopCurrencyItem {
  id: string;
  name: string;
  image_url: string;
  economy_type: EconomyType;
  primary_value: number;
  primary_currency_name: string;
  primary_currency_image_url: string;
  volume: number | null;
  change_percent: number | null;
  cached_at: string; // RFC3339 timestamp
}

/**
 * Processed currency exchange data from backend
 */
export interface CurrencyExchangeData {
  /** Primary currency info (usually Divine Orb) */
  primary_currency: CurrencyInfo;
  /** Secondary currency info (usually Chaos Orb) */
  secondary_currency: CurrencyInfo;
  /** Tertiary currency info (usually Exalted Orb) - optional, may not exist for some leagues */
  tertiary_currency: CurrencyInfo | null;
  /** Exchange rate from primary to secondary (e.g., 1 Divine = X Chaos) */
  secondary_rate: number;
  /** Exchange rate from primary to tertiary (e.g., 1 Divine = X Exalted) - optional */
  tertiary_rate: number | null;
  /** All currency exchange rates */
  currencies: CurrencyExchangeRate[];
  /** Timestamp of when this data was fetched (RFC3339 format) */
  fetched_at: string;
  /** Whether this data is stale (returned from cache when poe.ninja was unavailable) */
  is_stale?: boolean;
}

/**
 * Search result for cross-economy currency search
 */
export interface CurrencySearchResult {
  id: string;
  name: string;
  image_url: string;
  economy_type: EconomyType;
  primary_value: number;
  primary_currency_name: string;
  primary_currency_image_url: string;
  secondary_value: number;
  tertiary_value: number;
  volume: number | null;
  change_percent: number | null;
  display_value: DisplayValue;
}
