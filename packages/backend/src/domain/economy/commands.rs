use super::models::{CurrencyExchangeData, EconomyType, TopCurrencyItem};
use super::service::EconomyService;
use crate::to_command_result;
use crate::CommandResult;
use tauri::State;

#[tauri::command]
pub async fn get_currency_exchange_data(
    league: String,
    economy_type: EconomyType,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<CurrencyExchangeData> {
    log::info!(
        "Command: get_currency_exchange_data for league: {}, type: {}",
        league,
        economy_type
    );

    let result = economy_service
        .fetch_currency_exchange_data(&league, economy_type)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn get_aggregated_top_currencies(
    league: String,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<Vec<TopCurrencyItem>> {
    log::info!(
        "Command: get_aggregated_top_currencies for league: {}",
        league
    );

    let result = economy_service
        .load_aggregated_top_currencies(&league)
        .await;

    to_command_result(result)
}
