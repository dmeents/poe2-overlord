use crate::domain::character::models::{
    is_valid_ascendency_for_class, Ascendency, Character, CharacterClass, CharacterData,
    CharacterUpdateParams, League,
};
use crate::domain::character::traits::CharacterService as CharacterServiceTrait;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
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

/// Character service that handles business logic for character management
#[derive(Clone)]
pub struct CharacterService {
    /// Character data with thread-safe access
    character_data: Arc<RwLock<CharacterData>>,
    /// Path to the character data file
    data_file_path: PathBuf,
}

impl CharacterService {
    /// Create a new character service
    pub fn new() -> Self {
        // Use system config directory
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord");

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

        let content = serde_json::to_string_pretty(&data).map_err(|e| {
            AppError::serialization_error("serialize_character_data", &e.to_string())
        })?;

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

    /// Add a new character
    async fn add_character(&self, character: Character) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;
        character_data.characters.push(character);
        drop(character_data);
        self.save_character_data().await
    }

    /// Update an existing character
    async fn update_character_internal(
        &self,
        character_id: &str,
        character: Character,
    ) -> AppResult<()> {
        let mut character_data = self.character_data.write().await;

        if let Some(existing) = character_data
            .characters
            .iter_mut()
            .find(|c| c.id == character_id)
        {
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
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    ascendency, class
                ),
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
        let characters = self.get_all_characters().await;
        if characters.is_empty() {
            // We need to add the character first, then set it as active
            self.add_character(character.clone()).await?;
            self.set_active_character(&character.id).await?;
        } else {
            self.add_character(character.clone()).await?;
        }

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
        self.save_character_data().await?;
        info!("Set active character: {}", character_id);
        Ok(())
    }

    /// Delete a character by ID
    pub async fn delete_character(&self, character_id: &str) -> AppResult<Character> {
        let mut character_data = self.character_data.write().await;

        let index = character_data
            .characters
            .iter()
            .position(|c| c.id == character_id)
            .ok_or_else(|| {
                AppError::internal_error(
                    "Character with ID '{}' not found",
                    &character_id.to_string(),
                )
            })?;

        let character = character_data.characters.remove(index);

        // If we removed the active character, clear the active character
        if character_data.active_character_id.as_ref() == Some(&character_id.to_string()) {
            character_data.active_character_id = None;
        }

        drop(character_data);
        self.save_character_data().await?;
        info!("Deleted character: {}", character.name);
        Ok(character)
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
                &format!(
                    "Ascendency '{:?}' is not valid for class '{:?}'",
                    params.ascendency, params.class
                ),
            ));
        }

        // Get the existing character
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
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

        self.update_character_internal(character_id, character.clone())
            .await?;

        info!("Updated character: {}", params.name);
        Ok(character)
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
            fs::remove_file(&self.data_file_path).map_err(|e| {
                AppError::file_system_error("Failed to remove data file: {}", &e.to_string())
            })?;
        }

        debug!("All character data cleared");
        Ok(())
    }

    /// Update a character's level (system-managed)
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
        self.update_character_internal(character_id, character)
            .await
    }

    /// Increment a character's death count (system-managed)
    pub async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        let mut character = self.get_character(character_id).await.ok_or_else(|| {
            AppError::character_management_error(
                "increment_character_deaths",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        character.death_count += 1;
        character.last_played = Some(Utc::now());
        self.update_character_internal(character_id, character)
            .await
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

    async fn increment_character_deaths(&self, character_id: &str) -> AppResult<()> {
        CharacterService::increment_character_deaths(self, character_id).await
    }
}
