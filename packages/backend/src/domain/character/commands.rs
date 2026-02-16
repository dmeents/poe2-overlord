use std::sync::Arc;
use tauri::State;

use crate::domain::character::models::CharacterDataResponse;
use crate::domain::character::traits::CharacterService;
use crate::{to_command_result, CommandResult};

#[tauri::command]
pub async fn create_character(
    name: String,
    class: crate::domain::character::models::CharacterClass,
    ascendency: crate::domain::character::models::Ascendency,
    league: crate::domain::character::models::League,
    hardcore: bool,
    solo_self_found: bool,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<CharacterDataResponse> {
    log::info!(
        "[DEBUG] create_character command called: name={}, class={:?}, ascendency={:?}, league={:?}",
        name, class, ascendency, league
    );
    let result = character_service
        .create_character(
            name.clone(),
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
        )
        .await;
    match &result {
        Ok(char) => log::info!("[DEBUG] create_character success: id={}", char.id),
        Err(e) => log::error!("[DEBUG] create_character error: {:?}", e),
    }
    to_command_result(result)
}

#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<CharacterDataResponse> {
    to_command_result(character_service.get_character(&character_id).await)
}

#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<CharacterDataResponse>> {
    log::info!("[DEBUG] get_all_characters command called");
    let result = character_service.get_all_characters().await;
    match &result {
        Ok(chars) => log::info!(
            "[DEBUG] get_all_characters returning {} characters",
            chars.len()
        ),
        Err(e) => log::error!("[DEBUG] get_all_characters error: {:?}", e),
    }
    to_command_result(result)
}

#[tauri::command]
pub async fn update_character(
    character_id: String,
    update_params: crate::domain::character::models::CharacterUpdateParams,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<CharacterDataResponse> {
    to_command_result(
        character_service
            .update_character(&character_id, update_params)
            .await,
    )
}

#[tauri::command]
pub async fn delete_character(
    character_id: String,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.delete_character(&character_id).await)
}

#[tauri::command]
pub async fn set_active_character(
    character_id: Option<String>,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .set_active_character(character_id.as_deref())
            .await,
    )
}

#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<CharacterDataResponse>> {
    to_command_result(character_service.get_active_character().await)
}

#[tauri::command]
pub async fn reconcile_character_storage(
    strategy: crate::domain::character::models::CleanupStrategy,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::OrphanCleanupReport> {
    to_command_result(
        character_service
            .reconcile_character_storage(strategy)
            .await,
    )
}
