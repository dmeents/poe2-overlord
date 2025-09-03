use crate::handlers::{
    event_utils::emit_scene_change_event,
    runtime_manager::RuntimeManager,
    task_manager::TaskManager,
};
use crate::models::{events::SceneChangeEvent, LocationType};
use crate::services::{log_monitor::LogMonitorService, time_tracking::TimeTrackingService};
use log::{debug, error};
use std::sync::Arc;
use tauri::WebviewWindow;

pub struct LogEventHandler;

impl LogEventHandler {
    pub fn start_event_emission(
        window: WebviewWindow,
        log_monitor: Arc<LogMonitorService>,
        time_tracking: Arc<TimeTrackingService>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task("log_event_emission".to_string(), move || async move {
            let mut event_receiver = log_monitor.subscribe();

            debug!("Log event emission started, listening for scene changes");

            // Listen for scene change events and emit them to the frontend
            while let Ok(event) = event_receiver.recv().await {
                // Emit the unified scene change event to the frontend
                emit_scene_change_event(&window, &event);

                // Handle time tracking based on the event type
                Self::handle_scene_change_event(&event, &time_tracking).await;
            }
        });
        
        task_manager.register_task("log_event_emission".to_string(), handle);
    }

    async fn handle_scene_change_event(
        event: &SceneChangeEvent,
        time_tracking: &Arc<TimeTrackingService>,
    ) {
        match event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Hideout change detected: {}", hideout_event.hideout_name);

                // End any active act session when entering a hideout
                if let Err(e) = time_tracking.end_sessions_by_type(&LocationType::Act).await {
                    error!("Failed to end act sessions when entering hideout: {}", e);
                }

                // Start time tracking for the hideout
                if let Err(e) = time_tracking
                    .start_session(hideout_event.hideout_name.clone(), LocationType::Hideout)
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
}
