//! Tauri Command Handlers for Server Monitoring
//!
//! This module contains Tauri command handlers that expose server monitoring
//! functionality to the frontend application. Simplified to only essential operations.

use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::ServerMonitoringService;
use crate::infrastructure::tauri::CommandResult;
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
