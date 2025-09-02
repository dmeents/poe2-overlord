use crate::models::{LocationSession, LocationStats, LocationType};
use crate::services::time_tracking::TimeTrackingService;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get current active sessions
#[tauri::command]
pub async fn get_active_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationSession>, String> {
    let sessions = time_tracking.get_active_sessions();
    debug!("Retrieved {} active sessions", sessions.len());
    Ok(sessions)
}

/// Get completed sessions
#[tauri::command]
pub async fn get_completed_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationSession>, String> {
    let sessions = time_tracking.get_completed_sessions();
    debug!("Retrieved {} completed sessions", sessions.len());
    Ok(sessions)
}

/// Get statistics for all locations
#[tauri::command]
pub async fn get_all_location_stats(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationStats>, String> {
    let stats = time_tracking.get_all_stats();
    debug!("Retrieved statistics for {} locations", stats.len());
    Ok(stats)
}

/// Get statistics for a specific location
#[tauri::command]
pub async fn get_location_stats(
    location_id: String,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Option<LocationStats>, String> {
    let stats = time_tracking.get_location_stats(&location_id);
    debug!("Retrieved statistics for location: {}", location_id);
    Ok(stats)
}

/// Start a time tracking session for a location
#[tauri::command]
pub async fn start_time_tracking_session(
    location_name: String,
    location_type: LocationType,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking
        .start_session(location_name, location_type)
        .await
        .map_err(|e| format!("Failed to start time tracking session: {}", e))?;

    info!("Successfully started time tracking session");
    Ok(())
}

/// End a time tracking session for a location
#[tauri::command]
pub async fn end_time_tracking_session(
    location_id: String,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking
        .end_session(&location_id)
        .await
        .map_err(|e| format!("Failed to end time tracking session: {}", e))?;

    info!("Successfully ended time tracking session");
    Ok(())
}

/// End all active time tracking sessions
#[tauri::command]
pub async fn end_all_active_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking
        .end_all_active_sessions()
        .await
        .map_err(|e| format!("Failed to end all active sessions: {}", e))?;

    info!("Successfully ended all active time tracking sessions");
    Ok(())
}

/// Clear all time tracking data
#[tauri::command]
pub async fn clear_all_time_tracking_data(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking
        .clear_all_data()
        .map_err(|e| format!("Failed to clear time tracking data: {}", e))?;

    info!("Successfully cleared all time tracking data");
    Ok(())
}

/// Set the POE process start time for time tracking
#[tauri::command]
pub async fn set_poe_process_start_time(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking.set_poe_process_start_time();
    debug!("POE process start time set successfully");
    Ok(())
}

/// Clear the POE process start time
#[tauri::command]
pub async fn clear_poe_process_start_time(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    time_tracking.clear_poe_process_start_time();
    debug!("POE process start time cleared successfully");
    Ok(())
}

/// Get time tracking summary (active sessions and recent stats)
#[tauri::command]
pub async fn get_time_tracking_summary(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<serde_json::Value, String> {
    debug!("Getting time tracking summary");

    let active_sessions = time_tracking.get_active_sessions();
    let all_stats = time_tracking.get_all_stats();

    // Filter out Hideouts and Acts, keeping only Zones for top locations calculation
    let zone_stats: Vec<LocationStats> = all_stats
        .into_iter()
        .filter(|stat| stat.location_type == LocationType::Zone)
        .collect();

    // Sort zone stats by total time (descending) and take top 10
    let mut sorted_stats = zone_stats;
    sorted_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
    let top_stats = sorted_stats.into_iter().take(10).collect::<Vec<_>>();

    // Calculate new metrics
    let total_play_time = time_tracking.get_total_play_time();
    let total_play_time_since_process_start =
        time_tracking.get_total_play_time_since_process_start();
    let total_hideout_time = time_tracking.get_total_hideout_time();

    let summary = serde_json::json!({
        "active_sessions": active_sessions,
        "top_locations": top_stats,
        "total_locations_tracked": top_stats.len(),
        "total_active_sessions": active_sessions.len(),
        "total_play_time_seconds": total_play_time,
        "total_play_time_since_process_start_seconds": total_play_time_since_process_start,
        "total_hideout_time_seconds": total_hideout_time
    });

    debug!("Retrieved time tracking summary");
    Ok(summary)
}
