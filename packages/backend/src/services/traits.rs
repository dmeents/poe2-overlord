use crate::errors::AppResult;
use crate::models::events::LogEvent;
use crate::services::server_monitor::ServerStatus;
use crate::domain::character::traits::CharacterService;
use crate::domain::configuration::traits::ConfigurationService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

// ConfigurationService trait moved to domain::configuration::traits

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
