use crate::domain::character::traits::CharacterService;
use crate::domain::configuration::traits::ConfigurationService;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::infrastructure::tauri::EventService;
use std::sync::Arc;

/// Service registry trait for dependency injection
pub trait ServiceRegistry: Send + Sync {
    fn get_character_service(&self) -> Arc<dyn CharacterService>;
    fn get_time_tracking_service(&self) -> Arc<dyn TimeTrackingService>;
    fn get_configuration_service(&self) -> Arc<dyn ConfigurationService>;
    fn get_event_service(&self) -> Arc<dyn EventService>;
    fn get_server_monitoring_service(&self) -> Arc<dyn ServerMonitoringService>;
    fn get_log_analysis_service(&self) -> Arc<dyn LogAnalysisService>;
}
