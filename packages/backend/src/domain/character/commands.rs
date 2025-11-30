use tauri::State;

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
) -> CommandResult<crate::domain::character::models::CharacterDataResponse> {
    to_command_result(
        character_service
            .get_character_response(&character_id)
            .await,
    )
}

#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<crate::domain::character::models::CharacterDataResponse>> {
    to_command_result(character_service.get_all_characters_response().await)
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
) -> CommandResult<Option<crate::domain::character::models::CharacterDataResponse>> {
    match to_command_result(character_service.get_active_character().await)? {
        Some(character) => {
            let response = to_command_result(
                character_service
                    .get_character_response(&character.id)
                    .await,
            )?;
            Ok(Some(response))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn get_characters_index(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharactersIndex> {
    to_command_result(character_service.get_characters_index().await)
}

#[tauri::command]
pub async fn is_character_name_unique(
    name: String,
    exclude_id: Option<String>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<bool> {
    to_command_result(
        character_service
            .is_name_unique(&name, exclude_id.as_deref())
            .await,
    )
}

#[tauri::command]
pub async fn get_available_character_classes(
) -> CommandResult<Vec<crate::domain::character::models::CharacterClass>> {
    Ok(crate::domain::character::models::get_all_character_classes())
}

#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<crate::domain::character::models::League>>
{
    Ok(crate::domain::character::models::get_all_leagues())
}

#[tauri::command]
pub async fn get_available_ascendencies_for_class(
    class: crate::domain::character::models::CharacterClass,
) -> CommandResult<Vec<crate::domain::character::models::Ascendency>> {
    Ok(crate::domain::character::models::get_ascendencies_for_class(&class))
}

#[tauri::command]
pub async fn get_character_tracking_data(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::CharacterData>> {
    let result = to_command_result(character_service.get_character(&character_id).await)?;
    Ok(Some(result))
}

#[tauri::command]
pub async fn get_character_current_location(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::LocationState>> {
    to_command_result(character_service.get_current_location(&character_id).await)
}

#[tauri::command]
pub async fn enter_zone(
    character_id: String,
    location_id: String,
    location_name: String,
    location_type: crate::domain::character::models::LocationType,
    act: Option<String>,
    is_town: bool,
    zone_level: Option<u32>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .enter_zone(
                &character_id,
                location_id,
                location_name,
                location_type,
                act,
                is_town,
                zone_level,
            )
            .await,
    )
}

#[tauri::command]
pub async fn leave_zone(
    character_id: String,
    location_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .leave_zone(&character_id, &location_id)
            .await,
    )
}

#[tauri::command]
pub async fn record_death(
    character_id: String,
    location_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .record_death(&character_id, &location_id)
            .await,
    )
}

#[tauri::command]
pub async fn add_zone_time(
    character_id: String,
    location_id: String,
    seconds: u64,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .add_zone_time(&character_id, &location_id, seconds)
            .await,
    )
}

#[tauri::command]
pub async fn finalize_all_active_zones(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.finalize_all_active_zones().await)
}
