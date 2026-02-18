use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::AppError;
use crate::infrastructure::events::EventBus;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, EnrichedLocationState, EnrichedZoneStats, League, LocationState,
};
use super::traits::{CharacterRepository, CharacterService};
use crate::domain::zone_tracking::{is_hideout_zone, ZoneStats, HIDEOUT_ACT};

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
    ) -> Result<CharacterDataResponse, AppError> {
        if !super::models::is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
            ));
        }

        if self.repository.is_name_taken(&name, None).await? {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", name),
            ));
        }

        let character_id = uuid::Uuid::new_v4().to_string();
        let character_data = CharacterData::new(
            character_id.clone(),
            name.clone(),
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
        );

        // Check if this is the first character
        let is_first_character = self.repository.get_character_ids().await?.is_empty();

        // Insert character
        self.repository.save_character_data(&character_data).await?;

        // Set as active if first character
        if is_first_character {
            self.repository
                .set_active_character(Some(&character_id))
                .await?;
        }

        log::info!("Created new character: {}", name);
        Ok(self.enrich_character_data(character_data).await)
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
    ) -> Result<CharacterDataResponse, AppError> {
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

        // Validate level is within valid range (1-100)
        if update_params.level < 1 || update_params.level > 100 {
            return Err(AppError::validation_error(
                "validate_level",
                &format!(
                    "Character level must be between 1 and 100, got {}",
                    update_params.level
                ),
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
        Ok(self.enrich_character_data(character_data).await)
    }

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError> {
        // DELETE CASCADE handles zone_stats and walkthrough_progress cleanup
        self.repository.delete_character_data(character_id).await?;

        // Publish character deleted event for frontend reactivity
        let event =
            crate::infrastructure::events::AppEvent::character_deleted(character_id.to_string());
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!(
                "Failed to publish character deleted event: {}. UI may show stale data.",
                e
            );
        }

        log::info!("Deleted character: {}", character_id);
        Ok(())
    }

    async fn set_active_character(&self, character_id: Option<&str>) -> Result<(), AppError> {
        // Validation (character exists) is handled by repository
        self.repository.set_active_character(character_id).await?;

        if let Some(id) = character_id {
            log::info!("Set active character: {}", id);
        } else {
            log::info!("Cleared active character");
        }
        Ok(())
    }

    async fn get_active_character(&self) -> Result<Option<CharacterDataResponse>, AppError> {
        let active_id = self.repository.get_active_character_id().await?;

        if let Some(id) = active_id {
            let character = self.repository.load_character_data(&id).await?;
            let enriched = self.enrich_character_data(character).await;
            Ok(Some(enriched))
        } else {
            Ok(None)
        }
    }

    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError> {
        let character_ids = self.repository.get_character_ids().await?;
        let active_character_id = self.repository.get_active_character_id().await?;
        Ok(CharactersIndex {
            character_ids,
            active_character_id,
        })
    }

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
        let is_taken = self.repository.is_name_taken(name, exclude_id).await?;
        Ok(!is_taken)
    }

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError> {
        // Validate level is within valid range (1-100)
        if !(1..=100).contains(&new_level) {
            return Err(AppError::validation_error(
                "validate_level",
                &format!(
                    "Character level must be between 1 and 100, got {}",
                    new_level
                ),
            ));
        }

        // Load existing character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Update level
        character_data.profile.level = new_level;
        character_data.touch();

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        // Enrich character data before emitting event
        let enriched_data = self.enrich_character_data(character_data).await;

        // Emit character updated event - log warning but don't fail the operation
        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched_data,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!(
                "Failed to publish character level update event: {}. UI may show stale data.",
                e
            );
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

        // Override act for hideouts to separate from act playtimes
        if is_hideout_zone(zone_name) {
            act = Some(HIDEOUT_ACT);
        }

        // Apply zone tracking business logic
        self.zone_tracking
            .enter_zone(&mut character_data, zone_name, act, is_town)?;

        // Update current location (character identity concern)
        character_data.current_location = Some(LocationState::new(zone_name.to_string()));
        character_data.timestamps.last_played = Some(chrono::Utc::now());
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

    async fn leave_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError> {
        log::debug!("Character {} leaving zone: {}", character_id, zone_name);

        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Apply zone tracking business logic to leave zone
        self.zone_tracking
            .leave_zone(&mut character_data, zone_name)?;

        // Update timestamps
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

        log::info!("Character {} left zone: {}", character_id, zone_name);

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

        log::info!(
            "finalize_all_active_zones called - found {} characters to process",
            characters.len()
        );

        for mut character_data in characters {
            log::info!(
                "Processing character {} for zone finalization",
                character_data.id
            );

            // Apply zone tracking business logic
            self.zone_tracking
                .finalize_active_zones(&mut character_data)?;

            // Clear current_location to make stale state more explicit
            if character_data.current_location.is_some() {
                log::info!(
                    "Clearing current_location for character {} during finalization",
                    character_data.id
                );
                character_data.current_location = None;
            }

            character_data.touch();

            // Save character data
            self.repository.save_character_data(&character_data).await?;
            log::info!(
                "Saved character data for {} after zone finalization",
                character_data.id
            );

            // Enrich character data before emitting event
            let enriched_data = self.enrich_character_data(character_data).await;

            // Publish event
            let character_id = enriched_data.id.clone();
            let event = crate::infrastructure::events::AppEvent::character_updated(
                enriched_data.id.clone(),
                enriched_data,
            );
            self.event_bus.publish(event).await?;
            log::info!("Published character_updated event for {}", character_id);
        }

        log::info!("finalize_all_active_zones completed successfully");
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

            // Override act for hideouts to separate from act playtimes
            if is_hideout_zone(&zone_name) {
                act = Some(HIDEOUT_ACT);
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
    /// Optimized: Loads zone config once and uses for all lookups (Issue #38)
    async fn enrich_character_data(&self, character_data: CharacterData) -> CharacterDataResponse {
        let mut response = CharacterDataResponse::from(character_data.clone());

        // Load zone configuration once for all lookups
        let zone_config = self.zone_config.load_configuration().await.ok();

        // Enrich current location
        if let Some(ref location) = character_data.current_location {
            let zone_metadata = zone_config
                .as_ref()
                .and_then(|c| c.get_zone_by_name(&location.zone_name).cloned());

            response.current_location = Some(if let Some(metadata) = zone_metadata {
                EnrichedLocationState::from_location_and_metadata(location, &metadata)
            } else {
                EnrichedLocationState::from_location_minimal(location)
            });
        }

        // Enrich zones using the already-loaded config
        let mut enriched_zones = Vec::new();
        for zone_stats in &response.zones {
            let zone_metadata = zone_config
                .as_ref()
                .and_then(|c| c.get_zone_by_name(&zone_stats.zone_name).cloned());

            if let Some(metadata) = zone_metadata {
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
                    &base_zone, &metadata,
                ));
            } else {
                enriched_zones.push(zone_stats.clone());
            }
        }

        response.zones = enriched_zones;
        response
    }
}
