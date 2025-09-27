use async_trait::async_trait;
use serde_json;
use std::path::PathBuf;
use tokio::fs;

use crate::errors::AppError;

use super::models::{CharacterData, CharactersIndex};
use super::traits::CharacterRepository;

/// File-based implementation of the CharacterRepository trait.
///
/// This repository handles persistence using the new file structure:
/// - characters.json: Contains the characters index
/// - character_data_{id}.json: Individual character data files
pub struct CharacterRepositoryImpl {
    data_dir: PathBuf,
}

impl CharacterRepositoryImpl {
    /// Creates a new CharacterRepositoryImpl instance
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    /// Gets the path to the characters index file
    fn get_characters_index_path(&self) -> PathBuf {
        self.data_dir.join("characters.json")
    }

    /// Gets the path to a character data file
    fn get_character_data_path(&self, character_id: &str) -> PathBuf {
        self.data_dir
            .join(format!("character_data_{}.json", character_id))
    }

    /// Ensures the data directory exists
    async fn ensure_data_dir(&self) -> Result<(), AppError> {
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).await.map_err(|e| {
                AppError::file_system_error("create_data_dir", &format!("Failed to create data directory: {}", e))
            })?;
        }
        Ok(())
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {
    /// Loads the characters index from the characters.json file
    async fn load_characters_index(&self) -> Result<CharactersIndex, AppError> {
        let index_path = self.get_characters_index_path();
        
        if !index_path.exists() {
            return Ok(CharactersIndex::new());
        }

        let content = fs::read_to_string(&index_path)
            .await
            .map_err(|e| AppError::file_system_error("read_characters_index", &format!("Failed to read characters index: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AppError::serialization_error("parse_characters_index", &format!("Failed to parse characters index: {}", e)))
    }

    /// Saves the characters index to the characters.json file
    async fn save_characters_index(&self, index: &CharactersIndex) -> Result<(), AppError> {
        self.ensure_data_dir().await?;
        
        let index_path = self.get_characters_index_path();
        let content = serde_json::to_string_pretty(index)
            .map_err(|e| AppError::serialization_error("serialize_characters_index", &format!("Failed to serialize characters index: {}", e)))?;

        fs::write(&index_path, content)
            .await
            .map_err(|e| AppError::file_system_error("write_characters_index", &format!("Failed to write characters index: {}", e)))?;

        Ok(())
    }

    /// Loads character data from a character_data_{id}.json file
    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
        let character_path = self.get_character_data_path(character_id);
        
        if !character_path.exists() {
            return Err(AppError::character_management_error("load_character_data", &format!("Character with ID '{}' not found", character_id)));
        }

        let content = fs::read_to_string(&character_path)
            .await
            .map_err(|e| AppError::file_system_error("read_character_data", &format!("Failed to read character data: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AppError::serialization_error("parse_character_data", &format!("Failed to parse character data: {}", e)))
    }

    /// Saves character data to a character_data_{id}.json file
    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError> {
        self.ensure_data_dir().await?;
        
        let character_path = self.get_character_data_path(&character_data.id);
        let content = serde_json::to_string_pretty(character_data)
            .map_err(|e| AppError::serialization_error("serialize_character_data", &format!("Failed to serialize character data: {}", e)))?;

        fs::write(&character_path, content)
            .await
            .map_err(|e| AppError::file_system_error("write_character_data", &format!("Failed to write character data: {}", e)))?;

        Ok(())
    }

    /// Deletes a character data file
    async fn delete_character_data(&self, character_id: &str) -> Result<(), AppError> {
        let character_path = self.get_character_data_path(character_id);
        
        if !character_path.exists() {
            return Err(AppError::character_management_error("load_character_data", &format!("Character with ID '{}' not found", character_id)));
        }

        fs::remove_file(&character_path)
            .await
            .map_err(|e| AppError::file_system_error("delete_character_data", &format!("Failed to delete character data: {}", e)))?;

        Ok(())
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
        let character_path = self.get_character_data_path(character_id);
        Ok(character_path.exists())
    }
}
