//! Economy domain models for currency exchange data from poe.ninja API

use crate::errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CurrencyTier {
    Primary,
    Secondary,
    Tertiary,
}

impl CurrencyTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrencyTier::Primary => "Primary",
            CurrencyTier::Secondary => "Secondary",
            CurrencyTier::Tertiary => "Tertiary",
        }
    }
}

impl fmt::Display for CurrencyTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for CurrencyTier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Primary" => Ok(CurrencyTier::Primary),
            "Secondary" => Ok(CurrencyTier::Secondary),
            "Tertiary" => Ok(CurrencyTier::Tertiary),
            _ => Err(format!("Unknown currency tier: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayValue {
    pub tier: CurrencyTier,
    pub value: f64,
    pub inverted: bool,
    pub currency_id: String,
    pub currency_name: String,
    pub currency_image_url: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EconomyType {
    Currency,
    Fragments,
    Abyss,
    UncutGems,
    LineageSupportGems,
    Essences,
    SoulCores,
    Idols,
    Runes,
    Ritual,
    Expedition,
    Delirium,
    Breach,
}

impl EconomyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EconomyType::Currency => "Currency",
            EconomyType::Fragments => "Fragments",
            EconomyType::Abyss => "Abyss",
            EconomyType::UncutGems => "UncutGems",
            EconomyType::LineageSupportGems => "LineageSupportGems",
            EconomyType::Essences => "Essences",
            EconomyType::SoulCores => "SoulCores",
            EconomyType::Idols => "Idols",
            EconomyType::Runes => "Runes",
            EconomyType::Ritual => "Ritual",
            EconomyType::Expedition => "Expedition",
            EconomyType::Delirium => "Delirium",
            EconomyType::Breach => "Breach",
        }
    }

    pub fn all() -> Vec<EconomyType> {
        vec![
            EconomyType::Currency,
            EconomyType::Fragments,
            EconomyType::Abyss,
            EconomyType::UncutGems,
            EconomyType::LineageSupportGems,
            EconomyType::Essences,
            EconomyType::SoulCores,
            EconomyType::Idols,
            EconomyType::Runes,
            EconomyType::Ritual,
            EconomyType::Expedition,
            EconomyType::Delirium,
            EconomyType::Breach,
        ]
    }
}

