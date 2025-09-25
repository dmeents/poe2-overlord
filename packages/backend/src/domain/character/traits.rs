use crate::domain::character::models::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterUpdateParams, League,
};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait for character management operations
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
    ) -> AppResult<Character>;

    async fn get_all_characters(&self) -> Vec<Character>;
    async fn get_character(&self, character_id: &str) -> Option<Character>;
    async fn update_character(
        &self,
        character_id: &str,
        params: CharacterUpdateParams,
    ) -> AppResult<Character>;
    async fn delete_character(&self, character_id: &str) -> AppResult<Character>;
    async fn set_active_character(&self, character_id: &str) -> AppResult<()>;
    async fn get_active_character(&self) -> Option<Character>;
    async fn clear_all_characters(&self) -> AppResult<()>;
    async fn update_character_level(&self, character_id: &str, level: u32) -> AppResult<()>;
    async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()>;
}

/// Trait for character data repository operations
#[async_trait]
pub trait CharacterRepository: Send + Sync {
    // Persistence operations
    async fn save(&self, data: &CharacterData) -> AppResult<()>;
    async fn load(&self) -> AppResult<CharacterData>;

    // Query operations
    async fn find_by_id(&self, id: &str) -> AppResult<Option<Character>>;
    async fn get_active_character(&self) -> AppResult<Option<Character>>;
    async fn get_all_characters(&self) -> AppResult<Vec<Character>>;

    // Data manipulation
    async fn add_character(&self, character: Character) -> AppResult<()>;
    async fn update_character(&self, character: Character) -> AppResult<()>;
    async fn delete_character(&self, id: &str) -> AppResult<Character>;
    async fn set_active_character(&self, id: &str) -> AppResult<()>;

    // Business rules and validation
    async fn ensure_unique_name(&self, name: &str, exclude_id: Option<&str>) -> AppResult<()>;

    // Utility operations
    async fn clear_all_characters(&self) -> AppResult<()>;
    async fn is_first_character(&self) -> AppResult<bool>;
}
