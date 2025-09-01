use crate::services::server_status::{ServerStatus, ServerStatusManager};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::State;

/// Get the current server status
#[tauri::command]
pub async fn get_server_status(
    server_manager: State<'_, Arc<ServerStatusManager>>,
) -> Result<Option<ServerStatus>, String> {
    debug!("Getting server status via Tauri command");
    
    match server_manager.get_server_status().await {
        Some(status) => {
            debug!("Retrieved server status: {}:{} (online: {})", 
                   status.ip_address, status.port, status.is_online);
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
) -> Result<Option<(String, u16)>, String> {
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
) -> Result<Option<u64>, String> {
    debug!("Pinging server via Tauri command");
    
    match server_manager.ping_server().await {
        Ok(Some(ping_ms)) => {
            info!("Server ping successful: {}ms", ping_ms);
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

/// Start server status monitoring
#[tauri::command]
pub async fn start_server_monitoring(
    server_manager: State<'_, Arc<ServerStatusManager>>,
) -> Result<(), String> {
    debug!("Starting server monitoring via Tauri command");
    
    match server_manager.start_monitoring().await {
        Ok(()) => {
            info!("Server monitoring started successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to start server monitoring: {}", e);
            Err(format!("Failed to start server monitoring: {}", e))
        }
    }
}
