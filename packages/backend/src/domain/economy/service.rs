use super::models::{
    CurrencyExchangeApiResponse, CurrencyExchangeData, EconomyType, LeagueTopCurrenciesCache,
    TopCurrencyItem,
};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::file_management::paths::AppPaths;
use crate::infrastructure::file_management::service::FileService;
use reqwest;
use std::collections::HashMap;
use std::path::PathBuf;

const POE_NINJA_API_BASE: &str = "https://poe.ninja/poe2/api/economy/exchange/current/overview";

#[derive(Debug, Clone)]
pub struct EconomyService {
    pub(crate) client: reqwest::Client,
}

impl EconomyService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_currency_exchange_data(
        &self,
        league: &str,
        economy_type: EconomyType,
    ) -> AppResult<CurrencyExchangeData> {
        let url = format!(
            "{}?league={}&type={}",
            POE_NINJA_API_BASE,
            urlencoding::encode(league),
            economy_type.as_str()
        );

        log::info!(
            "Fetching economy data from poe.ninja - league: {}, type: {}",
            league,
            economy_type
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
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

        log::info!(
            "Successfully fetched {} economy data: {} items, {} exchange rates",
            economy_type,
            api_response.items.len(),
            api_response.lines.len()
        );

        let data = api_response.into_frontend_data().map_err(|e| {
            log::error!("Failed to convert API response to frontend data: {}", e);
            AppError::Serialization {
                message: format!("Failed to process currency data: {}", e),
            }
        })?;

        // Cache top 10 currencies (don't fail if caching fails)
        if let Err(e) = self
            .save_top_currencies_to_cache(league, economy_type, &data)
            .await
        {
            log::warn!("Failed to cache top currencies: {}", e);
        }

        Ok(data)
    }

    /// Save top 10 currencies for a specific economy type to the league cache
    pub async fn save_top_currencies_to_cache(
        &self,
        league: &str,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()> {
        // Get cache file path
        let cache_path = Self::get_cache_path(league).await?;

        // Load existing cache or create new one
        let mut cache = FileService::read_json_optional::<LeagueTopCurrenciesCache>(&cache_path)
            .await?
            .unwrap_or_else(|| LeagueTopCurrenciesCache {
                league: league.to_string(),
                types: HashMap::new(),
                last_updated: chrono::Utc::now().to_rfc3339(),
                primary_currency_name: data.primary_currency.name.clone(),
            });

        // Extract top 10 by primary_value
        let mut sorted_currencies = data.currencies.clone();
        sorted_currencies.sort_by(|a, b| {
            b.primary_value
                .partial_cmp(&a.primary_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let top_items: Vec<TopCurrencyItem> = sorted_currencies
            .into_iter()
            .take(10)
            .map(|curr| TopCurrencyItem {
                id: curr.id,
                name: curr.name,
                image_url: curr.image_url,
                economy_type,
                primary_value: curr.primary_value,
                primary_currency_name: data.primary_currency.name.clone(),
                primary_currency_image_url: data.primary_currency.image_url.clone(),
                volume: curr.volume,
                change_percent: curr.change_percent,
                cached_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect();

        // Update cache for this type
        cache
            .types
            .insert(economy_type.as_str().to_string(), top_items);
        cache.last_updated = chrono::Utc::now().to_rfc3339();
        cache.primary_currency_name = data.primary_currency.name.clone();

        // Save to disk
        FileService::write_json(&cache_path, &cache).await?;

        log::info!(
            "Saved top 10 {} currencies to cache for league: {}",
            economy_type,
            league
        );

        Ok(())
    }

    /// Load and aggregate top currencies across all economy types for a league
    pub async fn load_aggregated_top_currencies(
        &self,
        league: &str,
    ) -> AppResult<Vec<TopCurrencyItem>> {
        let cache_path = Self::get_cache_path(league).await?;

        // Load cache file
        let cache =
            FileService::read_json_optional::<LeagueTopCurrenciesCache>(&cache_path).await?;

        // If no cache exists, return empty array
        let Some(cache) = cache else {
            log::info!("No cache found for league: {}", league);
            return Ok(Vec::new());
        };

        // Collect all items from all types
        let mut all_items: Vec<TopCurrencyItem> = cache.types.values().flatten().cloned().collect();

        // Sort by primary_value descending
        all_items.sort_by(|a, b| {
            b.primary_value
                .partial_cmp(&a.primary_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top 10
        all_items.truncate(10);

        log::info!(
            "Loaded {} aggregated top currencies for league: {}",
            all_items.len(),
            league
        );

        Ok(all_items)
    }

    async fn get_cache_path(league: &str) -> AppResult<PathBuf> {
        let data_dir = AppPaths::ensure_data_dir().await?;
        let cache_dir = data_dir.join("economy_cache");

        // Ensure cache directory exists
        AppPaths::ensure_dir(&cache_dir).await?;

        // Sanitize league name for filename
        let safe_league_name = league.replace(" ", "_").replace("/", "-");
        let filename = format!("{}.json", safe_league_name);

        Ok(cache_dir.join(filename))
    }
}

impl Default for EconomyService {
    fn default() -> Self {
        Self::new()
    }
}
