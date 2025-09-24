use log::{error, info};
use std::sync::Arc;
use tauri::WebviewWindow;

use crate::handlers::game_process_handler::GameProcessHandler;
use crate::handlers::log_event_handler::LogEventHandler;
use crate::handlers::ping_event_handler::PingEventHandler;
use crate::handlers::runtime_manager::RuntimeManager;
use crate::handlers::task_manager::TaskManager;
use crate::handlers::time_tracking_handler::TimeTrackingHandler;
use crate::domain::time_tracking::CharacterSessionTracker;
use crate::services::{
    event_dispatcher::EventDispatcher,
    log_analyzer::LogAnalyzer,
};

/// Helper function to start process monitoring with only required services
pub fn start_game_process_monitoring(
    window: WebviewWindow,
    time_tracking: Arc<CharacterSessionTracker>,
    runtime_manager: Arc<RuntimeManager>,
    task_manager: Arc<TaskManager>,
) {
    let window_clone = window.clone();
    let time_tracking_clone = time_tracking.clone();
    let runtime_manager_clone = runtime_manager.clone();
    let task_manager_clone = task_manager.clone();

    let _handle = runtime_manager.spawn_background_task(
        "game_process_monitoring_setup".to_string(),
        move || async move {
            GameProcessHandler::start_monitoring(
                window_clone,
                time_tracking_clone,
                runtime_manager_clone,
                task_manager_clone,
            )
            .await;
        },
    );
}

/// Helper function to start log event emission with only required services
pub fn start_log_event_emission(
    window: WebviewWindow,
    log_monitor: Arc<LogAnalyzer>,
    time_tracking: Arc<CharacterSessionTracker>,
    runtime_manager: Arc<RuntimeManager>,
    task_manager: Arc<TaskManager>,
) {
    let window_clone = window.clone();
    let log_monitor_clone = log_monitor.clone();
    let time_tracking_clone = time_tracking.clone();
    let runtime_manager_clone = runtime_manager.clone();
    let task_manager_clone = task_manager.clone();

    let _handle = runtime_manager.spawn_background_task(
        "log_event_emission_setup".to_string(),
        move || async move {
            LogEventHandler::start_event_emission(
                window_clone,
                log_monitor_clone,
                time_tracking_clone,
                runtime_manager_clone,
                task_manager_clone,
            )
            .await;
        },
    );
}

/// Helper function to start time tracking event emission with only required services
pub fn start_time_tracking_emission(
    window: WebviewWindow,
    time_tracking: Arc<CharacterSessionTracker>,
    runtime_manager: Arc<RuntimeManager>,
    task_manager: Arc<TaskManager>,
) {
    let window_clone = window.clone();
    let time_tracking_clone = time_tracking.clone();
    let runtime_manager_clone = runtime_manager.clone();
    let task_manager_clone = task_manager.clone();

    let _handle = runtime_manager.spawn_background_task(
        "time_tracking_setup".to_string(),
        move || async move {
            TimeTrackingHandler::start_event_emission(
                window_clone,
                time_tracking_clone,
                runtime_manager_clone,
                task_manager_clone,
            )
            .await;
        },
    );
}

/// Helper function to start ping event emission
pub fn start_ping_event_emission(
    window: WebviewWindow,
    event_dispatcher: Arc<EventDispatcher>,
    runtime_manager: Arc<RuntimeManager>,
    task_manager: Arc<TaskManager>,
) {
    let window_clone = window.clone();
    let event_dispatcher_clone = event_dispatcher.clone();
    let runtime_manager_clone = runtime_manager.clone();
    let task_manager_clone = task_manager.clone();

    let _handle = runtime_manager.spawn_background_task(
        "ping_event_emission_setup".to_string(),
        move || async move {
            PingEventHandler::start_event_emission(
                window_clone,
                event_dispatcher_clone,
                runtime_manager_clone,
                task_manager_clone,
            )
            .await;
        },
    );
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
