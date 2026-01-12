use async_trait::async_trait;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::infrastructure::file_management::FileService;

use super::models::{CharacterData, CharactersIndex};
use super::traits::CharacterRepository;

/// File-based implementation of the CharacterRepository trait.
///
/// This repository handles persistence using the new infrastructure layer:
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
    fn index_path(&self) -> PathBuf {
        self.data_dir.join("characters.json")
    }

    /// Gets the path to an individual character data file
    fn character_path(&self, character_id: &str) -> PathBuf {
        self.data_dir
            .join(format!("character_data_{}.json", character_id))
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {
    /// Loads the characters index from the characters.json file
    async fn load_characters_index(&self) -> Result<CharactersIndex, AppError> {
        let index_path = self.index_path();
        let index = FileService::read_json_optional(&index_path)
            .await?
            .unwrap_or_default();
        Ok(index)
    }

    /// Saves the characters index to the characters.json file
    async fn save_characters_index(&self, index: &CharactersIndex) -> Result<(), AppError> {
        let index_path = self.index_path();
        FileService::write_json(&index_path, index).await
    }

    /// Loads character data from a character_data_{id}.json file
    async fn load_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
        let character_path = self.character_path(character_id);

        match FileService::read_json_optional(&character_path).await? {
            Some(data) => Ok(data),
            None => Err(AppError::internal_error(
                "load_character_data",
                &format!("Character with ID '{}' not found", character_id),
            )),
        }
    }

    /// Saves character data to a character_data_{id}.json file
    async fn save_character_data(&self, character_data: &CharacterData) -> Result<(), AppError> {
        let character_path = self.character_path(&character_data.id);
        FileService::write_json(&character_path, character_data).await
    }

    /// Deletes a character data file
    async fn delete_character_data(&self, character_id: &str) -> Result<(), AppError> {
        let character_path = self.character_path(character_id);
        FileService::delete(&character_path).await
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
        let character_path = self.character_path(character_id);
        Ok(FileService::exists(&character_path).await?)
    }

    /// Lists all character data files in the data directory.
    /// Returns character IDs extracted from filenames (character_data_{id}.json pattern).
    async fn list_character_data_files(&self) -> Result<Vec<String>, AppError> {
        let mut character_ids = Vec::new();

        // Read directory entries
        let mut entries = tokio::fs::read_dir(&self.data_dir).await.map_err(|e| {
            AppError::file_system_error(
                "list_character_files",
                &format!(
                    "Failed to read data directory {:?}: {}",
                    self.data_dir, e
                ),
            )
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::file_system_error(
                "list_character_files",
                &format!("Failed to read directory entry: {}", e),
            )
        })? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Match pattern: character_data_{id}.json
            if file_name_str.starts_with("character_data_") && file_name_str.ends_with(".json") {
                // Extract ID from filename
                let id = file_name_str
                    .trim_start_matches("character_data_")
                    .trim_end_matches(".json")
                    .to_string();
                character_ids.push(id);
            }
        }

        Ok(character_ids)
    }
}
