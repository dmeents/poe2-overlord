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

  /** Raw value in Divine (primary currency) - for advanced users */
  raw_divine_value: number | null;
  /** Raw value in Chaos (secondary currency) - for advanced users */
  raw_chaos_value: number | null;
  /** Raw value in Exalted (tertiary currency) - for advanced users */
  raw_exalted_value: number | null;

  /** Trading volume in primary currency */
  volume: number | null;
  /** Price change percentage over time period */
  change_percent: number | null;
  /** Historical price data points (can contain null for missing data) */
  price_history: (number | null)[];
}

/**
 * Processed currency exchange data from backend
 */
export interface CurrencyExchangeData {
  /** Primary currency info (usually Divine Orb) */
  primary_currency: CurrencyInfo;
  /** Secondary currency info (usually Chaos Orb) */
  secondary_currency: CurrencyInfo;
  /** Tertiary currency info (usually Exalted Orb) */
  tertiary_currency: CurrencyInfo;
  /** Exchange rate from primary to secondary (e.g., 1 Divine = X Chaos) */
  secondary_rate: number;
  /** Exchange rate from primary to tertiary (e.g., 1 Divine = X Exalted) */
  tertiary_rate: number;
  /** All currency exchange rates */
  currencies: CurrencyExchangeRate[];
  /** Timestamp of when this data was fetched (RFC3339 format) */
  fetched_at: string;
}
