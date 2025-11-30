use async_trait::async_trait;

use crate::errors::AppError;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterUpdateParams,
    CharactersIndex, League, LocationState, LocationType,
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

    async fn get_character(&self, character_id: &str) -> Result<CharacterData, AppError>;

    async fn get_all_characters(&self) -> Result<Vec<CharacterData>, AppError>;

    async fn get_character_response(
        &self,
        character_id: &str,
    ) -> Result<CharacterDataResponse, AppError>;

    async fn get_all_characters_response(&self) -> Result<Vec<CharacterDataResponse>, AppError>;

    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> Result<CharacterData, AppError>;

    async fn delete_character(&self, character_id: &str) -> Result<(), AppError>;

    async fn set_active_character(&self, character_id: Option<&str>) -> Result<(), AppError>;

    async fn get_active_character(&self) -> Result<Option<CharacterData>, AppError>;

    async fn get_characters_index(&self) -> Result<CharactersIndex, AppError>;

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, AppError>;

    async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> Result<(), AppError>;

    /// Returns None if no actual scene change occurred
    async fn process_scene_content(
        &self,
        content: &str,
        character_id: &str,
    ) -> Result<Option<crate::domain::log_analysis::models::SceneChangeEvent>, AppError>;

    /// Returns None if no actual scene change occurred
    async fn process_scene_content_with_zone_level(
        &self,
        content: &str,
        character_id: &str,
        zone_level: u32,
    ) -> Result<Option<crate::domain::log_analysis::models::SceneChangeEvent>, AppError>;

    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> Result<Option<LocationState>, AppError>;

    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError>;

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

    async fn leave_zone(&self, character_id: &str, location_id: &str) -> Result<(), AppError>;

    async fn record_death(&self, character_id: &str, location_id: &str) -> Result<(), AppError>;

    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> Result<(), AppError>;

    async fn finalize_all_active_zones(&self) -> Result<(), AppError>;
}
