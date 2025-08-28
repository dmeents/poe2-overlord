use crate::services::{ConfigService, LogMonitorService, ProcessMonitor, SceneChangeEvent};
use log;
use std::sync::Arc;
use tauri::{App, Emitter, Manager, WebviewWindow};
use tokio::runtime::Runtime;
use tokio::time::{interval, Duration};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }

    log::info!("Starting application setup...");

    // Initialize configuration service
    log::info!("Initializing ConfigService...");
    let config_service = ConfigService::new(&app.handle());
    app.manage(config_service.clone());
    log::info!("ConfigService managed successfully");

    // Initialize log monitor service
    log::info!("Initializing LogMonitorService...");
    let log_monitor_service = LogMonitorService::new(Arc::new(config_service));
    let log_monitor_arc = Arc::new(log_monitor_service);
    app.manage(log_monitor_arc.clone());
    log::info!("LogMonitorService managed successfully");

    // Get main window and start process monitoring
    if let Some(main_window) = app.get_webview_window("main") {
        log::info!("Starting POE2 process monitoring");

        // Start process monitoring in the background using a dedicated runtime
        start_process_monitoring(main_window.clone(), log_monitor_arc.clone());

        // Start log event emission in the background using a dedicated runtime
        start_log_event_emission(main_window, log_monitor_arc);
    } else {
        log::warn!("Main window not found during setup");
    }

    log::info!("Application setup completed successfully");
    Ok(())
}

fn start_process_monitoring(window: WebviewWindow, log_monitor: Arc<LogMonitorService>) {
    // Create a dedicated runtime for this background task
    std::thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut interval = interval(Duration::from_secs(5));
            let mut was_poe_running = false;

            loop {
                interval.tick().await;
                match ProcessMonitor::check_poe2_process() {
                    Ok(process_info) => {
                        let is_poe_running = process_info.running;

                        // Emit process status to frontend
                        let _ = window.emit("poe2-process-status", &process_info);

                        // Manage log monitoring based on process status
                        if is_poe_running && !was_poe_running {
                            // POE process just started, start log monitoring
                            log::info!("POE2 process started, starting log monitoring");
                            if let Err(e) = log_monitor.start_monitoring().await {
                                log::error!("Failed to start log monitoring: {}", e);
                            }
                        } else if !is_poe_running && was_poe_running {
                            // POE process just stopped, stop log monitoring
                            log::info!("POE2 process stopped, stopping log monitoring");
                            if let Err(e) = log_monitor.stop_monitoring().await {
                                log::error!("Failed to stop log monitoring: {}", e);
                            }
                        }

                        was_poe_running = is_poe_running;
                    }
                    Err(e) => {
                        log::error!("Error checking POE2 process: {}", e);
                    }
                }
            }
        });
    });
}

fn start_log_event_emission(window: WebviewWindow, log_monitor: Arc<LogMonitorService>) {
    // Create a dedicated runtime for this background task
    std::thread::spawn(move || {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut event_receiver = log_monitor.subscribe();

            log::info!("Log event emission started, listening for scene changes");

            // Listen for scene change events and emit them to the frontend
            while let Ok(event) = event_receiver.recv().await {
                match event {
                    SceneChangeEvent::Zone(zone_event) => {
                        let _ = window.emit(
                            "log-zone-change",
                            serde_json::json!({
                                "zone_name": zone_event.zone_name,
                                "timestamp": zone_event.timestamp
                            }),
                        );
                    }
                    SceneChangeEvent::Act(act_event) => {
                        let _ = window.emit(
                            "log-act-change",
                            serde_json::json!({
                                "act_name": act_event.act_name,
                                "timestamp": act_event.timestamp
                            }),
                        );
                    }
                }
            }
        });
    });
}
