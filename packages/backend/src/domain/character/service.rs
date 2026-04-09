use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::AppError;
use crate::infrastructure::events::EventBus;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterProfile,
    CharacterSummaryResponse, CharacterUpdateParams, EnrichedLocationState, EnrichedZoneStats,
    League, LocationState,
};
use super::traits::{CharacterRepository, CharacterService};
use crate::domain::walkthrough::models::WalkthroughProgress;

pub struct CharacterServiceImpl {
    repository: Arc<dyn CharacterRepository + Send + Sync>,
    event_bus: Arc<EventBus>,
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
}

impl CharacterServiceImpl {
    pub fn new(
        repository: Arc<dyn CharacterRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Self {
        Self {
            repository,
            event_bus,
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

        let is_first_character = self.repository.get_character_ids().await?.is_empty();

        self.repository.save_character_data(&character_data).await?;

        if is_first_character {
            self.repository
                .set_active_character(Some(&character_id))
                .await?;
        }

        log::info!("Created new character: {}", name);
        Ok(self.enrich_character_data(character_data).await)
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

    async fn get_all_characters_summary(&self) -> Result<Vec<CharacterSummaryResponse>, AppError> {
        let character_data_list = self.repository.load_all_characters_summary().await?;

        // Get active character ID once for all characters
        let active_id = self.repository.get_active_character_id().await?;

        let mut summaries = Vec::new();
        for character_data in character_data_list {
            let is_active = active_id.as_deref() == Some(&character_data.id);

            let current_location = match character_data.current_location.as_ref() {
                None => None,
                Some(location) => {
                    let zone_metadata = self
                        .zone_config
                        .get_zone_metadata(&location.zone_name)
                        .await;
                    Some(if let Some(metadata) = zone_metadata {
                        EnrichedLocationState::from_location_and_metadata(location, &metadata)
                    } else {
                        EnrichedLocationState::from_location_minimal(location)
                    })
                }
            };

            summaries.push(CharacterSummaryResponse {
                id: character_data.id,
                name: character_data.profile.name,
                class: character_data.profile.class,
                ascendency: character_data.profile.ascendency,
                league: character_data.profile.league,
                hardcore: character_data.profile.hardcore,
                solo_self_found: character_data.profile.solo_self_found,
                level: character_data.profile.level,
                created_at: character_data.timestamps.created_at,
                last_played: character_data.timestamps.last_played,
                last_updated: character_data.timestamps.last_updated,
                current_location,
                summary: character_data.summary,
                walkthrough_progress: character_data.walkthrough_progress,
                is_active,
            });
        }
        Ok(summaries)
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

        if !self
            .is_name_unique(&update_params.name, Some(character_id))
            .await?
        {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", update_params.name),
            ));
        }

        if update_params.level < 1 || update_params.level > 100 {
            return Err(AppError::validation_error(
                "validate_level",
                &format!(
                    "Character level must be between 1 and 100, got {}",
                    update_params.level
                ),
            ));
        }

        let new_profile = CharacterProfile {
            name: update_params.name.clone(),
            class: update_params.class,
            ascendency: update_params.ascendency,
            league: update_params.league,
            hardcore: update_params.hardcore,
            solo_self_found: update_params.solo_self_found,
            level: update_params.level,
        };

        // Targeted SQL update — single statement instead of load+mutate+save
        self.repository
            .update_character_profile(character_id, &new_profile)
            .await?;

        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched.clone(),
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!("Failed to publish character update event: {}", e);
        }

        log::info!("Updated character: {}", update_params.name);
        Ok(enriched)
    }

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError> {
        self.repository.delete_character_data(character_id).await?;

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

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
        let is_taken = self.repository.is_name_taken(name, exclude_id).await?;
        Ok(!is_taken)
    }

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError> {
        if !(1..=100).contains(&new_level) {
            return Err(AppError::validation_error(
                "validate_level",
                &format!(
                    "Character level must be between 1 and 100, got {}",
                    new_level
                ),
            ));
        }

        // Targeted SQL update — single statement instead of load+mutate+save
        self.repository
            .update_character_level(character_id, new_level)
            .await?;

        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched,
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

    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError> {
        // Atomically transitions zone: stops old active zone timer, starts new zone timer
        self.repository
            .transition_zone(character_id, zone_name)
            .await?;

        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!("Failed to publish character event: {}", e);
        }

        Ok(())
    }

