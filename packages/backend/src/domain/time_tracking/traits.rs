use crate::domain::time_tracking::{
    events::TimeTrackingEvent,
    models::{LocationSession, LocationStats, LocationType},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tauri::WebviewWindow;
use tokio::sync::broadcast;

#[async_trait]
pub trait TimeTrackingEventPublisher: Send + Sync {
    async fn publish_event(&self, event: TimeTrackingEvent) -> AppResult<()>;

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;
}

#[async_trait]
pub trait TimeTrackingService: Send + Sync {
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()>;

    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;

    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession>;

    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats>;

    async fn get_total_play_time(&self, character_id: &str) -> u64;

    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64;

    async fn get_total_hideout_time(&self, character_id: &str) -> u64;

    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession>;

    async fn clear_character_data(&self, character_id: &str) -> AppResult<()>;

    async fn load_all_character_data(&self) -> AppResult<()>;

    async fn save_all_character_data(&self) -> AppResult<()>;

    async fn load_character_data(&self, character_id: &str) -> AppResult<()>;

    async fn save_character_data(&self, character_id: &str) -> AppResult<()>;

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;

    async fn set_poe_process_start_time(&self, start_time: chrono::DateTime<chrono::Utc>);

    async fn clear_poe_process_start_time(&self);

    async fn end_all_active_sessions_global(&self) -> AppResult<()>;

    async fn start_frontend_event_emission(&self, window: WebviewWindow);
}

#[async_trait]
pub trait TimeTrackingRepository: Send + Sync {
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

    async fn get_active_sessions(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationSession>>;
    async fn get_completed_sessions(&self, character_id: &str) -> AppResult<Vec<LocationSession>>;
    async fn get_stats_cache(
        &self,
        character_id: &str,
    ) -> AppResult<std::collections::HashMap<String, LocationStats>>;

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

    async fn start_session(&self, character_id: &str, session: LocationSession) -> AppResult<()>;
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;
    async fn update_stats(
        &self,
        character_id: &str,
        location_id: &str,
        stats: LocationStats,
    ) -> AppResult<()>;

    async fn calculate_total_play_time(&self, character_id: &str) -> AppResult<u64>;
    async fn calculate_total_hideout_time(&self, character_id: &str) -> AppResult<u64>;
    async fn get_top_locations(
        &self,
        character_id: &str,
        limit: usize,
    ) -> AppResult<Vec<LocationStats>>;

    async fn validate_no_overlapping_sessions(
        &self,
        character_id: &str,
        new_session: &LocationSession,
    ) -> AppResult<()>;
}

pub trait TimeTrackingValidator: Send + Sync {
    fn validate_session_start(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    fn validate_session_end(
        &self,
        character_id: &str,
        location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()>;

    fn validate_session_data(&self, session: &LocationSession) -> AppResult<()>;
}

pub trait TimeTrackingStatsCalculator: Send + Sync {
    fn calculate_location_stats(
        &self,
        character_id: &str,
        completed_sessions: &[LocationSession],
    ) -> Vec<LocationStats>;

    fn update_location_stats(&self, stats: &mut LocationStats, session: &LocationSession);

    fn calculate_total_play_time(&self, completed_sessions: &[LocationSession]) -> u64;

    fn calculate_total_hideout_time(&self, completed_sessions: &[LocationSession]) -> u64;
}
