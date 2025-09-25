use crate::handlers::{
    event_utils::emit_event, runtime_manager::RuntimeManager, task_manager::TaskManager,
};
use crate::infrastructure::tauri::EventDispatcher;
use log::debug;
use std::sync::Arc;
use tauri::WebviewWindow;

pub struct PingEventHandler;

impl PingEventHandler {
    pub async fn start_event_emission(
        window: WebviewWindow,
        event_dispatcher: Arc<EventDispatcher>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task(
            "ping_event_emission".to_string(),
            move || async move {
                let mut ping_event_receiver = event_dispatcher.subscribe_ping_events();

                debug!("Ping event emission started");

                // Listen for ping events and emit them to the frontend
                while let Ok(ping_event) = ping_event_receiver.recv().await {
                    debug!("Received ping event: {:?}", ping_event);

                    // Emit the ping event as a server status update to the frontend
                    emit_event(&window, "server-status-updated", &ping_event);
                }

                debug!("Ping event emission stopped");
            },
        );

        task_manager
            .register_task("ping_event_emission".to_string(), handle)
            .await;
    }
}
