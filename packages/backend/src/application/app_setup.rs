use log::{debug, info, warn};
use std::sync::Arc;
use tauri::Manager;

use crate::application::service_registry::ServiceInitializer;
use crate::application::service_orchestrator::{
    start_game_process_monitoring, start_log_monitoring, start_ping_event_emission,
    start_time_tracking_emission,
};
use crate::domain::configuration::traits::ConfigurationService;
use crate::infrastructure::runtime::{RuntimeManager, TaskManager};

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let services = ServiceInitializer::initialize_services(app)?;

    let config_service = services.config_service.clone();
    let log_level = tauri::async_runtime::block_on(async {
        config_service
            .get_log_level()
            .await
            .unwrap_or_else(|_| "info".to_string())
    })
    .to_lowercase();

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

    debug!("Loading existing character time tracking data...");
    let time_tracking_service = services.time_tracking_service.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = time_tracking_service.load_all_character_data().await {
            warn!(
                "Failed to load existing character time tracking data: {}",
                e
            );
        } else {
            info!("Successfully loaded existing character time tracking data");
        }
    });

    let runtime_manager = Arc::new(RuntimeManager::new()?);
    let task_manager = Arc::new(TaskManager::new());

    app.manage(runtime_manager.clone());
    app.manage(task_manager.clone());

    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting background services");

        start_log_monitoring(
            services.log_monitor.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        start_game_process_monitoring(
            main_window.clone(),
            services.game_monitoring_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        start_time_tracking_emission(
            main_window.clone(),
            services.time_tracking_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        start_ping_event_emission(
            main_window.clone(),
            services.server_status.clone(),
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