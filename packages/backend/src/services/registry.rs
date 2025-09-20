use crate::services::{
    character_manager::CharacterManager,
    character_session_tracker::CharacterSessionTracker,
    configuration_manager::ConfigurationManager,
    event_dispatcher::EventDispatcher,
    log_analyzer::LogAnalyzer,
    server_monitor::ServerMonitor,
    traits::{
        CharacterService, ConfigurationService, EventService, LogAnalysisService,
        ServerMonitoringService, ServiceRegistry, TimeTrackingService,
    },
};
use std::sync::Arc;

/// Service registry implementation that manages all service dependencies
pub struct ServiceRegistryImpl {
    character_service: Arc<dyn CharacterService>,
    time_tracking_service: Arc<dyn TimeTrackingService>,
    configuration_service: Arc<dyn ConfigurationService>,
    event_service: Arc<dyn EventService>,
    server_monitoring_service: Arc<dyn ServerMonitoringService>,
    log_analysis_service: Arc<dyn LogAnalysisService>,
}

impl ServiceRegistryImpl {
    /// Create a new service registry with all services initialized
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        // Initialize core services first (no dependencies)
        let configuration_service = Arc::new(ConfigurationManager::new(app_handle));
        let event_service = Arc::new(EventDispatcher::new());

        // Initialize character service (no dependencies)
        let character_service = Arc::new(CharacterManager::new());

        // Initialize server monitoring service (depends on event service)
        let server_monitoring_service = Arc::new(ServerMonitor::new(event_service.clone()));

        // Initialize time tracking service (depends on character service)
        let time_tracking_service = Arc::new(CharacterSessionTracker::with_character_manager(
            character_service.clone(),
        ));

        // Initialize log analysis service (depends on character service and server monitoring)
        let log_analysis_service = Arc::new(LogAnalyzer::new(
            configuration_service.get_config().poe_client_log_path,
            server_monitoring_service.clone(),
            character_service.clone(),
        ));

        Self {
            character_service,
            time_tracking_service,
            configuration_service,
            event_service,
            server_monitoring_service,
            log_analysis_service,
        }
    }

    /// Create a new service registry with custom services (useful for testing)
    pub fn with_services(
        character_service: Arc<dyn CharacterService>,
        time_tracking_service: Arc<dyn TimeTrackingService>,
        configuration_service: Arc<dyn ConfigurationService>,
        event_service: Arc<dyn EventService>,
        server_monitoring_service: Arc<dyn ServerMonitoringService>,
        log_analysis_service: Arc<dyn LogAnalysisService>,
    ) -> Self {
        Self {
            character_service,
            time_tracking_service,
            configuration_service,
            event_service,
            server_monitoring_service,
            log_analysis_service,
        }
    }
}

impl ServiceRegistry for ServiceRegistryImpl {
    fn get_character_service(&self) -> Arc<dyn CharacterService> {
        self.character_service.clone()
    }

    fn get_time_tracking_service(&self) -> Arc<dyn TimeTrackingService> {
        self.time_tracking_service.clone()
    }

    fn get_configuration_service(&self) -> Arc<dyn ConfigurationService> {
        self.configuration_service.clone()
    }

    fn get_event_service(&self) -> Arc<dyn EventService> {
        self.event_service.clone()
    }

    fn get_server_monitoring_service(&self) -> Arc<dyn ServerMonitoringService> {
        self.server_monitoring_service.clone()
    }

    fn get_log_analysis_service(&self) -> Arc<dyn LogAnalysisService> {
        self.log_analysis_service.clone()
    }
}

impl Default for ServiceRegistryImpl {
    fn default() -> Self {
        // This is a placeholder - in practice, you'd need an AppHandle
        // For testing purposes, we'll create a minimal registry
        // In a real app, you should use ServiceRegistryImpl::new(app_handle) instead
        panic!("ServiceRegistryImpl::default() should not be called. Use ServiceRegistryImpl::new(app_handle) instead.")
    }
}
