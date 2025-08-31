use crate::models::{events::SceneChangeEvent, LocationType, TimeTrackingEvent};
use crate::services::{
    config::ConfigService, log_monitor::LogMonitorService, process_monitor::ProcessMonitor,
    time_tracking::TimeTrackingService,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::{Emitter, Manager};

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

    // Initialize configuration service
    info!("Initializing ConfigService...");
    let config_service = ConfigService::new(&app.handle());
    app.manage(config_service.clone());
    info!("ConfigService managed successfully");

    // Initialize time tracking service
    info!("Initializing TimeTrackingService...");
    let time_tracking_service = TimeTrackingService::new();
    let time_tracking_arc = Arc::new(time_tracking_service);
    app.manage(time_tracking_arc.clone());
    info!("TimeTrackingService managed successfully");

    // Initialize log monitor service
    info!("Initializing LogMonitorService...");
    let log_path = config_service.get_poe_client_log_path();
    let log_monitor_service = LogMonitorService::new(log_path);
    let log_monitor_arc = Arc::new(log_monitor_service);
    app.manage(log_monitor_arc.clone());
    info!("LogMonitorService managed successfully");

    // Get main window and start process monitoring
    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting POE2 process monitoring");

        // Start process monitoring in the background using a dedicated runtime
        start_process_monitoring(
            main_window.clone(),
            log_monitor_arc.clone(),
            time_tracking_arc.clone(),
        );

        // Start log event emission in the background using a dedicated runtime
        start_log_event_emission(
            main_window.clone(),
            log_monitor_arc.clone(),
            time_tracking_arc.clone(),
        );

        // Start time tracking event emission
        start_time_tracking_event_emission(main_window, time_tracking_arc);
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");
    Ok(())
}

fn start_process_monitoring(
    window: tauri::WebviewWindow,
    log_monitor: Arc<LogMonitorService>,
    time_tracking: Arc<TimeTrackingService>,
) {
    // Create a dedicated runtime for this background task
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
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
                            info!("POE2 process started, starting log monitoring");
                            if let Err(e) = log_monitor.start_monitoring().await {
                                error!("Failed to start log monitoring: {}", e);
                            }

                            // Set POE process start time for time tracking
                            time_tracking.set_poe_process_start_time();
                        } else if !is_poe_running && was_poe_running {
                            // POE process just stopped, stop log monitoring
                            info!("POE2 process stopped, stopping log monitoring");
                            if let Err(e) = log_monitor.stop_monitoring().await {
                                error!("Failed to stop log monitoring: {}", e);
                            }

                            // End all active time tracking sessions when game exits
                            info!("POE2 process stopped, ending all active time tracking sessions");
                            if let Err(e) = time_tracking.end_all_active_sessions().await {
                                error!("Failed to end active time tracking sessions: {}", e);
                            }

                            // Clear POE process start time for time tracking
                            time_tracking.clear_poe_process_start_time();
                        }

                        was_poe_running = is_poe_running;
                    }
                    Err(e) => {
                        error!("Error checking POE2 process: {}", e);
                    }
                }
            }
        });
    });
}

fn start_log_event_emission(
    window: tauri::WebviewWindow,
    log_monitor: Arc<LogMonitorService>,
    time_tracking: Arc<TimeTrackingService>,
) {
    // Create a dedicated runtime for this background task
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut event_receiver = log_monitor.subscribe();

            info!("Log event emission started, listening for scene changes");

            // Listen for scene change events and emit them to the frontend
            while let Ok(event) = event_receiver.recv().await {
                // Emit the unified scene change event to the frontend
                if let Err(e) = window.emit("log-scene-change", &event) {
                    error!("Failed to emit scene change event: {}", e);
                }

                // Handle time tracking based on the event type
                match &event {
                    SceneChangeEvent::Hideout(hideout_event) => {
                        debug!("Hideout change detected: {}", hideout_event.hideout_name);

                        // End any active act session when entering a hideout
                        if let Err(e) = time_tracking.end_sessions_by_type(&LocationType::Act).await
                        {
                            error!("Failed to end act sessions when entering hideout: {}", e);
                        }

                        // Start time tracking for the hideout
                        if let Err(e) = time_tracking
                            .start_session(
                                hideout_event.hideout_name.clone(),
                                LocationType::Hideout,
                            )
                            .await
                        {
                            error!("Failed to start hideout time tracking: {}", e);
                        }
                    }
                    SceneChangeEvent::Zone(zone_event) => {
                        debug!("Zone change detected: {}", zone_event.zone_name);

                        // Start time tracking for the new zone
                        if let Err(e) = time_tracking
                            .start_session(zone_event.zone_name.clone(), LocationType::Zone)
                            .await
                        {
                            error!("Failed to start zone time tracking: {}", e);
                        }
                    }
                    SceneChangeEvent::Act(act_event) => {
                        debug!("Act change detected: {}", act_event.act_name);

                        // Start time tracking for the new act
                        if let Err(e) = time_tracking
                            .start_session(act_event.act_name.clone(), LocationType::Act)
                            .await
                        {
                            error!("Failed to start act time tracking: {}", e);
                        }
                    }
                }
            }
        });
    });
}

fn start_time_tracking_event_emission(
    window: tauri::WebviewWindow,
    time_tracking: Arc<TimeTrackingService>,
) {
    // Create a dedicated runtime for this background task
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let mut event_receiver = time_tracking.subscribe();

            info!("Time tracking event emission started");

            // Listen for time tracking events and emit them to the frontend
            while let Ok(event) = event_receiver.recv().await {
                match event {
                    TimeTrackingEvent::SessionStarted(session) => {
                        let _ = window.emit(
                            "time-tracking-session-started",
                            serde_json::json!({
                                "location_id": session.location_id,
                                "location_name": session.location_name,
                                "location_type": session.location_type,
                                "entry_timestamp": session.entry_timestamp
                            }),
                        );
                    }
                    TimeTrackingEvent::SessionEnded(session) => {
                        let _ = window.emit(
                            "time-tracking-session-ended",
                            serde_json::json!({
                                "location_id": session.location_id,
                                "location_name": session.location_name,
                                "location_type": session.location_type,
                                "duration_seconds": session.duration_seconds,
                                "entry_timestamp": session.entry_timestamp,
                                "exit_timestamp": session.exit_timestamp
                            }),
                        );
                    }
                    TimeTrackingEvent::StatsUpdated(stats) => {
                        let _ = window.emit(
                            "time-tracking-stats-updated",
                            serde_json::json!({
                                "location_id": stats.location_id,
                                "location_name": stats.location_name,
                                "location_type": stats.location_type,
                                "total_visits": stats.total_visits,
                                "total_time_seconds": stats.total_time_seconds,
                                "average_session_seconds": stats.average_session_seconds,
                                "last_visited": stats.last_visited
                            }),
                        );
                    }
                }
            }
        });
    });
}
