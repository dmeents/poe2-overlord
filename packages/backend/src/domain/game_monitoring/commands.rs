//! Tauri Command Handlers for Game Process Monitoring
//!
//! This module contains Tauri command handlers that expose game process monitoring
//! functionality to the frontend application. These commands provide a bridge between
//! the frontend JavaScript/TypeScript code and the backend Rust game monitoring service.
//!
//! # Command Categories
//!
//! - **Process Status**: Get current game process status
//! - **Monitoring Control**: Start/stop monitoring operations
//!
//! # Error Handling
//!
//! All commands use the `CommandResult<T>` type for consistent error handling
//! across the frontend-backend boundary. Errors are properly logged and
//! converted to user-friendly messages.

use crate::domain::game_monitoring::models::ProcessInfo;
use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::infrastructure::tauri::CommandResult;
use log::{debug, info};
use std::sync::Arc;
use tauri::State;

/// Get the current game process status
///
/// This command provides the current status of the game process monitoring.
/// It returns the most recent process information including whether the game
/// is running, the process ID, and detection timestamp.
///
/// # Returns
///
/// The current `ProcessInfo` containing process status details
///
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
