use crate::domain::character::service::CharacterService;
use crate::domain::character::traits::CharacterService as CharacterServiceTrait;
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::configuration::traits::ConfigurationService;
use crate::domain::time_tracking::{service::TimeTrackingServiceImpl, traits::TimeTrackingService};
use crate::infrastructure::tauri::{EventDispatcher, EventService};
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::LogAnalyzer;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::services::traits::ServiceRegistry;
use log::error;
use std::sync::Arc;

/// Service registry implementation that manages all service dependencies
pub struct ServiceRegistryImpl {
    character_service: Arc<dyn CharacterServiceTrait>,
    time_tracking_service: Arc<dyn TimeTrackingService>,
    configuration_service: Arc<dyn ConfigurationService>,
    event_service: Arc<dyn EventService>,
    server_monitoring_service: Arc<dyn ServerMonitoringService>,
    log_analysis_service: Arc<dyn LogAnalysisService>,
}

impl ServiceRegistryImpl {
    /// Create a new service registry with all services initialized
    pub fn new(_app_handle: &tauri::AppHandle) -> Result<Self, crate::errors::AppError> {
        // Initialize core services first (no dependencies)
        let configuration_service = Arc::new(
            ConfigurationServiceImpl::new().expect("Failed to create configuration service"),
        );
        let event_service = Arc::new(EventDispatcher::new());

        // Initialize character service (no dependencies)
        let character_service = CharacterService::new().map_err(|e| {
            error!("Failed to initialize CharacterService: {}", e);
            e
        })?;
        let character_service = Arc::new(character_service);

        // Initialize server monitoring service (depends on event service)
        let server_monitoring_service = Arc::new(ServerMonitor::new(event_service.clone()));

        // Initialize time tracking service
        let time_tracking_service = TimeTrackingServiceImpl::new().map_err(|e| {
            error!("Failed to initialize TimeTrackingService: {}", e);
            e
        })?;
        let time_tracking_service = Arc::new(time_tracking_service) as Arc<dyn TimeTrackingService>;

        // Initialize log analysis service (depends on character service and server monitoring)
        // We need to get the config path asynchronously, so we'll use a default for now
        // and let the log analyzer update it when the config is loaded
        let log_analysis_service = Arc::new(LogAnalyzer::new(
            "".to_string(), // Will be updated when config is loaded
            server_monitoring_service.clone(),
            character_service.clone(),
        ));

        Ok(Self {
            character_service,
            time_tracking_service,
            configuration_service,
            event_service,
            server_monitoring_service,
            log_analysis_service,
        })
    }

    /// Create a new service registry with custom services (useful for testing)
    pub fn with_services(
        character_service: Arc<dyn CharacterServiceTrait>,
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
    fn get_character_service(&self) -> Arc<dyn CharacterServiceTrait> {
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
