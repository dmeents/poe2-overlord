use crate::models::TimeTrackingEvent;
use crate::services::time_tracking::TimeTrackingService;
use log::info;
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};

pub struct TimeTrackingHandler;

impl TimeTrackingHandler {
    pub fn start_event_emission(
        window: WebviewWindow,
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
                    Self::emit_time_tracking_event(&window, &event);
                }
            });
        });
    }

    fn emit_time_tracking_event(window: &WebviewWindow, event: &TimeTrackingEvent) {
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
}
