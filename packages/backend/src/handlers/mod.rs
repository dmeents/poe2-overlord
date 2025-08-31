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
    // Setup logging
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }

    info!("Starting application setup...");

    // Initialize all services
    let services = ServiceInitializer::initialize_services(app)?;

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
        TimeTrackingHandler::start_event_emission(main_window, services.time_tracking);
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");
    Ok(())
}
