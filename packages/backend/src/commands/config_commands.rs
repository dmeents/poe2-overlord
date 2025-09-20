use crate::commands::{to_command_result, CommandResult};
use crate::models::AppConfig;
use crate::services::configuration_manager::ConfigurationManager;
use crate::services::process_detector::ProcessDetector;
use log::error;
use tauri::State;

/// Check game process status
#[tauri::command]
pub async fn check_game_process() -> CommandResult<crate::models::ProcessInfo> {
    to_command_result(ProcessDetector::check_game_process().map_err(|e| {
        error!("Failed to check game process: {}", e);
        crate::errors::AppError::process_monitor_error(&format!(
            "Failed to check game process: {}",
            e
        ))
    }))
}

/// Get the current application configuration
#[tauri::command]
pub async fn get_config(
    config_service: State<'_, ConfigurationManager>,
) -> CommandResult<AppConfig> {
    Ok(config_service.get_config())
}

/// Get the default application configuration (without modifying current config)
#[tauri::command]
pub async fn get_default_config() -> CommandResult<AppConfig> {
    Ok(AppConfig::default())
}

/// Update the entire application configuration
#[tauri::command]
pub async fn update_config(
    config_service: State<'_, ConfigurationManager>,
    new_config: AppConfig,
) -> CommandResult<()> {
    to_command_result(config_service.update_config(new_config).map_err(|e| {
        error!("Failed to update config: {}", e);
        crate::errors::AppError::config_error(&format!("Failed to update configuration: {}", e))
    }))
}

/// Reset configuration to defaults
#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, ConfigurationManager>,
) -> CommandResult<()> {
    to_command_result(
        config_service
            .update_config(AppConfig::default())
            .map_err(|e| {
                error!("Failed to reset config to defaults: {}", e);
                crate::errors::AppError::config_error(&format!(
                    "Failed to reset configuration to defaults: {}",
                    e
                ))
            }),
    )
}
