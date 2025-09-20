use crate::services::{
    registry::ServiceRegistryImpl,
    traits::ServiceRegistry,
};
use log::{debug, info};
use std::sync::Arc;
use tauri::Manager;

/// Service registry initializer that uses dependency injection
pub struct ServiceRegistryInitializer;

impl ServiceRegistryInitializer {
    /// Initialize all services using the registry pattern
    pub fn initialize_services(
        app: &mut tauri::App,
    ) -> Result<Arc<dyn ServiceRegistry>, Box<dyn std::error::Error>> {
        info!("Starting service registry initialization...");

        // Create the service registry
        debug!("Creating service registry...");
        let registry = Arc::new(ServiceRegistryImpl::new(app.handle()));
        debug!("Service registry created successfully");

        // Register services with Tauri's state management
        debug!("Registering services with Tauri...");
        
        // Register character service
        let character_service = registry.get_character_service();
        app.manage(character_service.clone());
        debug!("Character service registered");

        // Register time tracking service
        let time_tracking_service = registry.get_time_tracking_service();
        app.manage(time_tracking_service.clone());
        debug!("Time tracking service registered");

        // Register configuration service
        let configuration_service = registry.get_configuration_service();
        app.manage(configuration_service.clone());
        debug!("Configuration service registered");

        // Register event service
        let event_service = registry.get_event_service();
        app.manage(event_service.clone());
        debug!("Event service registered");

        // Register server monitoring service
        let server_monitoring_service = registry.get_server_monitoring_service();
        app.manage(server_monitoring_service.clone());
        debug!("Server monitoring service registered");

        // Register log analysis service
        let log_analysis_service = registry.get_log_analysis_service();
        app.manage(log_analysis_service.clone());
        debug!("Log analysis service registered");

        // Register the registry itself for advanced use cases
        app.manage(registry.clone());
        debug!("Service registry registered");

        info!("Service registry initialization completed successfully");

        Ok(registry)
    }

    /// Initialize services for testing with custom registry
    pub fn initialize_services_for_testing(
        app: &mut tauri::App,
        registry: Arc<dyn ServiceRegistry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting test service registry initialization...");

        // Register all services
        app.manage(registry.get_character_service());
        app.manage(registry.get_time_tracking_service());
        app.manage(registry.get_configuration_service());
        app.manage(registry.get_event_service());
        app.manage(registry.get_server_monitoring_service());
        app.manage(registry.get_log_analysis_service());
        app.manage(registry);

        info!("Test service registry initialization completed successfully");
        Ok(())
    }
}
