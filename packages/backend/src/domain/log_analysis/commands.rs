use crate::infrastructure::tauri::{to_command_result, CommandResult};
use crate::domain::log_analysis::traits::LogAnalysisService;
use log::debug;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn is_log_monitoring_active(
    log_analysis_service: State<'_, Arc<dyn LogAnalysisService>>,
) -> CommandResult<bool> {
    debug!("Checking if log monitoring is active");
    Ok(log_analysis_service.is_monitoring().await)
}

#[tauri::command]
pub async fn get_log_file_size(
    log_analysis_service: State<'_, Arc<dyn LogAnalysisService>>,
) -> CommandResult<u64> {
    debug!("Getting log file size");
    
    let file_info = to_command_result(log_analysis_service.get_log_file_info().await.map_err(|e| {
        crate::errors::AppError::file_system_error("Failed to get log file info: {}", &e.to_string())
    }))?;
    
    Ok(file_info.size)
}

#[tauri::command]
pub async fn read_last_log_lines(
    count: usize,
    log_analysis_service: State<'_, Arc<dyn LogAnalysisService>>,
) -> CommandResult<Vec<String>> {
    debug!("Reading last {} lines from log file", count);
    
    to_command_result(log_analysis_service.read_log_lines(0, count).await.map_err(|e| {
        crate::errors::AppError::file_system_error("Failed to read log lines: {}", &e.to_string())
    }))
}
