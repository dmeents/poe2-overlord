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

/// Character service implementation providing business logic for character management.
///
/// This service acts as the orchestrator for character-related operations, implementing
/// business rules, validation, and coordinating between different components. It follows
/// the service layer pattern to separate business logic from data access concerns.
///
/// Key responsibilities:
/// - Enforcing business rules (ascendency-class validation, name uniqueness)
/// - Orchestrating character lifecycle operations
/// - Managing the active character state
/// - Providing a clean interface for the application layer
///
/// The service uses dependency injection to work with any repository implementation,
/// enabling easy testing and potential future changes to the data layer.
#[derive(Clone)]
pub struct CharacterService {
    /// Repository for character data persistence operations
    repository: Arc<dyn CharacterRepository>,
}

impl CharacterService {
    /// Creates a new CharacterService instance with the default repository implementation.
    ///
    /// This constructor initializes the service with a CharacterRepositoryImpl instance,
    /// which provides file-based persistence for character data.
    ///
    /// # Returns
    /// * `Ok(CharacterService)` - Successfully initialized service
    /// * `Err(AppError)` - If repository initialization fails
    pub fn new() -> AppResult<Self> {
        let repository = Arc::new(CharacterRepositoryImpl::new()?);
        Ok(Self { repository })
    }

    /// Creates a new CharacterService instance with a custom repository.
    ///
    /// This constructor is primarily used for testing and dependency injection,
    /// allowing the service to work with mock repositories or alternative
    /// persistence implementations.
    ///
    /// # Arguments
    /// * `repository` - The repository implementation to use
    ///
    /// # Returns
    /// A new CharacterService instance with the provided repository
    pub fn with_repository(repository: Arc<dyn CharacterRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new character with the specified parameters.
    ///
    /// This method implements the complete character creation workflow:
    /// 1. Validates the ascendency-class combination
    /// 2. Ensures character name uniqueness
    /// 3. Creates a new character with generated UUID and default values
    /// 4. Adds the character to the repository
    /// 5. Sets the character as active if it's the first character
    ///
    /// # Arguments
    /// * `name` - Unique character name
    /// * `class` - Base character class
    /// * `ascendency` - Specialized ascendency (must be valid for the class)
    /// * `league` - Game league/mode
    /// * `hardcore` - Whether character is in hardcore mode
    /// * `solo_self_found` - Whether character is in SSF mode
    ///
    /// # Returns
    /// * `Ok(Character)` - The newly created character
    /// * `Err(AppError)` - If validation fails or name is not unique
    pub async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character> {
        // Validate ascendency-class combination
        if !is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
            ));
        }

        // Ensure character name is unique
        self.repository.ensure_unique_name(&name, None).await?;

        // Create new character with generated UUID and default values
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
        };

        // Add character to repository and set as active if it's the first character
        if self.repository.is_first_character().await? {
            self.repository.add_character(character.clone()).await?;
            self.repository.set_active_character(&character.id).await?;
        } else {
            self.repository.add_character(character.clone()).await?;
        }

        info!("Created new character: {}", name);
        Ok(character)
    }

    /// Retrieves all characters in the system.
    ///
    /// This method delegates to the repository and handles any errors gracefully
    /// by returning an empty vector if the operation fails.
    ///
    /// # Returns
    /// A vector containing all characters, or an empty vector if retrieval fails
    pub async fn get_all_characters(&self) -> Vec<Character> {
        self.repository
            .get_all_characters()
            .await
            .unwrap_or_default()
    }

    /// Retrieves a specific character by ID.
    ///
    /// This method delegates to the repository and handles any errors gracefully
    /// by returning None if the operation fails.
    ///
    /// # Arguments
    /// * `character_id` - The unique identifier of the character
    ///
    /// # Returns
    /// * `Some(Character)` - If the character exists
    /// * `None` - If no character with the given ID exists or if lookup fails
    pub async fn get_character(&self, character_id: &str) -> Option<Character> {
        self.repository
            .find_by_id(character_id)
            .await
            .unwrap_or(None)
    }

    /// Retrieves the currently active character.
    ///
    /// This method delegates to the repository and handles any errors gracefully
    /// by returning None if the operation fails.
    ///
    /// # Returns
    /// * `Some(Character)` - The active character if one exists
    /// * `None` - If no character is currently active or if lookup fails
    pub async fn get_active_character(&self) -> Option<Character> {
        self.repository.get_active_character().await.unwrap_or(None)
    }

    /// Sets a character as the active character.
    ///
    /// This method delegates to the repository to handle the active character
    /// state management and persistence.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to set as active
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If character not found or operation fails
    pub async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        self.repository.set_active_character(character_id).await
    }

    /// Deletes a character and all associated data.
    ///
    /// This method delegates to the repository to handle character deletion
    /// and cleanup of associated data.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to delete
    ///
    /// # Returns
    /// * `Ok(Character)` - The deleted character
    /// * `Err(AppError)` - If character not found or operation fails
    pub async fn delete_character(&self, character_id: &str) -> AppResult<Character> {
        self.repository.delete_character(character_id).await
    }

    /// Updates an existing character with new parameters.
    ///
    /// This method implements the complete character update workflow:
    /// 1. Validates the new ascendency-class combination
    /// 2. Ensures the new name is unique (excluding the current character)
    /// 3. Retrieves the existing character
    /// 4. Updates the character with new parameters
    /// 5. Persists the changes to the repository
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to update
    /// * `params` - The new character parameters
    ///
    /// # Returns
    /// * `Ok(Character)` - The updated character
    /// * `Err(AppError)` - If validation fails or character not found
    pub async fn update_character(
        &self,
        character_id: &str,
        params: CharacterUpdateParams,
    ) -> AppResult<Character> {
        // Validate new ascendency-class combination
        if !is_valid_ascendency_for_class(&params.ascendency, &params.class) {
            return Err(AppError::validation_error(
                "validate_ascendency",
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    params.ascendency, params.class
                ),
            ));
        }

        // Ensure new name is unique (excluding current character)
        self.repository
            .ensure_unique_name(&params.name, Some(character_id))
            .await?;

        // Retrieve existing character
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
            AppError::character_management_error(
                "update_character",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        // Update character with new parameters
        character.name = params.name.clone();
        character.class = params.class;
        character.ascendency = params.ascendency;
        character.league = params.league;
        character.hardcore = params.hardcore;
        character.solo_self_found = params.solo_self_found;

        // Persist changes to repository
        self.repository.update_character(character.clone()).await?;

        info!("Updated character: {}", params.name);
        Ok(character)
    }

    /// Clears all character data from the system.
    ///
    /// This is a destructive operation that removes all characters and resets
    /// the active character state. Use with caution as this operation cannot be undone.
    ///
    /// # Returns
    /// * `Ok(())` - If operation succeeds
    /// * `Err(AppError)` - If operation fails
    pub async fn clear_all_data(&self) -> AppResult<()> {
        self.repository.clear_all_characters().await
    }

    /// Updates a character's level and last played timestamp.
    ///
    /// This method is typically called when the game reports a level change
    /// for the currently active character. It updates both the level and
    /// the last_played timestamp to reflect recent activity.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to update
    /// * `new_level` - The new level
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If character not found or operation fails
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
}

/// Implementation of the CharacterServiceTrait for the CharacterService.
///
/// This implementation delegates all trait methods to the corresponding
/// public methods in the CharacterService struct, providing a clean
/// interface that matches the trait contract.
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
}
