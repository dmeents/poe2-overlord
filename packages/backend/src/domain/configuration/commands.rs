use crate::commands::{to_command_result, CommandResult};
use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::configuration::traits::ConfigurationService;
// ProcessDetector import removed - no longer needed for Tauri commands
use log::{debug, error, info};
use std::sync::Arc;
use tauri::State;

// Note: check_game_process command removed
// Frontend listens to game-monitoring events instead of polling for status

/// Get the current application configuration
#[tauri::command]
pub async fn get_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<AppConfig> {
    debug!("Getting current configuration");

    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config: {}", e);
        crate::errors::AppError::config_error(&format!("Failed to get configuration: {}", e))
    }))?;

    debug!("Successfully retrieved configuration");
    Ok(config)
}

/// Get the default application configuration (without modifying current config)
#[tauri::command]
pub async fn get_default_config() -> CommandResult<AppConfig> {
    debug!("Getting default configuration");
    Ok(AppConfig::default())
}

/// Update the entire application configuration
#[tauri::command]
pub async fn update_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    new_config: AppConfig,
) -> CommandResult<()> {
    debug!("Updating entire configuration");

    to_command_result(config_service.update_config(new_config).await.map_err(|e| {
        error!("Failed to update config: {}", e);
        crate::errors::AppError::config_error(&format!("Failed to update configuration: {}", e))
    }))?;

    info!("Configuration updated successfully");
    Ok(())
}

/// Reset configuration to defaults
#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    debug!("Resetting configuration to defaults");

    to_command_result(config_service.reset_to_defaults().await.map_err(|e| {
        error!("Failed to reset config to defaults: {}", e);
        crate::errors::AppError::config_error(&format!(
            "Failed to reset configuration to defaults: {}",
            e
        ))
    }))?;

    info!("Configuration reset to defaults successfully");
    Ok(())
}

/// Get the POE client log path
#[tauri::command]
pub async fn get_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting POE client log path");

    let path = to_command_result(config_service.get_poe_client_log_path().await.map_err(|e| {
        error!("Failed to get POE client log path: {}", e);
        crate::errors::AppError::config_error(&format!("Failed to get POE client log path: {}", e))
    }))?;

    debug!("POE client log path: {}", path);
    Ok(path)
}

/// Set the POE client log path
#[tauri::command]
pub async fn set_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    path: String,
) -> CommandResult<()> {
    debug!("Setting POE client log path to: {}", path);

    to_command_result(
        config_service
            .set_poe_client_log_path(path.clone())
            .await
            .map_err(|e| {
                error!("Failed to set POE client log path: {}", e);
                crate::errors::AppError::config_error(&format!(
                    "Failed to set POE client log path: {}",
                    e
                ))
            }),
    )?;

    info!("POE client log path set to: {}", path);
    Ok(())
}

/// Get the OS-specific default POE client log path
#[tauri::command]
pub async fn get_default_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting default POE client log path");

    let default_path = config_service.get_default_poe_client_log_path();
    debug!("Default POE client log path: {}", default_path);
    Ok(default_path)
}

/// Reset the POE client log path to the OS-specific default
#[tauri::command]
pub async fn reset_poe_client_log_path_to_default(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    debug!("Resetting POE client log path to default");

    to_command_result(
        config_service
            .reset_poe_client_log_path_to_default()
            .await
            .map_err(|e| {
                error!("Failed to reset POE client log path to default: {}", e);
                crate::errors::AppError::config_error(&format!(
                    "Failed to reset POE client log path to default: {}",
                    e
                ))
            }),
    )?;

    info!("POE client log path reset to default successfully");
    Ok(())
}

/// Get the current log level
#[tauri::command]
pub async fn get_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting current log level");

    let log_level = to_command_result(config_service.get_log_level().await.map_err(|e| {
        error!("Failed to get log level: {}", e);
        crate::errors::AppError::config_error(&format!("Failed to get log level: {}", e))
    }))?;

    debug!("Current log level: {}", log_level);
    Ok(log_level)
}

/// Set the log level
#[tauri::command]
pub async fn set_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    level: String,
) -> CommandResult<()> {
    debug!("Setting log level to: {}", level);

    to_command_result(
        config_service
            .set_log_level(level.clone())
            .await
            .map_err(|e| {
                error!("Failed to set log level: {}", e);
                crate::errors::AppError::config_error(&format!("Failed to set log level: {}", e))
            }),
    )?;

    info!("Log level set to: {}", level);
    Ok(())
}

/// Get configuration file information
#[tauri::command]
pub async fn get_config_file_info(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationFileInfo> {
    debug!("Getting configuration file information");

    let file_info = to_command_result(config_service.get_file_info().await.map_err(|e| {
        error!("Failed to get config file info: {}", e);
        crate::errors::AppError::config_error(&format!(
            "Failed to get configuration file information: {}",
            e
        ))
    }))?;

    debug!("Configuration file info retrieved successfully");
    Ok(file_info)
}

/// Validate the current configuration
#[tauri::command]
pub async fn validate_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationValidationResult> {
    debug!("Validating current configuration");

    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config for validation: {}", e);
        crate::errors::AppError::config_error(&format!(
            "Failed to get configuration for validation: {}",
            e
        ))
    }))?;

    let validation_result =
        to_command_result(config_service.validate_config(&config).await.map_err(|e| {
            error!("Failed to validate config: {}", e);
            crate::errors::AppError::config_error(&format!(
                "Failed to validate configuration: {}",
                e
            ))
        }))?;

    debug!(
        "Configuration validation completed: valid = {}",
        validation_result.is_valid
    );
    Ok(validation_result)
}
