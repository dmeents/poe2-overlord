use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult, ZoneRefreshInterval,
};
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::configuration::traits::ConfigurationService;
use crate::{to_command_result, CommandResult};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<AppConfig> {
    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config: {}", e);
        crate::errors::AppError::internal_error(
            "get_config",
            &format!("Failed to get configuration: {}", e),
        )
    }))?;

    Ok(config)
}

#[tauri::command]
pub async fn get_default_config() -> CommandResult<AppConfig> {
    Ok(AppConfig::default())
}

#[tauri::command]
pub async fn update_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    new_config: AppConfig,
) -> CommandResult<()> {
    to_command_result(config_service.update_config(new_config).await.map_err(|e| {
        error!("Failed to update config: {}", e);
        crate::errors::AppError::internal_error(
            "update_config",
            &format!("Failed to update configuration: {}", e),
        )
    }))?;

    info!("Configuration updated successfully");
    Ok(())
}

#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    to_command_result(config_service.reset_to_defaults().await.map_err(|e| {
        error!("Failed to reset config to defaults: {}", e);
        crate::errors::AppError::internal_error(
            "reset_to_defaults",
            &format!("Failed to reset configuration to defaults: {}", e),
        )
    }))?;

    info!("Configuration reset to defaults successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    let path = to_command_result(config_service.get_poe_client_log_path().await.map_err(|e| {
        error!("Failed to get POE client log path: {}", e);
        crate::errors::AppError::internal_error(
            "get_poe_client_log_path",
            &format!("Failed to get POE client log path: {}", e),
        )
    }))?;

    Ok(path)
}

#[tauri::command]
pub async fn set_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    path: String,
) -> CommandResult<()> {
    to_command_result(
        config_service
            .set_poe_client_log_path(path.clone())
            .await
            .map_err(|e| {
                error!("Failed to set POE client log path: {}", e);
                crate::errors::AppError::internal_error(
                    "set_poe_client_log_path",
                    &format!("Failed to set POE client log path: {}", e),
                )
            }),
    )?;

    info!("POE client log path set to: {}", path);
    Ok(())
}

#[tauri::command]
pub async fn get_default_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    let default_path = config_service.get_default_poe_client_log_path();
    Ok(default_path)
}

#[tauri::command]
pub async fn reset_poe_client_log_path_to_default(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    to_command_result(
        config_service
            .reset_poe_client_log_path_to_default()
            .await
            .map_err(|e| {
                error!("Failed to reset POE client log path to default: {}", e);
                crate::errors::AppError::internal_error(
                    "reset_poe_client_log_path_to_default",
                    &format!("Failed to reset POE client log path to default: {}", e),
                )
            }),
    )?;

    info!("POE client log path reset to default successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    let log_level = to_command_result(config_service.get_log_level().await.map_err(|e| {
        error!("Failed to get log level: {}", e);
        crate::errors::AppError::internal_error(
            "get_log_level",
            &format!("Failed to get log level: {}", e),
        )
    }))?;

    Ok(log_level)
}

#[tauri::command]
pub async fn set_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    level: String,
) -> CommandResult<()> {
    to_command_result(
        config_service
            .set_log_level(level.clone())
            .await
            .map_err(|e| {
                error!("Failed to set log level: {}", e);
                crate::errors::AppError::internal_error(
                    "set_log_level",
                    &format!("Failed to set log level: {}", e),
                )
            }),
    )?;

    info!("Log level set to: {}", level);
    Ok(())
}

#[tauri::command]
pub async fn get_config_file_info(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationFileInfo> {
    let file_info = to_command_result(config_service.get_file_info().await.map_err(|e| {
        error!("Failed to get config file info: {}", e);
        crate::errors::AppError::internal_error(
            "get_config_file_info",
            &format!("Failed to get configuration file information: {}", e),
        )
    }))?;

    Ok(file_info)
}

#[tauri::command]
pub async fn validate_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationValidationResult> {
    let config = to_command_result(config_service.get_config().await.map_err(|e| {
        error!("Failed to get config for validation: {}", e);
        crate::errors::AppError::internal_error(
            "get_config_for_validation",
            &format!("Failed to get configuration for validation: {}", e),
        )
    }))?;

    let validation_result =
        to_command_result(config_service.validate_config(&config).await.map_err(|e| {
            error!("Failed to validate config: {}", e);
            crate::errors::AppError::internal_error(
                "validate_config",
                &format!("Failed to validate configuration: {}", e),
            )
        }))?;

    Ok(validation_result)
}

#[tauri::command]
pub async fn get_zone_refresh_interval(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ZoneRefreshInterval> {
    let interval = to_command_result(config_service.get_zone_refresh_interval().await.map_err(
        |e| {
            error!("Failed to get zone refresh interval: {}", e);
            crate::errors::AppError::internal_error(
                "get_zone_refresh_interval",
                &format!("Failed to get zone refresh interval: {}", e),
            )
        },
    ))?;

    Ok(interval)
}

#[tauri::command]
pub async fn set_zone_refresh_interval(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    interval: ZoneRefreshInterval,
) -> CommandResult<()> {
    to_command_result(
        config_service
            .set_zone_refresh_interval(interval)
            .await
            .map_err(|e| {
                error!("Failed to set zone refresh interval: {}", e);
                crate::errors::AppError::internal_error(
                    "set_zone_refresh_interval",
                    &format!("Failed to set zone refresh interval: {}", e),
                )
            }),
    )?;

    info!("Zone refresh interval set to: {}", interval);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRefreshIntervalOption {
    pub value: String,
    pub label: String,
    pub seconds: i64,
}

#[tauri::command]
pub async fn get_zone_refresh_interval_options() -> CommandResult<Vec<ZoneRefreshIntervalOption>> {
    let options = ZoneRefreshInterval::all_options()
        .into_iter()
        .map(|interval| ZoneRefreshIntervalOption {
            value: format!("{:?}", interval),
            label: interval.label().to_string(),
            seconds: interval.to_seconds(),
        })
        .collect();

    Ok(options)
}
