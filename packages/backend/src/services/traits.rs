use crate::errors::AppResult;
use crate::models::events::LogEvent;
use crate::models::{
    AppConfig, LocationSession, LocationStats, TimeTrackingEvent,
};
use crate::services::server_monitor::ServerStatus;
use crate::domain::character::traits::CharacterService;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;


/// Trait for time tracking operations
#[async_trait]
pub trait TimeTrackingService: Send + Sync {
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: crate::models::LocationType,
    ) -> AppResult<()>;

    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()>;
    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession>;
    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession>;
    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats>;
    async fn get_total_play_time(&self, character_id: &str) -> u64;
    async fn clear_character_data(&self, character_id: &str) -> AppResult<()>;
    async fn load_all_character_data(&self) -> AppResult<()>;
    async fn save_all_character_data(&self) -> AppResult<()>;

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent>;
}

/// Trait for configuration management
pub trait ConfigurationService: Send + Sync {
    fn get_config(&self) -> AppConfig;
    fn update_config(&self, config: AppConfig) -> AppResult<()>;
    fn reset_to_defaults(&self) -> AppResult<()>;
    fn load_config(&self) -> AppResult<()>;
    fn save_config(&self) -> AppResult<()>;
}

/// Trait for event dispatching
pub trait EventService: Send + Sync {
    fn subscribe_to_log_events(&self) -> broadcast::Receiver<LogEvent>;
    fn subscribe_to_ping_events(&self) -> broadcast::Receiver<ServerStatus>;
    fn broadcast_log_event(
        &self,
        event: LogEvent,
    ) -> Result<(), broadcast::error::SendError<LogEvent>>;
    fn broadcast_ping_event(
        &self,
        event: ServerStatus,
    ) -> Result<(), broadcast::error::SendError<ServerStatus>>;
}

/// Trait for server monitoring
#[async_trait]
pub trait ServerMonitoringService: Send + Sync {
    async fn get_current_status(&self) -> ServerStatus;
    async fn update_status(&self, status: ServerStatus) -> AppResult<()>;
    async fn save_status(&self) -> AppResult<()>;
    async fn load_status(&self) -> AppResult<()>;
}

/// Trait for log analysis
#[async_trait]
pub trait LogAnalysisService: Send + Sync {
    async fn start_monitoring(&self) -> AppResult<()>;
    async fn stop_monitoring(&self) -> AppResult<()>;
    async fn get_log_file_size(&self) -> AppResult<u64>;
    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>>;
    fn subscribe_to_events(&self) -> broadcast::Receiver<LogEvent>;
}

/// Service registry trait for dependency injection
pub trait ServiceRegistry: Send + Sync {
    fn get_character_service(&self) -> Arc<dyn CharacterService>;
    fn get_time_tracking_service(&self) -> Arc<dyn TimeTrackingService>;
    fn get_configuration_service(&self) -> Arc<dyn ConfigurationService>;
    fn get_event_service(&self) -> Arc<dyn EventService>;
    fn get_server_monitoring_service(&self) -> Arc<dyn ServerMonitoringService>;
    fn get_log_analysis_service(&self) -> Arc<dyn LogAnalysisService>;
}
