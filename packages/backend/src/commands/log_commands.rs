use crate::services::LogMonitorService;
use log::info;
use std::sync::Arc;
use tauri::{command, State};

/// Start monitoring the POE client log file
#[command]
pub async fn start_log_monitoring(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> Result<(), String> {
    info!("Starting log monitoring via Tauri command");

    log_monitor
        .start_monitoring()
        .await
        .map_err(|e| format!("Failed to start log monitoring: {}", e))?;

    Ok(())
}

/// Stop monitoring the POE client log file
#[command]
pub async fn stop_log_monitoring(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> Result<(), String> {
    info!("Stopping log monitoring via Tauri command");

    log_monitor
        .stop_monitoring()
        .await
        .map_err(|e| format!("Failed to stop log monitoring: {}", e))?;

    Ok(())
}

/// Check if log monitoring is currently active
#[command]
pub async fn is_log_monitoring_active(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> Result<bool, String> {
    Ok(log_monitor.is_monitoring().await)
}

/// Get the current size of the log file
#[command]
pub fn get_log_file_size(log_monitor: State<'_, Arc<LogMonitorService>>) -> Result<u64, String> {
    log_monitor
        .get_log_file_size()
        .map_err(|e| format!("Failed to get log file size: {}", e))
}

/// Read the last N lines from the log file
#[command]
pub fn read_last_log_lines(
    count: usize,
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> Result<Vec<String>, String> {
    log_monitor
        .read_last_lines(count)
        .map_err(|e| format!("Failed to read log lines: {}", e))
}

/// Subscribe to log events (this will be handled by the frontend through events)
#[command]
pub fn subscribe_to_log_events(
    _log_monitor: State<'_, Arc<LogMonitorService>>,
) -> Result<(), String> {
    // This command is mainly for the frontend to know when to start listening
    // The actual event subscription happens through Tauri's event system
    info!("Log event subscription requested");
    Ok(())
}
