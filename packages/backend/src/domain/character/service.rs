use async_trait::async_trait;
use chrono::Utc;
use log::{debug, error, info, warn};
use std::sync::Arc;

use crate::domain::log_analysis::models::SceneChangeEvent;
use crate::errors::AppError;
use crate::infrastructure::events::EventBus;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, EnrichedZoneStats, League, LocationState, LocationType, ZoneStats,
};
use super::traits::{CharacterRepository, CharacterService};

/// Implementation of the CharacterService trait.
///
/// This service provides business logic for character management using the new
/// consolidated data model. It coordinates between the repository layer and
/// enforces business rules. Includes character tracking functionality for
/// monitoring zone visits, time spent, and deaths.
pub struct CharacterServiceImpl {
    /// Repository for character data persistence
    repository: Arc<dyn CharacterRepository + Send + Sync>,
    /// Event bus for publishing character tracking events
    event_bus: Arc<EventBus>,
    /// Zone configuration service for act and town detection
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    /// Wiki scraping service for fetching zone data
    wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
    /// Configuration service for application settings
    config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
}

impl CharacterServiceImpl {
    /// Creates a new CharacterService instance with tracking dependencies
    pub fn new(
        repository: Arc<dyn CharacterRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            zone_config,
            wiki_service,
            config_service,
        }
    }

    /// Triggers background wiki fetch for a zone
    async fn trigger_wiki_fetch(&self, zone_name: &str) {
        info!("Triggering wiki fetch for zone: {}", zone_name);
        let wiki_service = self.wiki_service.clone();
        let zone_config = self.zone_config.clone();
        let zone_name = zone_name.to_string();

        // Spawn background task for wiki fetch
        tokio::spawn(async move {
            info!("Starting wiki fetch for zone: {}", zone_name);
            match wiki_service.fetch_zone_data(&zone_name).await {
                Ok(wiki_data) => {
                    info!(
                        "Successfully fetched wiki data for zone '{}': act={}, level={:?}, town={}",
                        zone_name, wiki_data.act, wiki_data.area_level, wiki_data.is_town
                    );

                    // Get the area_id from the wiki data or generate it from zone name
                    let _area_id = wiki_data
                        .area_id
                        .as_ref()
                        .map(|id| id.clone())
                        .unwrap_or_else(|| {
                            // Generate area_id from zone name if not provided by wiki
                            zone_name
                                .to_lowercase()
                                .replace(' ', "_")
                                .replace('-', "_")
                                .chars()
                                .filter(|c| c.is_alphanumeric() || *c == '_')
                                .collect::<String>()
                                .trim_matches('_')
                                .to_string()
                        });

                    // Update zone metadata with wiki data using area_id
                    info!("Looking up zone '{}' in configuration...", zone_name);
                    if let Some(zone_metadata) = zone_config.get_zone_metadata(&zone_name).await {
                        info!(
                            "Found zone '{}' in configuration, updating with wiki data",
                            zone_name
                        );
                        let mut updated_metadata = zone_metadata;
                        updated_metadata.update_from_wiki_data(&wiki_data);
                        if let Err(e) = zone_config.update_zone(updated_metadata).await {
                            error!(
                                "Failed to update zone '{}' with wiki data: {}",
                                zone_name, e
                            );
                        } else {
                            info!("Successfully updated zone '{}' with wiki data", zone_name);
                        }
                    } else {
                        error!(
                            "Zone '{}' not found in configuration after wiki fetch",
                            zone_name
                        );
                        info!("Available zones in configuration:");
                        // TODO: Add debug logging to list available zones
                    }
                }
                Err(e) => {
                    error!("Failed to fetch wiki data for zone '{}': {}", zone_name, e);
                }
            }
        });
    }

    /// Enters a zone using zone metadata
    async fn enter_zone_with_metadata(
        &self,
        character_id: &str,
        zone_metadata: &crate::domain::zone_configuration::models::ZoneMetadata,
        _zone_level: Option<u32>,
    ) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Deactivate any currently active zone
        if let Some(active_zone) = character_data.get_active_zone() {
            let mut deactivated_zone = active_zone.clone();
            let _time_spent = deactivated_zone.stop_timer_and_add_time();
            deactivated_zone.deactivate();
            character_data.upsert_zone(deactivated_zone);
        }

        // Create or update the new zone using area_id
        let area_id = zone_metadata
            .area_id
            .clone()
            .unwrap_or_else(|| zone_metadata.zone_name.clone());

        if let Some(existing_zone) = character_data.find_zone_by_area_id(&area_id) {
            // Update existing zone
            let mut zone = existing_zone.clone();
            zone.activate();
            zone.start_timer();
            character_data.upsert_zone(zone);
        } else {
            // Create new zone
            let mut zone = ZoneStats::new(area_id);
            zone.activate();
            zone.start_timer();
            character_data.upsert_zone(zone);
        }

        // Update current location
        character_data.current_location = Some(LocationState::new_for_location(
            Some(zone_metadata.zone_name.clone()),
            Some(zone_metadata.act.to_string()),
            zone_metadata.is_town,
            if zone_metadata.is_town {
                LocationType::Zone
            } else {
                LocationType::Zone
            },
        ));

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        info!(
            "Character {} entered zone '{}' (act {})",
            character_id, zone_metadata.zone_name, zone_metadata.act
        );
        Ok(())
    }
}

