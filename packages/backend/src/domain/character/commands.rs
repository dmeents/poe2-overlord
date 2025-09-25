use crate::infrastructure::tauri::{to_command_result, CommandResult};
use crate::domain::character::models::{
    get_all_character_classes, get_all_leagues, get_ascendencies_for_class, Ascendency, Character,
    CharacterClass, CharacterUpdateParams, League,
};
use crate::domain::character::service::CharacterService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use log::{debug, info, warn};
use std::sync::Arc;
use tauri::State;

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

#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Vec<Character>> {
    debug!("Getting all characters");

    let characters = character_service.get_all_characters().await;
    debug!("Retrieved {} characters", characters.len());
    Ok(characters)
}

#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting character by ID: {}", character_id);

    let character = character_service.get_character(&character_id).await;
    Ok(character)
}

#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Arc<CharacterService>>,
) -> CommandResult<Option<Character>> {
    debug!("Getting active character");

    let character = character_service.get_active_character().await;
    Ok(character)
}

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

#[tauri::command]
pub async fn get_available_character_classes() -> CommandResult<Vec<CharacterClass>> {
    debug!("Getting available character classes");

    let classes = get_all_character_classes();
    debug!("Retrieved {} character classes", classes.len());
    Ok(classes)
}

#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<League>> {
    debug!("Getting available leagues");

    let leagues = get_all_leagues();
    debug!("Retrieved {} leagues", leagues.len());
    Ok(leagues)
}

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
