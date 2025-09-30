//! Tauri Command Handlers for Server Monitoring
//!
//! This module contains Tauri command handlers that expose server monitoring
//! functionality to the frontend application. Simplified to only essential operations.

use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::ServerMonitoringService;
use crate::infrastructure::tauri::{to_command_result, CommandResult};
use log::debug;
use std::sync::Arc;
use tauri::State;

/// Get the current server status
///
/// This command provides the current status of the server monitoring.
/// It returns the most recent server information including IP address,
/// online status, and latency measurements.
///
/// # Returns
///
/// The current `ServerStatus` containing server status details
///
#[tauri::command]
pub async fn get_server_status(
    service: State<'_, Arc<dyn ServerMonitoringService>>,
) -> CommandResult<ServerStatus> {
    debug!("Getting current server status");
    Ok(service.get_current_status().await)
}

/// Ping the current server
///
/// This command performs a ping operation to the current server
/// and updates the server status with the result.
///
/// # Returns
///
/// Success or error result
///
#[tauri::command]
pub async fn ping_server(
    service: State<'_, Arc<dyn ServerMonitoringService>>,
) -> CommandResult<()> {
    debug!("Pinging current server");
    to_command_result(service.ping_current_server().await)
}

/// Start periodic server monitoring
///
/// This command starts the background ping monitoring service
/// that periodically checks server connectivity.
///
/// # Returns
///
/// Success or error result
///
#[tauri::command]
pub async fn start_server_monitoring(
    service: State<'_, Arc<dyn ServerMonitoringService>>,
) -> CommandResult<()> {
    debug!("Starting server monitoring");
    to_command_result(service.start_ping_monitoring().await)
}

/// Stop periodic server monitoring
///
/// This command stops the background ping monitoring service.
///
/// # Returns
///
/// Success or error result
///
#[tauri::command]
pub async fn stop_server_monitoring(
    service: State<'_, Arc<dyn ServerMonitoringService>>,
) -> CommandResult<()> {
    debug!("Stopping server monitoring");
    to_command_result(service.stop_ping_monitoring().await)
}
