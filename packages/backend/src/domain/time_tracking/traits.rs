use crate::domain::time_tracking::{
    events::TimeTrackingEvent,
    models::{LocationSession, LocationStats, LocationType},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tauri::WebviewWindow;
use tokio::sync::broadcast;

/// Trait for publishing time tracking events to subscribers
#[async_trait]
pub trait TimeTrackingEventPublisher: Send + Sync {
    /// Publishes a time tracking event to all subscribers
    async fn publish_event(&self, event: TimeTrackingEvent) -> AppResult<()>;

    /// Returns a receiver for subscribing to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;
}

/// Main service trait for time tracking operations
#[async_trait]
pub trait TimeTrackingService: Send + Sync {
    /// Starts a new location session for a character
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()>;

    /// Ends an active location session for a character
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Gets all currently active sessions for a character
    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    /// Gets all completed sessions for a character
    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    /// Gets all location statistics for a character
    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats>;

    /// Gets total play time for a character across all completed sessions
    async fn get_total_play_time(&self, character_id: &str) -> u64;

    /// Gets total play time since the PoE process started
    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64;

    /// Gets total time spent in hideouts for a character
    async fn get_total_hideout_time(&self, character_id: &str) -> u64;

    /// Gets the last known location for a character
    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession>;

    /// Clears all time tracking data for a character
    async fn clear_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Loads time tracking data for all characters
    async fn load_all_character_data(&self) -> AppResult<()>;

    /// Saves time tracking data for all characters
    async fn save_all_character_data(&self) -> AppResult<()>;

    /// Loads time tracking data for a specific character
    async fn load_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Saves time tracking data for a specific character
    async fn save_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Returns a receiver for subscribing to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;

    /// Sets the PoE process start time for time calculations
    async fn set_poe_process_start_time(&self, start_time: chrono::DateTime<chrono::Utc>);

    /// Clears the PoE process start time
    async fn clear_poe_process_start_time(&self);

    /// Ends all active sessions globally (used when game exits)
    async fn end_all_active_sessions_global(&self) -> AppResult<()>;

    /// Starts emitting time tracking events to the frontend
    async fn start_frontend_event_emission(&self, window: WebviewWindow);
}

/// Repository trait for time tracking data persistence and retrieval
#[async_trait]
pub trait TimeTrackingRepository: Send + Sync {
    /// Saves character time tracking data to persistent storage
    async fn save_character_data(
        &self,
        data: &crate::domain::time_tracking::models::CharacterTimeTrackingData,
    ) -> AppResult<()>;

    /// Loads character time tracking data from persistent storage
    async fn load_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<crate::domain::time_tracking::models::CharacterTimeTrackingData>>;

    /// Deletes all time tracking data for a character
    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;

    /// Checks if time tracking data exists for a character
    async fn character_data_exists(&self, character_id: &str) -> AppResult<bool>;

    /// Gets all active sessions for a character (in-memory cache)
    async fn get_active_sessions(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationSession>>;

    /// Gets all completed sessions for a character
    async fn get_completed_sessions(&self, character_id: &str) -> AppResult<Vec<LocationSession>>;

    /// Gets location statistics cache for a character
    async fn get_stats_cache(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationStats>>;

    /// Finds an active session by location ID
    async fn find_session_by_location(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationSession>>;

    /// Gets the most recent location for a character
    async fn get_last_known_location(
        &self,
        character_id: &str,
    ) -> AppResult<Option<LocationSession>>;

    /// Gets statistics for a specific location
    async fn get_location_stats(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationStats>>;

    /// Starts a new session in the repository
    async fn start_session(&self, character_id: &str, session: LocationSession) -> AppResult<()>;

    /// Ends a session and moves it to completed sessions
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    /// Updates location statistics in the cache
    async fn update_stats(
        &self,
        character_id: &str,
        location_id: &str,
        stats: LocationStats,
    ) -> AppResult<()>;

    /// Calculates total play time from completed sessions
    async fn calculate_total_play_time(&self, character_id: &str) -> AppResult<u64>;

    /// Calculates total hideout time from completed sessions
    async fn calculate_total_hideout_time(&self, character_id: &str) -> AppResult<u64>;

    /// Gets top locations by time spent
    async fn get_top_locations(
        &self,
        character_id: &str,
        limit: usize,
    ) -> AppResult<Vec<LocationStats>>;

    /// Validates that a new session doesn't overlap with existing ones
    async fn validate_no_overlapping_sessions(
        &self,
        character_id: &str,
        new_session: &LocationSession,
    ) -> AppResult<()>;
}

/// Trait for validating time tracking operations and data integrity
pub trait TimeTrackingValidator: Send + Sync {
    /// Validates that a session can be started (no conflicts, valid data)
    fn validate_session_start(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    /// Validates that a session can be ended (session exists and is active)
    fn validate_session_end(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    /// Validates session data integrity
    fn validate_session_data(&self, session: &LocationSession) -> AppResult<()>;
}

/// Trait for calculating time tracking statistics and aggregations
pub trait TimeTrackingStatsCalculator: Send + Sync {
    /// Calculates location statistics from completed sessions
    fn calculate_location_stats(
        &self,
        character_id: &str,
        completed_sessions: &[LocationSession],
    ) -> Vec<LocationStats>;

    /// Updates location stats with a new session
    fn update_location_stats(&self, stats: &mut LocationStats, session: &LocationSession);

    /// Calculates total play time from completed sessions
    fn calculate_total_play_time(&self, completed_sessions: &[LocationSession]) -> u64;

    /// Calculates total hideout time from completed sessions
    fn calculate_total_hideout_time(&self, completed_sessions: &[LocationSession]) -> u64;
}
