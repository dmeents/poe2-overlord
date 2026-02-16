use crate::domain::configuration::models::{
    AppConfig, ConfigurationFileInfo, ConfigurationValidationResult, ZoneRefreshInterval,
};
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::configuration::traits::ConfigurationService;
use crate::{to_command_result, CommandResult};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<AppConfig> {
    to_command_result(config_service.get_config().await)
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
    let result = to_command_result(config_service.update_config(new_config).await)?;
    info!("Configuration updated successfully");
    Ok(result)
}

#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    let result = to_command_result(config_service.reset_to_defaults().await)?;
    info!("Configuration reset to defaults successfully");
    Ok(result)
}

#[tauri::command]
pub async fn get_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    to_command_result(config_service.get_poe_client_log_path().await)
}

#[tauri::command]
pub async fn set_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    path: String,
) -> CommandResult<()> {
    let result = to_command_result(config_service.set_poe_client_log_path(path.clone()).await)?;
    info!("POE client log path set to: {}", path);
    Ok(result)
}

#[tauri::command]
pub async fn get_default_poe_client_log_path(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    let default_path = config_service.get_default_poe_client_log_path().await;
    Ok(default_path)
}

#[tauri::command]
pub async fn reset_poe_client_log_path_to_default(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<()> {
    let result = to_command_result(config_service.reset_poe_client_log_path_to_default().await)?;
    info!("POE client log path reset to default successfully");
    Ok(result)
}

#[tauri::command]
pub async fn get_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<String> {
    to_command_result(config_service.get_log_level().await)
}

#[tauri::command]
pub async fn set_log_level(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    level: String,
) -> CommandResult<()> {
    let result = to_command_result(config_service.set_log_level(level.clone()).await)?;
    info!("Log level set to: {}", level);
    Ok(result)
}

#[tauri::command]
pub async fn get_config_file_info(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationFileInfo> {
    to_command_result(config_service.get_file_info().await)
}

#[tauri::command]
pub async fn validate_config(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ConfigurationValidationResult> {
    let config = to_command_result(config_service.get_config().await)?;
    to_command_result(config_service.validate_config(&config).await)
}

#[tauri::command]
pub async fn get_zone_refresh_interval(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
) -> CommandResult<ZoneRefreshInterval> {
    to_command_result(config_service.get_zone_refresh_interval().await)
}

#[tauri::command]
pub async fn set_zone_refresh_interval(
    config_service: State<'_, Arc<ConfigurationServiceImpl>>,
    interval: ZoneRefreshInterval,
) -> CommandResult<()> {
    let result = to_command_result(config_service.set_zone_refresh_interval(interval).await)?;
    info!("Zone refresh interval set to: {}", interval);
    Ok(result)
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
