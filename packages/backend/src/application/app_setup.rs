use log::{error, info, warn};
use std::sync::Arc;
use tauri::Manager;

use crate::application::service_orchestrator::{
    start_game_process_monitoring, start_log_monitoring, start_ping_event_emission,
};
use crate::application::service_registry::ServiceInitializer;
use crate::infrastructure::events::TauriEventBridge;

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let mut services = ServiceInitializer::initialize_services(app)?;

    app.manage(services.clone());

    let config_service = services.config_service.clone();

    let log_level = tauri::async_runtime::block_on(async {
        config_service
            .get_config()
            .await
            .map_or_else(|_| "info".to_string(), |config| config.log_level)
    })
    .to_lowercase();

    let level_filter = match log_level.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" | "warning" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => {
            eprintln!("Invalid log level '{log_level}', defaulting to Info");
            log::LevelFilter::Info
        }
    };

    app.handle().plugin(
        tauri_plugin_log::Builder::default()
            .level(level_filter)
            .filter(|metadata| {
                if metadata.target().starts_with("selectors")
                    || metadata.target().starts_with("html5ever")
                {
                    return false;
                }
                true
            })
            .build(),
    )?;

    info!("Starting application setup...");

    info!("Logging configured with level: {log_level} ({level_filter:?})");

    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting background services");

        let event_bridge = Arc::new(TauriEventBridge::new(
            services.event_bus.clone(),
            main_window.clone(),
        ));

        if let Err(e) = tauri::async_runtime::block_on(event_bridge.start_forwarding()) {
            error!("Failed to start Tauri event bridge: {e}");
        } else {
            info!("Tauri event bridge started successfully");
        }

        // Store bridge in services for shutdown
        services.set_event_bridge(event_bridge);

        let _log_monitoring_handle = start_log_monitoring(services.log_analysis_service.clone());

        let _game_monitoring_handle =
            start_game_process_monitoring(services.game_monitoring_service.clone());

        let _ping_monitoring_handle =
            start_ping_event_emission(services.server_monitoring_service.clone());

        info!("Background services started successfully");

        // Register shutdown handler
        let services_clone = services.clone();
        main_window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                log::info!("Application shutdown requested");

                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        if let Err(e) = services_clone.shutdown_services().await {
                            log::error!("Error during application shutdown: {e}");
                        }
                    });
                });
            }
        });
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");
    Ok(())
}
