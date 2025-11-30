use tauri::State;

use crate::domain::character::models::{
    CharacterData, CharacterDataResponse, EnrichedLocationState, EnrichedZoneStats,
};
use crate::domain::character::traits::CharacterService;
use crate::domain::zone_tracking::ZoneStats;
use crate::{to_command_result, CommandResult};

async fn enrich_character_data(
    character_data: CharacterData,
    zone_config: &dyn crate::domain::zone_configuration::traits::ZoneConfigurationService,
) -> CharacterDataResponse {
    let mut response = CharacterDataResponse::from(character_data.clone());

    if let Some(ref location) = character_data.current_location {
        if let Some(zone_metadata) = zone_config.get_zone_metadata(&location.zone_name).await {
            response.current_location = Some(EnrichedLocationState::from_location_and_metadata(
                location,
                &zone_metadata,
            ));
        } else {
            response.current_location =
                Some(EnrichedLocationState::from_location_minimal(location));
        }
    }

    let mut enriched_zones = Vec::new();
    for zone_stats in &response.zones {
        if let Some(zone_metadata) = zone_config.get_zone_metadata(&zone_stats.zone_name).await {
            let base_zone = ZoneStats {
                zone_name: zone_stats.zone_name.clone(),
                duration: zone_stats.duration,
                deaths: zone_stats.deaths,
                visits: zone_stats.visits,
                first_visited: zone_stats.first_visited,
                last_visited: zone_stats.last_visited,
                is_active: zone_stats.is_active,
                entry_timestamp: zone_stats.entry_timestamp,
            };
            enriched_zones.push(EnrichedZoneStats::from_stats_and_metadata(
                &base_zone,
                &zone_metadata,
            ));
        } else {
            enriched_zones.push(zone_stats.clone());
        }
    }

    response.zones = enriched_zones;
    response
}

#[tauri::command]
pub async fn create_character(
    name: String,
    class: crate::domain::character::models::CharacterClass,
    ascendency: crate::domain::character::models::Ascendency,
    league: crate::domain::character::models::League,
    hardcore: bool,
    solo_self_found: bool,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharacterData> {
    to_command_result(
        character_service
            .create_character(name, class, ascendency, league, hardcore, solo_self_found)
            .await,
    )
}

#[tauri::command]
pub async fn get_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
    zone_config_service: State<
        '_,
        std::sync::Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    >,
) -> CommandResult<crate::domain::character::models::CharacterDataResponse> {
    let character_data = to_command_result(character_service.get_character(&character_id).await)?;

    Ok(enrich_character_data(character_data, &**zone_config_service).await)
}

#[tauri::command]
pub async fn get_all_characters(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
    zone_config_service: State<
        '_,
        std::sync::Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    >,
) -> CommandResult<Vec<crate::domain::character::models::CharacterDataResponse>> {
    let characters = to_command_result(character_service.get_all_characters().await)?;

    let mut enriched_characters = Vec::new();
    for character_data in characters {
        enriched_characters
            .push(enrich_character_data(character_data, &**zone_config_service).await);
    }

    Ok(enriched_characters)
}

#[tauri::command]
pub async fn update_character(
    character_id: String,
    update_params: crate::domain::character::models::CharacterUpdateParams,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharacterData> {
    to_command_result(
        character_service
            .update_character(&character_id, update_params)
            .await,
    )
}

#[tauri::command]
pub async fn delete_character(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.delete_character(&character_id).await)
}

#[tauri::command]
pub async fn set_active_character(
    character_id: Option<String>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .set_active_character(character_id.as_deref())
            .await,
    )
}

#[tauri::command]
pub async fn get_active_character(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
    zone_config_service: State<
        '_,
        std::sync::Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    >,
) -> CommandResult<Option<crate::domain::character::models::CharacterDataResponse>> {
    match to_command_result(character_service.get_active_character().await)? {
        Some(character) => {
            let enriched = enrich_character_data(character, &**zone_config_service).await;
            Ok(Some(enriched))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn get_characters_index(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<crate::domain::character::models::CharactersIndex> {
    to_command_result(character_service.get_characters_index().await)
}

#[tauri::command]
pub async fn is_character_name_unique(
    name: String,
    exclude_id: Option<String>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<bool> {
    to_command_result(
        character_service
            .is_name_unique(&name, exclude_id.as_deref())
            .await,
    )
}

#[tauri::command]
pub async fn get_available_character_classes(
) -> CommandResult<Vec<crate::domain::character::models::CharacterClass>> {
    Ok(crate::domain::character::models::get_all_character_classes())
}

#[tauri::command]
pub async fn get_available_leagues() -> CommandResult<Vec<crate::domain::character::models::League>>
{
    Ok(crate::domain::character::models::get_all_leagues())
}

#[tauri::command]
pub async fn get_available_ascendencies_for_class(
    class: crate::domain::character::models::CharacterClass,
) -> CommandResult<Vec<crate::domain::character::models::Ascendency>> {
    Ok(crate::domain::character::models::get_ascendencies_for_class(&class))
}

#[tauri::command]
pub async fn get_character_tracking_data(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::CharacterData>> {
    let result = to_command_result(character_service.get_character(&character_id).await)?;
    Ok(Some(result))
}

#[tauri::command]
pub async fn get_character_current_location(
    character_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::character::models::LocationState>> {
    to_command_result(character_service.get_current_location(&character_id).await)
}

#[tauri::command]
pub async fn enter_zone(
    character_id: String,
    location_id: String,
    location_name: String,
    location_type: crate::domain::character::models::LocationType,
    act: Option<String>,
    is_town: bool,
    zone_level: Option<u32>,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .enter_zone(
                &character_id,
                location_id,
                location_name,
                location_type,
                act,
                is_town,
                zone_level,
            )
            .await,
    )
}

#[tauri::command]
pub async fn leave_zone(
    character_id: String,
    location_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .leave_zone(&character_id, &location_id)
            .await,
    )
}

#[tauri::command]
pub async fn record_death(
    character_id: String,
    location_id: String,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .record_death(&character_id, &location_id)
            .await,
    )
}

#[tauri::command]
pub async fn add_zone_time(
    character_id: String,
    location_id: String,
    seconds: u64,
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        character_service
            .add_zone_time(&character_id, &location_id, seconds)
            .await,
    )
}

#[tauri::command]
pub async fn finalize_all_active_zones(
    character_service: State<'_, Box<dyn CharacterService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(character_service.finalize_all_active_zones().await)
}
