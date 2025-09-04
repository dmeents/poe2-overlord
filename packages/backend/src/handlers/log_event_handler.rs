use crate::handlers::{
    event_utils::emit_scene_change_event, runtime_manager::RuntimeManager,
    task_manager::TaskManager,
};
use crate::models::events::LogEvent;
use crate::services::{
    character_session_tracker::CharacterSessionTracker, log_analyzer::LogAnalyzer,
};
use log::{debug, error};
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct LogEventHandler;

impl LogEventHandler {
    pub async fn start_event_emission(
        window: WebviewWindow,
        log_monitor: Arc<LogAnalyzer>,
        time_tracking: Arc<CharacterSessionTracker>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task(
            "log_event_router".to_string(),
            move || async move {
                let mut event_receiver = log_monitor.subscribe();

                debug!("Log event router started, listening for all log events");

                // Listen for all log events and route them to appropriate services
                while let Ok(event) = event_receiver.recv().await {
                    debug!("Received log event: {:?}", event);

                    match event {
                        LogEvent::SceneChange(scene_event) => {
                            debug!("Routing scene change event to services");

                            // Emit to frontend
                            emit_scene_change_event(&window, &scene_event);

                            // Handle time tracking for active character (if any)
                            time_tracking
                                .handle_scene_change_for_active_character(&scene_event)
                                .await;
                        }
                        LogEvent::ServerConnection(server_event) => {
                            debug!("Routing server connection event to services");

                            // Emit to frontend
                            if let Err(e) = window.emit("server-status-updated", &server_event) {
                                error!(
                                    "Failed to emit server status updated event to frontend: {}",
                                    e
                                );
                            }
                        }
                    }
                }

                debug!("Log event router stopped");
            },
        );

        task_manager
            .register_task("log_event_router".to_string(), handle)
            .await;
    }
}
