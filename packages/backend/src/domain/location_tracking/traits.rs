use crate::domain::location_tracking::models::SceneType;
use crate::domain::location_tracking::models::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingSession,
    LocationTrackingStats, SceneTypeConfig,
};
use crate::domain::log_analysis::models::SceneChangeEvent;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait LocationTrackingService: Send + Sync {
    async fn process_scene_content(&self, content: &str) -> AppResult<Option<SceneChangeEvent>>;

    async fn get_current_location_state(&self) -> AppResult<LocationState>;

    async fn reset_tracking(&self) -> AppResult<()>;

    async fn get_current_scene(&self) -> AppResult<Option<String>>;

    async fn get_current_act(&self) -> AppResult<Option<String>>;

    async fn start_session(&self) -> AppResult<()>;

    async fn end_session(&self) -> AppResult<()>;

    async fn is_session_active(&self) -> bool;

    async fn get_current_session(&self) -> AppResult<Option<LocationTrackingSession>>;

    async fn get_stats(&self) -> AppResult<LocationTrackingStats>;

    async fn get_location_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;

    async fn clear_history(&self) -> AppResult<()>;

    async fn update_scene_type_config(&self, config: SceneTypeConfig) -> AppResult<()>;

    async fn get_config(&self) -> AppResult<LocationTrackingConfig>;

    async fn update_config(&self, config: LocationTrackingConfig) -> AppResult<()>;
}

#[async_trait]
pub trait LocationStateRepository: Send + Sync {
    async fn save_state(&self, state: &LocationState) -> AppResult<()>;

    async fn load_state(&self) -> AppResult<Option<LocationState>>;

    async fn update_state(&self, state: &LocationState) -> AppResult<()>;

    async fn clear_state(&self) -> AppResult<()>;
}

#[async_trait]
pub trait LocationTrackingSessionRepository: Send + Sync {
    async fn save_session(&self, session: &LocationTrackingSession) -> AppResult<()>;

    async fn load_session(&self, session_id: &str) -> AppResult<Option<LocationTrackingSession>>;

    async fn get_active_session(&self) -> AppResult<Option<LocationTrackingSession>>;

    async fn update_session(&self, session: &LocationTrackingSession) -> AppResult<()>;

    async fn end_current_session(&self) -> AppResult<()>;

    async fn get_all_sessions(&self) -> AppResult<Vec<LocationTrackingSession>>;
}

#[async_trait]
pub trait LocationTrackingStatsRepository: Send + Sync {
    async fn save_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;

    async fn load_stats(&self) -> AppResult<LocationTrackingStats>;

    async fn update_stats(&self, stats: &LocationTrackingStats) -> AppResult<()>;

    async fn increment_scene_change_count(&self, scene_type: SceneType) -> AppResult<()>;

    async fn update_most_visited_scenes(&self, scene_name: &str) -> AppResult<()>;

    async fn reset_stats(&self) -> AppResult<()>;
}

#[async_trait]
pub trait LocationHistoryRepository: Send + Sync {
    async fn save_history_entry(&self, entry: &LocationHistoryEntry) -> AppResult<()>;

    async fn load_history(&self) -> AppResult<Vec<LocationHistoryEntry>>;

    async fn add_history_entry(&self, entry: LocationHistoryEntry) -> AppResult<()>;

    async fn clear_history(&self) -> AppResult<()>;

    async fn get_history_by_scene_type(
        &self,
        scene_type: SceneType,
    ) -> AppResult<Vec<LocationHistoryEntry>>;

    async fn get_history_in_range(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Vec<LocationHistoryEntry>>;
}

pub trait SceneTypeDetector: Send + Sync {
    fn detect_scene_type(&self, content: &str) -> SceneType;

    fn is_hideout_content(&self, content: &str) -> bool;

    fn is_act_content(&self, content: &str) -> bool;

    fn is_zone_content(&self, content: &str) -> bool;

    fn get_scene_type_config(&self) -> &SceneTypeConfig;

    fn update_scene_type_config(&mut self, config: SceneTypeConfig);
}
