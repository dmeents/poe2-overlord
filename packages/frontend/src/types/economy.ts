/**
 * Types for economy data from the backend
 * These match the Rust structs in backend/src/domain/economy/models.rs
 * Using snake_case to match project convention
 */

/**
 * Currency tier levels
 */
export type CurrencyTier = 'Primary' | 'Secondary' | 'Tertiary';

/**
 * Display value with tier and direction info
 */
export interface DisplayValue {
  tier: CurrencyTier;
  value: number;
  inverted: boolean;
  currency_id: string;
  currency_name: string;
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
 * Types of economy data available from poe.ninja API.
 * 'All' is a frontend-only view that aggregates all cached types.
 */
export type EconomyType =
  | 'All'
  | 'Currency'
  | 'Fragments'
  | 'Abyss'
  | 'UncutGems'
  | 'LineageSupportGems'
  | 'Essences'
  | 'SoulCores'
  | 'Idols'
  | 'Runes'
  | 'Ritual'
  | 'Expedition'
  | 'Delirium'
  | 'Breach';

/** Backend economy types (excludes the frontend-only 'All' view) */
export type BackendEconomyType = Exclude<EconomyType, 'All'>;

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
  /** Economy type this currency belongs to — populated when data comes from cross-type queries */
  economy_type?: BackendEconomyType;
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
  economy_type: BackendEconomyType;
  primary_value: number;
  primary_currency_name: string;
  primary_currency_image_url: string;
  secondary_value: number;
  tertiary_value: number;
  volume: number | null;
  change_percent: number | null;
  display_value: DisplayValue;
}

/**
 * Convert a search result to a CurrencyExchangeRate for use in EconomyRow.
 * Centralizes the mapping so TypeScript catches any CurrencyExchangeRate field changes.
 */
export function searchResultToExchangeRate(result: CurrencySearchResult): CurrencyExchangeRate {
  return {
    id: result.id,
    name: result.name,
    image_url: result.image_url,
    display_value: result.display_value,
    primary_value: result.primary_value,
    secondary_value: result.secondary_value,
    tertiary_value: result.tertiary_value,
    volume: result.volume,
    change_percent: result.change_percent,
    price_history: [],
    economy_type: result.economy_type,
  };
}
