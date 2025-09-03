pub mod event_utils;
pub mod game_process_handler;
pub mod log_event_handler;
pub mod ping_event_handler;
pub mod runtime_manager;
pub mod service_initializer;
pub mod service_launcher;
pub mod task_manager;
pub mod time_tracking_handler;

use log::{info, warn};
use std::sync::Arc;
use tauri::Manager;

use crate::handlers::runtime_manager::RuntimeManager;
use crate::handlers::service_initializer::ServiceInitializer;
use crate::handlers::service_launcher::{
    start_game_process_monitoring, start_log_event_emission, start_log_monitoring,
    start_ping_event_emission, start_time_tracking_emission,
};
use crate::handlers::task_manager::TaskManager;

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let services = ServiceInitializer::initialize_services(app)?;
    let log_level = services.config_service.get_log_level().to_lowercase();

    let level_filter = match log_level.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" | "warning" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => {
            eprintln!("Invalid log level '{}', defaulting to Info", log_level);
            log::LevelFilter::Info
        }
    };

    // Setup logging with the configured level
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(level_filter)
                .build(),
        )?;
    }

    info!("Starting application setup...");

    info!(
        "Logging configured with level: {} ({:?})",
        log_level, level_filter
    );

    let runtime_manager = Arc::new(RuntimeManager::new()?);
    let task_manager = Arc::new(TaskManager::new());

    app.manage(runtime_manager.clone());
    app.manage(task_manager.clone());

    // Get main window and start background services
    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting background services");

        // Start client log monitoring
        start_log_monitoring(
            services.log_monitor.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start process monitoring
        start_game_process_monitoring(
            main_window.clone(),
            services.time_tracking.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start log event emission
        start_log_event_emission(
            main_window.clone(),
            services.log_monitor.clone(),
            services.time_tracking.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start time tracking event emission
        start_time_tracking_emission(
            main_window.clone(),
            services.time_tracking.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start ping event emission
        start_ping_event_emission(
            main_window.clone(),
            services.event_broadcaster.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        info!("Background services started successfully");
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");
    Ok(())
}
