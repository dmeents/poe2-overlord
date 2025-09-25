use crate::domain::location_tracking::models::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingSession,
    LocationTrackingStats, SceneTypeConfig,
};
use crate::models::events::SceneChangeEvent;
use crate::models::scene_type::SceneType;
use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait for location tracking service operations
#[async_trait]
pub trait LocationTrackingService: Send + Sync {
    /// Process raw scene content and return a validated scene change event if it represents an actual change
    async fn process_scene_content(&self, content: &str) -> AppResult<Option<SceneChangeEvent>>;
    
    /// Get the current location state
    async fn get_current_location_state(&self) -> AppResult<LocationState>;
    
    /// Reset the location tracking state
    async fn reset_tracking(&self) -> AppResult<()>;
    
    /// Get the current scene name
    async fn get_current_scene(&self) -> AppResult<Option<String>>;
    
    /// Get the current act name
    async fn get_current_act(&self) -> AppResult<Option<String>>;
    
    /// Start a new location tracking session
    async fn start_session(&self) -> AppResult<()>;
    
    /// End the current location tracking session
    async fn end_session(&self) -> AppResult<()>;
    
    /// Check if a session is active
    async fn is_session_active(&self) -> bool;
    
    /// Get the current session
    async fn get_current_session(&self) -> AppResult<Option<LocationTrackingSession>>;
    
    /// Get location tracking statistics
    async fn get_stats(&self) -> AppResult<LocationTrackingStats>;
    
    /// Get location history
    async fn get_location_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;
    
    /// Clear location history
    async fn clear_history(&self) -> AppResult<()>;
    
    /// Update scene type configuration
    async fn update_scene_type_config(&self, config: SceneTypeConfig) -> AppResult<()>;
    
    /// Get current configuration
    async fn get_config(&self) -> AppResult<LocationTrackingConfig>;
    
    /// Update configuration
    async fn update_config(&self, config: LocationTrackingConfig) -> AppResult<()>;
}

/// Trait for location state repository operations
#[async_trait]
pub trait LocationStateRepository: Send + Sync {
    /// Save location state
    async fn save_state(&self, state: &LocationState) -> AppResult<()>;
    
    /// Load location state
    async fn load_state(&self) -> AppResult<Option<LocationState>>;
    
    /// Update location state
    async fn update_state(&self, state: &LocationState) -> AppResult<()>;
    
    /// Clear location state
    async fn clear_state(&self) -> AppResult<()>;
}

/// Trait for location tracking session repository operations
#[async_trait]
pub trait LocationTrackingSessionRepository: Send + Sync {
    /// Save location tracking session
    async fn save_session(&self, session: &LocationTrackingSession) -> AppResult<()>;
    
    /// Load session by ID
    async fn load_session(&self, session_id: &str) -> AppResult<Option<LocationTrackingSession>>;
    
    /// Get current active session
    async fn get_active_session(&self) -> AppResult<Option<LocationTrackingSession>>;
    
    /// Update session
    async fn update_session(&self, session: &LocationTrackingSession) -> AppResult<()>;
    
    /// End current session
    async fn end_current_session(&self) -> AppResult<()>;
    
    /// Get all sessions
    async fn get_all_sessions(&self) -> AppResult<Vec<LocationTrackingSession>>;
}

/// Trait for location tracking statistics repository operations
#[async_trait]
pub trait LocationTrackingStatsRepository: Send + Sync {
    /// Save location tracking statistics
    async fn save_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;
    
    /// Load location tracking statistics
    async fn load_stats(&self) -> AppResult<LocationTrackingStats>;
    
    /// Update statistics
    async fn update_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;
    
    /// Increment scene change counter
    async fn increment_scene_change_count(&self, scene_type: SceneType) -> AppResult<()>;
    
    /// Update most visited scenes
    async fn update_most_visited_scenes(&self, scene_name: &str) -> AppResult<()>;
    
    /// Reset statistics
    async fn reset_stats(&self) -> AppResult<()>;
}

/// Trait for location history repository operations
#[async_trait]
pub trait LocationHistoryRepository: Send + Sync {
    /// Save location history entry
    async fn save_history_entry(&self, entry: &LocationHistoryEntry) -> AppResult<()>;
    
    /// Load location history
    async fn load_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;
    
    /// Add history entry
    async fn add_history_entry(&self, entry: LocationHistoryEntry) -> AppResult<()>;
    
    /// Clear location history
    async fn clear_history(&self) -> AppResult<()>;
    
    /// Get history entries by scene type
    async fn get_history_by_scene_type(&self, scene_type: SceneType) -> AppResult<Vec<LocationHistoryEntry>>;
    
    /// Get history entries within time range
    async fn get_history_in_range(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Vec<LocationHistoryEntry>>;
}

/// Trait for scene type detection
pub trait SceneTypeDetector: Send + Sync {
    /// Detect the scene type based on content
    fn detect_scene_type(&self, content: &str) -> SceneType;
    
    /// Check if content represents a hideout
    fn is_hideout_content(&self, content: &str) -> bool;
    
    /// Check if content represents an act
    fn is_act_content(&self, content: &str) -> bool;
    
    /// Check if content represents a zone
    fn is_zone_content(&self, content: &str) -> bool;
    
    /// Get scene type configuration
    fn get_scene_type_config(&self) -> &SceneTypeConfig;
    
    /// Update scene type configuration
    fn update_scene_type_config(&mut self, config: SceneTypeConfig);
}
