//! Trait definitions for economy domain services

use super::models::{CurrencyExchangeData, CurrencySearchResult, EconomyType, TopCurrencyItem};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Repository for economy data caching with SQLite
#[async_trait]
pub trait EconomyRepository: Send + Sync {
    /// Load exchange data if cache is fresh (within ttl_seconds).
    /// Checks exchange_rates.last_updated, then loads items if fresh.
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
    /// Preserves is_active on existing items (not overwritten on update).
    async fn save_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()>;

    /// Top currencies across ALL economy types for a league.
    /// JOIN on FK, WHERE is_active=1, ORDER BY primary_value DESC LIMIT.
    async fn load_top_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        limit: u32,
    ) -> AppResult<Vec<TopCurrencyItem>>;

    /// Search currencies by name across ALL economy types.
    /// JOIN on FK, LIKE with COLLATE NOCASE, WHERE is_active=1.
    async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
    ) -> AppResult<Vec<CurrencySearchResult>>;
}
