use crate::domain::character_tracking::models::CharacterTrackingData;
use crate::domain::character_tracking::traits::CharacterTrackingRepository;
use crate::errors::AppResult;
use crate::infrastructure::persistence::{
    ScopedPersistenceRepository, ScopedPersistenceRepositoryImpl,
};
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;
use tokio::sync::RwLock;

/// File prefix for character data files
const CHARACTER_DATA_FILE_PREFIX: &str = "character_data_";
/// File suffix for character data files
const CHARACTER_DATA_FILE_SUFFIX: &str = ".json";

/// Implementation of character tracking repository with in-memory caching and persistent storage
#[derive(Clone)]
pub struct CharacterTrackingRepositoryImpl {
    /// In-memory cache of character tracking data
    data_cache: Arc<RwLock<std::collections::HashMap<String, CharacterTrackingData>>>,
    /// Persistent storage for character tracking data
    persistence: ScopedPersistenceRepositoryImpl<CharacterTrackingData, String>,
}

impl CharacterTrackingRepositoryImpl {
    /// Creates a new character tracking repository with persistent storage
    pub fn new() -> AppResult<Self> {
        let config_dir =
            crate::infrastructure::persistence::DirectoryManager::ensure_config_directory()?;
        let persistence = ScopedPersistenceRepositoryImpl::<CharacterTrackingData, String>::new(
            config_dir,
            CHARACTER_DATA_FILE_PREFIX.to_string(),
            CHARACTER_DATA_FILE_SUFFIX.to_string(),
        );

        // Initialize repository with empty in-memory cache
        let repository = Self {
            data_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            persistence,
        };

        Ok(repository)
    }
}

#[async_trait]
impl CharacterTrackingRepository for CharacterTrackingRepositoryImpl {
    /// Saves character tracking data to persistent storage
    async fn save_character_data(&self, data: &CharacterTrackingData) -> AppResult<()> {
        // Update in-memory cache
        {
            let mut cache = self.data_cache.write().await;
            cache.insert(data.character_id.clone(), data.clone());
        }

        // Save to persistent storage
        self.persistence.save_scoped(&data.character_id, data).await
    }

    /// Loads character tracking data from persistent storage
    async fn load_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTrackingData>> {
        // Check in-memory cache first
        {
            let cache = self.data_cache.read().await;
            if let Some(data) = cache.get(character_id) {
                return Ok(Some(data.clone()));
            }
        }

        // Load from persistent storage
        let data = self
            .persistence
            .load_scoped(&character_id.to_string())
            .await?;

        // Update cache if data was found
        if let Some(ref data) = data {
            let mut cache = self.data_cache.write().await;
            cache.insert(character_id.to_string(), data.clone());
        }

        Ok(data)
    }

    /// Checks if character tracking data exists for a character in persistent storage
    async fn character_data_exists(&self, character_id: &str) -> AppResult<bool> {
        self.persistence
            .exists_scoped(&character_id.to_string())
            .await
    }

    /// Deletes all character tracking data for a character from both storage and memory
    async fn delete_character_data(&self, character_id: &str) -> AppResult<()> {
        // Delete from persistent storage
        self.persistence
            .delete_scoped(&character_id.to_string())
            .await?;

        // Clear from in-memory cache
        {
            let mut cache = self.data_cache.write().await;
            cache.remove(character_id);
        }

        debug!(
            "Character tracking data deleted for character: {}",
            character_id
        );
        Ok(())
    }

    /// Finds a zone by location ID
    async fn find_zone(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<crate::domain::character_tracking::models::ZoneStats>> {
        if let Some(data) = self.load_character_data(character_id).await? {
            Ok(data.find_zone(location_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Updates or creates a zone for a character
    async fn upsert_zone(
        &self,
        character_id: &str,
        zone: crate::domain::character_tracking::models::ZoneStats,
    ) -> AppResult<()> {
        let mut data = self
            .load_character_data(character_id)
            .await?
            .unwrap_or_else(|| CharacterTrackingData::new(character_id.to_string()));

        data.upsert_zone(zone);
        self.save_character_data(&data).await
    }

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        if let Some(mut data) = self.load_character_data(character_id).await? {
            if let Some(zone) = data.find_zone_mut(location_id) {
                zone.record_death();
                data.update_summary();
                self.save_character_data(&data).await?;
            }
        }
        Ok(())
    }

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> AppResult<()> {
        if let Some(mut data) = self.load_character_data(character_id).await? {
            if let Some(zone) = data.find_zone_mut(location_id) {
                zone.add_time(seconds);
                data.update_summary();
                self.save_character_data(&data).await?;
            }
        }
        Ok(())
    }

    /// Gets all character IDs that have tracking data
    async fn get_all_character_ids(&self) -> AppResult<Vec<String>> {
        // Get character IDs from in-memory cache first
        let cache_ids: Vec<String> = {
            let cache = self.data_cache.read().await;
            cache.keys().cloned().collect()
        };

        // If cache is empty, try to get from persistent storage
        if cache_ids.is_empty() {
            // For now, return empty vector - in a real implementation,
            // you might want to scan the persistent storage directory
            Ok(Vec::new())
        } else {
            Ok(cache_ids)
        }
    }
}
