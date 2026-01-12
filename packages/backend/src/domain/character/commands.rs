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
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharacterData> {
    to_command_result(
        character_service
            .create_character(name, class, ascendency, league, hardcore, solo_self_found)
            .await,
    )
}

#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<CharacterDataResponse> {
    to_command_result(character_service.get_character(&character_id).await)
}

#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<CharacterDataResponse>> {
    to_command_result(character_service.get_all_characters().await)
}

#[tauri::command]
pub async fn update_character(
    character_id: String,
    update_params: crate::domain::character::models::CharacterUpdateParams,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharacterData> {
    to_command_result(
        character_service
            .update_character(&character_id, update_params)
            .await,
    )
}

#[tauri::command]
pub async fn delete_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.delete_character(&character_id).await)
}

#[tauri::command]
pub async fn set_active_character(
    character_id: Option<String>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .set_active_character(character_id.as_deref())
            .await,
    )
}

#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<CharacterDataResponse>> {
    to_command_result(character_service.get_active_character().await)
}

#[tauri::command]
pub async fn reconcile_character_storage(
    strategy: crate::domain::character::models::CleanupStrategy,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::OrphanCleanupReport> {
    to_command_result(
        character_service
            .reconcile_character_storage(strategy)
            .await,
    )
}
