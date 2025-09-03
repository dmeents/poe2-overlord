use crate::services::{
    config::ConfigService,
    log_monitor::LogMonitorService,
    server_status::ServerStatusManager,
    time_tracking::TimeTrackingService,
};
use log::{debug, info};
use std::sync::Arc;
use tauri::Manager;

pub struct ServiceInitializer;

impl ServiceInitializer {
    pub fn initialize_services(
        app: &mut tauri::App,
    ) -> Result<ServiceInstances, Box<dyn std::error::Error>> {
        info!("Starting service initialization...");

        // Initialize configuration service
        debug!("Initializing ConfigService...");
        let config_service = ConfigService::new(&app.handle());
        app.manage(config_service.clone());
        debug!("ConfigService managed successfully");

        // Initialize time tracking service
        debug!("Initializing TimeTrackingService...");
        let time_tracking_service = TimeTrackingService::new();
        let time_tracking_arc = Arc::new(time_tracking_service);
        app.manage(time_tracking_arc.clone());
        debug!("TimeTrackingService managed successfully");

        // Initialize server status manager
        debug!("Initializing ServerStatusManager...");
        let server_status_manager = ServerStatusManager::new();
        let server_status_arc = Arc::new(server_status_manager);
        app.manage(server_status_arc.clone());
        debug!("ServerStatusManager managed successfully");

        // Initialize log monitor service
        debug!("Initializing LogMonitorService...");
        let log_path = config_service.get_poe_client_log_path();
        let log_monitor_service = LogMonitorService::new(log_path, server_status_arc.clone());
        let log_monitor_arc = Arc::new(log_monitor_service);
        app.manage(log_monitor_arc.clone());
        debug!("LogMonitorService managed successfully");

        info!("Service initialization completed successfully");

        Ok(ServiceInstances {
            config_service,
            log_monitor: log_monitor_arc,
            time_tracking: time_tracking_arc,
            server_status: server_status_arc,
        })
    }
}

#[derive(Clone)]
pub struct ServiceInstances {
    pub config_service: ConfigService,
    pub log_monitor: Arc<LogMonitorService>,
    pub time_tracking: Arc<TimeTrackingService>,
    pub server_status: Arc<ServerStatusManager>,
}
