use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::AppError;
use crate::infrastructure::events::EventBus;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, CleanupStrategy, EnrichedLocationState, EnrichedZoneStats, League,
    LocationState, OrphanCleanupReport,
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

        // TRANSACTION SAFETY: Write index FIRST, then character file
        // This prevents orphaned files - if index write fails, no file is created
        // If file write fails, we roll back the index entry

        // Step 1: Update index with new character ID
        let mut index = self.repository.load_characters_index().await?;
        let is_first_character = index.character_ids.is_empty();
        index.add_character(character_data.id.clone());

        if is_first_character {
            index.set_active_character(Some(character_data.id.clone()));
        }

        // Step 2: Save index (if this fails, no character file exists - clean state)
        self.repository.save_characters_index(&index).await?;
        log::debug!("Index updated with new character ID: {}", character_data.id);

        // Step 3: Write character file (if this fails, rollback index)
        match self.repository.save_character_data(&character_data).await {
            Ok(_) => {
                log::info!("Created new character: {}", name);
                Ok(character_data)
            }
            Err(e) => {
                // ROLLBACK: Character file write failed, remove from index
                log::error!(
                    "Failed to write character file for {}, rolling back index: {}",
                    character_data.id,
                    e
                );

                // Re-load index to ensure we have latest state (handles concurrent modifications)
                let mut rollback_index = self.repository.load_characters_index().await?;
                rollback_index.remove_character(&character_data.id);

                // Clear active character if it was the one we're rolling back
                if rollback_index.active_character_id.as_ref() == Some(&character_data.id) {
                    rollback_index.set_active_character(None);
                }

                // Save cleaned index
                if let Err(rollback_err) =
                    self.repository.save_characters_index(&rollback_index).await
                {
                    log::error!(
                        "CRITICAL: Rollback failed for character {}: {}. Manual cleanup may be required.",
                        character_data.id,
                        rollback_err
                    );
                } else {
                    log::info!(
                        "Successfully rolled back index for failed character creation: {}",
                        character_data.id
                    );
                }

                // Return original error
                Err(e)
            }
        }
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
        Ok(character_data)
    }

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError> {
        // Delete file first - if this fails, index remains consistent
        self.repository.delete_character_data(character_id).await?;

        // Then update index
        let mut index = self.repository.load_characters_index().await?;
        index.remove_character(character_id);
        self.repository.save_characters_index(&index).await?;

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
            let active_id_owned = active_id.clone();
            // Load active character data
            match self.repository.load_character_data(&active_id_owned).await {
                Ok(character) => {
                    let enriched = self.enrich_character_data(character).await;
                    Ok(Some(enriched))
                }
                Err(_) => {
                    // Character file might be missing, reload index and clear only if same character is still active
                    let mut fresh_index = self.repository.load_characters_index().await?;
                    if fresh_index.active_character_id.as_ref() == Some(&active_id_owned) {
                        fresh_index.set_active_character(None);
                        self.repository.save_characters_index(&fresh_index).await?;
                        log::warn!(
                            "Cleared missing active character from index: {}",
                            active_id_owned
                        );
                    }
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
    /// Optimized: Uses index to load characters individually and stops early on match
    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError> {
        // Load index first (lightweight - just IDs)
        let index = self.repository.load_characters_index().await?;

        // Check each character individually, stopping early if match found
        for character_id in &index.character_ids {
            // Skip the excluded character (e.g., when updating an existing character)
            if exclude_id == Some(character_id.as_str()) {
                continue;
            }

            // Load only this character's data
            if let Ok(character) = self.repository.load_character_data(character_id).await {
                if character.profile.name == name {
                    return Ok(false); // Name already exists
                }
            }
        }

        Ok(true) // Name is unique
    }

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError> {
        // Validate level is within valid range (1-100)
        if new_level < 1 || new_level > 100 {
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

    async fn reconcile_character_storage(
        &self,
        strategy: CleanupStrategy,
    ) -> Result<OrphanCleanupReport, AppError> {
        log::info!(
            "Starting character storage reconciliation with {:?} strategy",
            strategy
        );

        // Load current index and filesystem state
        let mut index = self.repository.load_characters_index().await?;
        let files_on_disk = self.repository.list_character_data_files().await?;

        let mut orphaned_files = Vec::new();
        let mut missing_files = Vec::new();

        // Find files on disk not in index (orphans)
        for file_id in &files_on_disk {
            if !index.has_character(file_id) {
                orphaned_files.push(file_id.clone());
            }
        }

        // Find IDs in index with no file (reverse orphans)
        for index_id in &index.character_ids {
            if !files_on_disk.contains(index_id) {
                missing_files.push(index_id.clone());
            }
        }

        log::info!(
            "Found {} orphaned files and {} missing files",
            orphaned_files.len(),
            missing_files.len()
        );

        // Handle orphaned files based on strategy
        for orphan_id in &orphaned_files {
            match strategy {
                CleanupStrategy::Conservative => {
                    // Try to load the file and add to index if valid
                    match self.repository.load_character_data(orphan_id).await {
                        Ok(character_data) => {
                            index.add_character(orphan_id.clone());
                            log::info!(
                                "Added orphaned character '{}' ({}) back to index",
                                character_data.profile.name,
                                orphan_id
                            );
                        }
                        Err(e) => {
                            log::warn!(
                                "Failed to load orphaned file {}, deleting: {}",
                                orphan_id,
                                e
                            );
                            self.repository.delete_character_data(orphan_id).await?;
                        }
                    }
                }
                CleanupStrategy::Aggressive => {
                    // Delete orphaned file
                    self.repository.delete_character_data(orphan_id).await?;
                    log::info!("Deleted orphaned character file: {}", orphan_id);
                }
            }
        }

        // Always remove missing files from index (they don't exist)
        for missing_id in &missing_files {
            index.remove_character(missing_id);
            log::info!("Removed missing character {} from index", missing_id);
        }

        // Save reconciled index if there were any changes
        if !orphaned_files.is_empty() || !missing_files.is_empty() {
            self.repository.save_characters_index(&index).await?;
            log::info!("Saved reconciled character index");
        }

        let report = OrphanCleanupReport {
            orphaned_files,
            missing_files,
            total_characters: index.character_ids.len(),
            strategy,
        };

        log::info!(
            "Character storage reconciliation complete: {} total characters",
            report.total_characters
        );

        Ok(report)
    }
}

/// Hideout act number constant - used to separate hideout time from act playtimes
const HIDEOUT_ACT: u32 = 10;

/// Checks if a zone name indicates a hideout
/// Centralizes hideout detection logic to avoid duplication (Issue #41)
fn is_hideout_zone(zone_name: &str) -> bool {
    zone_name.to_lowercase().contains("hideout")
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
                    &base_zone,
                    &metadata,
                ));
            } else {
                enriched_zones.push(zone_stats.clone());
            }
        }

        response.zones = enriched_zones;
        response
    }
}
