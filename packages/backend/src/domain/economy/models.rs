//! Economy domain models for currency exchange data from poe.ninja API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CurrencyTier {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Debug, Clone, Serialize)]
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
    Ultimatum,
    Talismans,
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
            EconomyType::Ultimatum => "Ultimatum",
            EconomyType::Talismans => "Talismans",
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
            EconomyType::Ultimatum,
            EconomyType::Talismans,
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
    pub fn get_tertiary_currency(&self) -> Option<&CurrencyItem> {
        self.items
            .iter()
            .find(|item| item.id != self.primary && item.id != self.secondary)
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

#[derive(Debug, Clone, Serialize)]
pub struct CurrencyExchangeData {
    pub primary_currency: CurrencyInfo,
    pub secondary_currency: CurrencyInfo,
    pub tertiary_currency: CurrencyInfo,
    pub secondary_rate: f64,
    pub tertiary_rate: f64,
    pub currencies: Vec<CurrencyExchangeRate>,
    pub fetched_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrencyInfo {
    pub id: String,
    pub name: String,
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrencyExchangeRate {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub display_value: DisplayValue,
    pub raw_divine_value: Option<f64>,
    pub raw_chaos_value: Option<f64>,
    pub raw_exalted_value: Option<f64>,
    pub volume: Option<f64>,
    pub change_percent: Option<f64>,
    pub price_history: Vec<Option<f64>>,
}

pub(crate) struct TierConfig {
    pub(crate) secondary_threshold_min: f64,
    pub(crate) primary_threshold_min: f64,
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            secondary_threshold_min: 0.05,
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

    fn build_display_value(
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
    pub fn into_frontend_data(self) -> Result<CurrencyExchangeData, String> {
        if self.core.primary.is_empty() {
            return Err("API response missing primary currency".to_string());
        }
        if self.core.secondary.is_empty() {
            return Err("API response missing secondary currency".to_string());
        }

        let primary_currency = self
            .core
            .items
            .iter()
            .find(|item| item.id == self.core.primary)
            .ok_or_else(|| {
                format!(
                    "Primary currency '{}' not found in items",
                    self.core.primary
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
                format!(
                    "Secondary currency '{}' not found in items",
                    self.core.secondary
                )
            })
            .map(|item| CurrencyInfo {
                id: item.id.clone(),
                name: item.name.clone(),
                image_url: format!("https://web.poecdn.com{}", item.image),
            })?;

        let tertiary_item = self
            .core
            .get_tertiary_currency()
            .ok_or_else(|| "No tertiary currency found in core items".to_string())?;

        let tertiary_currency = CurrencyInfo {
            id: tertiary_item.id.clone(),
            name: tertiary_item.name.clone(),
            image_url: format!("https://web.poecdn.com{}", tertiary_item.image),
        };

        let secondary_rate = self
            .core
            .rates
            .get(&self.core.secondary)
            .copied()
            .ok_or_else(|| {
                format!(
                    "Missing exchange rate for secondary currency: {}",
                    self.core.secondary
                )
            })?;

        let tertiary_rate = self
            .core
            .rates
            .get(&tertiary_currency.id)
            .copied()
            .ok_or_else(|| {
                format!(
                    "Missing exchange rate for tertiary currency: {}",
                    tertiary_currency.id
                )
            })?;

        let config = TierConfig::default();

        let currencies: Vec<CurrencyExchangeRate> = self
            .lines
            .into_iter()
            .filter_map(|line| {
                let item = self.items.iter().find(|item| item.id == line.id)?;
                let primary_value = line.primary_value?;

                let raw_divine_value = Some(primary_value);
                let raw_chaos_value = Some(primary_value * secondary_rate);
                let raw_exalted_value = Some(primary_value * tertiary_rate);

                let tier = CurrencyExchangeRate::select_optimal_tier(
                    primary_value,
                    secondary_rate,
                    tertiary_rate,
                    &config,
                );

                let display_currency_info = match tier {
                    CurrencyTier::Primary => &primary_currency,
                    CurrencyTier::Secondary => &secondary_currency,
                    CurrencyTier::Tertiary => &tertiary_currency,
                };

                let display_value = CurrencyExchangeRate::build_display_value(
                    primary_value,
                    tier,
                    secondary_rate,
                    tertiary_rate,
                    display_currency_info,
                );

                Some(CurrencyExchangeRate {
                    id: line.id.clone(),
                    name: item.name.clone(),
                    image_url: format!("https://web.poecdn.com{}", item.image),
                    display_value,
                    raw_divine_value,
                    raw_chaos_value,
                    raw_exalted_value,
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
        })
    }
}
