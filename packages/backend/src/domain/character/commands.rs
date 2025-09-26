use crate::domain::character::models::{
    get_all_character_classes, get_all_leagues, get_ascendencies_for_class, Ascendency, Character,
    CharacterClass, CharacterUpdateParams, League,
};
use crate::domain::character::service::CharacterService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::infrastructure::tauri::{to_command_result, CommandResult};
use log::{debug, info, warn};
use std::sync::Arc;
use tauri::State;

/// Tauri command handlers for character management operations.
///
/// This module provides the bridge between the frontend and the character domain
/// by exposing character service functionality as Tauri commands. Each command
/// handler is responsible for:
/// - Receiving parameters from the frontend
/// - Calling the appropriate service method
/// - Converting results to the expected Tauri format
/// - Handling and logging errors appropriately
///
/// The commands follow a consistent pattern of logging debug information for
/// incoming requests and info/warn messages for successful operations or errors.
/// Creates a new character with the specified parameters.
///
/// This command handler validates the input parameters and creates a new character
/// through the character service. The character will be automatically set as active
/// if it's the first character in the system.
///
/// # Arguments
/// * `name` - Unique character name
/// * `class` - Base character class
/// * `ascendency` - Specialized ascendency (must be valid for the class)
/// * `league` - Game league/mode
/// * `hardcore` - Whether character is in hardcore mode
/// * `solo_self_found` - Whether character is in SSF mode
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(Character)` - The newly created character
/// * `Err(CommandError)` - If validation fails or name is not unique
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

/// Retrieves all characters in the system.
///
/// This command handler returns a list of all characters, which is typically
/// used by the frontend to display the character list or selection interface.
///
/// # Arguments
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(Vec<Character>)` - All characters in the system
#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting all characters");

    let characters = character_service.get_all_characters().await;
    debug!("Retrieved {} characters", characters.len());
    Ok(characters)
}

/// Retrieves a specific character by ID.
///
/// This command handler is used to fetch detailed information about a specific
/// character, typically for editing or viewing character details.
///
/// # Arguments
/// * `character_id` - The unique identifier of the character
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(Some(Character))` - If the character exists
/// * `Ok(None)` - If no character with the given ID exists
#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting character by ID: {}", character_id);

    let character = character_service.get_character(&character_id).await;
    Ok(character)
}

/// Retrieves the currently active character.
///
/// This command handler returns the character that is currently set as active,
/// which is used by the frontend to display the current character's information
/// and for game monitoring purposes.
///
/// # Arguments
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(Some(Character))` - The active character if one exists
/// * `Ok(None)` - If no character is currently active
#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting active character");

    let character = character_service.get_active_character().await;
    Ok(character)
}

/// Sets a character as the active character.
///
/// This command handler changes which character is currently active. Only one
/// character can be active at a time, and setting a new active character will
/// deactivate the previously active character.
///
/// # Arguments
/// * `character_id` - The ID of the character to set as active
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(())` - If successful
/// * `Err(CommandError)` - If character not found or operation fails
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

/// Deletes a character and all associated data.
///
/// This command handler performs a complete cleanup by:
/// 1. Deleting the character from the character service
/// 2. Clearing all time tracking data associated with the character
///
/// The time tracking cleanup is performed as a best-effort operation - if it fails,
/// the character is still deleted but a warning is logged.
///
/// # Arguments
/// * `character_id` - The ID of the character to delete
/// * `character_service` - Injected character service instance
/// * `time_tracking_service` - Injected time tracking service instance
///
/// # Returns
/// * `Ok(Character)` - The deleted character
/// * `Err(CommandError)` - If character not found or operation fails
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

    // Attempt to clear time tracking data (best effort)
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

/// Retrieves all available character classes.
///
/// This command handler returns a list of all character classes that can be
/// selected when creating a new character. Used by the frontend to populate
/// class selection dropdowns.
///
/// # Returns
/// * `Ok(Vec<CharacterClass>)` - All available character classes
#[tauri::command]
pub async fn get_available_character_classes() -> CommandResult<Vec<CharacterClass>> {
    debug!("Getting available character classes");

    let classes = get_all_character_classes();
    debug!("Retrieved {} character classes", classes.len());
    Ok(classes)
}

/// Retrieves all available leagues.
///
/// This command handler returns a list of all leagues that can be selected
/// when creating a new character. Used by the frontend to populate league
/// selection dropdowns.
///
/// # Returns
/// * `Ok(Vec<League>)` - All available leagues
#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<League>> {
    debug!("Getting available leagues");

    let leagues = get_all_leagues();
    debug!("Retrieved {} leagues", leagues.len());
    Ok(leagues)
}

/// Retrieves all available ascendencies for a specific character class.
///
/// This command handler returns the ascendencies that are valid for the given
/// character class. Used by the frontend to dynamically populate ascendency
/// selection dropdowns based on the selected class.
///
/// # Arguments
/// * `class` - The character class to get ascendencies for
///
/// # Returns
/// * `Ok(Vec<Ascendency>)` - All ascendencies available for the specified class
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

/// Updates an existing character with new parameters.
///
/// This command handler validates the new parameters and updates the character
/// through the character service. The update includes validation of ascendency-class
/// combinations and name uniqueness.
///
/// # Arguments
/// * `character_id` - The ID of the character to update
/// * `params` - The new character parameters
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(Character)` - The updated character
/// * `Err(CommandError)` - If validation fails or character not found
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

/// Clears all character data from the system.
///
/// This command handler performs a destructive operation that removes all characters
/// and resets the active character state. Use with caution as this operation cannot be undone.
///
/// # Arguments
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(())` - If operation succeeds
/// * `Err(CommandError)` - If operation fails
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

/// Updates a character's level and last played timestamp.
///
/// This command handler is typically called when the game reports a level change
/// for the currently active character. It updates both the level and the last_played
/// timestamp to reflect recent activity.
///
/// # Arguments
/// * `character_id` - The ID of the character to update
/// * `new_level` - The new level
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(())` - If successful
/// * `Err(CommandError)` - If character not found or operation fails
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

/// Increments a character's death count and updates last played timestamp.
///
/// This command handler is typically called when the game reports a character death.
/// It increments the death count and updates the last_played timestamp to reflect
/// recent activity.
///
/// # Arguments
/// * `character_id` - The ID of the character to update
/// * `character_service` - Injected character service instance
///
/// # Returns
/// * `Ok(())` - If successful
/// * `Err(CommandError)` - If character not found or operation fails
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
