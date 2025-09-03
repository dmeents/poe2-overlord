use crate::handlers::{
    event_utils::emit_scene_change_event, runtime_manager::RuntimeManager,
    task_manager::TaskManager,
};
use crate::models::{events::SceneChangeEvent, LocationType};
use crate::services::{log_monitor::LogMonitorService, time_tracking::TimeTrackingService};
use log::{debug, error};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct LogEventHandler;

impl LogEventHandler {
    pub async fn start_event_emission(
        window: WebviewWindow,
        log_monitor: Arc<LogMonitorService>,
        time_tracking: Arc<TimeTrackingService>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let window_clone = window.clone();
        let log_monitor_clone = log_monitor.clone();
        let time_tracking_clone = time_tracking.clone();
        let runtime_manager_clone = runtime_manager.clone();
        let task_manager_clone = task_manager.clone();

        let handle = runtime_manager_clone.spawn_background_task(
            "log_event_emission".to_string(),
            move || async move {
                let mut event_receiver = log_monitor_clone.subscribe();

                debug!("Log event emission started, listening for scene changes");

                // Listen for scene change events and emit them to the frontend
                while let Ok(event) = event_receiver.recv().await {
                    // Emit the unified scene change event to the frontend
                    emit_scene_change_event(&window_clone, &event);

                    // Handle time tracking based on the event type
                    Self::handle_scene_change_event(&event, &time_tracking_clone).await;
                }
            },
        );

        task_manager_clone
            .register_task("log_event_emission".to_string(), handle)
            .await;

        // Start server event emission
        let window_clone = window.clone();
        let log_monitor_clone = log_monitor.clone();
        let runtime_manager_clone = runtime_manager.clone();
        let task_manager_clone = task_manager.clone();
        Self::start_server_event_emission(
            window_clone,
            log_monitor_clone,
            runtime_manager_clone,
            task_manager_clone,
        )
        .await;
    }

    /// Start server event emission to frontend
    async fn start_server_event_emission(
        window: WebviewWindow,
        log_monitor: Arc<LogMonitorService>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task(
            "server_event_emission".to_string(),
            move || async move {
                let mut server_event_receiver = log_monitor.subscribe_server_events();

                debug!("Server event emission started, listening for server connection events");

                // Listen for server connection events and emit them to the frontend
                while let Ok(event) = server_event_receiver.recv().await {
                    debug!("Emitting server connection event to frontend: {:?}", event);

                    // Emit server connection event to frontend with complete server status
                    if let Err(e) = window.emit("server-status-updated", &event) {
                        error!(
                            "Failed to emit server status updated event to frontend: {}",
                            e
                        );
                    }
                }
            },
        );

        task_manager
            .register_task("server_event_emission".to_string(), handle)
            .await;
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
