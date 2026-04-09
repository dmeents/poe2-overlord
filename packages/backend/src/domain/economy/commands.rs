use super::models::{CurrencyExchangeData, CurrencySearchResult, EconomyType};
use super::service::EconomyService;
use crate::to_command_result;
use crate::CommandResult;
use tauri::State;

#[tauri::command]
pub async fn get_currency_exchange_data(
    league: String,
    is_hardcore: bool,
    economy_type: EconomyType,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<CurrencyExchangeData> {
    log::info!(
        "Command: get_currency_exchange_data for league: {}, hardcore: {}, type: {}",
        league,
        is_hardcore,
        economy_type
    );

    let result = economy_service
        .fetch_currency_exchange_data(&league, is_hardcore, economy_type)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn refresh_all_economy_data(
    league: String,
    is_hardcore: bool,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<()> {
    log::info!(
        "Command: refresh_all_economy_data for league: {}, hardcore: {}",
        league,
        is_hardcore
    );

    let result = economy_service
        .refresh_all_economy_types(&league, is_hardcore)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn get_all_currencies(
    league: String,
    is_hardcore: bool,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<Vec<CurrencySearchResult>> {
    log::info!(
        "Command: get_all_currencies for league: {}, hardcore: {}",
        league,
        is_hardcore
    );

    let result = economy_service
        .load_all_currencies(&league, is_hardcore)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn search_currencies(
    league: String,
    is_hardcore: bool,
    query: String,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<Vec<CurrencySearchResult>> {
    log::info!(
        "Command: search_currencies for league: {}, hardcore: {}, query: '{}'",
        league,
        is_hardcore,
        query
    );

    let result = economy_service
        .search_currencies(&league, is_hardcore, &query)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn toggle_currency_star(
    league: String,
    is_hardcore: bool,
    economy_type: EconomyType,
    currency_id: String,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<bool> {
    log::info!(
        "Command: toggle_currency_star for league: {}, type: {}, currency: {}",
        league,
        economy_type,
        currency_id
    );

    let result = economy_service
        .toggle_currency_star(&league, is_hardcore, economy_type, &currency_id)
        .await;

    to_command_result(result)
}

#[tauri::command]
pub async fn get_starred_currencies(
    league: String,
    is_hardcore: bool,
    economy_service: State<'_, EconomyService>,
) -> CommandResult<Vec<CurrencySearchResult>> {
    log::info!(
        "Command: get_starred_currencies for league: {}, hardcore: {}",
        league,
        is_hardcore
    );

    let result = economy_service
        .get_starred_currencies(&league, is_hardcore)
        .await;

    to_command_result(result)
}
