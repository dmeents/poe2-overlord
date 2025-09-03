use crate::services::server_status::{ServerStatus, ServerStatusManager};
use crate::services::process_monitor::ProcessMonitor;
use crate::commands::{CommandResult, to_command_result};
use log::{debug, error};
use std::sync::Arc;
use tauri::State;

/// Get the current server status
#[tauri::command]
pub async fn get_server_status(
    server_manager: State<'_, Arc<ServerStatusManager>>,
) -> CommandResult<Option<ServerStatus>> {
    debug!("Getting server status via Tauri command");

    match server_manager.get_server_status().await {
        Some(status) => {
            debug!(
                "Retrieved server status: {}:{} (online: {})",
                status.ip_address, status.port, status.is_online
            );
            Ok(Some(status))
        }
        None => {
            debug!("No server status available");
            Ok(None)
        }
    }
}

/// Get the last known server address
#[tauri::command]
pub async fn get_last_known_server(
    server_manager: State<'_, Arc<ServerStatusManager>>,
) -> CommandResult<Option<(String, u16)>> {
    debug!("Getting last known server via Tauri command");

    match server_manager.get_last_known_server().await {
        Some((ip, port)) => {
            debug!("Retrieved last known server: {}:{}", ip, port);
            Ok(Some((ip, port)))
        }
        None => {
            debug!("No last known server available");
            Ok(None)
        }
    }
}

/// Ping the current server and return ping time
#[tauri::command]
pub async fn ping_server(
    server_manager: State<'_, Arc<ServerStatusManager>>,
) -> CommandResult<Option<u64>> {
    debug!("Pinging server via Tauri command");

    match server_manager.ping_server().await {
        Ok(Some(ping_ms)) => {
            debug!("Server ping successful: {}ms", ping_ms);
            Ok(Some(ping_ms))
        }
        Ok(None) => {
            debug!("No server to ping");
            Ok(None)
        }
        Err(e) => {
            error!("Failed to ping server: {}", e);
            Err(format!("Failed to ping server: {}", e))
        }
    }
}

/// Check POE2 process status
#[tauri::command]
pub async fn check_poe2_process() -> CommandResult<crate::models::ProcessInfo> {
    debug!("Checking POE2 process status via Tauri command");
    
    to_command_result(ProcessMonitor::check_poe2_process().map_err(|e| {
        error!("Failed to check POE2 process: {}", e);
        crate::errors::AppError::ProcessMonitor(format!("Failed to check POE2 process: {}", e))
    }))
}
