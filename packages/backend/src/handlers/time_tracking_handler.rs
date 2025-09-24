use crate::handlers::{
    event_utils::emit_time_tracking_event, runtime_manager::RuntimeManager,
    task_manager::TaskManager,
};
use crate::models::TimeTrackingEvent;
use crate::domain::time_tracking::CharacterSessionTracker;
use log::debug;
use std::sync::Arc;
use tauri::WebviewWindow;

pub struct TimeTrackingHandler;

impl TimeTrackingHandler {
    pub async fn start_event_emission(
        window: WebviewWindow,
        time_tracking: Arc<CharacterSessionTracker>,
        runtime_manager: Arc<RuntimeManager>,
        task_manager: Arc<TaskManager>,
    ) {
        let handle = runtime_manager.spawn_background_task(
            "time_tracking_event_emission".to_string(),
            move || async move {
                let mut event_receiver = time_tracking.subscribe();

                debug!("Time tracking event emission started");

                // Listen for time tracking events and emit them to the frontend
                while let Ok(event) = event_receiver.recv().await {
                    Self::emit_time_tracking_event(&window, &event);
                }
            },
        );

        task_manager
            .register_task("time_tracking_event_emission".to_string(), handle)
            .await;
    }

    fn emit_time_tracking_event(window: &WebviewWindow, event: &TimeTrackingEvent) {
        match event {
            TimeTrackingEvent::SessionStarted(session) => {
                emit_time_tracking_event(
                    window,
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
                emit_time_tracking_event(
                    window,
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
                emit_time_tracking_event(
                    window,
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
}
