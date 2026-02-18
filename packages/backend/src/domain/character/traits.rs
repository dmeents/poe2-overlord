use async_trait::async_trait;

use crate::errors::AppResult;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, League, LocationState,
};

#[async_trait]
pub trait CharacterRepository {
    async fn load_character_data(&self, character_id: &str) -> AppResult<CharacterData>;

    async fn save_character_data(&self, character_data: &CharacterData) -> AppResult<()>;

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;

    async fn load_all_characters(&self) -> AppResult<Vec<CharacterData>>;

    async fn character_exists(&self, character_id: &str) -> AppResult<bool>;

    /// Sets the active character by ID. Pass None to deactivate all characters.
    /// Returns an error if the character doesn't exist.
    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()>;

    /// Gets the currently active character ID, if any.
    async fn get_active_character_id(&self) -> AppResult<Option<String>>;

    /// Checks if a character name is already taken, optionally excluding a specific character ID.
    async fn is_name_taken(&self, name: &str, exclude_id: Option<&str>) -> AppResult<bool>;

    /// Gets all character IDs, ordered by last_played DESC.
    async fn get_character_ids(&self) -> AppResult<Vec<String>>;
}

#[async_trait]
pub trait CharacterService: Send + Sync {
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<CharacterDataResponse>;

    async fn get_character(&self, character_id: &str) -> AppResult<CharacterDataResponse>;

    async fn get_all_characters(&self) -> AppResult<Vec<CharacterDataResponse>>;

    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> AppResult<CharacterDataResponse>;

    async fn delete_character(&self, character_id: &str) -> AppResult<()>;

    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()>;

    async fn get_active_character(&self) -> AppResult<Option<CharacterDataResponse>>;

    async fn get_characters_index(&self) -> AppResult<CharactersIndex>;

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> AppResult<bool>;

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> AppResult<()>;

    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> AppResult<Option<LocationState>>;

    /// Loads raw character data for internal mutations (not enriched)
    async fn load_character_data(&self, character_id: &str) -> AppResult<CharacterData>;

    async fn save_character_data(&self, character_data: &CharacterData) -> AppResult<()>;

    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()>;

    /// Leaves a zone, stopping its timer and recording duration.
    /// Should be called before entering a new zone for explicit leave/enter semantics.
    async fn leave_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()>;

    async fn record_death(&self, character_id: &str) -> AppResult<()>;

    async fn finalize_all_active_zones(&self) -> AppResult<()>;

    /// Syncs zone metadata (act, is_town) for all zones in a character's data with current zone configuration
    async fn sync_zone_metadata(&self, character_id: &str) -> AppResult<()>;
}
