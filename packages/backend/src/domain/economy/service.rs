//! Economy service for fetching and caching currency exchange data from poe.ninja.
//!
//! # Configuration
//!
//! Service behavior is controlled by module-level constants:
//!
//! ## HTTP Timeouts
//! - `HTTP_TOTAL_TIMEOUT_SECS` (10s) - Total request timeout
//! - `HTTP_CONNECT_TIMEOUT_SECS` (5s) - Connection establishment timeout
//!
//! ## Caching
//! - `CACHE_TTL_SECONDS` (600s / 10min) - How long cached data is considered fresh
//!
//! ## Retry Logic
//! - `MAX_RETRY_ATTEMPTS` (3) - Number of fetch attempts before fallback to stale cache
//! - `INITIAL_RETRY_DELAY_MS` (500ms) - First retry delay
//! - `RETRY_BACKOFF_MULTIPLIER` (3x) - Exponential backoff rate
//!
//! # Concurrency Safety
//!
//! This service is safe for concurrent use. Multiple simultaneous requests for the same
//! league+economy_type will be coalesced into a single API fetch, with subsequent requests
//! waiting for the first to complete. This prevents:
//! - Redundant API calls
//! - Cache race conditions (last-write-wins)
//! - Potential rate limiting from the API
//!
//! Requests for different leagues/types do not block each other.

use super::models::{
    CurrencyExchangeApiResponse, CurrencyExchangeData, CurrencySearchResult, EconomyType,
    TopCurrencyItem,
};
use super::traits::EconomyRepository;
use crate::errors::{AppError, AppResult};
use reqwest;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};

// =============================================================================
// POE Ninja API Configuration
// =============================================================================

/// Base URL for poe.ninja economy API
/// URL format: {BASE}?league={LEAGUE}&type={TYPE}
const POE_NINJA_API_BASE_URL: &str = "https://poe.ninja/poe2/api/economy/exchange/current/overview";

// =============================================================================
// HTTP Client Configuration
// =============================================================================

/// Total request timeout including connection, sending, and receiving.
/// Set to 10 seconds to fail fast on network issues while allowing for slow API responses.
const HTTP_TOTAL_TIMEOUT_SECS: u64 = 10;

/// Connection establishment timeout.
/// Set to 5 seconds - if we can't connect within this time, the API is likely unreachable.
const HTTP_CONNECT_TIMEOUT_SECS: u64 = 5;

// =============================================================================
// Cache Configuration
// =============================================================================

/// Time-to-live for cached economy data in seconds.
/// Set to 10 minutes (600s) to balance data freshness with API load.
/// POE economy prices change slowly, so 10-minute staleness is acceptable.
const CACHE_TTL_SECONDS: u64 = 600;

// =============================================================================
// Retry Configuration
// =============================================================================

/// Maximum number of fetch attempts before giving up and falling back to stale cache.
/// Set to 3 attempts to handle transient network issues without excessive delay.
const MAX_RETRY_ATTEMPTS: u32 = 3;

/// Initial delay before first retry in milliseconds.
/// Set to 500ms to quickly retry after brief network blips.
const INITIAL_RETRY_DELAY_MS: u64 = 500;

/// Multiplier for exponential backoff between retry attempts.
/// Each retry waits MULTIPLIER times longer than the previous.
/// Delays: 500ms -> 1500ms (total ~2s for all retries).
const RETRY_BACKOFF_MULTIPLIER: u32 = 3;

/// Economy service with concurrent request deduplication.
///
/// Uses per-cache-key semaphores to ensure only one API fetch happens at a time
/// for the same league+hardcore+economy_type combination.
pub struct EconomyService {
    pub(crate) client: reqwest::Client,
    /// Tracks in-flight API fetches per cache key to prevent race conditions.
    /// Key format: "{league}:{is_hardcore}:{economy_type}"
    in_flight: Arc<RwLock<HashMap<String, Arc<Semaphore>>>>,
    repository: Arc<dyn EconomyRepository>,
}

impl std::fmt::Debug for EconomyService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EconomyService")
            .field("client", &"reqwest::Client")
            .field("in_flight", &self.in_flight)
            .field("repository", &"Arc<dyn EconomyRepository>")
            .finish()
    }
}

impl Clone for EconomyService {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            in_flight: self.in_flight.clone(),
            repository: self.repository.clone(),
        }
    }
}

