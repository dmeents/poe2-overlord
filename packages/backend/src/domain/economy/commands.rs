use super::models::{CurrencyExchangeData, EconomyType};
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
