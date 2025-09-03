use crate::services::{
    configuration_manager::ConfigurationManager, event_dispatcher::EventDispatcher,
    log_analyzer::LogAnalyzer, server_monitor::ServerMonitor, session_tracker::SessionTracker,
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

        // Initialize configuration manager
        debug!("Initializing ConfigurationManager...");
        let config_service = ConfigurationManager::new(&app.handle());
        app.manage(config_service.clone());
        debug!("ConfigurationManager managed successfully");

        // Initialize event dispatcher
        debug!("Initializing EventDispatcher...");
        let event_broadcaster = Arc::new(EventDispatcher::new());
        app.manage(event_broadcaster.clone());
        debug!("EventDispatcher managed successfully");

        // Initialize session tracker
        debug!("Initializing SessionTracker...");
        let time_tracking_service = SessionTracker::new();
        let time_tracking_arc = Arc::new(time_tracking_service);
        app.manage(time_tracking_arc.clone());
        debug!("SessionTracker managed successfully");

        // Initialize server monitor
        debug!("Initializing ServerMonitor...");
        let server_status_manager = ServerMonitor::new(event_broadcaster.clone());
        let server_status_arc = Arc::new(server_status_manager);
        app.manage(server_status_arc.clone());
        debug!("ServerMonitor managed successfully");

        // Initialize log analyzer
        debug!("Initializing LogAnalyzer...");
        let log_path = config_service.get_poe_client_log_path();
        let log_monitor_service = LogAnalyzer::new(log_path, server_status_arc.clone());
        let log_monitor_arc = Arc::new(log_monitor_service);
        app.manage(log_monitor_arc.clone());
        debug!("LogAnalyzer managed successfully");

        info!("Service initialization completed successfully");

        Ok(ServiceInstances {
            config_service,
            event_broadcaster,
            log_monitor: log_monitor_arc,
            time_tracking: time_tracking_arc,
            server_status: server_status_arc,
        })
    }
}

#[derive(Clone)]
pub struct ServiceInstances {
    pub config_service: ConfigurationManager,
    pub event_broadcaster: Arc<EventDispatcher>,
    pub log_monitor: Arc<LogAnalyzer>,
    pub time_tracking: Arc<SessionTracker>,
    pub server_status: Arc<ServerMonitor>,
}
