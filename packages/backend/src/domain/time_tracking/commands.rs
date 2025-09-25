use crate::commands::{to_command_result, CommandResult};
use crate::domain::time_tracking::{
    models::{LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingSummary},
    traits::TimeTrackingService,
};
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get all time tracking data for a specific character
#[tauri::command]
pub async fn get_character_time_tracking_data(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<TimeTrackingData> {
    debug!("Getting time tracking data for character: {}", character_id);

    let active_sessions = time_tracking_service
        .get_active_sessions(&character_id)
        .await;
    let completed_sessions = time_tracking_service
        .get_completed_sessions(&character_id)
        .await;
    let all_location_stats = time_tracking_service.get_all_stats(&character_id).await;

    // Create summary with top locations
    let zone_stats: Vec<LocationStats> = all_location_stats
        .iter()
        .filter(|stat| stat.location_type == LocationType::Zone)
        .cloned()
        .collect();

    let mut sorted_stats = zone_stats;
    sorted_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
    let top_stats = sorted_stats.into_iter().take(10).collect::<Vec<_>>();

    let total_play_time = time_tracking_service
        .get_total_play_time(&character_id)
        .await;
    let total_play_time_since_process_start = time_tracking_service
        .get_total_play_time_since_process_start(&character_id)
        .await;
    let total_hideout_time = time_tracking_service
        .get_total_hideout_time(&character_id)
        .await;

    let summary = TimeTrackingSummary {
        character_id: character_id.clone(),
        active_sessions: active_sessions.clone(),
        top_locations: top_stats,
        total_locations_tracked: all_location_stats.len(),
        total_active_sessions: active_sessions.len(),
        total_play_time_seconds: total_play_time,
        total_play_time_since_process_start_seconds: total_play_time_since_process_start,
        total_hideout_time_seconds: total_hideout_time,
    };

    let data = TimeTrackingData {
        character_id,
        active_sessions,
        completed_sessions,
        all_location_stats,
        summary,
    };

    debug!("Retrieved time tracking data for character");
    Ok(data)
}

/// Clear all time tracking data for a specific character
#[tauri::command]
pub async fn clear_character_time_tracking_data(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<()> {
    debug!(
        "Clearing all time tracking data for character: {}",
        character_id
    );

    to_command_result(
        time_tracking_service
            .clear_character_data(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::time_tracking_error("clear_character_data", &e.to_string())
            }),
    )?;

    info!(
        "Successfully cleared all time tracking data for character {}",
        character_id
    );
    Ok(())
}

/// Get active sessions for a specific character
#[tauri::command]
pub async fn get_character_active_sessions(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<Vec<LocationSession>> {
    debug!("Getting active sessions for character: {}", character_id);

    let sessions = time_tracking_service
        .get_active_sessions(&character_id)
        .await;
    debug!(
        "Retrieved {} active sessions for character {}",
        sessions.len(),
        character_id
    );
    Ok(sessions)
}

/// Get completed sessions for a specific character
#[tauri::command]
pub async fn get_character_completed_sessions(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<Vec<LocationSession>> {
    debug!("Getting completed sessions for character: {}", character_id);

    let sessions = time_tracking_service
        .get_completed_sessions(&character_id)
        .await;
    debug!(
        "Retrieved {} completed sessions for character {}",
        sessions.len(),
        character_id
    );
    Ok(sessions)
}

/// Get the last known location for a specific character
#[tauri::command]
pub async fn get_character_last_known_location(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<Option<LocationSession>> {
    debug!(
        "Getting last known location for character: {}",
        character_id
    );

    // Use the service method to get the last known location
    let last_location = time_tracking_service
        .get_last_known_location(&character_id)
        .await;

    if let Some(ref location) = last_location {
        debug!(
            "Last known location for character {}: {} ({})",
            character_id, location.location_name, location.location_type
        );
    } else {
        debug!("No location data found for character {}", character_id);
    }

    Ok(last_location)
}

/// Get all location stats for a specific character
#[tauri::command]
pub async fn get_character_location_stats(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<Vec<LocationStats>> {
    debug!("Getting location stats for character: {}", character_id);

    let stats = time_tracking_service.get_all_stats(&character_id).await;
    debug!(
        "Retrieved {} location stats for character {}",
        stats.len(),
        character_id
    );
    Ok(stats)
}

/// Get total play time for a specific character
#[tauri::command]
pub async fn get_character_total_play_time(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<u64> {
    debug!("Getting total play time for character: {}", character_id);

    let total_time = time_tracking_service
        .get_total_play_time(&character_id)
        .await;
    debug!(
        "Total play time for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}

/// Get total play time since process start for a specific character
#[tauri::command]
pub async fn get_character_total_play_time_since_process_start(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<u64> {
    debug!(
        "Getting total play time since process start for character: {}",
        character_id
    );

    let total_time = time_tracking_service
        .get_total_play_time_since_process_start(&character_id)
        .await;
    debug!(
        "Total play time since process start for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}

/// Get total hideout time for a specific character
#[tauri::command]
pub async fn get_character_total_hideout_time(
    character_id: String,
    time_tracking_service: State<'_, Arc<dyn TimeTrackingService>>,
) -> CommandResult<u64> {
    debug!("Getting total hideout time for character: {}", character_id);

    let total_time = time_tracking_service
        .get_total_hideout_time(&character_id)
        .await;
    debug!(
        "Total hideout time for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}
