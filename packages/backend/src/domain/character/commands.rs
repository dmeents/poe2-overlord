use crate::commands::{to_command_result, CommandResult};
use crate::domain::character::models::{
    get_all_character_classes, get_all_leagues, get_ascendencies_for_class, Ascendency, Character,
    CharacterClass, CharacterUpdateParams, League,
};
use crate::domain::character::service::CharacterService;
use crate::services::traits::TimeTrackingService;
use log::{debug, info, warn};
use std::sync::Arc;
use tauri::State;

/// Create a new character
#[tauri::command]
pub async fn create_character(
    name: String,
    class: CharacterClass,
    ascendency: Ascendency,
    league: League,
    hardcore: bool,
    solo_self_found: bool,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Character> {
    debug!("Creating new character: {}", name);

    let character = to_command_result(
        character_service
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
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting all characters");

    let characters = character_service.get_all_characters().await;
    debug!("Retrieved {} characters", characters.len());
    Ok(characters)
}

/// Get character by ID
#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting character by ID: {}", character_id);

    let character = character_service.get_character(&character_id).await;
    Ok(character)
}

/// Get the currently active character
#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting active character");

    let character = character_service.get_active_character().await;
    Ok(character)
}

/// Set the active character by ID
#[tauri::command]
pub async fn set_active_character(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<()> {
    debug!("Setting active character: {}", character_id);

    to_command_result(character_service.set_active_character(&character_id).await)?;

    info!("Successfully set active character: {}", character_id);
    Ok(())
}

/// Delete a character by ID and all associated data
#[tauri::command]
pub async fn delete_character(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<Character> {
    debug!(
        "Deleting character and all associated data: {}",
        character_id
    );

    // First, delete the character from the character service
    let character = to_command_result(
        character_service
            .delete_character(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error(
                    "Failed to delete character: {}",
                    &e.to_string(),
                )
            }),
    )?;

    // Then, clear all time tracking data for this character
    match time_tracking_service
        .clear_character_data(&character_id)
        .await
    {
        Ok(_) => {
            debug!(
                "Successfully cleared time tracking data for character: {}",
                character_id
            );
        }
        Err(e) => {
            // Log the error but don't fail the entire operation
            // The character has already been deleted, so we can't rollback
            warn!(
                "Failed to clear time tracking data for character {}: {}. Character was still deleted.",
                character_id, e
            );
        }
    }

    info!(
        "Successfully deleted character and associated data: {}",
        character_id
    );
    Ok(character)
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
) -> CommandResult<Vec<Ascendency>> {
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
    params: CharacterUpdateParams,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Character> {
    debug!("Updating character: {} (ID: {})", params.name, character_id);

    let character = to_command_result(
        character_service
            .update_character(&character_id, params.clone())
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error(
                    "Failed to update character: {}",
                    &e.to_string(),
                )
            }),
    )?;

    info!("Successfully updated character: {}", params.name);
    Ok(character)
}

/// Clear all character data
#[tauri::command]
pub async fn clear_all_character_data(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<()> {
    debug!("Clearing all character data");

    to_command_result(character_service.clear_all_data().await.map_err(|e| {
        crate::errors::AppError::internal_error(
            "Failed to clear character data: {}",
            &e.to_string(),
        )
    }))?;

    info!("Successfully cleared all character data");
    Ok(())
}

/// Update a character's level (system-managed, for testing purposes)
#[tauri::command]
pub async fn update_character_level(
    character_id: String,
    new_level: u32,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<()> {
    debug!("Updating character {} level to {}", character_id, new_level);

    to_command_result(
        character_service
            .update_character_level(&character_id, new_level)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error(
                    "Failed to update character level: {}",
                    &e.to_string(),
                )
            }),
    )?;

    info!(
        "Successfully updated character {} level to {}",
        character_id, new_level
    );
    Ok(())
}

/// Increment a character's death count (system-managed, for testing purposes)
#[tauri::command]
pub async fn increment_character_deaths(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<()> {
    debug!("Incrementing character {} death count", character_id);

    to_command_result(
        character_service
            .increment_character_deaths(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::internal_error(
                    "Failed to increment character deaths: {}",
                    &e.to_string(),
                )
            }),
    )?;

    info!(
        "Successfully incremented character {} death count",
        character_id
    );
    Ok(())
}
