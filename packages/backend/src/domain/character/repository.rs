use async_trait::async_trait;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::infrastructure::persistence::{
    PersistenceRepository, PersistenceRepositoryImpl, ScopedPersistenceRepository,
    ScopedPersistenceRepositoryImpl,
};

use super::models::{CharacterData, CharactersIndex};
use super::traits::CharacterRepository;

/// File-based implementation of the CharacterRepository trait.
///
/// This repository handles persistence using the infrastructure layer:
/// - characters.json: Contains the characters index
/// - character_data_{id}.json: Individual character data files
pub struct CharacterRepositoryImpl {
    characters_index_repo: PersistenceRepositoryImpl<CharactersIndex>,
    character_data_repo: ScopedPersistenceRepositoryImpl<CharacterData, String>,
}

impl CharacterRepositoryImpl {
    /// Creates a new CharacterRepositoryImpl instance
    pub fn new(data_dir: PathBuf) -> Result<Self, AppError> {
        // Ensure data directory exists
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).map_err(|e| {
                AppError::file_system_error(
                    "create_data_dir",
                    &format!("Failed to create data directory: {}", e),
                )
            })?;
        }

        // Create repositories
        let characters_index_path = data_dir.join("characters.json");
        let characters_index_repo = PersistenceRepositoryImpl::new(characters_index_path);

        let character_data_repo = ScopedPersistenceRepositoryImpl::new(
            data_dir,
            "character_data_".to_string(),
            ".json".to_string(),
        );

        Ok(Self {
            characters_index_repo,
            character_data_repo,
        })
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {
    /// Loads the characters index from the characters.json file
    async fn load_characters_index(&self) -> Result<CharactersIndex, AppError> {
        self.characters_index_repo.load_or_default().await
    }

    /// Saves the characters index to the characters.json file
    async fn save_characters_index(&self, index: &CharactersIndex) -> Result<(), AppError> {
        self.characters_index_repo.save(index).await
    }

    /// Loads character data from a character_data_{id}.json file
    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
        let character_id_string = character_id.to_string();
        match self
            .character_data_repo
            .load_scoped(&character_id_string)
            .await?
        {
            Some(data) => Ok(data),
            None => Err(AppError::internal_error(
                "load_character_data",
                &format!("Character with ID '{}' not found", character_id),
            )),
        }
    }

    /// Saves character data to a character_data_{id}.json file
    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError> {
        self.character_data_repo
            .save_scoped(&character_data.id, character_data)
            .await
    }

    /// Deletes a character data file
    async fn delete_character_data(&self, character_id: &str) -> Result<(), AppError> {
        let character_id_string = character_id.to_string();
        self.character_data_repo
            .delete_scoped(&character_id_string)
            .await
    }

    /// Loads all character data files
    async fn load_all_characters(&self) -> Result<Vec<CharacterData>, AppError> {
        let index = self.load_characters_index().await?;
        let mut characters = Vec::new();

        for character_id in &index.character_ids {
            match self.load_character_data(character_id).await {
                Ok(character) => characters.push(character),
                Err(e) => {
                    // Log error but continue loading other characters
                    log::warn!("Failed to load character {}: {}", character_id, e);
                }
            }
        }

        Ok(characters)
    }

    /// Checks if a character data file exists
    async fn character_exists(&self, character_id: &str) -> Result<bool, AppError> {
        let character_id_string = character_id.to_string();
        self.character_data_repo
            .exists_scoped(&character_id_string)
            .await
    }
}
