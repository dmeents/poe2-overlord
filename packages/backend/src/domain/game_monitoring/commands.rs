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

    match game_monitoring_service.get_current_status().await {
        Some(game_status) => {
            let process_info = ProcessInfo {
                name: game_status.name,
                pid: game_status.pid,
                running: game_status.running,
            };
            info!(
                "Retrieved game process status: running={}, pid={}",
                process_info.running, process_info.pid
            );
            Ok(process_info)
        }
        None => {
            let process_info = ProcessInfo {
                name: "Not Found".to_string(),
                pid: 0,
                running: false,
            };
            info!("No game process currently detected");
            Ok(process_info)
        }
    }
}
