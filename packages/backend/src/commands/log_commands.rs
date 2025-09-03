use crate::services::LogMonitorService;
use crate::commands::{CommandResult, to_command_result};
use log::debug;
use std::sync::Arc;
use tauri::State;

/// Start monitoring the POE client log file
#[tauri::command]
pub async fn start_log_monitoring(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<()> {
    debug!("Starting log monitoring via Tauri command");

    to_command_result(log_monitor
        .start_monitoring()
        .await
        .map_err(|e| crate::errors::AppError::LogMonitor(format!("Failed to start log monitoring: {}", e))))
}

/// Stop monitoring the POE client log file
#[tauri::command]
pub async fn stop_log_monitoring(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<()> {
    debug!("Stopping log monitoring via Tauri command");

    to_command_result(log_monitor
        .stop_monitoring()
        .await
        .map_err(|e| crate::errors::AppError::LogMonitor(format!("Failed to stop log monitoring: {}", e))))
}

/// Check if log monitoring is currently active
#[tauri::command]
pub async fn is_log_monitoring_active(
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<bool> {
    Ok(log_monitor.is_monitoring().await)
}

/// Get the current size of the log file
#[tauri::command]
pub fn get_log_file_size(log_monitor: State<'_, Arc<LogMonitorService>>) -> CommandResult<u64> {
    to_command_result(log_monitor
        .get_log_file_size()
        .map_err(|e| crate::errors::AppError::FileSystem(format!("Failed to get log file size: {}", e))))
}

/// Read the last N lines from the log file
#[tauri::command]
pub fn read_last_log_lines(
    count: usize,
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<Vec<String>> {
    to_command_result(log_monitor
        .read_last_lines(count)
        .map_err(|e| crate::errors::AppError::FileSystem(format!("Failed to read log lines: {}", e))))
}

/// Subscribe to log events (this will be handled by the frontend through events)
#[tauri::command]
pub fn subscribe_to_log_events(
    _log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<()> {
    // This command is mainly for the frontend to know when to start listening
    // The actual event subscription happens through Tauri's event system
    debug!("Log event subscription requested");
    Ok(())
}
