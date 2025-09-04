use crate::commands::{to_command_result, CommandResult};
use crate::models::{LocationStats, LocationType, TimeTrackingData, TimeTrackingSummary};
use crate::services::character_session_tracker::CharacterSessionTracker;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get all time tracking data for a specific character
#[tauri::command]
pub async fn get_character_time_tracking_data(
    character_id: String,
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<TimeTrackingData> {
    debug!("Getting time tracking data for character: {}", character_id);

    let active_sessions = character_session_tracker
        .get_active_sessions(&character_id)
        .await;
    let completed_sessions = character_session_tracker
        .get_completed_sessions(&character_id)
        .await;
    let all_location_stats = character_session_tracker.get_all_stats(&character_id).await;

    // Create summary with top locations
    let zone_stats: Vec<LocationStats> = all_location_stats
        .iter()
        .filter(|stat| stat.location_type == LocationType::Zone)
        .cloned()
        .collect();

    let mut sorted_stats = zone_stats;
    sorted_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
    let top_stats = sorted_stats.into_iter().take(10).collect::<Vec<_>>();

    let total_play_time = character_session_tracker
        .get_total_play_time(&character_id)
        .await;
    let total_play_time_since_process_start = character_session_tracker
        .get_total_play_time_since_process_start(&character_id)
        .await;
    let total_hideout_time = character_session_tracker
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

/// Start a time tracking session for a character and location
#[tauri::command]
pub async fn start_character_time_tracking_session(
    character_id: String,
    location_name: String,
    location_type: LocationType,
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<()> {
    debug!(
        "Starting time tracking session for character {} in {}: {}",
        character_id,
        location_type.to_string(),
        location_name
    );

    to_command_result(
        character_session_tracker
            .start_session(&character_id, location_name, location_type)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "Failed to start time tracking session: {}",
                    e
                ))
            }),
    )?;

    info!(
        "Successfully started time tracking session for character {}",
        character_id
    );
    Ok(())
}

/// End a time tracking session for a character and location
#[tauri::command]
pub async fn end_character_time_tracking_session(
    character_id: String,
    location_id: String,
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<()> {
    debug!(
        "Ending time tracking session for character {} in location: {}",
        character_id, location_id
    );

    to_command_result(
        character_session_tracker
            .end_session(&character_id, &location_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "Failed to end time tracking session: {}",
                    e
                ))
            }),
    )?;

    info!(
        "Successfully ended time tracking session for character {}",
        character_id
    );
    Ok(())
}

/// End all active time tracking sessions for a character
#[tauri::command]
pub async fn end_all_character_active_sessions(
    character_id: String,
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<()> {
    debug!(
        "Ending all active time tracking sessions for character: {}",
        character_id
    );

    to_command_result(
        character_session_tracker
            .end_all_active_sessions(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "Failed to end all active sessions: {}",
                    e
                ))
            }),
    )?;

    info!(
        "Successfully ended all active time tracking sessions for character {}",
        character_id
    );
    Ok(())
}

/// Clear all time tracking data for a specific character
#[tauri::command]
pub async fn clear_character_time_tracking_data(
    character_id: String,
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<()> {
    debug!(
        "Clearing all time tracking data for character: {}",
        character_id
    );

    to_command_result(
        character_session_tracker
            .clear_character_data(&character_id)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "Failed to clear time tracking data: {}",
                    e
                ))
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<Vec<crate::models::LocationSession>> {
    debug!("Getting active sessions for character: {}", character_id);

    let sessions = character_session_tracker
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<Vec<crate::models::LocationSession>> {
    debug!("Getting completed sessions for character: {}", character_id);

    let sessions = character_session_tracker
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<Option<crate::models::LocationSession>> {
    debug!(
        "Getting last known location for character: {}",
        character_id
    );

    // Get all completed sessions and find the most recent one
    let sessions = character_session_tracker
        .get_completed_sessions(&character_id)
        .await;

    // Also check for active sessions (current location)
    let active_sessions = character_session_tracker
        .get_active_sessions(&character_id)
        .await;

    // Combine active and completed sessions
    let mut all_sessions = sessions;
    all_sessions.extend(active_sessions);

    // Sort by entry timestamp (most recent first)
    all_sessions.sort_by(|a, b| b.entry_timestamp.cmp(&a.entry_timestamp));

    let last_location = all_sessions.first().cloned();

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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<Vec<LocationStats>> {
    debug!("Getting location stats for character: {}", character_id);

    let stats = character_session_tracker.get_all_stats(&character_id).await;
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<u64> {
    debug!("Getting total play time for character: {}", character_id);

    let total_time = character_session_tracker
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<u64> {
    debug!(
        "Getting total play time since process start for character: {}",
        character_id
    );

    let total_time = character_session_tracker
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
    character_session_tracker: State<'_, Arc<CharacterSessionTracker>>,
) -> CommandResult<u64> {
    debug!("Getting total hideout time for character: {}", character_id);

    let total_time = character_session_tracker
        .get_total_hideout_time(&character_id)
        .await;
    debug!(
        "Total hideout time for character {}: {} seconds",
        character_id, total_time
    );
    Ok(total_time)
}
