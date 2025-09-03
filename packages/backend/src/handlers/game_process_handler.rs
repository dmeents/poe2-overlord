use crate::handlers::{runtime_manager::RuntimeManager, task_manager::TaskManager};
use crate::services::process_detector::ProcessDetector;
use crate::services::session_tracker::SessionTracker;
use log::{debug, error, info};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct GameProcessHandler;

impl GameProcessHandler {
    pub async fn start_monitoring(
        window: WebviewWindow,
        time_tracking: Arc<SessionTracker>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task("game_process_monitoring".to_string(), move || async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
            let mut was_poe_running = false;

            loop {
                interval.tick().await;
                
                match ProcessDetector::check_game_process() {
                    Ok(process_info) => {
                        let is_poe_running = process_info.running;

                        if let Err(e) = window.emit("game-process-status", &process_info) {
                            error!("Failed to emit process status: {}", e);
                        }

                        if is_poe_running && !was_poe_running {
                            info!("POE2 process started");
                            time_tracking.set_poe_process_start_time().await;
                        } else if !is_poe_running && was_poe_running {
                            info!("POE2 process stopped");
                            debug!("POE2 process stopped, ending all active time tracking sessions");

                            if let Err(e) = time_tracking.end_all_active_sessions().await {
                                error!("Failed to end active time tracking sessions: {}", e);
                            }

                            time_tracking.clear_poe_process_start_time().await;
                        }

                        was_poe_running = is_poe_running;
                    }
                    Err(e) => {
                        error!("Error checking POE2 process: {}", e);
                    }
                }
            }
        });

        task_manager
            .register_task("game_process_monitoring".to_string(), handle)
            .await;
    }
}
