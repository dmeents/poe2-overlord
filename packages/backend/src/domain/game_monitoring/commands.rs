//! Tauri command handlers for game process monitoring

use crate::domain::game_monitoring::models::ProcessInfo;
use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::CommandResult;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_game_process_status(
    game_monitoring_service: State<'_, Arc<dyn GameMonitoringService>>,
) -> CommandResult<ProcessInfo> {
    debug!("Getting current game process status");

    // Try cached status first, if available
    if let Some(game_status) = game_monitoring_service.get_current_status().await {
        let process_info = ProcessInfo {
            name: game_status.name,
            pid: game_status.pid,
            running: game_status.running,
        };
        info!(
            "Retrieved cached game process status: running={}, pid={}",
            process_info.running, process_info.pid
        );
        return Ok(process_info);
    }

    // No cached status - do a live check (handles race condition on startup)
    debug!("No cached status available, performing live check");
    match game_monitoring_service.check_status_now().await {
        Ok(game_status) => {
            let process_info = ProcessInfo {
                name: game_status.name,
                pid: game_status.pid,
                running: game_status.running,
            };
            info!(
                "Retrieved live game process status: running={}, pid={}",
                process_info.running, process_info.pid
            );
            Ok(process_info)
        }
        Err(e) => {
            info!("Failed to check game process status: {}", e);
            Ok(ProcessInfo {
                name: "Not Found".to_string(),
                pid: 0,
                running: false,
            })
        }
    }
}
