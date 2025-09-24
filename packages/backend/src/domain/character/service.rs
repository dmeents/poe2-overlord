use crate::errors::{AppError, AppResult};
use crate::models::character::{
    is_valid_ascendency_for_class, Character, CharacterClass, CharacterUpdateParams, League,
    Ascendency,
};
use crate::domain::character::repository::CharacterRepository;
use crate::services::traits::CharacterService as CharacterServiceTrait;
use async_trait::async_trait;
use chrono::Utc;
use log::info;
use std::sync::Arc;
use uuid::Uuid;

/// Character service that handles business logic for character management
#[derive(Clone)]
pub struct CharacterService {
    repository: Arc<CharacterRepository>,
}

impl CharacterService {
    /// Create a new character service
    pub fn new() -> Self {
        Self {
            repository: Arc::new(CharacterRepository::new()),
        }
    }

    /// Create a new character service with custom repository (mainly for testing)
    pub fn with_repository(repository: Arc<CharacterRepository>) -> Self {
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
                &format!("Ascendency '{:?}' is not valid for class '{:?}'", ascendency, class),
            ));
        }

        // Check for duplicate names
        if !self.repository.is_name_available(&name).await {
            return Err(AppError::character_management_error(
                "create_character",
                &format!("Character with name '{}' already exists", name),
            ));
        }

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
        let character_count = self.repository.get_character_count().await;
        if character_count == 0 {
            // We need to add the character first, then set it as active
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
        self.repository.get_all_characters().await
    }

    /// Get character by ID
    pub async fn get_character(&self, character_id: &str) -> Option<Character> {
        self.repository.get_character(character_id).await
    }

    /// Get the currently active character
    pub async fn get_active_character(&self) -> Option<Character> {
        self.repository.get_active_character().await
    }

    /// Set the active character by ID
    pub async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        self.repository.set_active_character(character_id).await?;
        info!("Set active character: {}", character_id);
        Ok(())
    }

    /// Remove a character by ID
    pub async fn remove_character(&self, character_id: &str) -> AppResult<Character> {
        let character = self.repository.remove_character(character_id).await?;
        info!("Removed character: {}", character.name);
        Ok(character)
    }

    /// Get characters sorted by last played (most recent first)
    pub async fn get_characters_by_last_played(&self) -> Vec<Character> {
        let mut characters = self.repository.get_all_characters().await;

        characters.sort_by(|a, b| match (&a.last_played, &b.last_played) {
            (Some(a_time), Some(b_time)) => b_time.cmp(a_time),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.created_at.cmp(&b.created_at),
        });

        characters
    }

    /// Get characters by class
    pub async fn get_characters_by_class(&self, class: &CharacterClass) -> Vec<Character> {
        let characters = self.repository.get_all_characters().await;
        characters
            .into_iter()
            .filter(|c| c.class == *class)
            .collect()
    }

    /// Get characters by league
    pub async fn get_characters_by_league(&self, league: &League) -> Vec<Character> {
        let characters = self.repository.get_all_characters().await;
        characters
            .into_iter()
            .filter(|c| c.league == *league)
            .collect()
    }

    /// Check if a character name is available
    pub async fn is_name_available(&self, name: &str) -> bool {
        self.repository.is_name_available(name).await
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
                &format!("Ascendency '{:?}' is not valid for class '{:?}'", params.ascendency, params.class),
            ));
        }

        // Check for duplicate names (excluding the current character)
        if !self.repository.is_name_available(&params.name).await {
            // Check if the name belongs to the current character
            if let Some(existing) = self.repository.get_character(character_id).await {
                if existing.name != params.name {
                    return Err(AppError::character_management_error(
                        "update_character",
                        &format!("Character with name '{}' already exists", params.name),
                    ));
                }
            }
        }

        // Get the existing character
        let mut character = self.repository.get_character(character_id).await
            .ok_or_else(|| {
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

        self.repository.update_character(character_id, character.clone()).await?;

        info!("Updated character: {}", params.name);
        Ok(character)
    }

    /// Update a character's last played timestamp
    pub async fn update_last_played(&self, character_id: &str) -> AppResult<()> {
        let mut character = self.repository.get_character(character_id).await
            .ok_or_else(|| {
                AppError::character_management_error(
                    "update_last_played",
                    &format!("Character with ID '{}' not found", character_id),
                )
            })?;

        character.last_played = Some(Utc::now());
        self.repository.update_character(character_id, character).await
    }

    /// Clear all character data
    pub async fn clear_all_data(&self) -> AppResult<()> {
        self.repository.clear_all_data().await
    }

    /// Get the total number of characters
    pub async fn get_character_count(&self) -> usize {
        self.repository.get_character_count().await
    }

    /// Check if there are any characters
    pub async fn has_characters(&self) -> bool {
        self.repository.has_characters().await
    }

    /// Update a character's level (system-managed)
    pub async fn update_character_level(&self, character_id: &str, new_level: u32) -> AppResult<()> {
        let mut character = self.repository.get_character(character_id).await
            .ok_or_else(|| {
                AppError::character_management_error(
                    "update_character_level",
                    &format!("Character with ID '{}' not found", character_id),
                )
            })?;

        character.level = new_level;
        character.last_played = Some(Utc::now());
        self.repository.update_character(character_id, character).await
    }

    /// Increment a character's death count (system-managed)
    pub async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        let mut character = self.repository.get_character(character_id).await
            .ok_or_else(|| {
                AppError::character_management_error(
                    "increment_character_deaths",
                    &format!("Character with ID '{}' not found", character_id),
                )
            })?;

        character.death_count += 1;
        character.last_played = Some(Utc::now());
        self.repository.update_character(character_id, character).await
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
        CharacterService::create_character(self, name, class, ascendency, league, hardcore, solo_self_found).await
    }

    async fn get_all_characters(&self) -> Vec<Character> {
        CharacterService::get_all_characters(self).await
    }

    async fn get_character(&self, character_id: &str) -> Option<Character> {
        CharacterService::get_character(self, character_id).await
    }

    async fn update_character(&self, character_id: &str, params: CharacterUpdateParams) -> AppResult<Character> {
        CharacterService::update_character(self, character_id, params).await
    }

    async fn remove_character(&self, character_id: &str) -> AppResult<Character> {
        CharacterService::remove_character(self, character_id).await
    }

    async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        CharacterService::set_active_character(self, character_id).await
    }

    async fn get_active_character(&self) -> Option<Character> {
        CharacterService::get_active_character(self).await
    }

    async fn update_last_played(&self, character_id: &str) -> AppResult<()> {
        CharacterService::update_last_played(self, character_id).await
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
