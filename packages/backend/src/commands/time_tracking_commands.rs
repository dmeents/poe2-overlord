use crate::commands::{to_command_result, CommandResult};
use crate::models::{LocationStats, LocationType, TimeTrackingData, TimeTrackingSummary};
use crate::services::session_tracker::SessionTracker;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get all time tracking data in a single call
#[tauri::command]
pub async fn get_time_tracking_data(
    time_tracking: State<'_, Arc<SessionTracker>>,
) -> CommandResult<TimeTrackingData> {
    debug!("Getting unified time tracking data");

    let active_sessions = time_tracking.get_active_sessions().await;
    let completed_sessions = time_tracking.get_completed_sessions().await;
    let all_location_stats = time_tracking.get_all_stats().await;

    // Create summary with top locations
    let zone_stats: Vec<LocationStats> = all_location_stats
        .iter()
        .filter(|stat| stat.location_type == LocationType::Zone)
        .cloned()
        .collect();

    let mut sorted_stats = zone_stats;
    sorted_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
    let top_stats = sorted_stats.into_iter().take(10).collect::<Vec<_>>();

    let total_play_time = time_tracking.get_total_play_time().await;
    let total_play_time_since_process_start = time_tracking
        .get_total_play_time_since_process_start()
        .await;
    let total_hideout_time = time_tracking.get_total_hideout_time().await;

    let summary = TimeTrackingSummary {
        character_id: "global".to_string(), // Legacy global tracking
        active_sessions: active_sessions.clone(),
        top_locations: top_stats,
        total_locations_tracked: all_location_stats.len(),
        total_active_sessions: active_sessions.len(),
        total_play_time_seconds: total_play_time,
        total_play_time_since_process_start_seconds: total_play_time_since_process_start,
        total_hideout_time_seconds: total_hideout_time,
    };

    let data = TimeTrackingData {
        character_id: "global".to_string(), // Legacy global tracking
        active_sessions,
        completed_sessions,
        all_location_stats,
        summary,
    };

    debug!("Retrieved unified time tracking data");
    Ok(data)
}

/// Start a time tracking session for a location
#[tauri::command]
pub async fn start_time_tracking_session(
    location_name: String,
    location_type: LocationType,
    time_tracking: State<'_, Arc<SessionTracker>>,
) -> CommandResult<()> {
    to_command_result(
        time_tracking
            .start_session(location_name, location_type)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "Failed to start time tracking session: {}",
                    e
                ))
            }),
    )?;

    info!("Successfully started time tracking session");
    Ok(())
}

/// End a time tracking session for a location
#[tauri::command]
pub async fn end_time_tracking_session(
    location_id: String,
    time_tracking: State<'_, Arc<SessionTracker>>,
) -> CommandResult<()> {
    to_command_result(time_tracking.end_session(&location_id).await.map_err(|e| {
        crate::errors::AppError::Internal(format!("Failed to end time tracking session: {}", e))
    }))?;

    info!("Successfully ended time tracking session");
    Ok(())
}

/// End all active time tracking sessions
#[tauri::command]
pub async fn end_all_active_sessions(
    time_tracking: State<'_, Arc<SessionTracker>>,
) -> CommandResult<()> {
    to_command_result(time_tracking.end_all_active_sessions().await.map_err(|e| {
        crate::errors::AppError::Internal(format!("Failed to end all active sessions: {}", e))
    }))?;

    info!("Successfully ended all active time tracking sessions");
    Ok(())
}

/// Clear all time tracking data
#[tauri::command]
pub async fn clear_all_time_tracking_data(
    time_tracking: State<'_, Arc<SessionTracker>>,
) -> CommandResult<()> {
    to_command_result(time_tracking.clear_all_data().await.map_err(|e| {
        crate::errors::AppError::Internal(format!("Failed to clear time tracking data: {}", e))
    }))?;

    info!("Successfully cleared all time tracking data");
    Ok(())
}