impl EconomyService {
    pub fn new(repository: Arc<dyn EconomyRepository>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(HTTP_TOTAL_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
            .build()
            .map_err(|e| AppError::internal_error("build_http_client", &format!("{}", e)))?;

        Ok(Self {
            client,
            in_flight: Arc::new(RwLock::new(HashMap::new())),
            repository,
        })
    }

    /// Build the poe.ninja API URL for a given league and economy type.
    pub(crate) fn build_poe_ninja_url(league_name: &str, economy_type: EconomyType) -> String {
        format!(
            "{}?league={}&type={}",
            POE_NINJA_API_BASE_URL,
            urlencoding::encode(league_name),
            economy_type.as_str()
        )
    }

    /// Calculate exponential backoff delay for a given retry attempt.
    pub(crate) fn calculate_retry_delay(attempt: u32) -> Duration {
        if attempt == 0 {
            Duration::from_millis(0)
        } else {
            let delay_ms =
                INITIAL_RETRY_DELAY_MS * (RETRY_BACKOFF_MULTIPLIER.pow(attempt - 1) as u64);
            Duration::from_millis(delay_ms)
        }
    }

    /// Check if an error should trigger a retry attempt.
    pub(crate) fn is_retryable_error(error: &AppError) -> bool {
        matches!(error, AppError::Network { .. })
    }

    /// Generate unique key for a cache entry (used for request deduplication)
    pub(crate) fn cache_key(league: &str, is_hardcore: bool, economy_type: EconomyType) -> String {
        format!("{}:{}:{}", league, is_hardcore, economy_type.as_str())
    }

    pub async fn fetch_currency_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
    ) -> AppResult<CurrencyExchangeData> {
        // Validate league name is not empty
        if league.trim().is_empty() {
            return Err(AppError::validation_error(
                "fetch_currency_exchange_data",
                "League name cannot be empty",
            ));
        }

        // FAST PATH: Check for fresh cache without acquiring any locks
        if let Some(data) = self
            .repository
            .load_fresh_exchange_data(league, is_hardcore, economy_type, CACHE_TTL_SECONDS)
            .await?
        {
            log::debug!(
                "Fast path: Fresh cache for {}:{:?} (no lock needed)",
                league,
                economy_type
            );
            return Ok(data);
        }

        // Cache is stale or missing - need to fetch with request deduplication
        // Acquire or create semaphore for this cache key
        let key = Self::cache_key(league, is_hardcore, economy_type);
        let semaphore = {
            let mut in_flight = self.in_flight.write().await;
            in_flight
                .entry(key.clone())
                .or_insert_with(|| Arc::new(Semaphore::new(1)))
                .clone()
        };

        // Acquire permit (blocks if another request is already fetching)
        let _permit = semaphore.acquire().await.map_err(|e| {
            AppError::internal_error(
                "fetch_currency_exchange_data",
                &format!("Failed to acquire fetch lock: {}", e),
            )
        })?;

        log::debug!("Acquired fetch lock for {}:{:?}", league, economy_type);

        // Re-check cache after acquiring lock (another request may have fetched)
        if let Some(data) = self
            .repository
            .load_fresh_exchange_data(league, is_hardcore, economy_type, CACHE_TTL_SECONDS)
            .await?
        {
            log::info!(
                "Cache became fresh while waiting for lock (coalesced fetch) for {}:{:?}",
                league,
                economy_type
            );
            // Clean up the semaphore since we're done
            self.cleanup_semaphore(&key).await;
            return Ok(data);
        }

        log::info!(
            "Cache is stale or missing for league: {}, type: {}",
            league,
            economy_type
        );

        // Still stale - proceed with fetch from poe.ninja
        // Handle league name for hardcore economies
        // Special case: Standard + hardcore = "Hardcore"
        // Special case: Remove "The " prefix from league names for poe.ninja API
        let mut league_name = league.to_string();
        if league_name.starts_with("The ") {
            league_name = league_name.strip_prefix("The ").unwrap().to_string();
        }

        let league_name = if is_hardcore {
            if league.eq_ignore_ascii_case("Standard") {
                "Hardcore".to_string()
            } else {
                format!("HC {}", league_name)
            }
        } else {
            league_name
        };

        let url = Self::build_poe_ninja_url(&league_name, economy_type);

        log::info!(
            "Fetching economy data from poe.ninja - league: {}, hardcore: {}, type: {}",
            league_name,
            is_hardcore,
            economy_type
        );

        // Try to fetch from poe.ninja
        let result = match self.fetch_from_poe_ninja(&url).await {
            Ok(data) => {
                log::info!(
                    "Successfully fetched {} economy data: {} items, {} exchange rates",
                    economy_type,
                    data.currencies.len(),
                    data.currencies.len()
                );

                // Save to database
                if let Err(e) = self
                    .repository
                    .save_exchange_data(league, is_hardcore, economy_type, &data)
                    .await
                {
                    log::warn!("Failed to save cache to database: {}", e);
                }

                Ok(data)
            }
            Err(e) => {
                log::warn!("Failed to fetch from poe.ninja: {}", e);

                // Graceful degradation - return stale cache if available
                if let Some(mut stale_data) = self
                    .repository
                    .load_exchange_data(league, is_hardcore, economy_type)
                    .await?
                {
                    log::info!(
                        "Returning stale cached data for league: {}, type: {}",
                        league,
                        economy_type
                    );
                    stale_data.is_stale = Some(true);
                    Ok(stale_data)
                } else {
                    // No cache available - return the error
                    Err(e)
                }
            }
        };

        // Clean up the semaphore after fetch completes (success or failure)
        self.cleanup_semaphore(&key).await;

        result
    }

