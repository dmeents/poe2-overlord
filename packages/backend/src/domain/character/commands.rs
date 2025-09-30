// Arc is no longer needed since we removed the character_tracking_service parameter
use tauri::State;

use crate::domain::character::traits::CharacterService;
use crate::infrastructure::tauri::{to_command_result, CommandResult};

/// Tauri command to create a new character (new domain)
///
/// This command creates a new character with the provided metadata and tracking data.
/// The character will be saved to its own character_data_{id}.json file and added to
/// the characters index.
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

/// Tauri command to get a character by ID
///
/// This command retrieves a character's complete data including metadata and tracking information.
#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharacterData> {
    to_command_result(character_service.get_character(&character_id).await)
}

/// Tauri command to get all characters
///
/// This command retrieves all characters with their complete data.
#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<crate::domain::character::models::CharacterData>> {
    to_command_result(character_service.get_all_characters().await)
}

/// Tauri command to update a character
///
/// This command updates an existing character's metadata and tracking data.
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

/// Tauri command to delete a character
///
/// This command deletes a character and removes it from the characters index.
#[tauri::command]
pub async fn delete_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.delete_character(&character_id).await)
}

/// Tauri command to set the active character
///
/// This command sets which character is currently active in the application.
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

/// Tauri command to get the active character
///
/// This command retrieves the currently active character's data.
#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::CharacterData>> {
    to_command_result(character_service.get_active_character().await)
}

/// Tauri command to get all available character classes
///
/// This command returns all character classes that can be selected when creating a character.
#[tauri::command]
pub async fn get_available_character_classes(
) -> CommandResult<Vec<crate::domain::character::models::CharacterClass>> {
    Ok(crate::domain::character::models::get_all_character_classes())
}

/// Tauri command to get all available leagues
///
/// This command returns all leagues that can be selected when creating a character.
#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<crate::domain::character::models::League>>
{
    Ok(crate::domain::character::models::get_all_leagues())
}

/// Tauri command to get available ascendencies for a character class
///
/// This command returns all ascendencies that are valid for the specified character class.
#[tauri::command]
pub async fn get_available_ascendencies_for_class(
    class: crate::domain::character::models::CharacterClass,
) -> CommandResult<Vec<crate::domain::character::models::Ascendency>> {
    Ok(crate::domain::character::models::get_ascendencies_for_class(&class))
}
