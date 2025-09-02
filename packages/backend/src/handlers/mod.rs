pub mod log_event_handler;
pub mod process_monitor_handler;
pub mod service_initializer;
pub mod time_tracking_handler;

use log::{info, warn};
use tauri::Manager;

use crate::handlers::log_event_handler::LogEventHandler;
use crate::handlers::process_monitor_handler::ProcessMonitorHandler;
use crate::handlers::service_initializer::ServiceInitializer;
use crate::handlers::time_tracking_handler::TimeTrackingHandler;

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize services first to get config
    let services = ServiceInitializer::initialize_services(app)?;

    // Get log level from config service
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

    // Now we can use logging
    info!("Starting application setup...");
    info!(
        "Logging configured with level: {} ({:?})",
        log_level, level_filter
    );

    // Get main window and start background services
    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting background services");

        // Start process monitoring
        ProcessMonitorHandler::start_monitoring(
            main_window.clone(),
            services.log_monitor.clone(),
            services.time_tracking.clone(),
        );

        // Start log event emission
        LogEventHandler::start_event_emission(
            main_window.clone(),
            services.log_monitor.clone(),
            services.time_tracking.clone(),
        );

        // Start time tracking event emission
        TimeTrackingHandler::start_event_emission(
            main_window.clone(),
            services.time_tracking.clone(),
        );

        // Note: Async service initialization will happen in background tasks
        // where the Tokio runtime is available
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");
    Ok(())
}
