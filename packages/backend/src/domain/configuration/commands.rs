//! Tauri Command Handlers for Configuration Management
//!
//! This module contains all Tauri command handlers that expose the configuration
//! domain functionality to the frontend application. These commands provide a
//! bridge between the frontend JavaScript/TypeScript code and the backend Rust
//! configuration service.
//!
//! # Command Categories
//!
//! - **Configuration CRUD**: Get, update, reset configuration
//! - **POE Client Path Management**: Get, set, reset POE client log paths
//! - **Log Level Management**: Get, set application log levels
//! - **Validation**: Validate configuration settings
//! - **File Information**: Get configuration file metadata
//!
//! # Error Handling
//!
//! All commands use the `CommandResult<T>` type for consistent error handling
//! across the frontend-backend boundary. Errors are properly logged and
//! converted to user-friendly messages.
//!
//! # Logging
//!
//! Commands include comprehensive debug and info logging for troubleshooting
//! and monitoring configuration operations.

use crate::infrastructure::tauri::{to_command_result, CommandResult};
use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult,
};
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::configuration::traits::ConfigurationService;
use log::{debug, error, info};
use std::sync::Arc;
use tauri::State;

/// Retrieve the current application configuration
/// 
/// This command provides read access to the current configuration state.
/// It returns the in-memory configuration which is kept synchronized with
/// the persistent storage.
/// 
/// # Returns
/// 
/// The current `AppConfig` containing all configuration settings
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// const config = await invoke('get_config');
/// ```
#[tauri::command]
pub async fn get_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<AppConfig> {
    debug!("Getting current configuration");

    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config: {}", e);
        crate::errors::AppError::config_error("get_config", &format!("Failed to get configuration: {}", e))
    }))?;

    debug!("Successfully retrieved configuration");
    Ok(config)
}

/// Get the default application configuration
/// 
/// Returns a new `AppConfig` instance with all default values.
/// This is useful for frontend forms and reset operations.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// const defaultConfig = await invoke('get_default_config');
/// ```
#[tauri::command]
pub async fn get_default_config() -> CommandResult<AppConfig> {
    debug!("Getting default configuration");
    Ok(AppConfig::default())
}

/// Update the entire application configuration
/// 
/// This command performs a complete configuration update with validation
/// and persistence. It will validate the new configuration before saving
/// and broadcast change events to all subscribers.
/// 
/// # Arguments
/// 
/// * `new_config` - The complete new configuration to apply
/// 
/// # Validation
/// 
/// The configuration will be validated before saving. If validation fails,
/// the command will return an error and no changes will be persisted.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// await invoke('update_config', { newConfig: config });
/// ```
#[tauri::command]
pub async fn update_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    new_config: AppConfig,
) -> CommandResult<()> {
    debug!("Updating entire configuration");

    to_command_result(config_service.update_config(new_config).await.map_err(|e| {
        error!("Failed to update config: {}", e);
        crate::errors::AppError::config_error("update_config", &format!("Failed to update configuration: {}", e))
    }))?;

    info!("Configuration updated successfully");
    Ok(())
}

#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    debug!("Resetting configuration to defaults");

    to_command_result(config_service.reset_to_defaults().await.map_err(|e| {
        error!("Failed to reset config to defaults: {}", e);
          crate::errors::AppError::config_error("reset_to_defaults", &format!(
            "Failed to reset configuration to defaults: {}",
            e
        ))
    }))?;

    info!("Configuration reset to defaults successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting POE client log path");

    let path = to_command_result(config_service.get_poe_client_log_path().await.map_err(|e| {
        error!("Failed to get POE client log path: {}", e);
        crate::errors::AppError::config_error("get_poe_client_log_path", &format!("Failed to get POE client log path: {}", e))
    }))?;

    debug!("POE client log path: {}", path);
    Ok(path)
}

/// Set the Path of Exile client log file path
/// 
/// Updates the POE client log path setting with validation and persistence.
/// This setting determines where the application looks for POE client log files
/// to monitor game events.
/// 
/// # Arguments
/// 
/// * `path` - The file system path to the POE client log file
/// 
/// # Validation
/// 
/// The path will be validated to ensure it's not empty. Additional file
/// existence validation may be performed by the service layer.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// await invoke('set_poe_client_log_path', { path: '/path/to/Client.txt' });
/// ```
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
                  crate::errors::AppError::config_error("set_poe_client_log_path", &format!(
                    "Failed to set POE client log path: {}",
                    e
                ))
            }),
    )?;

    info!("POE client log path set to: {}", path);
    Ok(())
}

#[tauri::command]
pub async fn get_default_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting default POE client log path");

    let default_path = config_service.get_default_poe_client_log_path();
    debug!("Default POE client log path: {}", default_path);
    Ok(default_path)
}

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
                  crate::errors::AppError::config_error("reset_poe_client_log_path_to_default", &format!(
                    "Failed to reset POE client log path to default: {}",
                    e
                ))
            }),
    )?;

    info!("POE client log path reset to default successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    debug!("Getting current log level");

    let log_level = to_command_result(config_service.get_log_level().await.map_err(|e| {
        error!("Failed to get log level: {}", e);
        crate::errors::AppError::config_error("get_log_level", &format!("Failed to get log level: {}", e))
    }))?;

    debug!("Current log level: {}", log_level);
    Ok(log_level)
}

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
                crate::errors::AppError::config_error("set_log_level", &format!("Failed to set log level: {}", e))
            }),
    )?;

    info!("Log level set to: {}", level);
    Ok(())
}

#[tauri::command]
pub async fn get_config_file_info(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationFileInfo> {
    debug!("Getting configuration file information");

    let file_info = to_command_result(config_service.get_file_info().await.map_err(|e| {
        error!("Failed to get config file info: {}", e);
          crate::errors::AppError::config_error("get_config_file_info", &format!(
            "Failed to get configuration file information: {}",
            e
        ))
    }))?;

    debug!("Configuration file info retrieved successfully");
    Ok(file_info)
}

/// Validate the current configuration
/// 
/// Performs comprehensive validation of the current configuration state
/// and returns detailed validation results including any error messages.
/// 
/// This command is useful for frontend forms to provide real-time validation
/// feedback to users before they attempt to save changes.
/// 
/// # Returns
/// 
/// A `ConfigurationValidationResult` containing:
/// - `is_valid`: Boolean indicating if validation passed
/// - `errors`: Array of error messages (empty if validation passed)
/// 
/// # Validation Checks
/// 
/// - Log level must be one of the supported values
/// - POE client log path must not be empty
/// - Additional domain-specific validation rules
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// const result = await invoke('validate_config');
/// if (!result.is_valid) {
///   console.error('Validation errors:', result.errors);
/// }
/// ```
#[tauri::command]
pub async fn validate_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationValidationResult> {
    debug!("Validating current configuration");

    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config for validation: {}", e);
          crate::errors::AppError::config_error("get_config_for_validation", &format!(
            "Failed to get configuration for validation: {}",
            e
        ))
    }))?;

    let validation_result =
        to_command_result(config_service.validate_config(&config).await.map_err(|e| {
            error!("Failed to validate config: {}", e);
              crate::errors::AppError::config_error("validate_config", &format!(
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
