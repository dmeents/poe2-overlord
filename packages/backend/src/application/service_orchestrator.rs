use log::{error, info};
use std::sync::Arc;
use tauri::WebviewWindow;

use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::LogAnalyzer;
use crate::infrastructure::runtime::{RuntimeManager, TaskManager};

pub fn start_game_process_monitoring(
    _window: WebviewWindow,
    game_monitoring_service: Arc<dyn GameMonitoringService>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let service_clone = game_monitoring_service.clone();

    let _handle = runtime_manager.spawn_background_task(
        "game_process_monitoring_setup".to_string(),
        move || async move {
            info!("Automatically starting game monitoring on application startup");
            if let Err(e) = service_clone.start_monitoring().await {
                error!("Failed to start game monitoring: {}", e);
            }
        },
    );
}

pub fn start_time_tracking_emission(
    window: WebviewWindow,
    time_tracking: Arc<dyn TimeTrackingService>,
    _runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let time_tracking_clone = time_tracking.clone();
    tokio::spawn(async move {
        time_tracking_clone
            .start_frontend_event_emission(window)
            .await;
    });
}

pub fn start_ping_event_emission(
    _window: WebviewWindow,
    server_status: Arc<ServerMonitor>,
    _runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let server_status_clone = server_status.clone();
    tokio::spawn(async move {
        server_status_clone.start_periodic_ping().await;
    });
}

pub fn start_log_monitoring(
    log_monitor: Arc<LogAnalyzer>,
    _runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let log_monitor_clone = log_monitor.clone();
    tokio::spawn(async move {
        if let Err(e) = log_monitor_clone.start_monitoring().await {
            error!("Failed to start log monitoring: {}", e);
        }
    });
}