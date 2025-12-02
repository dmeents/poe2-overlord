use super::models::{CurrencyExchangeApiResponse, CurrencyExchangeData, EconomyType};
use crate::errors::{AppError, AppResult};
use reqwest;

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

        api_response.into_frontend_data().map_err(|e| {
            log::error!("Failed to convert API response to frontend data: {}", e);
            AppError::Serialization {
                message: format!("Failed to process currency data: {}", e),
            }
        })
    }
}

impl Default for EconomyService {
    fn default() -> Self {
        Self::new()
    }
}
