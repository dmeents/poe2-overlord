//! Economy repository implementation for SQLite-based caching

use super::models::{
    CurrencyExchangeData, CurrencyExchangeRate, CurrencyInfo, CurrencySearchResult, CurrencyTier,
    DisplayValue, EconomyType, TopCurrencyItem,
};
use super::traits::EconomyRepository;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use chrono::Utc;
use serde_json;
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

pub struct EconomyRepositoryImpl {
    pool: SqlitePool,
}

impl EconomyRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Parse RFC3339 timestamp and check if elapsed time is within TTL
    fn is_fresh(last_updated: &str, ttl_seconds: u64) -> bool {
        chrono::DateTime::parse_from_rfc3339(last_updated)
            .ok()
            .map(|time| {
                let now = Utc::now();
                let cached_time_utc = time.with_timezone(&Utc);
                let elapsed = now.signed_duration_since(cached_time_utc);
                let ttl_i64 = i64::try_from(ttl_seconds).unwrap_or(i64::MAX);
                elapsed.num_seconds() < ttl_i64
            })
            .unwrap_or(false)
    }

    /// Build CurrencyExchangeData from exchange rate row and currency items
    async fn build_exchange_data(
        &self,
        exchange_rate_id: i64,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
    ) -> AppResult<CurrencyExchangeData> {
        // Load exchange rate metadata
        let rate_row = sqlx::query(
            "SELECT primary_currency_id, primary_currency_name, primary_currency_image,
                    secondary_currency_id, secondary_currency_name, secondary_currency_image,
                    tertiary_currency_id, tertiary_currency_name, tertiary_currency_image,
                    secondary_rate, tertiary_rate, fetched_at
             FROM economy_exchange_rates
             WHERE league = ? AND is_hardcore = ? AND economy_type = ?",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "build_exchange_data",
                &format!("Failed to load exchange rate metadata: {}", e),
            )
        })?;

        let primary_currency = CurrencyInfo {
            id: rate_row.get("primary_currency_id"),
            name: rate_row.get("primary_currency_name"),
            image_url: rate_row.get("primary_currency_image"),
        };

        let secondary_currency = CurrencyInfo {
            id: rate_row.get("secondary_currency_id"),
            name: rate_row.get("secondary_currency_name"),
            image_url: rate_row.get("secondary_currency_image"),
        };

        let tertiary_currency: Option<String> = rate_row.get("tertiary_currency_id");
        let tertiary_currency = tertiary_currency.map(|id| CurrencyInfo {
            id: id.clone(),
            name: rate_row.get("tertiary_currency_name"),
            image_url: rate_row.get("tertiary_currency_image"),
        });

        let secondary_rate: f64 = rate_row.get("secondary_rate");
        let tertiary_rate: Option<f64> = rate_row.get("tertiary_rate");
        let fetched_at: String = rate_row.get("fetched_at");

        // Load currency items
        let item_rows = sqlx::query(
            "SELECT currency_id, name, image_url, primary_value, secondary_value, tertiary_value,
                    volume, change_percent, display_tier, display_value, display_inverted,
                    display_currency_id, display_currency_name, display_currency_image, price_history
             FROM currency_items
             WHERE exchange_rate_id = ? AND is_active = 1
             ORDER BY primary_value DESC",
        )
        .bind(exchange_rate_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "build_exchange_data",
                &format!("Failed to load currency items: {}", e),
            )
        })?;

        let currencies: Vec<CurrencyExchangeRate> = item_rows
            .into_iter()
            .map(|row| {
                let tier_str: String = row.get("display_tier");
                let tier = match tier_str.as_str() {
                    "Primary" => CurrencyTier::Primary,
                    "Secondary" => CurrencyTier::Secondary,
                    "Tertiary" => CurrencyTier::Tertiary,
                    _ => CurrencyTier::Primary,
                };

                let price_history_json: String = row.get("price_history");
                let price_history: Vec<Option<f64>> =
                    serde_json::from_str(&price_history_json).unwrap_or_default();

                CurrencyExchangeRate {
                    id: row.get("currency_id"),
                    name: row.get("name"),
                    image_url: row.get("image_url"),
                    display_value: DisplayValue {
                        tier,
                        value: row.get("display_value"),
                        inverted: row.get::<i32, _>("display_inverted") != 0,
                        currency_id: row.get("display_currency_id"),
                        currency_name: row.get("display_currency_name"),
                        currency_image_url: row.get("display_currency_image"),
                    },
                    primary_value: row.get("primary_value"),
                    secondary_value: row.get("secondary_value"),
                    tertiary_value: row.get("tertiary_value"),
                    volume: row.get("volume"),
                    change_percent: row.get("change_percent"),
                    price_history,
                }
            })
            .collect();

        Ok(CurrencyExchangeData {
            primary_currency,
            secondary_currency,
            tertiary_currency,
            secondary_rate,
            tertiary_rate,
            currencies,
            fetched_at,
            is_stale: None,
        })
    }
}

