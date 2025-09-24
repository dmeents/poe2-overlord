use crate::errors::{AppError, AppResult};
use crate::models::character::{Character, CharacterData};
use chrono::Utc;
use log::{debug, warn};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Character management constants
const CHARACTER_DATA_FILE_NAME: &str = "characters.json";
const TEMP_FILE_EXTENSION: &str = "tmp";

/// Character repository that handles data persistence
#[derive(Clone)]
pub struct CharacterRepository {
    /// Character data with thread-safe access
    character_data: Arc<RwLock<CharacterData>>,
    /// Path to the character data file
    data_file_path: PathBuf,
}

impl Default for CharacterRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterRepository {
    /// Create a new character repository
    pub fn new() -> Self {
        Self::with_data_directory(None)
    }

    /// Create a new character repository with a custom data directory (mainly for testing)
    pub fn with_data_directory(custom_dir: Option<PathBuf>) -> Self {
        // Use custom directory if provided, otherwise use system config directory
        let config_dir = custom_dir.unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("poe2-overlord")
        });

        // Ensure config directory exists
        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                warn!("Failed to create config directory: {}", e);
            }
        }

        let data_file_path = config_dir.join(CHARACTER_DATA_FILE_NAME);

        let repository = Self {
            character_data: Arc::new(RwLock::new(CharacterData::default())),
            data_file_path,
        };

        // Load existing character data
        if let Err(e) = repository.load_character_data() {
            warn!("Failed to load character data, starting fresh: {}", e);
        }

        repository
    }

    /// Load character data from file
    fn load_character_data(&self) -> AppResult<()> {
        if !self.data_file_path.exists() {
            debug!("No character data file found, starting fresh");
            return Ok(());
        }

        let content = fs::read_to_string(&self.data_file_path).map_err(|e| {
            AppError::file_system_error("Failed to read character data file: {}", &e.to_string())
        })?;

        let data: CharacterData = serde_json::from_str(&content).map_err(|e| {
            AppError::serialization_error("Failed to parse character data: {}", &e.to_string())
        })?;

        {
            let mut character_data = self.character_data.blocking_write();
            *character_data = data;
        }

        debug!("Character data loaded successfully");
        Ok(())
    }

    /// Save character data to file
    async fn save_character_data(&self) -> AppResult<()> {
        let data = {
            let character_data = self.character_data.read().await;
            character_data.clone()
        };

        let content = serde_json::to_string_pretty(&data)
            .map_err(|e| AppError::serialization_error("serialize_character_data", &e.to_string()))?;

        // Write to a temporary file first, then rename to ensure atomic write
        let temp_path = self.data_file_path.with_extension(TEMP_FILE_EXTENSION);
        fs::write(&temp_path, content)
            .map_err(|e| AppError::file_system_error("write_temp_file", &e.to_string()))?;

        fs::rename(&temp_path, &self.data_file_path)
            .map_err(|e| AppError::file_system_error("rename_temp_file", &e.to_string()))?;

        debug!(
            "Character data saved successfully to {:?}",
            self.data_file_path
        );
        Ok(())
    }

    /// Get all characters
    pub async fn get_all_characters(&self) -> Vec<Character> {
        let character_data = self.character_data.read().await;
        character_data.characters.clone()
    }

    /// Get character by ID
    pub async fn get_character(&self, character_id: &str) -> Option<Character> {
        let character_data = self.character_data.read().await;
        character_data
            .characters
            .iter()
            .find(|c| c.id == character_id)
            .cloned()
    }

    /// Get the currently active character
    pub async fn get_active_character(&self) -> Option<Character> {
        let character_data = self.character_data.read().await;
        character_data.active_character_id.as_ref().and_then(|id| {
            character_data
                .characters
                .iter()
                .find(|c| c.id == *id)
                .cloned()
        })
    }

    /// Add a new character
    pub async fn add_character(&self, character: Character) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;
        character_data.characters.push(character);
        drop(character_data);
        self.save_character_data().await
    }

    /// Update an existing character
    pub async fn update_character(&self, character_id: &str, character: Character) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;
        
        if let Some(existing) = character_data.characters.iter_mut().find(|c| c.id == character_id) {
            *existing = character;
            drop(character_data);
            self.save_character_data().await
        } else {
            Err(AppError::character_management_error(
                "update_character",
                &format!("Character with ID '{}' not found", character_id),
            ))
        }
    }

    /// Remove a character by ID
    pub async fn remove_character(&self, character_id: &str) -> AppResult<Character> {
        let mut character_data = self.character_data.write().await;

        let index = character_data
            .characters
            .iter()
            .position(|c| c.id == character_id)
            .ok_or_else(|| {
                AppError::internal_error("Character with ID '{}' not found", &character_id.to_string())
            })?;

        let character = character_data.characters.remove(index);

        // If we removed the active character, set a new active one
        if character_data.active_character_id.as_ref() == Some(&character_id.to_string()) {
            let new_active_id = character_data.characters.first().map(|c| c.id.clone());
            character_data.active_character_id = new_active_id.clone();

            // Activate the new character if there is one
            if let Some(new_active_id) = new_active_id {
                if let Some(new_active) = character_data
                    .characters
                    .iter_mut()
                    .find(|c| c.id == new_active_id)
                {
                    new_active.is_active = true;
                    new_active.last_played = Some(Utc::now());
                }
            }
        }

        drop(character_data);
        self.save_character_data().await?;
        Ok(character)
    }

    /// Set the active character by ID
    pub async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        // Check if character exists
        if !character_data
            .characters
            .iter()
            .any(|c| c.id == character_id)
        {
            return Err(AppError::character_management_error(
                "set_active_character",
                &format!("Character with ID '{}' not found", character_id),
            ));
        }

        // Deactivate all characters
        for character in &mut character_data.characters {
            character.is_active = false;
        }

        // Activate the specified character
        if let Some(character) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            character.is_active = true;
            character.last_played = Some(Utc::now());
        }

        character_data.active_character_id = Some(character_id.to_string());

        drop(character_data);
        self.save_character_data().await
    }

    /// Check if a character name is available
    pub async fn is_name_available(&self, name: &str) -> bool {
        let character_data = self.character_data.read().await;
        !character_data.characters.iter().any(|c| c.name == name)
    }

    /// Clear all character data
    pub async fn clear_all_data(&self) -> AppResult<()> {
        {
            let mut character_data = self.character_data.write().await;
            character_data.characters.clear();
            character_data.active_character_id = None;
        }

        // Remove data file
        if self.data_file_path.exists() {
            fs::remove_file(&self.data_file_path)
                .map_err(|e| AppError::file_system_error("Failed to remove data file: {}", &e.to_string()))?;
        }

        debug!("All character data cleared");
        Ok(())
    }

    /// Get the total number of characters
    pub async fn get_character_count(&self) -> usize {
        let character_data = self.character_data.read().await;
        character_data.characters.len()
    }

    /// Check if there are any characters
    pub async fn has_characters(&self) -> bool {
        let character_data = self.character_data.read().await;
        !character_data.characters.is_empty()
    }
}
