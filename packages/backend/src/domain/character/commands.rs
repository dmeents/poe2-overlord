// Arc is no longer needed since we removed the character_tracking_service parameter
use tauri::State;

use crate::domain::character::traits::CharacterService;
use crate::{to_command_result, CommandResult};

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
) -> CommandResult<crate::domain::character::models::CharacterDataResponse> {
    to_command_result(
        character_service
            .get_character_response(&character_id)
            .await,
    )
}

/// Tauri command to get all characters
///
/// This command retrieves all characters with their complete data.
#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Vec<crate::domain::character::models::CharacterDataResponse>> {
    to_command_result(character_service.get_all_characters_response().await)
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

/// Tauri command to get the characters index
///
/// This command retrieves the characters index containing all character IDs and active character.
#[tauri::command]
pub async fn get_characters_index(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharactersIndex> {
    to_command_result(character_service.get_characters_index().await)
}

/// Tauri command to check if a character name is unique
///
/// This command validates that a character name is not already in use.
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

// Character Tracking Commands (merged from character_tracking domain)

/// Tauri command to get complete character tracking data for a character
///
/// This command retrieves all tracking data including zones, location, and statistics.
#[tauri::command]
pub async fn get_character_tracking_data(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::CharacterData>> {
    let result = to_command_result(character_service.get_character(&character_id).await)?;
    Ok(Some(result))
}

/// Tauri command to get current location for a character
///
/// This command retrieves the current location state of a character.
#[tauri::command]
pub async fn get_character_current_location(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::LocationState>> {
    to_command_result(character_service.get_current_location(&character_id).await)
}

/// Tauri command to enter a zone
///
/// This command handles a character entering a zone with time tracking.
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

/// Tauri command to leave a zone
///
/// This command handles a character leaving a zone with time calculation.
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

/// Tauri command to record a death in a zone
///
/// This command records a character death in a specific zone.
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

/// Tauri command to add time to a zone
///
/// This command adds time to a specific zone for a character.
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

/// Tauri command to finalize all active zones
///
/// This command stops all active timers and saves the data.
#[tauri::command]
pub async fn finalize_all_active_zones(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.finalize_all_active_zones().await)
}
