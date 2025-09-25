use crate::domain::time_tracking::{
    events::TimeTrackingEvent,
    models::{LocationSession, LocationStats, LocationType},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for publishing time tracking domain events
#[async_trait]
pub trait TimeTrackingEventPublisher: Send + Sync {
    /// Publish a time tracking domain event
    async fn publish_event(&self, event: TimeTrackingEvent) -> AppResult<()>;

    /// Subscribe to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;
}

/// Trait for the time tracking domain service
#[async_trait]
pub trait TimeTrackingService: Send + Sync {
    /// Start a new time tracking session for a character at a location
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()>;

    /// End an active time tracking session
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Get all active sessions for a character
    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    /// Get all completed sessions for a character
    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    /// Get all location statistics for a character
    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats>;

    /// Get total play time for a character (in seconds)
    async fn get_total_play_time(&self, character_id: &str) -> u64;

    /// Get total play time since process start for a character (in seconds)
    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64;

    /// Get total hideout time for a character (in seconds)
    async fn get_total_hideout_time(&self, character_id: &str) -> u64;

    /// Get the last known location for a character
    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession>;

    /// Clear all time tracking data for a character
    async fn clear_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Load all existing character time tracking data
    async fn load_all_character_data(&self) -> AppResult<()>;

    /// Save all character time tracking data
    async fn save_all_character_data(&self) -> AppResult<()>;

    /// Load time tracking data for a specific character
    async fn load_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Save time tracking data for a specific character
    async fn save_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Subscribe to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;

    /// Set the POE process start time
    async fn set_poe_process_start_time(&self, start_time: chrono::DateTime<chrono::Utc>);

    /// Clear the POE process start time
    async fn clear_poe_process_start_time(&self);

    /// End all active sessions globally (when game process stops)
    async fn end_all_active_sessions_global(&self) -> AppResult<()>;
}

/// Trait for time tracking data persistence and management
#[async_trait]
pub trait TimeTrackingRepository: Send + Sync {
    // Persistence operations
    async fn save_character_data(
        &self,
        data: &crate::domain::time_tracking::models::CharacterTimeTrackingData,
    ) -> AppResult<()>;

    async fn load_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<crate::domain::time_tracking::models::CharacterTimeTrackingData>>;

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;
    async fn character_data_exists(&self, character_id: &str) -> AppResult<bool>;

    // Data management
    async fn get_active_sessions(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationSession>>;
    async fn get_completed_sessions(&self, character_id: &str) -> AppResult<Vec<LocationSession>>;
    async fn get_stats_cache(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationStats>>;

    // Query operations
    async fn find_session_by_location(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationSession>>;
    async fn get_last_known_location(
        &self,
        character_id: &str,
    ) -> AppResult<Option<LocationSession>>;
    async fn get_location_stats(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationStats>>;

    // Data manipulation
    async fn start_session(&self, character_id: &str, session: LocationSession) -> AppResult<()>;
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;
    async fn update_stats(
        &self,
        character_id: &str,
        location_id: &str,
        stats: LocationStats,
    ) -> AppResult<()>;

    // Aggregation
    async fn calculate_total_play_time(&self, character_id: &str) -> AppResult<u64>;
    async fn calculate_total_hideout_time(&self, character_id: &str) -> AppResult<u64>;
    async fn get_top_locations(
        &self,
        character_id: &str,
        limit: usize,
    ) -> AppResult<Vec<LocationStats>>;

    // Business rules
    async fn validate_no_overlapping_sessions(
        &self,
        character_id: &str,
        new_session: &LocationSession,
    ) -> AppResult<()>;
}

/// Trait for time tracking session validation
pub trait TimeTrackingValidator: Send + Sync {
    /// Validate that a session can be started
    fn validate_session_start(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    /// Validate that a session can be ended
    fn validate_session_end(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    /// Validate session data integrity
    fn validate_session_data(&self, session: &LocationSession) -> AppResult<()>;
}

/// Trait for time tracking statistics calculation
pub trait TimeTrackingStatsCalculator: Send + Sync {
    /// Calculate location statistics from completed sessions
    fn calculate_location_stats(
        &self,
        character_id: &str,
        completed_sessions: &[LocationSession],
    ) -> Vec<LocationStats>;

    /// Update existing location statistics with a new session
    fn update_location_stats(&self, stats: &mut LocationStats, session: &LocationSession);

    /// Calculate total play time from completed sessions
    fn calculate_total_play_time(&self, completed_sessions: &[LocationSession]) -> u64;

    /// Calculate total hideout time from completed sessions
    fn calculate_total_hideout_time(&self, completed_sessions: &[LocationSession]) -> u64;
}
