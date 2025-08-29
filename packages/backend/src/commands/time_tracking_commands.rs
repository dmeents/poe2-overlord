use crate::models::{LocationSession, LocationStats, LocationType};
use crate::services::time_tracking::TimeTrackingService;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get all active time tracking sessions
#[tauri::command]
pub async fn get_active_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationSession>, String> {
    let sessions = time_tracking.get_active_sessions();

    Ok(sessions)
}

/// Get all completed time tracking sessions
#[tauri::command]
pub async fn get_completed_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationSession>, String> {
    debug!("Getting completed time tracking sessions");

    let sessions = time_tracking.get_completed_sessions();
    info!("Retrieved {} completed sessions", sessions.len());

    Ok(sessions)
}

/// Get statistics for all locations
#[tauri::command]
pub async fn get_all_location_stats(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Vec<LocationStats>, String> {
    debug!("Getting all location statistics");

    let stats = time_tracking.get_all_stats();
    info!("Retrieved statistics for {} locations", stats.len());

    Ok(stats)
}

/// Get statistics for a specific location
#[tauri::command]
pub async fn get_location_stats(
    location_id: String,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<Option<LocationStats>, String> {
    debug!("Getting statistics for location: {}", location_id);

    let stats = time_tracking.get_location_stats(&location_id);

    match stats {
        Some(stats) => {
            info!("Retrieved statistics for location: {}", location_id);
            Ok(Some(stats))
        }
        None => {
            debug!("No statistics found for location: {}", location_id);
            Ok(None)
        }
    }
}

/// Start a new time tracking session
#[tauri::command]
pub async fn start_time_tracking_session(
    location_name: String,
    location_type: String,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    debug!(
        "Starting time tracking session for {}: {}",
        location_type, location_name
    );

    let location_type_enum = match location_type.to_lowercase().as_str() {
        "zone" => LocationType::Zone,
        "act" => LocationType::Act,
        _ => return Err(format!("Invalid location type: {}", location_type)),
    };

    time_tracking
        .start_session(location_name, location_type_enum)
        .await
        .map_err(|e| format!("Failed to start session: {}", e))?;

    info!("Successfully started time tracking session");
    Ok(())
}

/// End a time tracking session
#[tauri::command]
pub async fn end_time_tracking_session(
    location_id: String,
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    debug!("Ending time tracking session for location: {}", location_id);

    time_tracking
        .end_session(&location_id)
        .await
        .map_err(|e| format!("Failed to end session: {}", e))?;

    info!("Successfully ended time tracking session");
    Ok(())
}

/// End all active time tracking sessions
#[tauri::command]
pub async fn end_all_active_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    debug!("Ending all active time tracking sessions");

    time_tracking
        .end_all_active_sessions()
        .await
        .map_err(|e| format!("Failed to end all active sessions: {}", e))?;

    info!("Successfully ended all active time tracking sessions");
    Ok(())
}

/// Check if there are any stale active sessions
#[tauri::command]
pub async fn has_stale_sessions(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<bool, String> {
    debug!("Checking for stale active sessions");

    let has_stale = time_tracking.has_stale_sessions();
    info!("Stale sessions check result: {}", has_stale);

    Ok(has_stale)
}

/// Clear all time tracking data
#[tauri::command]
pub async fn clear_all_time_tracking_data(
    time_tracking: State<'_, Arc<TimeTrackingService>>,
) -> Result<(), String> {
    debug!("Clearing all time tracking data");

    time_tracking
        .clear_all_data()
        .map_err(|e| format!("Failed to clear data: {}", e))?;

    info!("Successfully cleared all time tracking data");
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

    // Sort stats by total time (descending) and take top 10
    let mut sorted_stats = all_stats;
    sorted_stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));
    let top_stats = sorted_stats.into_iter().take(10).collect::<Vec<_>>();

    let summary = serde_json::json!({
        "active_sessions": active_sessions,
        "top_locations": top_stats,
        "total_locations_tracked": top_stats.len(),
        "total_active_sessions": active_sessions.len()
    });

    info!("Retrieved time tracking summary");
    Ok(summary)
}
