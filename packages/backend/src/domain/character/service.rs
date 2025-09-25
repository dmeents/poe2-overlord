use crate::domain::character::models::{
    is_valid_ascendency_for_class, Ascendency, Character, CharacterClass, CharacterUpdateParams,
    League,
};
use crate::domain::character::repository::CharacterRepositoryImpl;
use crate::domain::character::traits::{
    CharacterRepository, CharacterService as CharacterServiceTrait,
};
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use chrono::Utc;
use log::info;
use std::sync::Arc;
use uuid::Uuid;

/// Character service that handles business logic for character management
#[derive(Clone)]
pub struct CharacterService {
    /// Character repository for data operations
    repository: Arc<dyn CharacterRepository>,
}

impl CharacterService {
    /// Create a new character service with default repository
    pub fn new() -> AppResult<Self> {
        let repository = Arc::new(CharacterRepositoryImpl::new()?);
        Ok(Self { repository })
    }

    /// Create a new character service with custom repository
    pub fn with_repository(repository: Arc<dyn CharacterRepository>) -> Self {
        Self { repository }
    }

    /// Create a new character
    pub async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character> {
        // Validate that the ascendency belongs to the class
        if !is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::validation_error(
                "ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
            ));
        }

        // Ensure unique character name
        self.repository.ensure_unique_name(&name, None).await?;

        // Create the new character
        let character = Character {
            id: Uuid::new_v4().to_string(),
            name: name.clone(),
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
            created_at: Utc::now(),
            last_played: None,
            is_active: false,
            level: 1,
            death_count: 0,
        };

        // If this is the first character, make it active
        if self.repository.is_first_character().await? {
            // Add the character first, then set it as active
            self.repository.add_character(character.clone()).await?;
            self.repository.set_active_character(&character.id).await?;
        } else {
            self.repository.add_character(character.clone()).await?;
        }

        info!("Created new character: {}", name);
        Ok(character)
    }

    /// Get all characters
    pub async fn get_all_characters(&self) -> Vec<Character> {
        self.repository
            .get_all_characters()
            .await
            .unwrap_or_default()
    }

    /// Get character by ID
    pub async fn get_character(&self, character_id: &str) -> Option<Character> {
        self.repository
            .find_by_id(character_id)
            .await
            .unwrap_or(None)
    }

    /// Get the currently active character
    pub async fn get_active_character(&self) -> Option<Character> {
        self.repository.get_active_character().await.unwrap_or(None)
    }

    /// Set the active character by ID
    pub async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        self.repository.set_active_character(character_id).await
    }

    /// Delete a character by ID
    pub async fn delete_character(&self, character_id: &str) -> AppResult<Character> {
        self.repository.delete_character(character_id).await
    }

    /// Update a character's information
    pub async fn update_character(
        &self,
        character_id: &str,
        params: CharacterUpdateParams,
    ) -> AppResult<Character> {
        // Validate that the ascendency belongs to the class
        if !is_valid_ascendency_for_class(&params.ascendency, &params.class) {
            return Err(AppError::validation_error(
                "ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    params.ascendency, params.class
                ),
            ));
        }

        // Ensure unique character name (excluding current character)
        self.repository
            .ensure_unique_name(&params.name, Some(character_id))
            .await?;

        // Get the existing character
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
            AppError::character_management_error(
                "update_character",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        // Update the character
        character.name = params.name.clone();
        character.class = params.class;
        character.ascendency = params.ascendency;
        character.league = params.league;
        character.hardcore = params.hardcore;
        character.solo_self_found = params.solo_self_found;

        self.repository.update_character(character.clone()).await?;

        info!("Updated character: {}", params.name);
        Ok(character)
    }

    /// Clear all character data
    pub async fn clear_all_data(&self) -> AppResult<()> {
        self.repository.clear_all_characters().await
    }

    /// Update a character's level (system-managed)
    pub async fn update_character_level(
        &self,
        character_id: &str,
        new_level: u32,
    ) -> AppResult<()> {
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
            AppError::character_management_error(
                "update_character_level",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        character.level = new_level;
        character.last_played = Some(Utc::now());
        self.repository.update_character(character).await
    }

    /// Increment a character's death count (system-managed)
    pub async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
            AppError::character_management_error(
                "increment_character_deaths",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        character.death_count += 1;
        character.last_played = Some(Utc::now());
        self.repository.update_character(character).await
    }
}

#[async_trait]
impl CharacterServiceTrait for CharacterService {
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character> {
        CharacterService::create_character(
            self,
            name,
            class,
            ascendency,
            league,
            hardcore,
            solo_self_found,
        )
        .await
    }

    async fn get_all_characters(&self) -> Vec<Character> {
        CharacterService::get_all_characters(self).await
    }

    async fn get_character(&self, character_id: &str) -> Option<Character> {
        CharacterService::get_character(self, character_id).await
    }

    async fn update_character(
        &self,
        character_id: &str,
        params: CharacterUpdateParams,
    ) -> AppResult<Character> {
        CharacterService::update_character(self, character_id, params).await
    }

    async fn delete_character(&self, character_id: &str) -> AppResult<Character> {
        CharacterService::delete_character(self, character_id).await
    }

    async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        CharacterService::set_active_character(self, character_id).await
    }

    async fn get_active_character(&self) -> Option<Character> {
        CharacterService::get_active_character(self).await
    }

    async fn clear_all_characters(&self) -> AppResult<()> {
        CharacterService::clear_all_data(self).await
    }

    async fn update_character_level(&self, character_id: &str, level: u32) -> AppResult<()> {
        CharacterService::update_character_level(self, character_id, level).await
    }

    async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        CharacterService::increment_character_deaths(self, character_id).await
    }
}
