//! Trait definitions for economy domain services

use super::models::{CurrencyExchangeData, CurrencySearchResult, EconomyType};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Repository for economy data caching with `SQLite`
#[async_trait]
pub trait EconomyRepository: Send + Sync {
    /// Load exchange data if cache is fresh (within `ttl_seconds`).
    /// Checks `exchange_rates.last_updated`, then loads items if fresh.
    /// Returns None if stale or missing.
    async fn load_fresh_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        ttl_seconds: u64,
    ) -> AppResult<Option<CurrencyExchangeData>>;

    /// Load exchange data regardless of TTL (for stale fallback on API failure).
    /// Returns None only if no data exists at all.
    async fn load_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
    ) -> AppResult<Option<CurrencyExchangeData>>;

    /// Upsert exchange rates row, then upsert each currency item using the FK.
    /// Preserves `is_active` and `is_starred` on existing items (not overwritten on update).
    async fn save_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()>;

    /// All currencies across ALL economy types for a league, ordered by `primary_value` DESC.
    /// JOIN on FK, WHERE `is_active=1`.
    async fn load_all_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
    ) -> AppResult<Vec<CurrencySearchResult>>;

    /// Search currencies by name across ALL economy types.
    /// JOIN on FK, LIKE with COLLATE NOCASE, WHERE `is_active=1`, LIMIT limit.
    async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
        limit: u32,
    ) -> AppResult<Vec<CurrencySearchResult>>;

    /// Toggle `is_starred` for a currency item. Returns the new starred state.
    async fn toggle_currency_star(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        currency_id: &str,
    ) -> AppResult<bool>;

    /// Load all starred currencies for a league across all economy types.
    async fn load_starred_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
    ) -> AppResult<Vec<CurrencySearchResult>>;
}
