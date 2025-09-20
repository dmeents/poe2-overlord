use crate::commands::{to_command_result, CommandResult};
use crate::models::character::{
    get_all_character_classes, get_all_leagues, get_ascendencies_for_class, Character,
    CharacterClass, League,
};
use crate::services::character_manager::CharacterManager;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Create a new character
#[tauri::command]
pub async fn create_character(
    name: String,
    class: CharacterClass,
    ascendency: crate::models::Ascendency,
    league: League,
    hardcore: bool,
    solo_self_found: bool,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Character> {
    debug!("Creating new character: {}", name);

    let character = to_command_result(
        character_manager
            .create_character(
                name.clone(),
                class,
                ascendency,
                league,
                hardcore,
                solo_self_found,
            )
            .await,
    )?;

    info!("Successfully created character: {}", name);
    Ok(character)
}

/// Get all characters
#[tauri::command]
pub async fn get_all_characters(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting all characters");

    let characters = character_manager.get_all_characters().await;
    debug!("Retrieved {} characters", characters.len());
    Ok(characters)
}

/// Get character by ID
#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting character by ID: {}", character_id);

    let character = character_manager.get_character(&character_id).await;
    Ok(character)
}

/// Get the currently active character
#[tauri::command]
pub async fn get_active_character(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting active character");

    let character = character_manager.get_active_character().await;
    Ok(character)
}

/// Set the active character by ID
#[tauri::command]
pub async fn set_active_character(
    character_id: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<()> {
    debug!("Setting active character: {}", character_id);

    to_command_result(character_manager.set_active_character(&character_id).await)?;

    info!("Successfully set active character: {}", character_id);
    Ok(())
}

/// Remove a character by ID
#[tauri::command]
pub async fn remove_character(
    character_id: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Character> {
    debug!("Removing character: {}", character_id);

    let character = to_command_result(
        character_manager
            .remove_character(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error("Failed to remove character: {}", &e.to_string())
            }),
    )?;

    info!("Successfully removed character: {}", character_id);
    Ok(character)
}

/// Get characters sorted by last played (most recent first)
#[tauri::command]
pub async fn get_characters_by_last_played(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting characters by last played");

    let characters = character_manager.get_characters_by_last_played().await;
    debug!(
        "Retrieved {} characters sorted by last played",
        characters.len()
    );
    Ok(characters)
}

/// Get characters by class
#[tauri::command]
pub async fn get_characters_by_class(
    class: CharacterClass,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting characters by class: {:?}", class);

    let characters = character_manager.get_characters_by_class(&class).await;
    debug!(
        "Retrieved {} characters for class {:?}",
        characters.len(),
        class
    );
    Ok(characters)
}

/// Get characters by league
#[tauri::command]
pub async fn get_characters_by_league(
    league: League,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting characters by league: {:?}", league);

    let characters = character_manager.get_characters_by_league(&league).await;
    debug!(
        "Retrieved {} characters for league {:?}",
        characters.len(),
        league
    );
    Ok(characters)
}

/// Check if a character name is available
#[tauri::command]
pub async fn is_character_name_available(
    name: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<bool> {
    debug!("Checking if character name is available: {}", name);

    let is_available = character_manager.is_name_available(&name).await;
    Ok(is_available)
}

/// Get all available character classes
#[tauri::command]
pub async fn get_available_character_classes() -> CommandResult<Vec<CharacterClass>> {
    debug!("Getting available character classes");

    let classes = get_all_character_classes();
    debug!("Retrieved {} character classes", classes.len());
    Ok(classes)
}

/// Get all available leagues
#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<League>> {
    debug!("Getting available leagues");

    let leagues = get_all_leagues();
    debug!("Retrieved {} leagues", leagues.len());
    Ok(leagues)
}

/// Get available ascendencies for a given class
#[tauri::command]
pub async fn get_available_ascendencies_for_class(
    class: CharacterClass,
) -> CommandResult<Vec<crate::models::Ascendency>> {
    debug!("Getting available ascendencies for class: {:?}", class);

    let ascendencies = get_ascendencies_for_class(&class);
    debug!(
        "Retrieved {} ascendencies for class {:?}",
        ascendencies.len(),
        class
    );
    Ok(ascendencies)
}

/// Update a character's information
#[tauri::command]
pub async fn update_character(
    character_id: String,
    params: crate::models::CharacterUpdateParams,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<Character> {
    debug!("Updating character: {} (ID: {})", params.name, character_id);

    let character = to_command_result(
        character_manager
            .update_character(&character_id, params.clone())
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error("Failed to update character: {}", &e.to_string())
            }),
    )?;

    info!("Successfully updated character: {}", params.name);
    Ok(character)
}

/// Update a character's last played timestamp
#[tauri::command]
pub async fn update_character_last_played(
    character_id: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<()> {
    debug!("Updating last played for character: {}", character_id);

    to_command_result(
        character_manager
            .update_last_played(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error(
                    "update_character_last_played",
                    &e.to_string()
                )
            }),
    )?;

    debug!(
        "Successfully updated last played for character: {}",
        character_id
    );
    Ok(())
}

/// Get character count
#[tauri::command]
pub async fn get_character_count(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<usize> {
    debug!("Getting character count");

    let count = character_manager.get_character_count().await;
    debug!("Character count: {}", count);
    Ok(count)
}

/// Check if there are any characters
#[tauri::command]
pub async fn has_characters(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<bool> {
    debug!("Checking if there are any characters");

    let has_chars = character_manager.has_characters().await;
    debug!("Has characters: {}", has_chars);
    Ok(has_chars)
}

/// Clear all character data
#[tauri::command]
pub async fn clear_all_character_data(
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<()> {
    debug!("Clearing all character data");

    to_command_result(character_manager.clear_all_data().await.map_err(|e| {
        crate::errors::AppError::internal_error("Failed to clear character data: {}", &e.to_string())
    }))?;

    info!("Successfully cleared all character data");
    Ok(())
}


/// Update a character's level (system-managed, for testing purposes)
#[tauri::command]
pub async fn update_character_level(
    character_id: String,
    new_level: u32,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<()> {
    debug!("Updating character {} level to {}", character_id, new_level);

    to_command_result(
        character_manager
            .update_character_level(&character_id, new_level)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error("Failed to update character level: {}", &e.to_string())
            }),
    )?;

    info!("Successfully updated character {} level to {}", character_id, new_level);
    Ok(())
}

/// Increment a character's death count (system-managed, for testing purposes)
#[tauri::command]
pub async fn increment_character_deaths(
    character_id: String,
    character_manager: State<'_, Arc<CharacterManager>>,
) -> CommandResult<()> {
    debug!("Incrementing character {} death count", character_id);

    to_command_result(
        character_manager
            .increment_character_deaths(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error("Failed to increment character deaths: {}", &e.to_string())
            }),
    )?;

    info!("Successfully incremented character {} death count", character_id);
    Ok(())
}
