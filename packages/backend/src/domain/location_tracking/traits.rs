use crate::domain::location_tracking::models::SceneType;
use crate::domain::location_tracking::models::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingSession,
    LocationTrackingStats, SceneTypeConfig,
};
use crate::domain::log_analysis::models::SceneChangeEvent;
use crate::errors::AppResult;
use async_trait::async_trait;

/// Main service trait for location tracking functionality
/// Provides high-level operations for processing scene changes and managing tracking state
#[async_trait]
pub trait LocationTrackingService: Send + Sync {
    /// Processes raw scene content and returns a scene change event if detected
    /// Returns None if no actual scene change occurred
    async fn process_scene_content(&self, content: &str) -> AppResult<Option<SceneChangeEvent>>;

    /// Gets the current location state (scene, act, timestamps)
    async fn get_current_location_state(&self) -> AppResult<LocationState>;

    /// Resets the tracking state to initial values
    async fn reset_tracking(&self) -> AppResult<()>;

    /// Gets the current scene name if available
    async fn get_current_scene(&self) -> AppResult<Option<String>>;

    /// Gets the current act name if available
    async fn get_current_act(&self) -> AppResult<Option<String>>;

    /// Starts a new tracking session
    async fn start_session(&self) -> AppResult<()>;

    /// Ends the current tracking session
    async fn end_session(&self) -> AppResult<()>;

    /// Checks if there's an active tracking session
    async fn is_session_active(&self) -> bool;

    /// Gets the current active session if any
    async fn get_current_session(&self) -> AppResult<Option<LocationTrackingSession>>;

    /// Gets aggregated statistics across all sessions
    async fn get_stats(&self) -> AppResult<LocationTrackingStats>;

    /// Gets the complete location history
    async fn get_location_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;

    /// Clears all location history
    async fn clear_history(&self) -> AppResult<()>;

    /// Updates the scene type detection configuration
    async fn update_scene_type_config(&self, config: SceneTypeConfig) -> AppResult<()>;

    /// Gets the current tracking configuration
    async fn get_config(&self) -> AppResult<LocationTrackingConfig>;

    /// Updates the tracking configuration
    async fn update_config(&self, config: LocationTrackingConfig) -> AppResult<()>;
}

/// Repository trait for persisting and retrieving location state
/// Handles storage of current location information
#[async_trait]
pub trait LocationStateRepository: Send + Sync {
    /// Saves the current location state to storage
    async fn save_state(&self, state: &LocationState) -> AppResult<()>;

    /// Loads the current location state from storage
    async fn load_state(&self) -> AppResult<Option<LocationState>>;

    /// Updates an existing location state in storage
    async fn update_state(&self, state: &LocationState) -> AppResult<()>;

    /// Clears the stored location state
    async fn clear_state(&self) -> AppResult<()>;
}

/// Repository trait for managing tracking sessions
/// Handles storage and retrieval of session data
#[async_trait]
pub trait LocationTrackingSessionRepository: Send + Sync {
    /// Saves a new tracking session
    async fn save_session(&self, session: &LocationTrackingSession) -> AppResult<()>;

    /// Loads a specific session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<LocationTrackingSession>>;

    /// Gets the currently active session
    async fn get_active_session(&self) -> AppResult<Option<LocationTrackingSession>>;

    /// Updates an existing session
    async fn update_session(&self, session: &LocationTrackingSession) -> AppResult<()>;

    /// Ends the current active session
    async fn end_current_session(&self) -> AppResult<()>;

    /// Gets all stored sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<LocationTrackingSession>>;
}

/// Repository trait for managing tracking statistics
/// Handles storage and updates of aggregated statistics
#[async_trait]
pub trait LocationTrackingStatsRepository: Send + Sync {
    /// Saves statistics to storage
    async fn save_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;

    /// Loads statistics from storage
    async fn load_stats(&self) -> AppResult<LocationTrackingStats>;

    /// Updates existing statistics
    async fn update_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;

    /// Increments the count for a specific scene type
    async fn increment_scene_change_count(&self, scene_type: SceneType) -> AppResult<()>;

    /// Updates the most visited scenes counter
    async fn update_most_visited_scenes(&self, scene_name: &str) -> AppResult<()>;

    /// Resets all statistics to default values
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Repository trait for managing location history
/// Handles storage and retrieval of historical location data
#[async_trait]
pub trait LocationHistoryRepository: Send + Sync {
    /// Saves a single history entry
    async fn save_history_entry(&self, entry: &LocationHistoryEntry) -> AppResult<()>;

    /// Loads all history entries
    async fn load_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;

    /// Adds a new history entry
    async fn add_history_entry(&self, entry: LocationHistoryEntry) -> AppResult<()>;

    /// Clears all history entries
    async fn clear_history(&self) -> AppResult<()>;

    /// Gets history entries filtered by scene type
    async fn get_history_by_scene_type(
        &self,
        scene_type: SceneType,
    ) -> AppResult<Vec<LocationHistoryEntry>>;

    /// Gets history entries within a time range
    async fn get_history_in_range(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Vec<LocationHistoryEntry>>;
}

/// Trait for detecting scene types from game content
/// Uses keyword matching to categorize different types of scenes
pub trait SceneTypeDetector: Send + Sync {
    /// Detects the scene type from raw content
    fn detect_scene_type(&self, content: &str) -> SceneType;

    /// Checks if content indicates hideout scene
    fn is_hideout_content(&self, content: &str) -> bool;

    /// Checks if content indicates act scene
    fn is_act_content(&self, content: &str) -> bool;

    /// Checks if content indicates zone scene
    fn is_zone_content(&self, content: &str) -> bool;

    /// Gets the current scene type configuration
    fn get_scene_type_config(&self) -> &SceneTypeConfig;

    /// Updates the scene type configuration
    fn update_scene_type_config(&mut self, config: SceneTypeConfig);
}
