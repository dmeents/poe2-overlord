use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::debug;
use std::sync::Arc;

use crate::domain::events::EventBus;
use crate::domain::log_analysis::models::SceneChangeEvent;
use crate::errors::AppError;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterUpdateParams, CharactersIndex, League,
    LocationState, LocationType, ZoneStats,
};
use super::traits::{CharacterRepository, CharacterService};

/// Implementation of the CharacterService trait.
///
/// This service provides business logic for character management using the new
/// consolidated data model. It coordinates between the repository layer and
/// enforces business rules. Now includes character tracking functionality.
pub struct CharacterServiceImpl {
    /// Repository for character data persistence
    repository: Arc<dyn CharacterRepository + Send + Sync>,
    /// Event bus for publishing character tracking events
    event_bus: Arc<EventBus>,
    /// Zone configuration service for act and town detection
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    /// Tracks when the PoE process started for time calculations
    poe_process_start_time: Arc<tokio::sync::RwLock<Option<DateTime<Utc>>>>,
}

impl CharacterServiceImpl {
    /// Creates a new CharacterService instance with tracking dependencies
    pub fn new(
        repository: Arc<dyn CharacterRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            zone_config,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }
}

impl CharacterServiceImpl {
    /// Creates a new CharacterService instance with default repository and dependencies
    pub fn with_default_repository(
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Result<Self, AppError> {
        // Create data directory path using proper XDG data directory
        let data_dir = crate::infrastructure::persistence::DirectoryManager::ensure_data_directory(
        )
        .map_err(|e| {
            AppError::file_system_error(
                "ensure_data_directory",
                &format!("Failed to ensure data directory: {}", e),
            )
        })?;

        // Create repository
        let repository = Arc::new(super::repository::CharacterRepositoryImpl::new(data_dir)?);

        // Create service
        Ok(Self::new(repository, event_bus, zone_config))
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
                return Err(AppError::character_management_error(
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

    /// Enters a zone for a character
    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
        is_town: bool,
    ) -> Result<(), AppError> {
        debug!(
            "🔍 ENTER ZONE: Starting zone entry for character '{}'",
            character_id
        );
        debug!(
            "🔍 ENTER ZONE: Zone details - ID: '{}', Name: '{}', Type: {:?}, Act: {:?}, IsTown: {}",
            location_id, location_name, location_type, act, is_town
        );

        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;
        debug!(
            "🔍 ENTER ZONE: Character data loaded, currently has {} zones",
            character_data.zones.len()
        );

        // Deactivate any currently active zone
        if let Some(active_zone) = character_data.get_active_zone() {
            debug!(
                "🔍 ENTER ZONE: Deactivating previous active zone: '{}' (ID: '{}')",
                active_zone.location_name, active_zone.location_id
            );

            let mut deactivated_zone = active_zone.clone();
            let time_spent = deactivated_zone.stop_timer_and_add_time();
            deactivated_zone.deactivate();
            let zone_name = active_zone.location_name.clone();
            character_data.upsert_zone(deactivated_zone);

            debug!(
                "✅ ENTER ZONE: Character {} spent {} seconds in previous zone '{}'",
                character_id, time_spent, zone_name
            );
        } else {
            debug!("🔍 ENTER ZONE: No previous active zone to deactivate");
        }

        // Create or update the new zone
        debug!(
            "🔍 ENTER ZONE: Creating or updating zone stats for '{}'",
            location_name
        );

        // Check if zone already exists
        if let Some(existing_zone) = character_data.find_zone_mut(&location_id) {
            debug!(
                "🔍 ENTER ZONE: Updating existing zone '{}' (duration: {}s)",
                location_name, existing_zone.duration
            );
            // Update existing zone properties but preserve duration and other stats
            existing_zone.location_name = location_name.clone();
            existing_zone.location_type = location_type.clone();
            existing_zone.act = act.clone();
            existing_zone.is_town = is_town;
            existing_zone.activate();
            existing_zone.start_timer();
            debug!(
                "✅ ENTER ZONE: Existing zone '{}' updated and activated (preserved {}s duration)",
                location_name, existing_zone.duration
            );
            // Update summary after releasing the mutable reference
            character_data.update_summary();
        } else {
            debug!(
                "🔍 ENTER ZONE: Creating new zone stats for '{}'",
                location_name
            );
            let mut zone = ZoneStats::new(
                location_id.clone(),
                location_name.clone(),
                location_type.clone(),
                act.clone(),
                is_town,
            );
            zone.activate();
            zone.start_timer();
            character_data.upsert_zone(zone);
            debug!(
                "✅ ENTER ZONE: New zone '{}' created and activated",
                location_name
            );
        }

        // Update current location with the new zone information
        character_data.current_location = Some(LocationState::new_for_location(
            Some(location_name.clone()),
            act,
            is_town,
            location_type.clone(),
        ));
        debug!(
            "✅ ENTER ZONE: Current location updated to '{}'",
            location_name
        );

        // Save updated character data
        self.repository.save_character_data(&character_data).await?;
        debug!("✅ ENTER ZONE: Character data saved successfully");

        log::debug!(
            "✅ ENTER ZONE: Character {} successfully entered zone '{}' ({}) - Total zones: {}",
            character_id,
            location_name,
            location_type,
            character_data.zones.len()
        );
        Ok(())
    }

    /// Leaves a zone for a character
    async fn leave_zone(&self, character_id: &str, location_id: &str) -> Result<(), AppError> {
        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;

        if let Some(zone) = character_data.find_zone_mut(location_id) {
            let was_active = zone.is_active;
            let time_spent = zone.stop_timer_and_add_time();
            zone.deactivate();
            character_data.update_summary();

            // Clear current location if this was the active zone
            if was_active {
                character_data.current_location = None;
            }

            // Save updated character data
            self.repository.save_character_data(&character_data).await?;

            log::debug!(
                "Character {} left zone {} after spending {} seconds",
                character_id,
                location_id,
                time_spent
            );
        }

        Ok(())
    }

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> Result<(), AppError> {
        debug!(
            "🔍 RECORD DEATH: Starting death recording for character '{}' in zone '{}'",
            character_id, location_id
        );

        // Load character data
        let mut character_data = self.repository.load_character_data(character_id).await?;
        debug!(
            "🔍 RECORD DEATH: Character data loaded, has {} zones",
            character_data.zones.len()
        );

        if let Some(zone) = character_data.find_zone_mut(location_id) {
            debug!(
                "✅ RECORD DEATH: Found zone '{}' (name: '{}'), current deaths: {}",
                location_id, zone.location_name, zone.deaths
            );

            zone.record_death();
            debug!(
                "✅ RECORD DEATH: Death recorded, new death count: {}",
                zone.deaths
            );

            character_data.update_summary();
            debug!(
                "🔍 RECORD DEATH: Summary updated, total deaths: {}",
                character_data.summary.total_deaths
            );

            // Save updated character data
            self.repository.save_character_data(&character_data).await?;
            debug!("✅ RECORD DEATH: Character data saved successfully");

            log::debug!(
                "✅ RECORD DEATH: Successfully recorded death for character {} in zone {} (total deaths: {})",
                character_id,
                location_id,
                character_data.summary.total_deaths
            );
        } else {
            debug!(
                "❌ RECORD DEATH: Zone '{}' not found for character '{}'",
                location_id, character_id
            );
            debug!(
                "🔍 RECORD DEATH: Available zones: {:?}",
                character_data
                    .zones
                    .iter()
                    .map(|z| (z.location_id.clone(), z.location_name.clone()))
                    .collect::<Vec<_>>()
            );
            debug!("❌ RECORD DEATH: Death will be lost - no zone found to record it in!");
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
                    let time_spent = zone.stop_timer_and_add_time();
                    zone.deactivate();
                    has_changes = true;

                    log::debug!(
                        "Finalized zone {} for character {} with {} seconds",
                        zone.location_name,
                        character_data.id,
                        time_spent
                    );
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

    /// Starts frontend event emission for character tracking events
    async fn start_frontend_event_emission(&self, _window: tauri::WebviewWindow) {
        // For now, this is a placeholder implementation
        // The actual event emission logic will be implemented when we fully integrate
        // the tracking functionality
        log::debug!("Character tracking frontend event emission started (placeholder)");
    }
}

impl CharacterServiceImpl {
    /// Generates a proper location ID in the format {zone|hideout|town}_{zone_name}
    /// where zone_name is converted to snake case
    fn generate_location_id(&self, zone_name: &str, is_town: bool) -> String {
        // Convert zone name to snake case
        let zone_slug = self.to_snake_case(zone_name);

        // Determine the prefix based on zone type
        let prefix = if is_town {
            "town"
        } else if self.is_hideout_zone(zone_name) {
            "hideout"
        } else {
            "zone"
        };

        format!("{}_{}", prefix, zone_slug)
    }

    /// Converts a zone name to snake case (all lower, underscores instead of spaces)
    fn to_snake_case(&self, input: &str) -> String {
        input
            .split_whitespace()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Determines if a zone is a hideout based on zone name patterns
    fn is_hideout_zone(&self, zone_name: &str) -> bool {
        let lower_name = zone_name.to_lowercase();
        lower_name.contains("hideout") || lower_name.contains("hide")
    }

    /// Helper method to process scene changes with optional zone level
    async fn process_scene_change(
        &self,
        content: &str,
        character_id: &str,
        zone_level: Option<u32>,
    ) -> Result<Option<SceneChangeEvent>, AppError> {
        let zone_name = content.trim();
        debug!(
            "🔍 SCENE CHANGE: Processing scene change for character '{}'",
            character_id
        );
        debug!("🔍 SCENE CHANGE: Raw content: '{}'", content);
        debug!("🔍 SCENE CHANGE: Zone name: '{}'", zone_name);

        if zone_name.is_empty() {
            debug!("❌ SCENE CHANGE: Empty scene content, ignoring");
            return Ok(None);
        }

        // Look up zone information
        debug!(
            "🔍 SCENE CHANGE: Looking up zone information for '{}'",
            zone_name
        );
        let act_name = self
            .zone_config
            .get_act_for_zone(zone_name)
            .await
            .unwrap_or_else(|| "Endgame".to_string());
        let is_town = self.zone_config.is_town_zone(zone_name).await;
        debug!(
            "🔍 SCENE CHANGE: Zone lookup result - Act: '{}', IsTown: {}",
            act_name, is_town
        );

        // Determine location type (all zones are LocationType::Zone, towns are just marked with is_town flag)
        let location_type = LocationType::Zone;

        // Create proper location ID in format {zone|hideout|town}_{zone_name}
        let location_id = self.generate_location_id(zone_name, is_town);
        debug!("🔍 SCENE CHANGE: Generated location ID: '{}'", location_id);

        // Enter the zone
        debug!(
            "🔍 SCENE CHANGE: Calling enter_zone for character '{}'",
            character_id
        );
        self.enter_zone(
            character_id,
            location_id,
            zone_name.to_string(),
            location_type,
            Some(act_name.clone()),
            is_town,
        )
        .await?;
        debug!("✅ SCENE CHANGE: Zone entry completed successfully");

        // Load character data to get the updated current_location and update last played timestamp
        let mut character_data = self.repository.load_character_data(character_id).await?;
        character_data.last_played = Some(Utc::now());
        character_data.touch();
        self.repository.save_character_data(&character_data).await?;
        debug!("✅ SCENE CHANGE: Character data updated and saved");

        // Create scene change event
        let scene_change_event =
            SceneChangeEvent::Zone(crate::domain::log_analysis::models::ZoneChangeEvent {
                zone_name: zone_name.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });

        // Emit character tracking data updated event
        let event = crate::domain::events::event_types::AppEvent::character_tracking_data_updated(
            character_id.to_string(),
            character_data,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            log::warn!(
                "❌ SCENE CHANGE: Failed to publish character tracking data updated event: {}",
                e
            );
        } else {
            debug!("✅ SCENE CHANGE: Character tracking data updated event published");
        }

        log::debug!(
            "✅ SCENE CHANGE: Successfully processed scene change for character {}: '{}' (Act: {}, Town: {}, Level: {:?})",
            character_id,
            zone_name,
            act_name,
            is_town,
            zone_level
        );

        Ok(Some(scene_change_event))
    }
}