    /// Remove semaphore from in-flight map after fetch completes
    async fn cleanup_semaphore(&self, key: &str) {
        let mut in_flight = self.in_flight.write().await;
        in_flight.remove(key);
        log::trace!("Removed fetch lock for {}", key);
    }

    /// Fetch data from poe.ninja API with automatic retry on transient failures.
    ///
    /// Retries up to MAX_RETRY_ATTEMPTS times with exponential backoff on network errors.
    /// Does NOT retry on 4xx errors (client errors) - only 5xx and network failures.
    async fn fetch_from_poe_ninja(&self, url: &str) -> AppResult<CurrencyExchangeData> {
        let mut last_error = None;

        for attempt in 0..MAX_RETRY_ATTEMPTS {
            // Apply delay (0ms for first attempt)
            let delay = Self::calculate_retry_delay(attempt);
            if delay.as_millis() > 0 {
                log::debug!(
                    "Retry attempt {}/{} after {}ms delay",
                    attempt + 1,
                    MAX_RETRY_ATTEMPTS,
                    delay.as_millis()
                );
                tokio::time::sleep(delay).await;
            }

            // Attempt fetch
            match self.try_fetch_from_poe_ninja(url).await {
                Ok(data) => {
                    if attempt > 0 {
                        log::info!("Successfully fetched data after {} retry attempts", attempt);
                    }
                    return Ok(data);
                }
                Err(e) => {
                    if Self::is_retryable_error(&e) {
                        log::warn!(
                            "Retryable error on attempt {}/{}: {}",
                            attempt + 1,
                            MAX_RETRY_ATTEMPTS,
                            e
                        );
                        last_error = Some(e);
                    } else {
                        // Non-retryable error - fail immediately
                        log::error!("Non-retryable error, not retrying: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        // All retries exhausted
        Err(last_error.unwrap_or_else(|| {
            AppError::network_error(
                "fetch_from_poe_ninja",
                "All retry attempts failed with no error captured",
            )
        }))
    }

    /// Single attempt to fetch data from poe.ninja (no retries).
    async fn try_fetch_from_poe_ninja(&self, url: &str) -> AppResult<CurrencyExchangeData> {
        let response = self.client.get(url).send().await.map_err(|e| {
            log::error!("Failed to fetch currency data: {}", e);
            AppError::network_error("fetch_currency_data", &format!("{}", e))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            log::error!("poe.ninja API returned error status: {}", status);

            // Differentiate between 4xx (client error) and 5xx (server error)
            return if status.is_client_error() {
                Err(AppError::validation_error(
                    "poe.ninja_api",
                    &format!("API client error: {}", status),
                ))
            } else {
                Err(AppError::network_error(
                    "poe.ninja_api",
                    &format!("API server error: {}", status),
                ))
            };
        }

        let api_response = response
            .json::<CurrencyExchangeApiResponse>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse currency data: {}", e);
                AppError::serialization_error("parse_currency_data", &format!("{}", e))
            })?;

        api_response.into_frontend_data().map_err(|e| {
            log::warn!(
                "Failed to convert API response to frontend data for {} economy type: {}",
                url,
                e
            );

            if e.contains("No currency data available") {
                AppError::validation_error("convert_api_response", &e)
            } else {
                AppError::serialization_error("process_currency_data", &e)
            }
        })
    }

    /// Load and aggregate top currencies across all economy types for a league
    pub async fn load_aggregated_top_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
    ) -> AppResult<Vec<TopCurrencyItem>> {
        let items = self
            .repository
            .load_top_currencies(league, is_hardcore, 10)
            .await?;

        log::info!(
            "Loaded {} aggregated top currencies for league: {}",
            items.len(),
            league
        );

        Ok(items)
    }

    /// Search for currencies across all cached economy types by name
    pub async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
    ) -> AppResult<Vec<CurrencySearchResult>> {
        let results = self
            .repository
            .search_currencies(league, is_hardcore, query)
            .await?;

        log::info!(
            "Found {} matching currencies for query '{}' in league: {}",
            results.len(),
            query,
            league
        );

        Ok(results)
    }
}
