use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::AppError;
use crate::infrastructure::events::EventBus;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, EnrichedLocationState, EnrichedZoneStats, League, LocationState,
};
use super::traits::{CharacterRepository, CharacterService};
use crate::domain::zone_tracking::ZoneStats;

pub struct CharacterServiceImpl {
    repository: Arc<dyn CharacterRepository + Send + Sync>,
    event_bus: Arc<EventBus>,
    zone_tracking: Arc<dyn crate::domain::zone_tracking::ZoneTrackingService>,
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
}

impl CharacterServiceImpl {
    pub fn new(
        repository: Arc<dyn CharacterRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
        zone_tracking: Arc<dyn crate::domain::zone_tracking::ZoneTrackingService>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            zone_tracking,
            zone_config,
        }
    }
}

impl CharacterServiceImpl {
    pub async fn with_default_repository(
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Result<Self, AppError> {
        let data_dir = crate::infrastructure::file_management::AppPaths::ensure_data_dir().await?;
        let repository = Arc::new(super::repository::CharacterRepositoryImpl::new(
            data_dir.clone(),
        ));

        let zone_tracking = Arc::new(crate::domain::zone_tracking::ZoneTrackingServiceImpl::new());

        Ok(Self::new(repository, event_bus, zone_tracking, zone_config))
    }
}

#[async_trait]
impl CharacterService for CharacterServiceImpl {
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> Result<CharacterData, AppError> {
        if !super::models::is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
            ));
        }

        if !self.is_name_unique(&name, None).await? {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", name),
            ));
        }

        let character_id = uuid::Uuid::new_v4().to_string();
        let character_data = CharacterData::new(
            character_id,
            name.clone(),
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
        );

        self.repository.save_character_data(&character_data).await?;

        let mut index = self.repository.load_characters_index().await?;
        index.add_character(character_data.id.clone());

        if index.character_ids.len() == 1 {
            index.set_active_character(Some(character_data.id.clone()));
        }

        self.repository.save_characters_index(&index).await?;

        log::info!("Created new character: {}", name);
        Ok(character_data)
    }

    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
        self.repository.load_character_data(character_id).await
    }

    async fn get_character(&self, character_id: &str) -> Result<CharacterDataResponse, AppError> {
        let character_data = self.repository.load_character_data(character_id).await?;
        Ok(self.enrich_character_data(character_data).await)
    }

    async fn get_all_characters(&self) -> Result<Vec<CharacterDataResponse>, AppError> {
        let characters = self.repository.load_all_characters().await?;
        let mut enriched_characters = Vec::new();
        for character_data in characters {
            enriched_characters.push(self.enrich_character_data(character_data).await);
        }
        Ok(enriched_characters)
    }

    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> Result<CharacterData, AppError> {
        if !super::models::is_valid_ascendency_for_class(
            &update_params.ascendency,
            &update_params.class,
        ) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    update_params.ascendency, update_params.class
                ),
            ));
        }

        // Ensure new name is unique (excluding current character)
        if !self
            .is_name_unique(&update_params.name, Some(character_id))
            .await?
        {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", update_params.name),
            ));
        }

        let mut character_data = self.repository.load_character_data(character_id).await?;

        character_data.profile.name = update_params.name.clone();
        character_data.profile.class = update_params.class;
        character_data.profile.ascendency = update_params.ascendency;
        character_data.profile.league = update_params.league;
        character_data.profile.hardcore = update_params.hardcore;
        character_data.profile.solo_self_found = update_params.solo_self_found;
        character_data.profile.level = update_params.level;
        character_data.touch();

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        log::info!("Updated character: {}", update_params.name);
        Ok(character_data)
    }

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError> {
        let mut index = self.repository.load_characters_index().await?;

        // Remove character ID from index
        index.remove_character(character_id);

        // Save updated index
        self.repository.save_characters_index(&index).await?;
        self.repository.delete_character_data(character_id).await?;

        log::info!("Deleted character: {}", character_id);
        Ok(())
    }

    async fn set_active_character(&self, character_id: Option<&str>) -> Result<(), AppError> {
        // Load characters index
        let mut index = self.repository.load_characters_index().await?;

        // Validate character exists (if not None)
        if let Some(id) = character_id {
            if !index.has_character(id) {
                return Err(AppError::internal_error(
                    "set_active_character",
                    &format!("Character with ID '{}' not found", id),
                ));
            }
        }

        // Update active character ID
        index.set_active_character(character_id.map(|s| s.to_string()));

        // Save updated index
        self.repository.save_characters_index(&index).await?;

        if let Some(id) = character_id {
            log::info!("Set active character: {}", id);
        } else {
            log::info!("Cleared active character");
        }
        Ok(())
    }

    async fn get_active_character(&self) -> Result<Option<CharacterDataResponse>, AppError> {
        // Load characters index
        let index = self.repository.load_characters_index().await?;

        // Get active character ID
        if let Some(active_id) = &index.active_character_id {
            // Load active character data
            match self.repository.load_character_data(active_id).await {
                Ok(character) => {
                    let enriched = self.enrich_character_data(character).await;
                    Ok(Some(enriched))
                }
                Err(_) => {
                    // Character file might be missing, clear active character
                    let mut index = self.repository.load_characters_index().await?;
                    index.set_active_character(None);
                    self.repository.save_characters_index(&index).await?;
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError> {
        self.repository.load_characters_index().await
    }

    /// Validates that a character name is unique
    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
        // Load all characters
        let characters = self.repository.load_all_characters().await?;

        // Check if any character (excluding exclude_id) has the same name
        let is_unique = !characters.iter().any(|character| {
            character.profile.name == name
                && exclude_id.map_or(true, |exclude| character.id != exclude)
        });

        Ok(is_unique)
    }

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError> {
        // Load existing character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Update level
        character_data.profile.level = new_level;
        character_data.touch();

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        // Enrich character data before emitting event
        let enriched_data = self.enrich_character_data(character_data).await;

        // Emit character updated event
        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched_data,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!(
                "LEVEL UP: Failed to publish character tracking data updated event: {}",
                e
            );
        } else {
        }

        log::info!("Updated character {} level to {}", character_id, new_level);
        Ok(())
    }

    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> Result<Option<LocationState>, AppError> {
        let character_data = self.repository.load_character_data(character_id).await?;
        Ok(character_data.current_location)
    }

    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError> {
        self.repository.save_character_data(character_data).await
    }

    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Look up zone metadata to get act and is_town
        let (mut act, is_town) =
            if let Some(metadata) = self.zone_config.get_zone_metadata(zone_name).await {
                (Some(metadata.act), metadata.is_town)
            } else {
                (None, false)
            };

        // Override act to 10 for hideouts to separate them from act playtimes
        if zone_name.to_lowercase().contains("hideout") {
            act = Some(10);
        }

        // Apply zone tracking business logic
        self.zone_tracking
            .enter_zone(&mut character_data, zone_name, act, is_town)?;

        // Update current location (character identity concern)
        character_data.current_location = Some(LocationState::new(zone_name.to_string()));
        character_data.touch();

        // Save character data
        self.repository.save_character_data(&character_data).await?;

        // Enrich character data before emitting event
        let enriched_data = self.enrich_character_data(character_data).await;

        // Publish event
        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched_data,
        );
        self.event_bus.publish(event).await?;

        Ok(())
    }

    async fn record_death(&self, character_id: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Apply zone tracking business logic
        self.zone_tracking.record_death(&mut character_data)?;
        character_data.touch();

        // Save character data
        self.repository.save_character_data(&character_data).await?;

        // Enrich character data before emitting event
        let enriched_data = self.enrich_character_data(character_data).await;

        // Publish event
        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched_data,
        );
        self.event_bus.publish(event).await?;

        Ok(())
    }

    async fn finalize_all_active_zones(&self) -> Result<(), AppError> {
        let characters = self.repository.load_all_characters().await?;

        for mut character_data in characters {
            // Apply zone tracking business logic
            self.zone_tracking
                .finalize_active_zones(&mut character_data)?;
            character_data.touch();

            // Save character data
            self.repository.save_character_data(&character_data).await?;

            // Enrich character data before emitting event
            let enriched_data = self.enrich_character_data(character_data).await;

            // Publish event
            let event = crate::infrastructure::events::AppEvent::character_updated(
                enriched_data.id.clone(),
                enriched_data,
            );
            self.event_bus.publish(event).await?;
        }

        Ok(())
    }

    async fn sync_zone_metadata(&self, character_id: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Update metadata for each zone from current zone configuration
        let zone_names: Vec<String> = character_data
            .zones
            .iter()
            .map(|z| z.zone_name.clone())
            .collect();

        for zone_name in zone_names {
            let (mut act, is_town) =
                if let Some(metadata) = self.zone_config.get_zone_metadata(&zone_name).await {
                    (Some(metadata.act), metadata.is_town)
                } else {
                    (None, false)
                };

            // Override act to 10 for hideouts to separate them from act playtimes
            if zone_name.to_lowercase().contains("hideout") {
                act = Some(10);
            }

            self.zone_tracking
                .update_zone_metadata(&mut character_data, &zone_name, act, is_town);
        }

        character_data.touch();

        // Save character data
        self.repository.save_character_data(&character_data).await?;

        // Enrich character data before emitting event
        let enriched_data = self.enrich_character_data(character_data).await;

        // Publish event
        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched_data,
        );
        self.event_bus.publish(event).await?;

        Ok(())
    }
}

