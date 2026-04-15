//! Tauri command handlers for game process monitoring

use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::CommandResult;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_game_process_status(
    game_monitoring_service: State<'_, Arc<dyn GameMonitoringService>>,
) -> CommandResult<GameProcessStatus> {
    debug!("Getting current game process status");

    // Try cached status first, if available
    if let Some(game_status) = game_monitoring_service.get_current_status().await {
        info!(
            "Retrieved cached game process status: running={}, pid={}",
            game_status.running, game_status.pid
        );
        return Ok(game_status);
    }

    // No cached status - do a live check (handles race condition on startup)
    debug!("No cached status available, performing live check");
    match game_monitoring_service.check_status_now().await {
        Ok(game_status) => {
            info!(
                "Retrieved live game process status: running={}, pid={}",
                game_status.running, game_status.pid
            );
            Ok(game_status)
        }
        Err(e) => {
            info!("Failed to check game process status: {e}");
            Ok(GameProcessStatus::not_running())
        }
    }
}
