use crate::domain::character::models::{Character, CharacterData};
use crate::domain::character::traits::CharacterRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{
    PersistenceRepository, PersistenceRepositoryImpl,
};
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Character management constants
const CHARACTER_DATA_FILE_NAME: &str = "characters.json";

/// Character repository implementation that handles all character data operations
#[derive(Clone)]
pub struct CharacterRepositoryImpl {
    /// Character data with thread-safe access
    character_data: Arc<RwLock<CharacterData>>,
    /// Persistence repository for character data
    persistence: PersistenceRepositoryImpl<CharacterData>,
}

impl CharacterRepositoryImpl {
    /// Create a new character repository
    pub fn new() -> AppResult<Self> {
        // Create persistence repository in config directory
        let persistence = PersistenceRepositoryImpl::<CharacterData>::new_in_config_dir(CHARACTER_DATA_FILE_NAME)?;

        let repository = Self {
            character_data: Arc::new(RwLock::new(CharacterData::default())),
            persistence,
        };

        // Load existing character data
        if let Err(e) = tokio::runtime::Handle::current().block_on(repository.load()) {
            warn!("Failed to load character data, starting fresh: {}", e);
        }

        Ok(repository)
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {
    // Persistence operations
    async fn save(&self, data: &CharacterData) -> AppResult<()> {
        self.persistence.save(data).await
    }

    async fn load(&self) -> AppResult<CharacterData> {
        let data = self.persistence.load().await?;

        {
            let mut character_data = self.character_data.write().await;
            *character_data = data.clone();
        }

        debug!("Character data loaded successfully");
        Ok(data)
    }

    // Query operations
    async fn find_by_id(&self, id: &str) -> AppResult<Option<Character>> {
        let character_data = self.character_data.read().await;
        let character = character_data
            .characters
            .iter()
            .find(|c| c.id == id)
            .cloned();
        Ok(character)
    }

    async fn get_active_character(&self) -> AppResult<Option<Character>> {
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

    async fn get_all_characters(&self) -> AppResult<Vec<Character>> {
        let character_data = self.character_data.read().await;
        Ok(character_data.characters.clone())
    }

    // Data manipulation
    async fn add_character(&self, character: Character) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;
        character_data.characters.push(character);
        drop(character_data);
        self.save(&self.character_data.read().await.clone())
            .await
    }

    async fn update_character(&self, character: Character) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        if let Some(existing) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character.id)
        {
            *existing = character;
            drop(character_data);
            self.save(&self.character_data.read().await.clone())
                .await
        } else {
            Err(AppError::character_management_error(
                "update_character",
                &format!("Character with ID '{}' not found", character.id),
            ))
        }
    }

    async fn delete_character(&self, id: &str) -> AppResult<Character> {
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

        // If we removed the active character, clear the active character
        if character_data.active_character_id.as_ref() == Some(&id.to_string()) {
            character_data.active_character_id = None;
        }

        drop(character_data);
        self.save(&self.character_data.read().await.clone())
            .await?;
        info!("Deleted character: {}", character.name);
        Ok(character)
    }

    async fn set_active_character(&self, id: &str) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        // Check if character exists
        if !character_data.characters.iter().any(|c| c.id == id) {
            return Err(AppError::character_management_error(
                "set_active_character",
                &format!("Character with ID '{}' not found", id),
            ));
        }

        // Deactivate all characters
        for character in &mut character_data.characters {
            character.is_active = false;
        }

        // Activate the specified character
        if let Some(character) = character_data.characters.iter_mut().find(|c| c.id == id) {
            character.is_active = true;
            character.last_played = Some(Utc::now());
        }

        character_data.active_character_id = Some(id.to_string());

        drop(character_data);
        self.save(&self.character_data.read().await.clone())
            .await?;
        info!("Set active character: {}", id);
        Ok(())
    }

    async fn ensure_unique_name(&self, name: &str, exclude_id: Option<&str>) -> AppResult<()> {
        let character_data = self.character_data.read().await;

        if character_data
            .characters
            .iter()
            .any(|c| c.name == name && exclude_id.map_or(true, |exclude| c.id != exclude))
        {
            return Err(AppError::validation_error(
                "character_name",
                &format!("Character name '{}' is already taken", name),
            ));
        }

        Ok(())
    }

    async fn clear_all_characters(&self) -> AppResult<()> {
        {
            let mut character_data = self.character_data.write().await;
            character_data.characters.clear();
            character_data.active_character_id = None;
        }

        // Remove data file
        self.persistence.delete().await?;

        debug!("All character data cleared");
        Ok(())
    }

    async fn is_first_character(&self) -> AppResult<bool> {
        let character_data = self.character_data.read().await;
        Ok(character_data.characters.is_empty())
    }
}
