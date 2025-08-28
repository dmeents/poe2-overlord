use crate::services::{ConfigService, LogMonitorService, ProcessMonitor};
use log;
use std::sync::Arc;
use tauri::{App, Emitter, Manager, WebviewWindow};
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

    // Initialize configuration service
    let config_service = ConfigService::new(&app.handle());
    let config_service_arc = Arc::new(config_service);
    app.manage(config_service_arc.clone());

    // Initialize log monitor service
    let log_monitor_service = LogMonitorService::new(config_service_arc);
    let log_monitor_arc = Arc::new(log_monitor_service);
    app.manage(log_monitor_arc.clone());

    // Get main window and start process monitoring
    if let Some(main_window) = app.get_webview_window("main") {
        log::info!("Starting POE2 process monitoring");

        // Start process monitoring in the background
        start_process_monitoring(main_window.clone(), Arc::clone(&log_monitor_arc));

        // Start log event emission in the background
        start_log_event_emission(main_window, log_monitor_arc);
    }

    Ok(())
}

fn start_process_monitoring(window: WebviewWindow, log_monitor: Arc<LogMonitorService>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
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
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut event_receiver = log_monitor.subscribe();

            log::info!("Log event emission started, listening for zone changes");

            // Listen for zone change events and emit them to the frontend
            while let Ok(event) = event_receiver.recv().await {
                let _ = window.emit(
                    "log-zone-change",
                    serde_json::json!({
                        "zone_name": event.zone_name,
                        "timestamp": event.timestamp
                    }),
                );
            }
        });
    });
}
