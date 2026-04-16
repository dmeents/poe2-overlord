use crate::domain::configuration::models::{AppConfig, BackgroundImage, ZoneRefreshInterval};
use crate::domain::configuration::traits::ConfigurationService;
use crate::{to_command_result, CommandResult};
use log::info;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_config(
    config_service: State<'_, Arc<dyn ConfigurationService + Send + Sync>>,
) -> CommandResult<AppConfig> {
    to_command_result(config_service.get_config().await)
}

#[tauri::command]
pub async fn get_default_config() -> CommandResult<AppConfig> {
    Ok(AppConfig::default())
}

#[tauri::command]
pub async fn update_config(
    config_service: State<'_, Arc<dyn ConfigurationService + Send + Sync>>,
    new_config: AppConfig,
) -> CommandResult<()> {
    to_command_result(config_service.update_config(new_config).await)
}

#[tauri::command]
pub async fn reset_config_to_defaults(
    config_service: State<'_, Arc<dyn ConfigurationService + Send + Sync>>,
) -> CommandResult<()> {
    info!("Configuration reset to defaults successfully");
    to_command_result(config_service.reset_to_defaults().await)
}

#[tauri::command]
pub async fn get_zone_refresh_interval(
    config_service: State<'_, Arc<dyn ConfigurationService + Send + Sync>>,
) -> CommandResult<ZoneRefreshInterval> {
    to_command_result(config_service.get_zone_refresh_interval().await)
}

#[tauri::command]
pub async fn set_zone_refresh_interval(
    config_service: State<'_, Arc<dyn ConfigurationService + Send + Sync>>,
    interval: ZoneRefreshInterval,
) -> CommandResult<()> {
    info!("Zone refresh interval set to: {interval}");
    to_command_result(config_service.set_zone_refresh_interval(interval).await)
}

#[tauri::command]
pub async fn get_zone_refresh_interval_options(
) -> CommandResult<Vec<crate::domain::configuration::models::ZoneRefreshIntervalOption>> {
    use crate::domain::configuration::models::ZoneRefreshIntervalOption;
    let options = ZoneRefreshInterval::all_options()
        .into_iter()
        .map(|interval| ZoneRefreshIntervalOption {
            value: format!("{interval:?}"),
            label: interval.label().to_string(),
            seconds: interval.to_seconds(),
        })
        .collect();

    Ok(options)
}

#[tauri::command]
pub async fn get_background_image_options(
) -> CommandResult<Vec<crate::domain::configuration::models::BackgroundImageOption>> {
    use crate::domain::configuration::models::BackgroundImageOption;
    let options = BackgroundImage::all_options()
        .into_iter()
        .map(|bg| BackgroundImageOption {
            value: format!("{bg:?}"),
            label: bg.label().to_string(),
            filename: bg.filename().map(|s| s.to_string()),
        })
        .collect();

    Ok(options)
}
