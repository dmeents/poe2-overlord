use std::sync::Arc;
use tauri::State;

use crate::{to_command_result, CommandResult};

use super::models::{GameDataVersion, Item, ItemCategory, ItemSearchParams, ItemSearchResult};
use super::traits::ItemDataService;

#[tauri::command]
pub async fn get_item(
    id: String,
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<Option<Item>> {
    to_command_result(item_data_service.get_item(&id).await)
}

#[tauri::command]
pub async fn search_items(
    params: ItemSearchParams,
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<ItemSearchResult> {
    to_command_result(item_data_service.search_items(params).await)
}

#[tauri::command]
pub async fn get_item_categories(
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<Vec<ItemCategory>> {
    to_command_result(item_data_service.get_categories().await)
}

#[tauri::command]
pub async fn get_game_data_version(
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<Option<GameDataVersion>> {
    to_command_result(item_data_service.get_version().await)
}

#[tauri::command]
pub async fn toggle_item_favorite(
    item_id: String,
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<bool> {
    to_command_result(item_data_service.toggle_favorite(&item_id).await)
}

#[tauri::command]
pub async fn get_favorite_items(
    item_data_service: State<'_, Arc<dyn ItemDataService>>,
) -> CommandResult<Vec<Item>> {
    to_command_result(item_data_service.get_favorites().await)
}
