//! Tauri command handlers for server monitoring

use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use crate::CommandResult;
use log::debug;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_server_status(
    server_monitoring_service: State<'_, Arc<dyn ServerMonitoringService>>,
) -> CommandResult<Option<ServerStatus>> {
    debug!("Getting current server status");
    Ok(server_monitoring_service.get_current_status().await)
}
