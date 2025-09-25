use log::{error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::domain::server_monitoring::traits::ServerMonitoringService;
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::LogAnalyzer;
use crate::infrastructure::runtime::{RuntimeManager, TaskManager};

/// Helper function to automatically start process monitoring on application startup
/// Game monitoring is always running when the application is running
/// The domain service handles its own background monitoring
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

/// Helper function to start time tracking event emission with only required services
pub fn start_time_tracking_emission(
    window: WebviewWindow,
    time_tracking: Arc<dyn TimeTrackingService>,
    _runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    // Direct service call instead of handler
    let time_tracking_clone = time_tracking.clone();
    tokio::spawn(async move {
        time_tracking_clone
            .start_frontend_event_emission(window)
            .await;
    });
}

/// Helper function to start ping event emission
pub fn start_ping_event_emission(
    window: WebviewWindow,
    server_monitor: Arc<ServerMonitor>,
    _runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    // Use infrastructure ServerMonitor for now
    let window_clone = window.clone();
    let server_monitor_clone = server_monitor.clone();

    tokio::spawn(async move {
        let mut status_receiver = server_monitor_clone.subscribe_to_status_changes();

        while let Ok(status) = status_receiver.recv().await {
            if let Err(e) = window_clone.emit("server-status-updated", &status) {
                log::warn!("Failed to emit server status event: {}", e);
            }
        }
    });
}

/// Helper function to start log monitoring immediately on application startup
pub fn start_log_monitoring(
    log_monitor: Arc<LogAnalyzer>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let log_monitor_clone = log_monitor.clone();

    let _handle = runtime_manager.spawn_background_task(
        "log_monitoring_setup".to_string(),
        move || async move {
            info!("Starting log monitoring immediately on application startup");
            if let Err(e) = log_monitor_clone.start_monitoring().await {
                error!("Failed to start log monitoring: {}", e);
            }
        },
    );
}
