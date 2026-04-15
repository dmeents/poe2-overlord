use std::sync::Arc;
use tauri::State;

use crate::domain::character::models::{
    CharacterDataResponse, CharacterSummaryResponse, EnrichedZoneStats,
};
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
        "create_character command called: name={name}, class={class:?}, ascendency={ascendency:?}, league={league:?}"
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
        Ok(char) => log::info!("create_character success: id={}", char.id),
        Err(e) => log::error!("create_character error: {e:?}"),
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
    log::info!("get_all_characters command called");
    let result = character_service.get_all_characters().await;
    match &result {
        Ok(chars) => log::info!("get_all_characters returning {} characters", chars.len()),
        Err(e) => log::error!("get_all_characters error: {e:?}"),
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
pub async fn get_all_characters_summary(
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<CharacterSummaryResponse>> {
    to_command_result(character_service.get_all_characters_summary().await)
}

#[tauri::command]
pub async fn get_character_zones(
    character_id: String,
    character_service: State<'_, Arc<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<EnrichedZoneStats>> {
    to_command_result(character_service.get_character_zones(&character_id).await)
}
