use crate::domain::character::models::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterUpdateParams, League,
};
use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait defining the character service interface for business logic operations.
///
/// This trait encapsulates all character-related business operations including
/// creation, retrieval, updates, and deletion. It serves as the contract for
/// the character domain service and enables dependency injection and testing.
///
/// The service layer handles business rules, validation, and orchestration
/// between different components while keeping the domain logic separate from
/// infrastructure concerns.
#[async_trait]
pub trait CharacterService: Send + Sync {
    /// Creates a new character with the specified parameters.
    ///
    /// Validates the ascendency-class combination and ensures character name uniqueness.
    /// If this is the first character created, it will automatically be set as active.
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
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character>;

    /// Retrieves all characters in the system.
    ///
    /// # Returns
    /// A vector containing all characters, ordered by creation date
    async fn get_all_characters(&self) -> Vec<Character>;

    /// Retrieves a specific character by ID.
    ///
    /// # Arguments
    /// * `character_id` - The unique identifier of the character
    ///
    /// # Returns
    /// * `Some(Character)` - If the character exists
    /// * `None` - If no character with the given ID exists
    async fn get_character(&self, character_id: &str) -> Option<Character>;

    /// Updates an existing character with new parameters.
    ///
    /// Validates the new ascendency-class combination and ensures name uniqueness.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to update
    /// * `params` - The new character parameters
    ///
    /// # Returns
    /// * `Ok(Character)` - The updated character
    /// * `Err(AppError)` - If validation fails or character not found
    async fn update_character(
        &self,
        character_id: &str,
        params: CharacterUpdateParams,
    ) -> AppResult<Character>;

    /// Deletes a character and all associated data.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to delete
    ///
    /// # Returns
    /// * `Ok(Character)` - The deleted character
    /// * `Err(AppError)` - If character not found
    async fn delete_character(&self, character_id: &str) -> AppResult<Character>;

    /// Sets a character as the active character.
    ///
    /// Only one character can be active at a time. Setting a new active character
    /// will deactivate the previously active character and update the last_played timestamp.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to set as active
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If character not found
    async fn set_active_character(&self, character_id: &str) -> AppResult<()>;

    /// Retrieves the currently active character.
    ///
    /// # Returns
    /// * `Some(Character)` - The active character if one exists
    /// * `None` - If no character is currently active
    async fn get_active_character(&self) -> Option<Character>;

    /// Clears all character data from the system.
    ///
    /// This is a destructive operation that removes all characters and resets
    /// the active character state. Use with caution.
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If the operation fails
    async fn clear_all_characters(&self) -> AppResult<()>;

    /// Updates a character's level and last played timestamp.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to update
    /// * `level` - The new level
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If character not found
    async fn update_character_level(&self, character_id: &str, level: u32) -> AppResult<()>;

    /// Increments a character's death count and updates last played timestamp.
    ///
    /// # Arguments
    /// * `character_id` - The ID of the character to update
    ///
    /// # Returns
    /// * `Ok(())` - If successful
    /// * `Err(AppError)` - If character not found
    async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()>;
}

/// Trait defining the character repository interface for data persistence operations.
///
/// This trait encapsulates all data access operations for characters, providing
/// an abstraction layer between the business logic and the persistence mechanism.
/// It enables dependency injection, testing with mocks, and potential future
/// changes to the storage backend without affecting the business logic.
///
/// The repository pattern ensures that data access concerns are separated from
/// business logic, making the codebase more maintainable and testable.
#[async_trait]
pub trait CharacterRepository: Send + Sync {
    /// Saves the complete character data to persistent storage.
    ///
    /// # Arguments
    /// * `data` - The character data to save
    ///
    /// # Returns
    /// * `Ok(())` - If save operation succeeds
    /// * `Err(AppError)` - If save operation fails
    async fn save(&self, data: &CharacterData) -> AppResult<()>;

    /// Loads the complete character data from persistent storage.
    ///
    /// # Returns
    /// * `Ok(CharacterData)` - The loaded character data
    /// * `Err(AppError)` - If load operation fails
    async fn load(&self) -> AppResult<CharacterData>;

    /// Finds a character by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The character's unique identifier
    ///
    /// # Returns
    /// * `Ok(Some(Character))` - If character exists
    /// * `Ok(None)` - If no character with the given ID exists
    /// * `Err(AppError)` - If lookup operation fails
    async fn find_by_id(&self, id: &str) -> AppResult<Option<Character>>;

    /// Retrieves the currently active character.
    ///
    /// # Returns
    /// * `Ok(Some(Character))` - The active character if one exists
    /// * `Ok(None)` - If no character is currently active
    /// * `Err(AppError)` - If lookup operation fails
    async fn get_active_character(&self) -> AppResult<Option<Character>>;

    /// Retrieves all characters in the system.
    ///
    /// # Returns
    /// * `Ok(Vec<Character>)` - All characters in the system
    /// * `Err(AppError)` - If retrieval operation fails
    async fn get_all_characters(&self) -> AppResult<Vec<Character>>;

    /// Adds a new character to the repository.
    ///
    /// # Arguments
    /// * `character` - The character to add
    ///
    /// # Returns
    /// * `Ok(())` - If character is added successfully
    /// * `Err(AppError)` - If add operation fails
    async fn add_character(&self, character: Character) -> AppResult<()>;

    /// Updates an existing character in the repository.
    ///
    /// # Arguments
    /// * `character` - The character with updated data
    ///
    /// # Returns
    /// * `Ok(())` - If character is updated successfully
    /// * `Err(AppError)` - If update operation fails or character not found
    async fn update_character(&self, character: Character) -> AppResult<()>;

    /// Deletes a character from the repository.
    ///
    /// # Arguments
    /// * `id` - The ID of the character to delete
    ///
    /// # Returns
    /// * `Ok(Character)` - The deleted character
    /// * `Err(AppError)` - If delete operation fails or character not found
    async fn delete_character(&self, id: &str) -> AppResult<Character>;

    /// Sets a character as the active character.
    ///
    /// This operation updates the active character state and persists the change.
    ///
    /// # Arguments
    /// * `id` - The ID of the character to set as active
    ///
    /// # Returns
    /// * `Ok(())` - If operation succeeds
    /// * `Err(AppError)` - If operation fails or character not found
    async fn set_active_character(&self, id: &str) -> AppResult<()>;

    /// Ensures that a character name is unique across all characters.
    ///
    /// # Arguments
    /// * `name` - The name to check for uniqueness
    /// * `exclude_id` - Optional character ID to exclude from the check (for updates)
    ///
    /// # Returns
    /// * `Ok(())` - If name is unique
    /// * `Err(AppError)` - If name is already taken
    async fn ensure_unique_name(&self, name: &str, exclude_id: Option<&str>) -> AppResult<()>;

    /// Clears all character data from the repository.
    ///
    /// This is a destructive operation that removes all characters and resets
    /// the active character state.
    ///
    /// # Returns
    /// * `Ok(())` - If operation succeeds
    /// * `Err(AppError)` - If operation fails
    async fn clear_all_characters(&self) -> AppResult<()>;

    /// Checks if this would be the first character in the system.
    ///
    /// # Returns
    /// * `Ok(true)` - If no characters exist
    /// * `Ok(false)` - If at least one character exists
    /// * `Err(AppError)` - If check operation fails
    async fn is_first_character(&self) -> AppResult<bool>;
}
