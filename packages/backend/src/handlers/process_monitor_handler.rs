use crate::services::process_monitor::ProcessMonitor;
use crate::services::{log_monitor::LogMonitorService, time_tracking::TimeTrackingService};
use log::{error, info, debug};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct ProcessMonitorHandler;

impl ProcessMonitorHandler {
    pub fn start_monitoring(
        window: WebviewWindow,
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
                                debug!("POE2 process stopped, ending all active time tracking sessions");
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
}