impl CharacterServiceImpl {
    /// Creates a new CharacterService instance with default repository and dependencies
    pub async fn with_default_repository(
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
    ) -> Result<Self, AppError> {
        // Create data directory path using proper XDG data directory
        let data_dir = crate::infrastructure::file_management::AppPaths::ensure_data_dir().await?;

        // Create repository
        let repository = Arc::new(super::repository::CharacterRepositoryImpl::new(data_dir));

        // Create service
        Ok(Self::new(
            repository,
            event_bus,
            zone_config,
            wiki_service,
            config_service,
        ))
    }
}

#[async_trait]
impl CharacterService for CharacterServiceImpl {
    /// Creates a new character with the provided data
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> Result<CharacterData, AppError> {
        // Validate ascendency-class combination
        if !super::models::is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
            ));
        }

        // Ensure character name is unique
        if !self.is_name_unique(&name, None).await? {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", name),
            ));
        }

        // Generate UUID for character ID
        let character_id = uuid::Uuid::new_v4().to_string();

        // Create CharacterData instance
        let character_data = CharacterData::new(
            character_id,
            name.clone(),
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
        );

        // Save character data file
        self.repository.save_character_data(&character_data).await?;

        // Update characters index
        let mut index = self.repository.load_characters_index().await?;
        index.add_character(character_data.id.clone());

        // If this is the first character, set it as active
        if index.character_ids.len() == 1 {
            index.set_active_character(Some(character_data.id.clone()));
        }

        self.repository.save_characters_index(&index).await?;

        log::info!("Created new character: {}", name);
        Ok(character_data)
    }

    /// Gets a character by ID
    async fn get_character(&self, character_id: &str) -> Result<CharacterData, AppError> {
        self.repository.load_character_data(character_id).await
    }

    /// Gets all characters
    async fn get_all_characters(&self) -> Result<Vec<CharacterData>, AppError> {
        self.repository.load_all_characters().await
    }

    /// Gets a character by ID with enriched zone data for frontend
    async fn get_character_response(
        &self,
        character_id: &str,
    ) -> Result<CharacterDataResponse, AppError> {
        let character_data = self.repository.load_character_data(character_id).await?;
        let mut response = CharacterDataResponse::from(character_data);

        // Enrich zone stats with zone metadata
        let mut enriched_zones = Vec::new();
        for zone_stats in &response.zones {
            if let Some(zone_metadata) = self
                .zone_config
                .get_zone_metadata(&zone_stats.area_id)
                .await
            {
                // Convert EnrichedZoneStats back to ZoneStats for the method call
                let base_zone = ZoneStats {
                    area_id: zone_stats.area_id.clone(),
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
                // Keep the minimal enriched zone from the conversion
                enriched_zones.push(zone_stats.clone());
            }
        }

        response.zones = enriched_zones;
        Ok(response)
    }

    /// Gets all characters with enriched zone data for frontend
    async fn get_all_characters_response(&self) -> Result<Vec<CharacterDataResponse>, AppError> {
        let characters = self.repository.load_all_characters().await?;
        let mut enriched_characters = Vec::new();

        for character_data in characters {
            let mut response = CharacterDataResponse::from(character_data);

            // Enrich zone stats with zone metadata
            let mut enriched_zones = Vec::new();
            for zone_stats in &response.zones {
                if let Some(zone_metadata) = self
                    .zone_config
                    .get_zone_metadata(&zone_stats.area_id)
                    .await
                {
                    // Convert EnrichedZoneStats back to ZoneStats for the method call
                    let base_zone = ZoneStats {
                        area_id: zone_stats.area_id.clone(),
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
                    // Keep the minimal enriched zone from the conversion
                    enriched_zones.push(zone_stats.clone());
                }
            }

            response.zones = enriched_zones;
            enriched_characters.push(response);
        }

        Ok(enriched_characters)
    }

    /// Updates an existing character
    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> Result<CharacterData, AppError> {
        // Validate new ascendency-class combination
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

        // Load existing character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Update character data
        character_data.name = update_params.name.clone();
        character_data.class = update_params.class;
        character_data.ascendency = update_params.ascendency;
        character_data.league = update_params.league;
        character_data.hardcore = update_params.hardcore;
        character_data.solo_self_found = update_params.solo_self_found;
        character_data.level = update_params.level;
        character_data.touch();

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        log::info!("Updated character: {}", update_params.name);
        Ok(character_data)
    }

    /// Deletes a character
    async fn delete_character(&self, character_id: &str) -> Result<(), AppError> {
        // Load characters index
        let mut index = self.repository.load_characters_index().await?;

        // Remove character ID from index
        index.remove_character(character_id);

        // Save updated index
        self.repository.save_characters_index(&index).await?;

        // Delete character data file
        self.repository.delete_character_data(character_id).await?;

        log::info!("Deleted character: {}", character_id);
        Ok(())
    }

    /// Sets the active character
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

    /// Gets the currently active character
    async fn get_active_character(&self) -> Result<Option<CharacterData>, AppError> {
        // Load characters index
        let index = self.repository.load_characters_index().await?;

        // Get active character ID
        if let Some(active_id) = &index.active_character_id {
            // Load active character data
            match self.repository.load_character_data(active_id).await {
                Ok(character) => Ok(Some(character)),
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

    /// Gets the characters index
    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError> {
        self.repository.load_characters_index().await
    }

    /// Validates that a character name is unique
    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
        // Load all characters
        let characters = self.repository.load_all_characters().await?;

        // Check if any character (excluding exclude_id) has the same name
        let is_unique = !characters.iter().any(|character| {
            character.name == name && exclude_id.map_or(true, |exclude| character.id != exclude)
        });

        Ok(is_unique)
    }

    /// Updates a character's level
    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError> {
        // Load existing character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Update level
        character_data.level = new_level;
        character_data.touch();

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        // Emit character tracking data updated event
        let event = crate::infrastructure::events::AppEvent::character_tracking_data_updated(
            character_id.to_string(),
            character_data,
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

    // Character Tracking Methods Implementation

    /// Processes raw scene content and returns a scene change event if detected
    async fn process_scene_content(
        &self,
        content: &str,
        character_id: &str,
    ) -> Result<Option<SceneChangeEvent>, AppError> {
        // Process scene change without zone level information
        self.process_scene_change(content, character_id, None).await
    }

    /// Processes raw scene content with zone level and returns a scene change event if detected
    async fn process_scene_content_with_zone_level(
        &self,
        content: &str,
        character_id: &str,
        zone_level: u32,
    ) -> Result<Option<SceneChangeEvent>, AppError> {
        // Process scene change with zone level information
        self.process_scene_change(content, character_id, Some(zone_level))
            .await
    }

    /// Gets the current location state for a character
    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> Result<Option<LocationState>, AppError> {
        // Load character data and return current location
        let character_data = self.repository.load_character_data(character_id).await?;
        Ok(character_data.current_location)
    }

    /// Saves character data directly (used by other services)
    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError> {
        self.repository.save_character_data(character_data).await
    }

    /// Enters a zone for a character (legacy method - use enter_zone_with_metadata instead)
    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        _location_type: LocationType,
        _act: Option<String>,
        _is_town: bool,
        _zone_level: Option<u32>,
    ) -> Result<(), AppError> {
        // This is a legacy method that's no longer used
        // The new system uses enter_zone_with_metadata instead
        warn!("Legacy enter_zone method called - this should be replaced with enter_zone_with_metadata");

        // For now, just create a basic zone entry
        let mut character_data = self.repository.load_character_data(character_id).await?;

        // Deactivate any currently active zone
        if let Some(active_zone) = character_data.get_active_zone() {
            let mut deactivated_zone = active_zone.clone();
            let _time_spent = deactivated_zone.stop_timer_and_add_time();
            deactivated_zone.deactivate();
            character_data.upsert_zone(deactivated_zone);
        }

        // Create new zone with just the area_id
        let mut zone = ZoneStats::new(location_id.clone());
        zone.activate();
        zone.start_timer();
        character_data.upsert_zone(zone);

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;

        info!(
            "Character {} entered zone '{}' (legacy method)",
            character_id, location_name
        );
        Ok(())
    }

    /// Leaves a zone for a character
    async fn leave_zone(&self, character_id: &str, location_id: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        if let Some(zone) = character_data.find_zone_mut(location_id) {
            let was_active = zone.is_active;
            let _time_spent = zone.stop_timer_and_add_time();
            zone.deactivate();
            character_data.update_summary();

            // Clear current location if this was the active zone
            if was_active {
                character_data.current_location = None;
            }

            // Save updated character data
            self.repository.save_character_data(&character_data).await?;
        }

        Ok(())
    }

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        if let Some(zone) = character_data.find_zone_mut(location_id) {
            zone.record_death();

            character_data.update_summary();

            // Save updated character data
            self.repository.save_character_data(&character_data).await?;

            info!(
                "Character {} died in zone {} (total deaths: {})",
                character_id, location_id, character_data.summary.total_deaths
            );
        } else {
        }

        Ok(())
    }

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        if let Some(zone) = character_data.find_zone_mut(location_id) {
            zone.add_time(seconds);
            character_data.update_summary();

            // Save updated character data
            self.repository.save_character_data(&character_data).await?;
        }

        Ok(())
    }

    /// Finalizes all active zones (stops timers and saves data)
    async fn finalize_all_active_zones(&self) -> Result<(), AppError> {
        // Load all characters and finalize their active zones
        let characters = self.repository.load_all_characters().await?;

        for mut character_data in characters {
            let mut has_changes = false;

            // Find and finalize any active zones
            for zone in &mut character_data.zones {
                if zone.is_active {
                    let _time_spent = zone.stop_timer_and_add_time();
                    zone.deactivate();
                    has_changes = true;
                }
            }

            // Update summary if there were changes
            if has_changes {
                character_data.update_summary();
                self.repository.save_character_data(&character_data).await?;
            }
        }

        Ok(())
    }
}

impl CharacterServiceImpl {
    /// Determines if a scene name is an act name that should be filtered out
    /// Act names should not be tracked as zones as they represent story progression, not playable areas
    fn is_act_name(&self, scene_name: &str) -> bool {
        let lower_name = scene_name.to_lowercase();

        // Check for exact act name matches
        let act_names = ["act 1", "act 2", "act 3", "act 4", "interlude", "endgame"];

        if act_names.iter().any(|act| lower_name == *act) {
            return true;
        }

        // Check for act keywords that indicate act transitions
        let act_keywords = ["act", "endgame", "interlude", "atlas"];
        act_keywords
            .iter()
            .any(|keyword| lower_name.contains(keyword))
    }

    /// Helper method to process scene changes with optional zone level
    async fn process_scene_change(
        &self,
        content: &str,
        character_id: &str,
        _zone_level: Option<u32>,
    ) -> Result<Option<SceneChangeEvent>, AppError> {
        let zone_name = content.trim();

        if zone_name.is_empty() {
            return Ok(None);
        }

        // Filter out act names - these should not be tracked as zones
        if self.is_act_name(zone_name) {
            debug!(
                "SCENE FILTER: Filtering out act name '{}' - not tracking as zone",
                zone_name
            );
            return Ok(None);
        }

        // Check if zone exists in configuration, if not create placeholder and trigger wiki fetch
        let zone_metadata =
            if let Some(metadata) = self.zone_config.get_zone_metadata(zone_name).await {
                metadata
            } else {
                // Create placeholder zone metadata
                let mut placeholder =
                    crate::domain::zone_configuration::models::ZoneMetadata::placeholder(
                        zone_name.to_string(),
                    );

                // Try to determine act from context or default to 0 (unknown)
                // This could be enhanced with better heuristics
                placeholder.act = 0;

                // Add to zone configuration
                if let Err(e) = self.zone_config.add_zone(placeholder.clone()).await {
                    debug!("Failed to add placeholder zone '{}': {}", zone_name, e);
                }

                // Trigger background wiki fetch
                self.trigger_wiki_fetch(zone_name).await;

                placeholder
            };

        // Check if zone needs refresh based on configured interval
        let refresh_interval = self
            .config_service
            .get_zone_refresh_interval()
            .await
            .unwrap_or_default()
            .to_seconds();

        if zone_metadata.needs_refresh(refresh_interval) {
            self.trigger_wiki_fetch(zone_name).await;
        }

        // Enter the zone using area_id
        self.enter_zone_with_metadata(character_id, &zone_metadata, _zone_level)
            .await?;

        // Load character data to get the updated current_location and update last played timestamp
        let mut character_data = self.repository.load_character_data(character_id).await?;
        character_data.last_played = Some(Utc::now());
        character_data.touch();
        self.repository.save_character_data(&character_data).await?;

        // Create scene change event
        let scene_change_event =
            SceneChangeEvent::Zone(crate::domain::log_analysis::models::ZoneChangeEvent {
                zone_name: zone_name.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });

        // Emit character tracking data updated event
        let event = crate::infrastructure::events::AppEvent::character_tracking_data_updated(
            character_id.to_string(),
            character_data,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!(
                "SCENE CHANGE: Failed to publish character tracking data updated event: {}",
                e
            );
        } else {
        }

        // Get zone metadata for logging
        let zone_metadata = self.zone_config.get_zone_metadata(zone_name).await;
        let act_info = zone_metadata
            .as_ref()
            .map(|z| z.act.to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let is_town_info = zone_metadata.map(|z| z.is_town).unwrap_or(false);

        info!(
            "Scene change: character {} entered '{}' (Act: {}, Town: {})",
            character_id, zone_name, act_info, is_town_info
        );

        Ok(Some(scene_change_event))
    }
}
