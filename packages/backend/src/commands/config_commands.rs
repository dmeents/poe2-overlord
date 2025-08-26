use crate::models::AppConfig;
use crate::services::ConfigService;
use tauri::{AppHandle, State};

/// Get the current application configuration
#[tauri::command]
pub async fn get_config(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
) -> Result<AppConfig, String> {
    Ok(config_service.get_config())
}

/// Update the entire application configuration
#[tauri::command]
pub async fn update_config(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
    new_config: AppConfig,
) -> Result<(), String> {
    config_service
        .update_config(new_config)
        .map_err(|e| e.to_string())
}

/// Get the POE client log file path
#[tauri::command]
pub async fn get_poe_client_log_path(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
) -> Result<String, String> {
    Ok(config_service.get_poe_client_log_path())
}

/// Set the POE client log file path
#[tauri::command]
pub async fn set_poe_client_log_path(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
    path: String,
) -> Result<(), String> {
    config_service
        .set_poe_client_log_path(path)
        .map_err(|e| e.to_string())
}

/// Get the auto-start monitoring setting
#[tauri::command]
pub async fn get_auto_start_monitoring(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
) -> Result<bool, String> {
    Ok(config_service.get_auto_start_monitoring())
}

/// Set the auto-start monitoring setting
#[tauri::command]
pub async fn set_auto_start_monitoring(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
    enabled: bool,
) -> Result<(), String> {
    config_service
        .set_auto_start_monitoring(enabled)
        .map_err(|e| e.to_string())
}

/// Get the log level setting
#[tauri::command]
pub async fn get_log_level(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
) -> Result<String, String> {
    Ok(config_service.get_log_level())
}

/// Set the log level setting
#[tauri::command]
pub async fn set_log_level(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
    level: String,
) -> Result<(), String> {
    config_service
        .set_log_level(level)
        .map_err(|e| e.to_string())
}

/// Reset configuration to defaults
#[tauri::command]
pub async fn reset_config_to_defaults(
    _app_handle: AppHandle,
    config_service: State<'_, ConfigService>,
) -> Result<(), String> {
    config_service
        .update_config(AppConfig::default())
        .map_err(|e| e.to_string())
}