#[async_trait]
impl EconomyRepository for EconomyRepositoryImpl {
    async fn load_fresh_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        ttl_seconds: u64,
    ) -> AppResult<Option<CurrencyExchangeData>> {
        // Query exchange rate row
        let rate_result = sqlx::query(
            "SELECT id, last_updated FROM economy_exchange_rates
             WHERE league = ? AND is_hardcore = ? AND economy_type = ?",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "load_fresh_exchange_data",
                &format!("Failed to query exchange rates: {}", e),
            )
        })?;

        // No row found
        let Some(row) = rate_result else {
            return Ok(None);
        };

        let exchange_rate_id: i64 = row.get("id");
        let last_updated: String = row.get("last_updated");

        // Check TTL
        if !Self::is_fresh(&last_updated, ttl_seconds) {
            return Ok(None);
        }

        // Build and return data
        let data = self
            .build_exchange_data(exchange_rate_id, league, is_hardcore, economy_type)
            .await?;
        Ok(Some(data))
    }

    async fn load_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
    ) -> AppResult<Option<CurrencyExchangeData>> {
        // Query exchange rate row (no TTL check)
        let rate_result = sqlx::query(
            "SELECT id FROM economy_exchange_rates
             WHERE league = ? AND is_hardcore = ? AND economy_type = ?",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "load_exchange_data",
                &format!("Failed to query exchange rates: {}", e),
            )
        })?;

        // No row found
        let Some(row) = rate_result else {
            return Ok(None);
        };

        let exchange_rate_id: i64 = row.get("id");

        // Build and return data
        let data = self
            .build_exchange_data(exchange_rate_id, league, is_hardcore, economy_type)
            .await?;
        Ok(Some(data))
    }

    async fn save_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            AppError::internal_error("save_exchange_data", &format!("Failed to begin transaction: {}", e))
        })?;

        let now = Utc::now().to_rfc3339();

        // Upsert exchange rate row
        sqlx::query(
            "INSERT INTO economy_exchange_rates (
                league, is_hardcore, economy_type,
                primary_currency_id, primary_currency_name, primary_currency_image,
                secondary_currency_id, secondary_currency_name, secondary_currency_image,
                tertiary_currency_id, tertiary_currency_name, tertiary_currency_image,
                secondary_rate, tertiary_rate, fetched_at, last_updated
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(league, is_hardcore, economy_type) DO UPDATE SET
                primary_currency_id = excluded.primary_currency_id,
                primary_currency_name = excluded.primary_currency_name,
                primary_currency_image = excluded.primary_currency_image,
                secondary_currency_id = excluded.secondary_currency_id,
                secondary_currency_name = excluded.secondary_currency_name,
                secondary_currency_image = excluded.secondary_currency_image,
                tertiary_currency_id = excluded.tertiary_currency_id,
                tertiary_currency_name = excluded.tertiary_currency_name,
                tertiary_currency_image = excluded.tertiary_currency_image,
                secondary_rate = excluded.secondary_rate,
                tertiary_rate = excluded.tertiary_rate,
                fetched_at = excluded.fetched_at,
                last_updated = excluded.last_updated",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .bind(&data.primary_currency.id)
        .bind(&data.primary_currency.name)
        .bind(&data.primary_currency.image_url)
        .bind(&data.secondary_currency.id)
        .bind(&data.secondary_currency.name)
        .bind(&data.secondary_currency.image_url)
        .bind(data.tertiary_currency.as_ref().map(|c| &c.id))
        .bind(data.tertiary_currency.as_ref().map(|c| &c.name))
        .bind(data.tertiary_currency.as_ref().map(|c| &c.image_url))
        .bind(data.secondary_rate)
        .bind(data.tertiary_rate)
        .bind(&data.fetched_at)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "save_exchange_data",
                &format!("Failed to upsert exchange rate: {}", e),
            )
        })?;

        // Get the exchange_rate_id
        let exchange_rate_id: i64 = sqlx::query_scalar(
            "SELECT id FROM economy_exchange_rates
             WHERE league = ? AND is_hardcore = ? AND economy_type = ?",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "save_exchange_data",
                &format!("Failed to retrieve exchange_rate_id: {}", e),
            )
        })?;

        // Upsert each currency item
        for currency in &data.currencies {
            let price_history_json = serde_json::to_string(&currency.price_history)
                .unwrap_or_else(|_| "[]".to_string());

            let display_tier = match currency.display_value.tier {
                CurrencyTier::Primary => "Primary",
                CurrencyTier::Secondary => "Secondary",
                CurrencyTier::Tertiary => "Tertiary",
            };

            sqlx::query(
                "INSERT INTO currency_items (
                    exchange_rate_id, currency_id, name, image_url,
                    primary_value, secondary_value, tertiary_value,
                    volume, change_percent,
                    display_tier, display_value, display_inverted,
                    display_currency_id, display_currency_name, display_currency_image,
                    price_history, is_active, last_updated
                 ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?)
                 ON CONFLICT(exchange_rate_id, currency_id) DO UPDATE SET
                    name = excluded.name,
                    image_url = excluded.image_url,
                    primary_value = excluded.primary_value,
                    secondary_value = excluded.secondary_value,
                    tertiary_value = excluded.tertiary_value,
                    volume = excluded.volume,
                    change_percent = excluded.change_percent,
                    display_tier = excluded.display_tier,
                    display_value = excluded.display_value,
                    display_inverted = excluded.display_inverted,
                    display_currency_id = excluded.display_currency_id,
                    display_currency_name = excluded.display_currency_name,
                    display_currency_image = excluded.display_currency_image,
                    price_history = excluded.price_history,
                    last_updated = excluded.last_updated
                    -- is_active intentionally omitted to preserve manual deactivations",
            )
            .bind(exchange_rate_id)
            .bind(&currency.id)
            .bind(&currency.name)
            .bind(&currency.image_url)
            .bind(currency.primary_value)
            .bind(currency.secondary_value)
            .bind(currency.tertiary_value)
            .bind(currency.volume)
            .bind(currency.change_percent)
            .bind(display_tier)
            .bind(currency.display_value.value)
            .bind(currency.display_value.inverted as i32)
            .bind(&currency.display_value.currency_id)
            .bind(&currency.display_value.currency_name)
            .bind(&currency.display_value.currency_image_url)
            .bind(&price_history_json)
            .bind(&now)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "save_exchange_data",
                    &format!("Failed to upsert currency item {}: {}", currency.id, e),
                )
            })?;
        }

        tx.commit().await.map_err(|e| {
            AppError::internal_error(
                "save_exchange_data",
                &format!("Failed to commit transaction: {}", e),
            )
        })?;

        Ok(())
    }

    async fn load_top_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        limit: u32,
    ) -> AppResult<Vec<TopCurrencyItem>> {
        let rows = sqlx::query(
            "SELECT ci.currency_id, ci.name, ci.image_url, er.economy_type,
                    ci.primary_value, er.primary_currency_name, er.primary_currency_image,
                    ci.volume, ci.change_percent, er.last_updated
             FROM currency_items ci
             JOIN economy_exchange_rates er ON ci.exchange_rate_id = er.id
             WHERE er.league = ? AND er.is_hardcore = ? AND ci.is_active = 1
             ORDER BY ci.primary_value DESC
             LIMIT ?",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "load_top_currencies",
                &format!("Failed to load top currencies: {}", e),
            )
        })?;

        let results = rows
            .into_iter()
            .filter_map(|row| {
                let economy_type_str: String = row.get("economy_type");
                let economy_type = EconomyType::from_str(&economy_type_str).ok()?;

                Some(TopCurrencyItem {
                    id: row.get("currency_id"),
                    name: row.get("name"),
                    image_url: row.get("image_url"),
                    economy_type,
                    primary_value: row.get("primary_value"),
                    primary_currency_name: row.get("primary_currency_name"),
                    primary_currency_image_url: row.get("primary_currency_image"),
                    volume: row.get("volume"),
                    change_percent: row.get("change_percent"),
                    cached_at: row.get("last_updated"),
                })
            })
            .collect();

        Ok(results)
    }

    async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
    ) -> AppResult<Vec<CurrencySearchResult>> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query(
            "SELECT ci.currency_id, ci.name, ci.image_url, er.economy_type,
                    ci.primary_value, er.primary_currency_name, er.primary_currency_image,
                    ci.secondary_value, ci.tertiary_value, ci.volume, ci.change_percent,
                    ci.display_tier, ci.display_value, ci.display_inverted,
                    ci.display_currency_id, ci.display_currency_name, ci.display_currency_image
             FROM currency_items ci
             JOIN economy_exchange_rates er ON ci.exchange_rate_id = er.id
             WHERE er.league = ? AND er.is_hardcore = ? AND ci.is_active = 1
               AND ci.name LIKE ? COLLATE NOCASE
             ORDER BY ci.primary_value DESC",
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "search_currencies",
                &format!("Failed to search currencies: {}", e),
            )
        })?;

        let results = rows
            .into_iter()
            .filter_map(|row| {
                let economy_type_str: String = row.get("economy_type");
                let economy_type = EconomyType::from_str(&economy_type_str).ok()?;

                let tier_str: String = row.get("display_tier");
                let tier = match tier_str.as_str() {
                    "Primary" => CurrencyTier::Primary,
                    "Secondary" => CurrencyTier::Secondary,
                    "Tertiary" => CurrencyTier::Tertiary,
                    _ => CurrencyTier::Primary,
                };

                Some(CurrencySearchResult {
                    id: row.get("currency_id"),
                    name: row.get("name"),
                    image_url: row.get("image_url"),
                    economy_type,
                    primary_value: row.get("primary_value"),
                    primary_currency_name: row.get("primary_currency_name"),
                    primary_currency_image_url: row.get("primary_currency_image"),
                    secondary_value: row.get("secondary_value"),
                    tertiary_value: row.get("tertiary_value"),
                    volume: row.get("volume"),
                    change_percent: row.get("change_percent"),
                    display_value: DisplayValue {
                        tier,
                        value: row.get("display_value"),
                        inverted: row.get::<i32, _>("display_inverted") != 0,
                        currency_id: row.get("display_currency_id"),
                        currency_name: row.get("display_currency_name"),
                        currency_image_url: row.get("display_currency_image"),
                    },
                })
            })
            .collect();

        Ok(results)
    }
}
