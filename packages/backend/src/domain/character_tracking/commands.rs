use crate::domain::character_tracking::{
    models::{CharacterTrackingData, ZoneStats},
    traits::CharacterTrackingService,
};
use crate::infrastructure::tauri::{to_command_result, CommandResult};
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Tauri command to get complete character tracking data for a character
#[tauri::command]
pub async fn get_character_tracking_data(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<CharacterTrackingData>> {
    debug!("Getting character tracking data for character: {}", character_id);

    let data = to_command_result(
        character_tracking_service
            .get_character_data(&character_id)
            .await,
    )?;

    debug!("Retrieved character tracking data for character");
    Ok(data)
}

/// Tauri command to clear all character tracking data for a character
#[tauri::command]
pub async fn clear_character_tracking_data(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<()> {
    debug!(
        "Clearing all character tracking data for character: {}",
        character_id
    );

    to_command_result(
        character_tracking_service
            .clear_character_data(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::time_tracking_error("clear_character_data", &e.to_string())
            }),
    )?;

    info!(
        "Successfully cleared all character tracking data for character {}",
        character_id
    );
    Ok(())
}

/// Tauri command to get the current active zone for a character
#[tauri::command]
pub async fn get_character_active_zone(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<ZoneStats>> {
    debug!("Getting active zone for character: {}", character_id);

    let zone = to_command_result(character_tracking_service.get_active_zone(&character_id).await)?;

    debug!(
        "Retrieved active zone for character {}: {}",
        character_id,
        zone.as_ref()
            .map(|z| z.location_name.as_str())
            .unwrap_or("None")
    );
    Ok(zone)
}

/// Tauri command to get all zones for a character
#[tauri::command]
pub async fn get_character_all_zones(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Vec<ZoneStats>> {
    debug!("Getting all zones for character: {}", character_id);

    let zones = to_command_result(character_tracking_service.get_all_zones(&character_id).await)?;

    debug!(
        "Retrieved {} zones for character {}",
        zones.len(),
        character_id
    );
    Ok(zones)
}

/// Tauri command to get zones sorted by time spent for a character
#[tauri::command]
pub async fn get_character_zones_by_time(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Vec<ZoneStats>> {
    debug!("Getting zones by time for character: {}", character_id);

    let zones = to_command_result(character_tracking_service.get_zones_by_time(&character_id).await)?;

    debug!(
        "Retrieved {} zones sorted by time for character {}",
        zones.len(),
        character_id
    );
    Ok(zones)
}

/// Tauri command to get total play time for a character
#[tauri::command]
pub async fn get_character_total_play_time(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<u64> {
    debug!("Getting total play time for character: {}", character_id);

    let total_time = to_command_result(
        character_tracking_service
            .get_total_play_time(&character_id)
            .await,
    )?;

    debug!(
        "Total play time for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}

/// Tauri command to get total hideout time for a character
#[tauri::command]
pub async fn get_character_total_hideout_time(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<u64> {
    debug!("Getting total hideout time for character: {}", character_id);

    let total_time = to_command_result(
        character_tracking_service
            .get_total_hideout_time(&character_id)
            .await,
    )?;

    debug!(
        "Total hideout time for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}

/// Tauri command to get total deaths for a character
#[tauri::command]
pub async fn get_character_total_deaths(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<u32> {
    debug!("Getting total deaths for character: {}", character_id);

    let total_deaths =
        to_command_result(character_tracking_service.get_total_deaths(&character_id).await)?;

    debug!(
        "Total deaths for character {}: {}",
        character_id, total_deaths
    );
    Ok(total_deaths)
}

/// Tauri command to get current location for a character
#[tauri::command]
pub async fn get_character_current_location(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<crate::domain::character_tracking::models::LocationState>> {
    debug!("Getting current location for character: {}", character_id);

    let location = to_command_result(character_tracking_service.get_current_location(&character_id).await)?;

    debug!(
        "Retrieved current location for character {}: {}",
        character_id,
        location.as_ref()
            .and_then(|l| l.get_current_scene())
            .map(|s| s.as_str())
            .unwrap_or("None")
    );
    Ok(location)
}

/// Tauri command to get current scene for a character
#[tauri::command]
pub async fn get_character_current_scene(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<String>> {
    debug!("Getting current scene for character: {}", character_id);

    let scene = to_command_result(character_tracking_service.get_current_scene(&character_id).await)?;

    debug!(
        "Retrieved current scene for character {}: {}",
        character_id,
        scene.as_deref().unwrap_or("None")
    );
    Ok(scene)
}

/// Tauri command to get current act for a character
#[tauri::command]
pub async fn get_character_current_act(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<String>> {
    debug!("Getting current act for character: {}", character_id);

    let act = to_command_result(character_tracking_service.get_current_act(&character_id).await)?;

    debug!(
        "Retrieved current act for character {}: {}",
        character_id,
        act.as_deref().unwrap_or("None")
    );
    Ok(act)
}

/// Tauri command to reset character tracking for a character
#[tauri::command]
pub async fn reset_character_tracking(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<()> {
    debug!("Resetting character tracking for character: {}", character_id);

    to_command_result(
        character_tracking_service
            .reset_tracking(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::time_tracking_error("reset_tracking", &e.to_string())
            }),
    )?;

    info!(
        "Successfully reset character tracking for character {}",
        character_id
    );
    Ok(())
}
