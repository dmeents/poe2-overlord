use crate::domain::character_tracking::models::{
    CharacterTrackingData, LocationState, LocationType, ZoneStats,
};
use crate::domain::log_analysis::models::SceneChangeEvent;
use crate::errors::AppResult;
use async_trait::async_trait;
use tauri::WebviewWindow;
use tokio::sync::broadcast;

/// Main service trait for character tracking functionality
/// Combines location tracking and time tracking into a unified service
#[async_trait]
pub trait CharacterTrackingService: Send + Sync {
    /// Processes raw scene content and returns a scene change event if detected
    /// Returns None if no actual scene change occurred
    async fn process_scene_content(
        &self,
        content: &str,
        character_id: &str,
    ) -> AppResult<Option<SceneChangeEvent>>;

    /// Processes raw scene content with zone level and returns a scene change event if detected
    /// Returns None if no actual scene change occurred
    async fn process_scene_content_with_zone_level(
        &self,
        content: &str,
        character_id: &str,
        zone_level: u32,
    ) -> AppResult<Option<SceneChangeEvent>>;

    /// Gets the current location state for a character (scene, act, timestamps)
    async fn get_current_location(&self, character_id: &str) -> AppResult<Option<LocationState>>;

    /// Gets complete character tracking data for a character
    async fn get_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTrackingData>>;

    /// Deletes all character tracking data for a character
    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Enters a zone for a character
    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
        is_town: bool,
    ) -> AppResult<()>;

    /// Leaves a zone for a character
    async fn leave_zone(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> AppResult<()>;

    /// Returns a receiver for subscribing to character tracking events
    async fn subscribe_to_events(
        &self,
    ) -> AppResult<broadcast::Receiver<crate::domain::events::AppEvent>>;

    /// Starts emitting character tracking events to the frontend
    async fn start_frontend_event_emission(&self, window: WebviewWindow);

    /// Finalizes all active zones for all characters during application shutdown
    /// Stops all active timers and calculates final durations
    async fn finalize_all_active_zones(&self) -> AppResult<()>;
}

/// Repository trait for character tracking data persistence and retrieval
#[async_trait]
pub trait CharacterTrackingRepository: Send + Sync {
    /// Saves character tracking data to persistent storage
    async fn save_character_data(&self, data: &CharacterTrackingData) -> AppResult<()>;

    /// Loads character tracking data from persistent storage
    async fn load_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTrackingData>>;

    /// Checks if character tracking data exists for a character
    async fn character_data_exists(&self, character_id: &str) -> AppResult<bool>;

    /// Deletes all character tracking data for a character
    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Finds a zone by location ID
    async fn find_zone(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<ZoneStats>>;

    /// Updates or creates a zone for a character
    async fn upsert_zone(&self, character_id: &str, zone: ZoneStats) -> AppResult<()>;

    /// Records a death in a specific zone
    async fn record_death(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> AppResult<()>;

    /// Gets all character IDs that have tracking data
    async fn get_all_character_ids(&self) -> AppResult<Vec<String>>;
}

/// Trait for detecting scene types from game content
/// Uses zone configuration for reliable act and town detection
pub trait SceneTypeDetector: Send + Sync {
    /// Detects the scene type from raw content
    fn detect_scene_type(&self, content: &str) -> LocationType;

    /// Checks if content indicates hideout scene
    fn is_hideout_content(&self, content: &str) -> bool;

    /// Checks if content indicates act scene (should be filtered out)
    fn is_act_content(&self, content: &str) -> bool;

    /// Checks if content indicates zone scene (all non-hideout content)
    fn is_zone_content(&self, content: &str) -> bool;
}

/// Trait for publishing character tracking events
#[async_trait]
pub trait CharacterTrackingEventPublisher: Send + Sync {
    /// Publishes a character tracking event to all subscribers
    async fn publish_event(&self, event: CharacterTrackingEvent) -> AppResult<()>;

    /// Returns a receiver for subscribing to character tracking events
    async fn subscribe_to_events(
        &self,
    ) -> AppResult<broadcast::Receiver<crate::domain::events::AppEvent>>;
}

/// Character tracking events
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CharacterTrackingEvent {
    /// A character entered a zone
    ZoneEntered(ZoneEntered),
    /// A character left a zone
    ZoneLeft(ZoneLeft),
    /// Zone statistics were updated
    StatsUpdated(StatsUpdated),
    /// All character tracking data was cleared for a character
    CharacterTrackingDataCleared(CharacterTrackingDataCleared),
    /// Character tracking data was loaded from storage
    CharacterTrackingDataLoaded(CharacterTrackingDataLoaded),
    /// Character tracking data was saved to storage
    CharacterTrackingDataSaved(CharacterTrackingDataSaved),
}

/// Event fired when a character enters a zone
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZoneEntered {
    /// Character who entered the zone
    pub character_id: String,
    /// Zone that was entered
    pub zone: ZoneStats,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl ZoneEntered {
    /// Creates a new zone entered event with current timestamp
    pub fn new(character_id: String, zone: ZoneStats) -> Self {
        Self {
            character_id,
            zone,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}

/// Event fired when a character leaves a zone
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZoneLeft {
    /// Character who left the zone
    pub character_id: String,
    /// Zone that was left
    pub zone: ZoneStats,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl ZoneLeft {
    /// Creates a new zone left event with current timestamp
    pub fn new(character_id: String, zone: ZoneStats) -> Self {
        Self {
            character_id,
            zone,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}

/// Event fired when zone statistics are updated
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsUpdated {
    /// Character whose stats were updated
    pub character_id: String,
    /// The updated zone statistics
    pub zone: ZoneStats,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl StatsUpdated {
    /// Creates a new stats updated event with current timestamp
    pub fn new(character_id: String, zone: ZoneStats) -> Self {
        Self {
            character_id,
            zone,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}

/// Event fired when all character tracking data is cleared for a character
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterTrackingDataCleared {
    /// Character whose data was cleared
    pub character_id: String,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl CharacterTrackingDataCleared {
    /// Creates a new data cleared event with current timestamp
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}

/// Event fired when character tracking data is loaded from storage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterTrackingDataLoaded {
    /// Character whose data was loaded
    pub character_id: String,
    /// Number of zones loaded
    pub zones_count: usize,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl CharacterTrackingDataLoaded {
    /// Creates a new data loaded event with current timestamp
    pub fn new(character_id: String, zones_count: usize) -> Self {
        Self {
            character_id,
            zones_count,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}

/// Event fired when character tracking data is saved to storage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterTrackingDataSaved {
    /// Character whose data was saved
    pub character_id: String,
    /// Number of zones saved
    pub zones_count: usize,
    /// When this event occurred
    pub occurred_at: std::time::SystemTime,
}

impl CharacterTrackingDataSaved {
    /// Creates a new data saved event with current timestamp
    pub fn new(character_id: String, zones_count: usize) -> Self {
        Self {
            character_id,
            zones_count,
            occurred_at: std::time::SystemTime::now(),
        }
    }
}
