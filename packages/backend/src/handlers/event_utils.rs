use log::error;
use serde::Serialize;
use tauri::WebviewWindow;

/// Trait for types that can emit events
pub trait EventEmitter {
    fn emit<T: Serialize>(&self, event: &str, payload: &T) -> Result<(), String>;
}

impl EventEmitter for WebviewWindow {
    fn emit<T: Serialize>(&self, event: &str, payload: &T) -> Result<(), String> {
        tauri::WebviewWindow::emit(self, event, payload).map_err(|e| e.to_string())
    }
}

/// Emit an event to the frontend with proper error handling
pub fn emit_event<T: Serialize, W: EventEmitter>(window: &W, event_name: &str, payload: &T) {
    if let Err(e) = window.emit(event_name, payload) {
        error!("Failed to emit event '{}': {}", event_name, e);
    }
}

/// Emit a JSON event to the frontend with proper error handling
pub fn emit_json_event<W: EventEmitter>(window: &W, event_name: &str, payload: serde_json::Value) {
    if let Err(e) = window.emit(event_name, &payload) {
        error!("Failed to emit JSON event '{}': {}", event_name, e);
    }
}

/// Emit a scene change event to the frontend
pub fn emit_scene_change_event<T: Serialize, W: EventEmitter>(window: &W, event: &T) {
    emit_event(window, "log-scene-change", event);
}

/// Emit a time tracking event to the frontend
pub fn emit_time_tracking_event<W: EventEmitter>(
    window: &W,
    event_name: &str,
    payload: serde_json::Value,
) {
    emit_json_event(window, event_name, payload);
}
