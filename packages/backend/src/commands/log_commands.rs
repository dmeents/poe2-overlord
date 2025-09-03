use crate::commands::{to_command_result, CommandResult};
use crate::services::LogMonitorService;
use std::sync::Arc;
use tauri::State;

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
    to_command_result(log_monitor.get_log_file_size().map_err(|e| {
        crate::errors::AppError::FileSystem(format!("Failed to get log file size: {}", e))
    }))
}

/// Read the last N lines from the log file
#[tauri::command]
pub fn read_last_log_lines(
    count: usize,
    log_monitor: State<'_, Arc<LogMonitorService>>,
) -> CommandResult<Vec<String>> {
    to_command_result(log_monitor.read_last_lines(count).map_err(|e| {
        crate::errors::AppError::FileSystem(format!("Failed to read log lines: {}", e))
    }))
}