impl fmt::Display for EconomyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for EconomyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Currency" => Ok(EconomyType::Currency),
            "Fragments" => Ok(EconomyType::Fragments),
            "Abyss" => Ok(EconomyType::Abyss),
            "UncutGems" => Ok(EconomyType::UncutGems),
            "LineageSupportGems" => Ok(EconomyType::LineageSupportGems),
            "Essences" => Ok(EconomyType::Essences),
            "SoulCores" => Ok(EconomyType::SoulCores),
            "Idols" => Ok(EconomyType::Idols),
            "Runes" => Ok(EconomyType::Runes),
            "Ritual" => Ok(EconomyType::Ritual),
            "Expedition" => Ok(EconomyType::Expedition),
            "Delirium" => Ok(EconomyType::Delirium),
            "Breach" => Ok(EconomyType::Breach),
            _ => Err(format!("Unknown economy type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyExchangeApiResponse {
    pub core: CurrencyCore,
    pub lines: Vec<CurrencyLine>,
    pub items: Vec<CurrencyItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyCore {
    pub items: Vec<CurrencyItem>,
    pub rates: HashMap<String, f64>,
    pub primary: String,
    pub secondary: String,
}

impl CurrencyCore {
    /// Get the tertiary currency (3rd highest value) based on exchange rates.
    /// Returns None if there are fewer than 3 currencies or no valid candidate with rates.
    ///
    /// Selection logic:
    /// 1. Primary currency has rate 1.0 (highest value)
    /// 2. Secondary currency is explicitly defined by API
    /// 3. Tertiary is the currency with the highest rate among remaining currencies
    ///    (highest rate = lowest value, so we want the smallest rate after primary/secondary)
    pub fn get_tertiary_currency(&self) -> Option<&CurrencyItem> {
        // Need at least 3 currencies for a tertiary
        if self.items.len() < 3 {
            return None;
        }

        // Find the currency with the smallest exchange rate among non-primary/non-secondary
        // (smaller rate = higher value currency, which is what we want for tertiary)
        let mut candidates: Vec<(&CurrencyItem, f64)> = self
            .items
            .iter()
            .filter(|item| item.id != self.primary && item.id != self.secondary)
            .filter_map(|item| {
                // Get exchange rate from rates map
                self.rates.get(&item.id).map(|rate| (item, *rate))
            })
            .collect();

        // Sort by rate ascending (smallest rate = highest value currency first)
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return the highest-value non-primary/non-secondary currency (smallest rate)
        candidates.first().map(|(item, _rate)| *item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyLine {
    pub id: String,
    #[serde(rename = "primaryValue")]
    pub primary_value: Option<f64>,
    #[serde(rename = "volumePrimaryValue")]
    pub volume_primary_value: Option<f64>,
    #[serde(rename = "maxVolumeCurrency")]
    pub max_volume_currency: Option<String>,
    #[serde(rename = "maxVolumeRate")]
    pub max_volume_rate: Option<f64>,
    pub sparkline: Sparkline,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sparkline {
    #[serde(rename = "totalChange")]
    pub total_change: Option<f64>,
    #[serde(default)]
    pub data: Vec<Option<f64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyItem {
    pub id: String,
    pub name: String,
    pub image: String,
    pub category: String,
    #[serde(rename = "detailsId")]
    pub details_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyExchangeData {
    pub primary_currency: CurrencyInfo,
    pub secondary_currency: CurrencyInfo,
    pub tertiary_currency: Option<CurrencyInfo>,
    pub secondary_rate: f64,
    pub tertiary_rate: Option<f64>,
    pub currencies: Vec<CurrencyExchangeRate>,
    pub fetched_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_stale: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub id: String,
    pub name: String,
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyExchangeRate {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub display_value: DisplayValue,
    pub primary_value: f64,
    pub secondary_value: f64,
    pub tertiary_value: f64,
    pub volume: Option<f64>,
    pub change_percent: Option<f64>,
    pub price_history: Vec<Option<f64>>,
}

/// Lightweight item for top currencies aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCurrencyItem {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub economy_type: EconomyType,
    pub primary_value: f64,
    pub primary_currency_name: String,
    pub primary_currency_image_url: String,
    pub volume: Option<f64>,
    pub change_percent: Option<f64>,
    pub cached_at: String,
}

/// Search result for cross-economy currency search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySearchResult {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub economy_type: EconomyType,
    pub primary_value: f64,
    pub primary_currency_name: String,
    pub primary_currency_image_url: String,
    pub secondary_value: f64,
    pub tertiary_value: f64,
    pub volume: Option<f64>,
    pub change_percent: Option<f64>,
    pub display_value: DisplayValue,
}

pub(crate) struct TierConfig {
    pub(crate) secondary_threshold_min: f64,
    pub(crate) primary_threshold_min: f64,
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            secondary_threshold_min: 0.003,
            primary_threshold_min: 0.05,
        }
    }
}

impl CurrencyExchangeRate {
    pub(crate) fn select_optimal_tier(
        primary_value: f64,
        secondary_rate: f64,
        _tertiary_rate: f64,
        config: &TierConfig,
    ) -> CurrencyTier {
        let secondary_value = primary_value * secondary_rate;

        if primary_value >= config.primary_threshold_min {
            CurrencyTier::Primary
        } else if secondary_value >= config.secondary_threshold_min {
            CurrencyTier::Secondary
        } else {
            CurrencyTier::Tertiary
        }
    }

    pub(crate) fn calculate_value_in_tier(
        primary_value: f64,
        tier: CurrencyTier,
        secondary_rate: f64,
        tertiary_rate: f64,
    ) -> f64 {
        match tier {
            CurrencyTier::Primary => primary_value,
            CurrencyTier::Secondary => primary_value * secondary_rate,
            CurrencyTier::Tertiary => primary_value * tertiary_rate,
        }
    }

    pub(crate) fn finalize_display_value(calculated_value: f64) -> (f64, bool) {
        if calculated_value < 1.0 && calculated_value > 0.0 {
            (1.0 / calculated_value, true)
        } else {
            (calculated_value, false)
        }
    }

    pub(crate) fn build_display_value(
        primary_value: f64,
        tier: CurrencyTier,
        secondary_rate: f64,
        tertiary_rate: f64,
        currency_info: &CurrencyInfo,
    ) -> DisplayValue {
        let calculated_value =
            Self::calculate_value_in_tier(primary_value, tier, secondary_rate, tertiary_rate);
        let (final_value, inverted) = Self::finalize_display_value(calculated_value);

        DisplayValue {
            tier,
            value: final_value,
            inverted,
            currency_id: currency_info.id.clone(),
            currency_name: currency_info.name.clone(),
            currency_image_url: currency_info.image_url.clone(),
        }
    }
}

impl CurrencyExchangeApiResponse {
    pub fn into_frontend_data(self) -> AppResult<CurrencyExchangeData> {
        // Check if items array is empty first
        if self.items.is_empty() && self.core.items.is_empty() {
            return Err(AppError::validation_error(
                "into_frontend_data",
                "No currency data available for this economy type",
            ));
        }

        if self.core.primary.is_empty() {
            return Err(AppError::validation_error(
                "into_frontend_data",
                "API response missing primary currency",
            ));
        }
        if self.core.secondary.is_empty() {
            return Err(AppError::validation_error(
                "into_frontend_data",
                "API response missing secondary currency",
            ));
        }

        let primary_currency = self
            .core
            .items
            .iter()
            .find(|item| item.id == self.core.primary)
            .ok_or_else(|| {
                AppError::validation_error(
                    "into_frontend_data",
                    &format!(
                        "No currency data available - '{}' currency not found",
                        self.core.primary
                    ),
                )
            })
            .map(|item| CurrencyInfo {
                id: item.id.clone(),
                name: item.name.clone(),
                image_url: format!("https://web.poecdn.com{}", item.image),
            })?;

        let secondary_currency = self
            .core
            .items
            .iter()
            .find(|item| item.id == self.core.secondary)
            .ok_or_else(|| {
                AppError::validation_error(
                    "into_frontend_data",
                    &format!(
                        "No currency data available - '{}' currency not found",
                        self.core.secondary
                    ),
                )
            })
            .map(|item| CurrencyInfo {
                id: item.id.clone(),
                name: item.name.clone(),
                image_url: format!("https://web.poecdn.com{}", item.image),
            })?;

        let tertiary_currency =
            self.core
                .get_tertiary_currency()
                .map(|tertiary_item| CurrencyInfo {
                    id: tertiary_item.id.clone(),
                    name: tertiary_item.name.clone(),
                    image_url: format!("https://web.poecdn.com{}", tertiary_item.image),
                });

        let secondary_rate = self
            .core
            .rates
            .get(&self.core.secondary)
            .copied()
            .ok_or_else(|| {
                AppError::serialization_error(
                    "into_frontend_data",
                    &format!(
                        "Missing exchange rate for secondary currency: {}",
                        self.core.secondary
                    ),
                )
            })?;

        let tertiary_rate = tertiary_currency
            .as_ref()
            .and_then(|tc| self.core.rates.get(&tc.id).copied());

        let config = TierConfig::default();

        // Build a lookup map for O(1) item lookups by id
        let items_by_id: HashMap<&str, &CurrencyItem> =
            self.items.iter().map(|item| (item.id.as_str(), item)).collect();

        let currencies: Vec<CurrencyExchangeRate> = self
            .lines
            .into_iter()
            .filter_map(|line| {
                let item = items_by_id.get(line.id.as_str())?;
                let primary_value = line.primary_value?;

                let secondary_value = primary_value * secondary_rate;
                let tertiary_value = tertiary_rate
                    .map(|rate| primary_value * rate)
                    .unwrap_or(0.0);

                let tier = CurrencyExchangeRate::select_optimal_tier(
                    primary_value,
                    secondary_rate,
                    tertiary_rate.unwrap_or(0.0),
                    &config,
                );

                // Special case: when the item itself is one of the core currencies,
                // we need to show its exchange with a different currency to avoid "1 primary <-> 1 primary"
                let (display_currency_info, display_tier) = if line.id == primary_currency.id {
                    // Primary currency should be exchanged with Secondary
                    (&secondary_currency, CurrencyTier::Secondary)
                } else if line.id == secondary_currency.id {
                    // Secondary currency should be exchanged with Primary
                    (&primary_currency, CurrencyTier::Primary)
                } else if tertiary_currency
                    .as_ref()
                    .is_some_and(|tc| line.id == tc.id)
                {
                    // Tertiary currency should be exchanged with Secondary
                    (&secondary_currency, CurrencyTier::Secondary)
                } else {
                    // Normal case: use the tier-based selection
                    // If tertiary currency is not available, default to secondary for tertiary tier
                    let currency_info = match tier {
                        CurrencyTier::Primary => &primary_currency,
                        CurrencyTier::Secondary => &secondary_currency,
                        CurrencyTier::Tertiary => {
                            tertiary_currency.as_ref().unwrap_or(&secondary_currency)
                        }
                    };
                    (currency_info, tier)
                };

                let display_value = CurrencyExchangeRate::build_display_value(
                    primary_value,
                    display_tier,
                    secondary_rate,
                    tertiary_rate.unwrap_or(0.0),
                    display_currency_info,
                );

                Some(CurrencyExchangeRate {
                    id: line.id.clone(),
                    name: item.name.clone(),
                    image_url: format!("https://web.poecdn.com{}", item.image),
                    display_value,
                    primary_value,
                    secondary_value,
                    tertiary_value,
                    volume: line.volume_primary_value,
                    change_percent: line.sparkline.total_change,
                    price_history: line.sparkline.data.clone(),
                })
            })
            .collect();

        Ok(CurrencyExchangeData {
            primary_currency,
            secondary_currency,
            tertiary_currency,
            secondary_rate,
            tertiary_rate,
            currencies,
            fetched_at: chrono::Utc::now().to_rfc3339(),
            is_stale: None,
        })
    }
}
