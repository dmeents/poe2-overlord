use async_trait::async_trait;

use crate::errors::AppError;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterUpdateParams, CharactersIndex, League,
    LocationState, LocationType,
};

/// Repository trait for character data persistence operations.
///
/// This trait defines the contract for persisting and retrieving character data
/// using the new consolidated file structure (characters.json + character_data_{id}.json).
#[async_trait]
pub trait CharacterRepository {
    /// Loads the characters index from the characters.json file
    async fn load_characters_index(&self) -> Result<CharactersIndex, AppError>;

    /// Saves the characters index to the characters.json file
    async fn save_characters_index(&self, index: &CharactersIndex) -> Result<(), AppError>;

    /// Loads character data from a character_data_{id}.json file
    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError>;

    /// Saves character data to a character_data_{id}.json file
    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError>;

    /// Deletes a character data file
    async fn delete_character_data(&self, character_id: &str) -> Result<(), AppError>;

    /// Loads all character data files
    async fn load_all_characters(&self) -> Result<Vec<CharacterData>, AppError>;

    /// Checks if a character data file exists
    async fn character_exists(&self, character_id: &str) -> Result<bool, AppError>;
}

/// Service trait for character business logic operations.
///
/// This trait defines the contract for character management business logic
/// using the new consolidated data model.
#[async_trait]
pub trait CharacterService: Send + Sync {
    /// Creates a new character with the provided data
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> Result<CharacterData, AppError>;

    /// Gets a character by ID
    async fn get_character(&self, character_id: &str) -> Result<CharacterData, AppError>;

    /// Gets all characters
    async fn get_all_characters(&self) -> Result<Vec<CharacterData>, AppError>;

    /// Updates an existing character
    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> Result<CharacterData, AppError>;

    /// Deletes a character
    async fn delete_character(&self, character_id: &str) -> Result<(), AppError>;

    /// Sets the active character
    async fn set_active_character(&self, character_id: Option<&str>) -> Result<(), AppError>;

    /// Gets the currently active character
    async fn get_active_character(&self) -> Result<Option<CharacterData>, AppError>;

    /// Gets the characters index
    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError>;

    /// Validates that a character name is unique
    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError>;

    /// Updates a character's level
    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError>;

    // Character Tracking Methods (merged from CharacterTrackingService)

    /// Processes raw scene content and returns a scene change event if detected
    /// Returns None if no actual scene change occurred
    async fn process_scene_content(
        &self,
        content: &str,
        character_id: &str,
    ) -> Result<Option<crate::domain::log_analysis::models::SceneChangeEvent>, AppError>;

    /// Processes raw scene content with zone level and returns a scene change event if detected
    /// Returns None if no actual scene change occurred
    async fn process_scene_content_with_zone_level(
        &self,
        content: &str,
        character_id: &str,
        zone_level: u32,
    ) -> Result<Option<crate::domain::log_analysis::models::SceneChangeEvent>, AppError>;

    /// Gets the current location state for a character (scene, act, timestamps)
    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> Result<Option<LocationState>, AppError>;

    /// Enters a zone for a character
    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
        is_town: bool,
        zone_level: Option<u32>,
    ) -> Result<(), AppError>;

    /// Leaves a zone for a character
    async fn leave_zone(&self, character_id: &str, location_id: &str) -> Result<(), AppError>;

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> Result<(), AppError>;

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> Result<(), AppError>;

    /// Finalizes all active zones (stops timers and saves data)
    async fn finalize_all_active_zones(&self) -> Result<(), AppError>;
}
