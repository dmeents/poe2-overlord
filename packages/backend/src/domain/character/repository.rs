use crate::domain::character::models::{Character, CharacterData};
use crate::domain::character::traits::CharacterRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{PersistenceRepository, PersistenceRepositoryImpl};
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, info, warn};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::RwLock;

/// File name for character data persistence.
///
/// This constant defines the JSON file name used to store character data
/// in the application's configuration directory.
const CHARACTER_DATA_FILE_NAME: &str = "characters.json";

/// Implementation of the CharacterRepository trait for character data persistence.
///
/// This repository uses a combination of in-memory caching with RwLock for thread-safe
/// concurrent access and file-based persistence for data durability. The architecture
/// provides fast read operations through in-memory access while ensuring data is
/// persisted to disk for reliability.
///
/// Key features:
/// - Thread-safe concurrent access using tokio::sync::RwLock
/// - Automatic data loading on initialization
/// - Graceful handling of missing or corrupted data files
/// - Atomic write operations to prevent data corruption
#[derive(Clone)]
pub struct CharacterRepositoryImpl {
    /// In-memory character data protected by a read-write lock for concurrent access
    character_data: Arc<RwLock<CharacterData>>,
    /// Persistence layer for saving/loading data to/from disk
    persistence: PersistenceRepositoryImpl<CharacterData>,
    /// Flag to track whether data has been loaded from disk
    data_loaded: Arc<AtomicBool>,
}

