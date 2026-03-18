use crate::domain::zone_configuration::models::ZoneMetadata;
use crate::domain::zone_configuration::service::ZoneConfigurationServiceImpl;
use crate::domain::zone_configuration::traits::ZoneConfigurationService;
use crate::CommandResult;
use std::sync::Arc;
use tauri::State;

/// Tauri command to get zone metadata by zone name.
///
/// Returns metadata for a zone from the global zone configuration cache,
/// regardless of which character has visited it. If no metadata exists
/// for the given zone name, returns None.
#[tauri::command]
pub async fn get_zone_metadata_by_name(
    zone_name: String,
    zone_config_service: State<'_, Arc<ZoneConfigurationServiceImpl>>,
) -> CommandResult<Option<ZoneMetadata>> {
    Ok(zone_config_service.get_zone_metadata(&zone_name).await)
}
