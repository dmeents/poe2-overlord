use crate::services::{
    config::ConfigService, log_monitor::LogMonitorService, server_status::ServerStatusManager,
    time_tracking::TimeTrackingService,
};
use log::info;
use std::sync::Arc;
use tauri::Manager;

pub struct ServiceInitializer;

impl ServiceInitializer {
    pub fn initialize_services(app: &mut tauri::App) -> Result<ServiceInstances, Box<dyn std::error::Error>> {
        info!("Starting service initialization...");

        // Initialize configuration service
        info!("Initializing ConfigService...");
        let config_service = ConfigService::new(&app.handle());
        app.manage(config_service.clone());
        info!("ConfigService managed successfully");

        // Initialize time tracking service
        info!("Initializing TimeTrackingService...");
        let time_tracking_service = TimeTrackingService::new();
        let time_tracking_arc = Arc::new(time_tracking_service);
        app.manage(time_tracking_arc.clone());
        info!("TimeTrackingService managed successfully");

        // Initialize server status manager
        info!("Initializing ServerStatusManager...");
        let server_status_manager = ServerStatusManager::new();
        let server_status_arc = Arc::new(server_status_manager);
        app.manage(server_status_arc.clone());
        info!("ServerStatusManager managed successfully");

        // Initialize log monitor service
        info!("Initializing LogMonitorService...");
        let log_path = config_service.get_poe_client_log_path();
        let log_monitor_service = LogMonitorService::new(log_path);
        let log_monitor_arc = Arc::new(log_monitor_service);
        app.manage(log_monitor_arc.clone());
        info!("LogMonitorService managed successfully");

        info!("Service initialization completed successfully");

        Ok(ServiceInstances {
            config_service,
            log_monitor: log_monitor_arc,
            time_tracking: time_tracking_arc,
            server_status: server_status_arc,
        })
    }
}

pub struct ServiceInstances {
    pub config_service: ConfigService,
    pub log_monitor: Arc<LogMonitorService>,
    pub time_tracking: Arc<TimeTrackingService>,
    pub server_status: Arc<ServerStatusManager>,
}