impl CharacterRepositoryImpl {
    /// Creates a new CharacterRepositoryImpl instance.
    ///
    /// This constructor initializes the repository with:
    /// - A persistence layer configured to use the character data file
    /// - An empty in-memory data structure protected by RwLock
    /// - A flag to track data loading status
    ///
    /// Data loading is deferred until the first operation that requires it,
    /// ensuring compatibility with both sync and async contexts.
    ///
    /// # Returns
    /// * `Ok(CharacterRepositoryImpl)` - Successfully initialized repository
    /// * `Err(AppError)` - If persistence layer initialization fails
    pub fn new() -> AppResult<Self> {
        let persistence = PersistenceRepositoryImpl::<CharacterData>::new_in_config_dir(
            CHARACTER_DATA_FILE_NAME,
        )?;

        Ok(Self {
            character_data: Arc::new(RwLock::new(CharacterData::default())),
            persistence,
            data_loaded: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Ensures that data has been loaded from disk.
    ///
    /// This method checks if data has already been loaded and loads it if necessary.
    /// It's safe to call multiple times and will only load data once.
    ///
    /// # Returns
    /// * `Ok(())` - Data is loaded and ready
    /// * `Err(AppError)` - If data loading fails
    async fn ensure_data_loaded(&self) -> AppResult<()> {
        if !self.data_loaded.load(Ordering::Relaxed) {
            if let Err(e) = self.load().await {
                warn!("Failed to load character data, starting fresh: {}", e);
                // Don't return error - allow repository to work with empty data
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {
    /// Saves character data to persistent storage.
    ///
    /// This method delegates to the persistence layer to write the data to disk.
    /// The in-memory cache is not updated here as it should already be in sync.
    ///
    /// # Arguments
    /// * `data` - The character data to save
    ///
    /// # Returns
    /// * `Ok(())` - If save operation succeeds
    /// * `Err(AppError)` - If save operation fails
    async fn save(&self, data: &CharacterData) -> AppResult<()> {
        self.persistence.save(data).await
    }

    /// Loads character data from persistent storage and updates the in-memory cache.
    ///
    /// This method performs a complete data reload from disk, updating both
    /// the in-memory cache and returning the loaded data. This is useful for
    /// initialization and potential data refresh scenarios.
    ///
    /// # Returns
    /// * `Ok(CharacterData)` - The loaded character data
    /// * `Err(AppError)` - If load operation fails
    async fn load(&self) -> AppResult<CharacterData> {
        let data = self.persistence.load().await?;

        // Update the in-memory cache with the loaded data
        {
            let mut character_data = self.character_data.write().await;
            *character_data = data.clone();
        }

        // Mark data as loaded
        self.data_loaded.store(true, Ordering::Relaxed);
        debug!("Character data loaded successfully");
        Ok(data)
    }

    /// Finds a character by its unique identifier.
    ///
    /// This method performs a linear search through the in-memory character list.
    /// The read lock is held for the minimum time necessary to find and clone the character.
    ///
    /// # Arguments
    /// * `id` - The character's unique identifier
    ///
    /// # Returns
    /// * `Ok(Some(Character))` - If character exists
    /// * `Ok(None)` - If no character with the given ID exists
    /// * `Err(AppError)` - If lookup operation fails
    async fn find_by_id(&self, id: &str) -> AppResult<Option<Character>> {
        self.ensure_data_loaded().await?;
        let character_data = self.character_data.read().await;
        let character = character_data
            .characters
            .iter()
            .find(|c| c.id == id)
            .cloned();
        Ok(character)
    }

    /// Retrieves the currently active character.
    ///
    /// This method first checks if there's an active character ID set, then
    /// searches for the corresponding character in the character list.
    ///
    /// # Returns
    /// * `Ok(Some(Character))` - The active character if one exists
    /// * `Ok(None)` - If no character is currently active
    /// * `Err(AppError)` - If lookup operation fails
    async fn get_active_character(&self) -> AppResult<Option<Character>> {
        self.ensure_data_loaded().await?;
        let character_data = self.character_data.read().await;
        let character = character_data.active_character_id.as_ref().and_then(|id| {
            character_data
                .characters
                .iter()
                .find(|c| c.id == *id)
                .cloned()
        });
        Ok(character)
    }

    /// Retrieves all characters in the system.
    ///
    /// This method returns a clone of the entire character list. For large
    /// character collections, consider implementing pagination or filtering.
    ///
    /// # Returns
    /// * `Ok(Vec<Character>)` - All characters in the system
    /// * `Err(AppError)` - If retrieval operation fails
    async fn get_all_characters(&self) -> AppResult<Vec<Character>> {
        self.ensure_data_loaded().await?;
        let character_data = self.character_data.read().await;
        Ok(character_data.characters.clone())
    }

    /// Adds a new character to the repository.
    ///
    /// This method adds the character to the in-memory list and immediately
    /// persists the change to disk. The write lock is held only for the
    /// minimum time necessary to add the character.
    ///
    /// # Arguments
    /// * `character` - The character to add
    ///
    /// # Returns
    /// * `Ok(())` - If character is added successfully
    /// * `Err(AppError)` - If add operation fails
    async fn add_character(&self, character: Character) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let mut character_data = self.character_data.write().await;
        character_data.characters.push(character);
        drop(character_data);
        self.save(&self.character_data.read().await.clone()).await
    }

    /// Updates an existing character in the repository.
    ///
    /// This method finds the character by ID and replaces it with the updated version.
    /// The change is immediately persisted to disk. If the character doesn't exist,
    /// an error is returned.
    ///
    /// # Arguments
    /// * `character` - The character with updated data
    ///
    /// # Returns
    /// * `Ok(())` - If character is updated successfully
    /// * `Err(AppError)` - If update operation fails or character not found
    async fn update_character(&self, character: Character) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let mut character_data = self.character_data.write().await;

        if let Some(existing) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character.id)
        {
            *existing = character;
            drop(character_data);
            self.save(&self.character_data.read().await.clone()).await
        } else {
            Err(AppError::character_management_error(
                "update_character",
                &format!("Character with ID '{}' not found", character.id),
            ))
        }
    }

    /// Deletes a character from the repository.
    ///
    /// This method removes the character from the in-memory list and clears
    /// the active character ID if the deleted character was active. The change
    /// is immediately persisted to disk.
    ///
    /// # Arguments
    /// * `id` - The ID of the character to delete
    ///
    /// # Returns
    /// * `Ok(Character)` - The deleted character
    /// * `Err(AppError)` - If delete operation fails or character not found
    async fn delete_character(&self, id: &str) -> AppResult<Character> {
        self.ensure_data_loaded().await?;
        let mut character_data = self.character_data.write().await;

        let index = character_data
            .characters
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| {
                AppError::character_management_error(
                    "delete_character",
                    &format!("Character with ID '{}' not found", id),
                )
            })?;

        let character = character_data.characters.remove(index);

        // Clear active character if the deleted character was active
        if character_data.active_character_id.as_ref() == Some(&id.to_string()) {
            character_data.active_character_id = None;
        }

        drop(character_data);
        self.save(&self.character_data.read().await.clone()).await?;
        info!("Deleted character: {}", character.name);
        Ok(character)
    }

    /// Sets a character as the active character.
    ///
    /// This method ensures only one character can be active at a time by:
    /// 1. Deactivating all characters
    /// 2. Activating the specified character
    /// 3. Updating the last_played timestamp
    /// 4. Setting the active_character_id
    ///
    /// The change is immediately persisted to disk.
    ///
    /// # Arguments
    /// * `id` - The ID of the character to set as active
    ///
    /// # Returns
    /// * `Ok(())` - If operation succeeds
    /// * `Err(AppError)` - If operation fails or character not found
    async fn set_active_character(&self, id: &str) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let mut character_data = self.character_data.write().await;

        if !character_data.characters.iter().any(|c| c.id == id) {
            return Err(AppError::character_management_error(
                "set_active_character",
                &format!("Character with ID '{}' not found", id),
            ));
        }

        // Deactivate all characters first
        for character in &mut character_data.characters {
            character.is_active = false;
        }

        // Activate the specified character and update last played time
        if let Some(character) = character_data.characters.iter_mut().find(|c| c.id == id) {
            character.is_active = true;
            character.last_played = Some(Utc::now());
        }

        character_data.active_character_id = Some(id.to_string());

        drop(character_data);
        self.save(&self.character_data.read().await.clone()).await?;
        info!("Set active character: {}", id);
        Ok(())
    }

    /// Ensures that a character name is unique across all characters.
    ///
    /// This method checks if the given name is already used by another character.
    /// When updating an existing character, the exclude_id parameter allows
    /// the character to keep its current name.
    ///
    /// # Arguments
    /// * `name` - The name to check for uniqueness
    /// * `exclude_id` - Optional character ID to exclude from the check (for updates)
    ///
    /// # Returns
    /// * `Ok(())` - If name is unique
    /// * `Err(AppError)` - If name is already taken
    async fn ensure_unique_name(&self, name: &str, exclude_id: Option<&str>) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        let character_data = self.character_data.read().await;

        if character_data
            .characters
            .iter()
            .any(|c| c.name == name && exclude_id.is_none_or(|exclude| c.id != exclude))
        {
            return Err(AppError::validation_error(
                "validate_unique_name",
                &format!("Character name '{}' is already taken", name),
            ));
        }

        Ok(())
    }

    /// Clears all character data from the repository.
    ///
    /// This is a destructive operation that:
    /// 1. Clears the in-memory character list
    /// 2. Resets the active character ID
    /// 3. Deletes the persistence file
    ///
    /// Use with caution as this operation cannot be undone.
    ///
    /// # Returns
    /// * `Ok(())` - If operation succeeds
    /// * `Err(AppError)` - If operation fails
    async fn clear_all_characters(&self) -> AppResult<()> {
        self.ensure_data_loaded().await?;
        {
            let mut character_data = self.character_data.write().await;
            character_data.characters.clear();
            character_data.active_character_id = None;
        }

        self.persistence.delete().await?;

        debug!("All character data cleared");
        Ok(())
    }

    /// Checks if this would be the first character in the system.
    ///
    /// This method is used to determine if a newly created character should
    /// automatically be set as active (first character behavior).
    ///
    /// # Returns
    /// * `Ok(true)` - If no characters exist
    /// * `Ok(false)` - If at least one character exists
    /// * `Err(AppError)` - If check operation fails
    async fn is_first_character(&self) -> AppResult<bool> {
        self.ensure_data_loaded().await?;
        let character_data = self.character_data.read().await;
        Ok(character_data.characters.is_empty())
    }
}