    async fn record_death(&self, character_id: &str) -> Result<(), AppError> {
        // Targeted SQL update — single statement instead of load+mutate+save
        self.repository
            .record_death_in_active_zone(character_id)
            .await?;

        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!("Failed to publish character event: {}", e);
        }

        Ok(())
    }

    async fn finalize_all_active_zones(&self) -> Result<(), AppError> {
        let character_ids = self.repository.get_character_ids().await?;

        log::info!(
            "finalize_all_active_zones called - found {} characters to process",
            character_ids.len()
        );

        for character_id in character_ids {
            self.repository
                .finalize_character_active_zones(&character_id)
                .await?;

            let character_data = self.repository.load_character_data(&character_id).await?;
            let enriched = self.enrich_character_data(character_data).await;

            let event = crate::infrastructure::events::AppEvent::character_updated(
                character_id.clone(),
                enriched,
            );
            if let Err(e) = self.event_bus.publish(event).await {
                log::warn!("Failed to publish character event: {}", e);
            }
            log::info!("Published character_updated event for {}", character_id);
        }

        log::info!("finalize_all_active_zones completed successfully");
        Ok(())
    }

    async fn sync_zone_metadata(&self, character_id: &str) -> Result<(), AppError> {
        // Zone metadata (act, is_town) comes from zone_metadata table via JOIN at load time.
        // Re-loading and re-publishing is sufficient to sync the frontend.
        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!("Failed to publish character event: {}", e);
        }

        Ok(())
    }

    async fn update_walkthrough_progress(
        &self,
        character_id: &str,
        progress: &WalkthroughProgress,
    ) -> Result<CharacterDataResponse, AppError> {
        // Targeted SQL upsert
        self.repository
            .update_walkthrough_progress(character_id, progress)
            .await?;

        let character_data = self.repository.load_character_data(character_id).await?;
        let enriched = self.enrich_character_data(character_data).await;

        let event = crate::infrastructure::events::AppEvent::character_updated(
            character_id.to_string(),
            enriched.clone(),
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!("Failed to publish character update event: {}", e);
        }

        Ok(enriched)
    }

    async fn get_character_zones(
        &self,
        character_id: &str,
    ) -> Result<Vec<EnrichedZoneStats>, AppError> {
        self.repository.get_character_zones(character_id).await
    }

    async fn has_character_visited_zone(
        &self,
        character_id: &str,
        zone_name: &str,
    ) -> Result<bool, AppError> {
        self.repository
            .has_character_visited_zone(character_id, zone_name)
            .await
    }
}

impl CharacterServiceImpl {
    /// Enriches character data with zone metadata for API responses.
    async fn enrich_character_data(&self, character_data: CharacterData) -> CharacterDataResponse {
        // Targeted lookup for current location only — no full config load
        let current_location = match character_data.current_location.as_ref() {
            None => None,
            Some(location) => {
                let zone_metadata = self
                    .zone_config
                    .get_zone_metadata(&location.zone_name)
                    .await;
                Some(if let Some(metadata) = zone_metadata {
                    EnrichedLocationState::from_location_and_metadata(location, &metadata)
                } else {
                    EnrichedLocationState::from_location_minimal(location)
                })
            }
        };

        CharacterDataResponse {
            id: character_data.id,
            name: character_data.profile.name,
            class: character_data.profile.class,
            ascendency: character_data.profile.ascendency,
            league: character_data.profile.league,
            hardcore: character_data.profile.hardcore,
            solo_self_found: character_data.profile.solo_self_found,
            level: character_data.profile.level,
            created_at: character_data.timestamps.created_at,
            last_played: character_data.timestamps.last_played,
            last_updated: character_data.timestamps.last_updated,
            current_location,
            summary: character_data.summary,
            walkthrough_progress: character_data.walkthrough_progress,
        }
    }
}
