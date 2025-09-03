use crate::models::AppConfig;
use crate::services::config::ConfigService;
use crate::commands::{CommandResult, to_command_result};
use log::{debug, error, info, trace, warn};
use tauri::State;

/// Test command to verify ConfigService is accessible
#[tauri::command]
pub async fn test_config_service(
    _config_service: State<'_, ConfigService>,
) -> CommandResult<String> {
    Ok("ConfigService is accessible!".to_string())
}

/// Get the current application configuration
#[tauri::command]
pub async fn get_config(config_service: State<'_, ConfigService>) -> CommandResult<AppConfig> {
    Ok(config_service.get_config())
}

/// Update the entire application configuration
#[tauri::command]
pub async fn update_config(
    config_service: State<'_, ConfigService>,
    new_config: AppConfig,
) -> CommandResult<()> {
    to_command_result(config_service.update_config(new_config).map_err(|e| {
        error!("Failed to update config: {}", e);
        crate::errors::AppError::Config(format!("Failed to update configuration: {}", e))
    }))
}

/// Get the POE client log file path
#[tauri::command]
pub async fn get_poe_client_log_path(
    config_service: State<'_, ConfigService>,
) -> CommandResult<String> {
    Ok(config_service.get_poe_client_log_path())
}

/// Set the POE client log file path
#[tauri::command]
pub async fn set_poe_client_log_path(
    config_service: State<'_, ConfigService>,
    path: String,
) -> CommandResult<()> {
    to_command_result(config_service.set_poe_client_log_path(path).map_err(|e| {
        error!("Failed to set POE client log path: {}", e);
        crate::errors::AppError::Config(format!("Failed to set POE client log path: {}", e))
    }))
}

/// Get the log level setting
#[tauri::command]
pub async fn get_log_level(config_service: State<'_, ConfigService>) -> CommandResult<String> {
    Ok(config_service.get_log_level())
}

/// Set the log level setting
#[tauri::command]
pub async fn set_log_level(
    config_service: State<'_, ConfigService>,
    level: String,
) -> CommandResult<()> {
    // Save the log level to config
    to_command_result(config_service.set_log_level(level.clone()).map_err(|e| {
        error!("Failed to set log level: {}", e);
        crate::errors::AppError::Config(format!("Failed to set log level: {}", e))
    }))?;

    // Note: Log level changes require app restart due to Tauri's logging plugin
    info!(
        "Log level changed to: {} - restart the app to see the new level",
        level
    );
    Ok(())
}

/// Reset configuration to defaults
#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, ConfigService>,
) -> CommandResult<()> {
    to_command_result(config_service
        .update_config(AppConfig::default())
        .map_err(|e| {
            error!("Failed to reset config to defaults: {}", e);
            crate::errors::AppError::Config(format!("Failed to reset configuration to defaults: {}", e))
        }))
}

/// Get the OS-specific default POE client log path
#[tauri::command]
pub async fn get_default_poe_client_log_path(
    config_service: State<'_, ConfigService>,
) -> CommandResult<String> {
    Ok(config_service.get_default_poe_client_log_path())
}

/// Reset the POE client log path to the OS-specific default
#[tauri::command]
pub async fn reset_poe_client_log_path_to_default(
    config_service: State<'_, ConfigService>,
) -> CommandResult<()> {
    to_command_result(config_service
        .reset_poe_client_log_path_to_default()
        .map_err(|e| {
            error!("Failed to reset POE client log path to default: {}", e);
            crate::errors::AppError::Config(format!("Failed to reset POE client log path to default: {}", e))
        }))
}

/// Test command to verify logging levels are working
#[tauri::command]
pub async fn test_logging_levels() -> CommandResult<String> {
    // Note: These logs will only appear if the current log level allows them
    // Use this command to test if your log level setting is working
    trace!("This is a TRACE level message");
    debug!("This is a DEBUG level message");
    info!("This is an INFO level message");
    warn!("This is a WARN level message");
    error!("This is an ERROR level message");

    Ok("Logging test completed - check your terminal for messages".to_string())
}
