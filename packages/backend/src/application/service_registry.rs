use crate::domain::character::service::CharacterService;
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::game_monitoring::{traits::GameMonitoringService, GameMonitoringServiceImpl};
use crate::domain::time_tracking::{service::TimeTrackingServiceImpl, traits::TimeTrackingService};
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::LogAnalyzer;
use crate::infrastructure::tauri::EventDispatcher;
use crate::infrastructure::{
    monitoring::ProcessMonitorImpl, tauri::TauriGameMonitoringEventPublisher,
};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::Manager;

pub struct ServiceInitializer;

impl ServiceInitializer {
    pub fn initialize_services(
        app: &mut tauri::App,
    ) -> Result<ServiceInstances, Box<dyn std::error::Error>> {
        info!("Starting service initialization...");

        debug!("Initializing ConfigurationService...");
        let config_service = Arc::new(
            ConfigurationServiceImpl::new().expect("Failed to create configuration service"),
        );
        app.manage(config_service.clone());
        debug!("ConfigurationService managed successfully");

        debug!("Initializing EventDispatcher...");
        let event_broadcaster = Arc::new(EventDispatcher::new());
        app.manage(event_broadcaster.clone());
        debug!("EventDispatcher managed successfully");

        debug!("Initializing CharacterService...");
        let character_service = CharacterService::new().map_err(|e| {
            error!("Failed to initialize CharacterService: {}", e);
            e
        })?;
        let character_arc = Arc::new(character_service);
        app.manage(character_arc.clone());
        debug!("CharacterService managed successfully");

        debug!("Initializing TimeTrackingService...");
        let time_tracking_service = TimeTrackingServiceImpl::new().map_err(|e| {
            error!("Failed to initialize TimeTrackingService: {}", e);
            e
        })?;
        let time_tracking_arc = Arc::new(time_tracking_service) as Arc<dyn TimeTrackingService>;
        app.manage(time_tracking_arc.clone());
        debug!("TimeTrackingService managed successfully");

        debug!("Initializing ServerMonitor...");
        let server_status_manager = ServerMonitor::new(event_broadcaster.clone());
        let server_status_arc = Arc::new(server_status_manager);
        app.manage(server_status_arc.clone());
        debug!("ServerMonitor managed successfully");

        debug!("Initializing LogAnalyzer...");
        let log_monitor_service = LogAnalyzer::new(
            "".to_string(),
            server_status_arc.clone(),
            character_arc.clone(),
        );
        let log_monitor_arc = Arc::new(log_monitor_service);
        app.manage(log_monitor_arc.clone());
        debug!("LogAnalyzer managed successfully");

        debug!("Initializing Game Monitoring services...");

        let process_detector = Arc::new(ProcessMonitorImpl::new());

        let event_publisher = Arc::new(TauriGameMonitoringEventPublisher::new(
            app.get_webview_window("main")
                .unwrap_or_else(|| panic!("Main window not found during service initialization")),
        ));

        let game_monitoring_service = Arc::new(GameMonitoringServiceImpl::new(
            time_tracking_arc.clone(),
            event_publisher.clone(),
            process_detector.clone(),
        ));

        app.manage(game_monitoring_service.clone());
        debug!("Game Monitoring services managed successfully");

        info!("Service initialization completed successfully");

        Ok(ServiceInstances {
            config_service,
            event_broadcaster,
            character_service: character_arc,
            time_tracking_service: time_tracking_arc,
            log_monitor: log_monitor_arc,
            server_status: server_status_arc,
            game_monitoring_service,
        })
    }
}

#[derive(Clone)]
pub struct ServiceInstances {
    pub config_service: Arc<ConfigurationServiceImpl>,
    pub event_broadcaster: Arc<EventDispatcher>,
    pub character_service: Arc<CharacterService>,
    pub time_tracking_service: Arc<dyn TimeTrackingService>,
    pub log_monitor: Arc<LogAnalyzer>,
    pub server_status: Arc<ServerMonitor>,
    pub game_monitoring_service: Arc<dyn GameMonitoringService>,
}