impl CharacterServiceImpl {
    /// Enriches character data with zone metadata for API responses
    async fn enrich_character_data(&self, character_data: CharacterData) -> CharacterDataResponse {
        let mut response = CharacterDataResponse::from(character_data.clone());

        // Enrich current location
        if let Some(ref location) = character_data.current_location {
            if let Some(zone_metadata) = self
                .zone_config
                .get_zone_metadata(&location.zone_name)
                .await
            {
                response.current_location = Some(
                    EnrichedLocationState::from_location_and_metadata(location, &zone_metadata),
                );
            } else {
                response.current_location =
                    Some(EnrichedLocationState::from_location_minimal(location));
            }
        }

        // Enrich zones
        let mut enriched_zones = Vec::new();
        for zone_stats in &response.zones {
            if let Some(zone_metadata) = self
                .zone_config
                .get_zone_metadata(&zone_stats.zone_name)
                .await
            {
                let base_zone = ZoneStats {
                    zone_name: zone_stats.zone_name.clone(),
                    duration: zone_stats.duration,
                    deaths: zone_stats.deaths,
                    visits: zone_stats.visits,
                    first_visited: zone_stats.first_visited,
                    last_visited: zone_stats.last_visited,
                    is_active: zone_stats.is_active,
                    entry_timestamp: zone_stats.entry_timestamp,
                    act: zone_stats.act,
                    is_town: zone_stats.is_town,
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
}
