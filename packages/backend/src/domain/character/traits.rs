use async_trait::async_trait;

use crate::errors::AppError;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, League, LocationState,
};

#[async_trait]
pub trait CharacterRepository {
    async fn load_characters_index(&self) -> Result<CharactersIndex, AppError>;

    async fn save_characters_index(&self, index: &CharactersIndex) -> Result<(), AppError>;

    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError>;

    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError>;

    async fn delete_character_data(&self, character_id: &str) -> Result<(), AppError>;

    async fn load_all_characters(&self) -> Result<Vec<CharacterData>, AppError>;

    async fn character_exists(&self, character_id: &str) -> Result<bool, AppError>;
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
    ) -> Result<CharacterData, AppError>;

    async fn get_character(&self, character_id: &str) -> Result<CharacterDataResponse, AppError>;

    async fn get_all_characters(&self) -> Result<Vec<CharacterDataResponse>, AppError>;

    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> Result<CharacterData, AppError>;

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError>;

    async fn set_active_character(&self, character_id: Option<&str>) -> Result<(), AppError>;

    async fn get_active_character(&self) -> Result<Option<CharacterDataResponse>, AppError>;

    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError>;

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError>;

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError>;

    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> Result<Option<LocationState>, AppError>;

    /// Loads raw character data for internal mutations (not enriched)
    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError>;

    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError>;

    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError>;

    async fn record_death(&self, character_id: &str) -> Result<(), AppError>;

    async fn finalize_all_active_zones(&self) -> Result<(), AppError>;

    /// Syncs zone metadata (act, is_town) for all zones in a character's data with current zone configuration
    async fn sync_zone_metadata(&self, character_id: &str) -> Result<(), AppError>;
}
