use crate::errors::{AppError, AppResult};
use crate::models::character::{
    is_valid_ascendency_for_class, Character, CharacterClass, CharacterData, League,
};
use chrono::Utc;
use log::{debug, info, warn};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Character management constants
const CHARACTER_DATA_FILE_NAME: &str = "characters.json";
const TEMP_FILE_EXTENSION: &str = "tmp";

/// Character manager that handles all character-related operations
#[derive(Clone)]
pub struct CharacterManager {
    /// Character data with thread-safe access
    character_data: Arc<RwLock<CharacterData>>,
    /// Path to the character data file
    data_file_path: PathBuf,
}

impl CharacterManager {
    /// Create a new character manager
    pub fn new() -> Self {
        Self::with_data_directory(None)
    }

    /// Create a new character manager with a custom data directory (mainly for testing)
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

        let service = Self {
            character_data: Arc::new(RwLock::new(CharacterData::default())),
            data_file_path,
        };

        // Load existing character data
        if let Err(e) = service.load_character_data() {
            warn!("Failed to load character data, starting fresh: {}", e);
        }

        service
    }

    /// Load character data from file
    fn load_character_data(&self) -> AppResult<()> {
        if !self.data_file_path.exists() {
            debug!("No character data file found, starting fresh");
            return Ok(());
        }

        let content = fs::read_to_string(&self.data_file_path).map_err(|e| {
            AppError::FileSystem(format!("Failed to read character data file: {}", e))
        })?;

        let data: CharacterData = serde_json::from_str(&content).map_err(|e| {
            AppError::Serialization(format!("Failed to parse character data: {}", e))
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

        let content = serde_json::to_string_pretty(&data).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize character data: {}", e))
        })?;

        // Write to a temporary file first, then rename to ensure atomic write
        let temp_path = self.data_file_path.with_extension(TEMP_FILE_EXTENSION);
        fs::write(&temp_path, content)
            .map_err(|e| AppError::FileSystem(format!("Failed to write temp file: {}", e)))?;

        fs::rename(&temp_path, &self.data_file_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to rename temp file: {}", e)))?;

        debug!(
            "Character data saved successfully to {:?}",
            self.data_file_path
        );
        Ok(())
    }

    /// Create a new character
    pub async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: crate::models::Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character> {
        // Validate that the ascendency belongs to the class
        if !is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::Internal(format!(
                "Ascendency '{:?}' is not valid for class '{:?}'",
                ascendency, class
            )));
        }

        let mut character_data = self.character_data.write().await;

        // Check for duplicate names
        if character_data.characters.iter().any(|c| c.name == name) {
            return Err(AppError::Internal(format!(
                "Character with name '{}' already exists",
                name
            )));
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
        if character_data.characters.is_empty() {
            character_data.active_character_id = Some(character.id.clone());
        }

        character_data.characters.push(character.clone());

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        info!("Created new character: {}", name);
        Ok(character)
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

    /// Set the active character by ID
    pub async fn set_active_character(&self, character_id: &str) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        // Check if character exists
        if !character_data
            .characters
            .iter()
            .any(|c| c.id == character_id)
        {
            return Err(AppError::Internal(format!(
                "Character with ID '{}' not found",
                character_id
            )));
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

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        info!("Set active character: {}", character_id);
        Ok(())
    }

    /// Remove a character by ID
    pub async fn remove_character(&self, character_id: &str) -> AppResult<Character> {
        let mut character_data = self.character_data.write().await;

        let index = character_data
            .characters
            .iter()
            .position(|c| c.id == character_id)
            .ok_or_else(|| {
                AppError::Internal(format!("Character with ID '{}' not found", character_id))
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

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        info!("Removed character: {}", character.name);
        Ok(character)
    }

    /// Get characters sorted by last played (most recent first)
    pub async fn get_characters_by_last_played(&self) -> Vec<Character> {
        let character_data = self.character_data.read().await;
        let mut characters = character_data.characters.clone();

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
        let character_data = self.character_data.read().await;
        character_data
            .characters
            .iter()
            .filter(|c| c.class == *class)
            .cloned()
            .collect()
    }

    /// Get characters by league
    pub async fn get_characters_by_league(&self, league: &League) -> Vec<Character> {
        let character_data = self.character_data.read().await;
        character_data
            .characters
            .iter()
            .filter(|c| c.league == *league)
            .cloned()
            .collect()
    }

    /// Check if a character name is available
    pub async fn is_name_available(&self, name: &str) -> bool {
        let character_data = self.character_data.read().await;
        !character_data.characters.iter().any(|c| c.name == name)
    }

    /// Update a character's information
    pub async fn update_character(
        &self,
        character_id: &str,
        name: String,
        class: CharacterClass,
        ascendency: crate::models::Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<Character> {
        // Validate that the ascendency belongs to the class
        if !is_valid_ascendency_for_class(&ascendency, &class) {
            return Err(AppError::Internal(format!(
                "Ascendency '{:?}' is not valid for class '{:?}'",
                ascendency, class
            )));
        }

        let mut character_data = self.character_data.write().await;

        // Check for duplicate names (excluding the current character) first
        if character_data
            .characters
            .iter()
            .any(|c| c.id != character_id && c.name == name)
        {
            return Err(AppError::Internal(format!(
                "Character with name '{}' already exists",
                name
            )));
        }

        // Find the character to update
        let character = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
            .ok_or_else(|| {
                AppError::Internal(format!("Character with ID '{}' not found", character_id))
            })?;

        // Update the character
        character.name = name.clone();
        character.class = class;
        character.ascendency = ascendency;
        character.league = league;
        character.hardcore = hardcore;
        character.solo_self_found = solo_self_found;

        let updated_character = character.clone();

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        info!("Updated character: {}", name);
        Ok(updated_character)
    }

    /// Update a character's last played timestamp
    pub async fn update_last_played(&self, character_id: &str) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        if let Some(character) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            character.last_played = Some(Utc::now());
        } else {
            return Err(AppError::Internal(format!(
                "Character with ID '{}' not found",
                character_id
            )));
        }

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        Ok(())
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
                .map_err(|e| AppError::FileSystem(format!("Failed to remove data file: {}", e)))?;
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

    /// Update a character's level (system-managed)
    pub async fn update_character_level(&self, character_id: &str, new_level: u32) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        if let Some(character) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            character.level = new_level;
            character.last_played = Some(Utc::now());
        } else {
            return Err(AppError::Internal(format!(
                "Character with ID '{}' not found",
                character_id
            )));
        }

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        debug!("Updated character {} level to {}", character_id, new_level);
        Ok(())
    }

    /// Increment a character's death count (system-managed)
    pub async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        if let Some(character) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
            character.death_count += 1;
            character.last_played = Some(Utc::now());
        } else {
            return Err(AppError::Internal(format!(
                "Character with ID '{}' not found",
                character_id
            )));
        }

        // Save the updated data
        drop(character_data);
        self.save_character_data().await?;

        debug!("Incremented character {} death count", character_id);
        Ok(())
    }

}
