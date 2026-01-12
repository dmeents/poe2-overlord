//! Economy service for fetching and caching currency exchange data from poe.ninja.
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
    LeagueEconomyCache, TopCurrencyItem,
};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::file_management::paths::AppPaths;
use crate::infrastructure::file_management::service::FileService;
use reqwest;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};

const POE_NINJA_API_BASE: &str = "https://poe.ninja/poe2/api/economy/exchange/current/overview";
const CACHE_TTL_SECONDS: u64 = 600; // 10 minutes

/// Economy service with concurrent request deduplication.
///
/// Uses per-cache-key semaphores to ensure only one API fetch happens at a time
/// for the same league+hardcore+economy_type combination.
#[derive(Debug, Clone)]
pub struct EconomyService {
    pub(crate) client: reqwest::Client,
    /// Tracks in-flight API fetches per cache key to prevent race conditions.
    /// Key format: "{league}:{is_hardcore}:{economy_type}"
    in_flight: Arc<RwLock<HashMap<String, Arc<Semaphore>>>>,
}

impl EconomyService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10)) // 10 second total timeout
                .connect_timeout(Duration::from_secs(5)) // 5 second connect timeout
                .build()
                .expect("Failed to build HTTP client"),
            in_flight: Arc::new(RwLock::new(HashMap::new())),
        }
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
            return Err(AppError::Validation {
                message: "League name cannot be empty".to_string(),
            });
        }

        // FAST PATH: Check for fresh cache without acquiring any locks
        let cache_path = Self::get_league_cache_path(league, is_hardcore).await?;
        let cache = FileService::read_json_optional::<LeagueEconomyCache>(&cache_path).await?;

        if let Some(ref cache) = cache {
            if let Some(cached) = cache.get_economy_type(economy_type) {
                if cached.is_fresh() {
                    log::debug!(
                        "Fast path: Fresh cache for {}:{:?} (no lock needed)",
                        league,
                        economy_type
                    );
                    return Ok(cached.data.clone());
                }
            }
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
        let mut cache = FileService::read_json_optional::<LeagueEconomyCache>(&cache_path)
            .await?
            .unwrap_or_else(|| LeagueEconomyCache::new(league.to_string(), is_hardcore));

        if let Some(cached) = cache.get_economy_type(economy_type) {
            if cached.is_fresh() {
                log::info!(
                    "Cache became fresh while waiting for lock (coalesced fetch) for {}:{:?}",
                    league,
                    economy_type
                );
                // Clean up the semaphore since we're done
                self.cleanup_semaphore(&key).await;
                return Ok(cached.data.clone());
            } else {
                log::info!(
                    "Cache exists but is stale for league: {}, type: {} (cached {} seconds ago)",
                    league,
                    economy_type,
                    Self::seconds_since(&cached.cached_at)
                );
            }
        } else {
            log::info!(
                "No cache found for league: {}, type: {}",
                league,
                economy_type
            );
        }

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

        let url = format!(
            "{}?league={}&type={}",
            POE_NINJA_API_BASE,
            urlencoding::encode(&league_name),
            economy_type.as_str()
        );

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

                // Update cache with fresh data
                cache.update_economy_type(economy_type, data.clone(), CACHE_TTL_SECONDS);

                // Save cache to disk (don't fail the request if this fails)
                if let Err(e) = FileService::write_json(&cache_path, &cache).await {
                    log::warn!("Failed to save cache to disk: {}", e);
                }

                Ok(data)
            }
            Err(e) => {
                log::warn!("Failed to fetch from poe.ninja: {}", e);

                // Graceful degradation - return stale cache if available
                if let Some(cached) = cache.get_economy_type(economy_type) {
                    log::info!(
                        "Returning stale cached data for league: {}, type: {} (cached {} seconds ago)",
                        league,
                        economy_type,
                        Self::seconds_since(&cached.cached_at)
                    );

                    let mut stale_data = cached.data.clone();
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

    /// Fetch data from poe.ninja API
    async fn fetch_from_poe_ninja(&self, url: &str) -> AppResult<CurrencyExchangeData> {
        let response = self.client.get(url).send().await.map_err(|e| {
            log::error!("Failed to fetch currency data: {}", e);
            AppError::Network {
                message: format!("Failed to fetch currency data: {}", e),
            }
        })?;

        if !response.status().is_success() {
            let status = response.status();
            log::error!("poe.ninja API returned error status: {}", status);
            return Err(AppError::Network {
                message: format!("poe.ninja API returned error: {}", status),
            });
        }

        let api_response = response
            .json::<CurrencyExchangeApiResponse>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse currency data: {}", e);
                AppError::Serialization {
                    message: format!("Failed to parse currency data: {}", e),
                }
            })?;

        api_response.into_frontend_data().map_err(|e| {
            log::warn!(
                "Failed to convert API response to frontend data for {} economy type: {}",
                url,
                e
            );

            // Provide more user-friendly error messages
            if e.contains("No currency data available") {
                AppError::Validation { message: e }
            } else {
                AppError::Serialization {
                    message: format!("Failed to process currency data: {}", e),
                }
            }
        })
    }

    /// Load and aggregate top currencies across all economy types for a league
    pub async fn load_aggregated_top_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
    ) -> AppResult<Vec<TopCurrencyItem>> {
        let cache_path = Self::get_league_cache_path(league, is_hardcore).await?;

        // Load cache file
        let cache = FileService::read_json_optional::<LeagueEconomyCache>(&cache_path).await?;

        // If no cache exists, return empty array
        let Some(cache) = cache else {
            log::info!("No cache found for league: {}", league);
            return Ok(Vec::new());
        };

        // Collect ALL items from all economy types (don't truncate per type)
        let mut all_items: Vec<TopCurrencyItem> = Vec::new();

        for (economy_type_str, cached_data) in cache.economy_types.iter() {
            // Parse economy type from string using FromStr
            let Ok(economy_type) = economy_type_str.parse::<EconomyType>() else {
                continue;
            };

            // Extract ALL currencies and convert to TopCurrencyItem
            let data = &cached_data.data;
            let currencies: Vec<TopCurrencyItem> = data
                .currencies
                .iter()
                .map(|curr| TopCurrencyItem {
                    id: curr.id.clone(),
                    name: curr.name.clone(),
                    image_url: curr.image_url.clone(),
                    economy_type,
                    primary_value: curr.primary_value,
                    primary_currency_name: data.primary_currency.name.clone(),
                    primary_currency_image_url: data.primary_currency.image_url.clone(),
                    volume: curr.volume,
                    change_percent: curr.change_percent,
                    cached_at: cached_data.cached_at.clone(),
                })
                .collect();

            all_items.extend(currencies);
        }

        // Sort all items by primary_value descending
        all_items.sort_by(|a, b| {
            b.primary_value
                .partial_cmp(&a.primary_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top 10 overall
        all_items.truncate(10);

        log::info!(
            "Loaded {} aggregated top currencies for league: {} from {} economy types",
            all_items.len(),
            league,
            cache.economy_types.len()
        );

        Ok(all_items)
    }

    /// Search for currencies across all cached economy types by name
    pub async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
    ) -> AppResult<Vec<CurrencySearchResult>> {
        let cache_path = Self::get_league_cache_path(league, is_hardcore).await?;

        // Load cache file
        let cache = FileService::read_json_optional::<LeagueEconomyCache>(&cache_path).await?;

        // If no cache exists, return empty array
        let Some(cache) = cache else {
            log::info!("No cache found for league: {}", league);
            return Ok(Vec::new());
        };

        // Normalize query for case-insensitive search
        let query_lower = query.to_lowercase();

        // Collect all matching items from all economy types
        let mut results: Vec<CurrencySearchResult> = Vec::new();

        for (economy_type_str, cached_data) in cache.economy_types.iter() {
            // Parse economy type from string using FromStr
            let Ok(economy_type) = economy_type_str.parse::<EconomyType>() else {
                continue;
            };

            // Filter currencies by name match
            let data = &cached_data.data;
            let matching_currencies: Vec<CurrencySearchResult> = data
                .currencies
                .iter()
                .filter(|curr| curr.name.to_lowercase().contains(&query_lower))
                .map(|curr| CurrencySearchResult {
                    id: curr.id.clone(),
                    name: curr.name.clone(),
                    image_url: curr.image_url.clone(),
                    economy_type,
                    primary_value: curr.primary_value,
                    primary_currency_name: data.primary_currency.name.clone(),
                    primary_currency_image_url: data.primary_currency.image_url.clone(),
                    secondary_value: curr.secondary_value,
                    tertiary_value: curr.tertiary_value,
                    volume: curr.volume,
                    change_percent: curr.change_percent,
                    display_value: curr.display_value.clone(),
                })
                .collect();

            results.extend(matching_currencies);
        }

        // Sort by primary_value descending
        results.sort_by(|a, b| {
            b.primary_value
                .partial_cmp(&a.primary_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        log::info!(
            "Found {} matching currencies for query '{}' in league: {} across {} economy types",
            results.len(),
            query,
            league,
            cache.economy_types.len()
        );

        Ok(results)
    }

    /// Get the cache file path for a league
    pub async fn get_league_cache_path(league: &str, is_hardcore: bool) -> AppResult<PathBuf> {
        let data_dir = AppPaths::ensure_data_dir().await?;
        let cache_dir = data_dir.join("economy_cache");

        // Ensure cache directory exists
        AppPaths::ensure_dir(&cache_dir).await?;

        // Strip "The " prefix to match API normalization
        let mut league_name = league.to_string();
        if league_name.starts_with("The ") {
            league_name = league_name.strip_prefix("The ").unwrap().to_string();
        }

        // Prepend "HC_" to filename for hardcore leagues
        let league_prefix = if is_hardcore { "HC_" } else { "" };

        // Sanitize league name for filename
        let safe_league_name = league_name.replace(" ", "_").replace("/", "-");
        let filename = format!("{}{}.json", league_prefix, safe_league_name);

        Ok(cache_dir.join(filename))
    }

    /// Calculate seconds since a given RFC3339 timestamp
    fn seconds_since(timestamp: &str) -> i64 {
        chrono::DateTime::parse_from_rfc3339(timestamp)
            .ok()
            .map(|time| {
                let now = chrono::Utc::now();
                now.signed_duration_since(time).num_seconds()
            })
            .unwrap_or(0)
    }
}

impl Default for EconomyService {
    fn default() -> Self {
        Self::new()
    }
}
