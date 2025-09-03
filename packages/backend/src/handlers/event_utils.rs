use log::error;
use serde::Serialize;
use tauri::{Emitter, WebviewWindow};

/// Emit an event to the frontend with proper error handling
pub fn emit_event<T: Serialize>(window: &WebviewWindow, event_name: &str, payload: &T) {
    if let Err(e) = window.emit(event_name, payload) {
        error!("Failed to emit event '{}': {}", event_name, e);
    }
}

/// Emit a JSON event to the frontend with proper error handling
pub fn emit_json_event(window: &WebviewWindow, event_name: &str, payload: serde_json::Value) {
    if let Err(e) = window.emit(event_name, &payload) {
        error!("Failed to emit JSON event '{}': {}", event_name, e);
    }
}

/// Emit a scene change event to the frontend
pub fn emit_scene_change_event<T: Serialize>(window: &WebviewWindow, event: &T) {
    emit_event(window, "log-scene-change", event);
}

/// Emit a time tracking event to the frontend
pub fn emit_time_tracking_event(window: &WebviewWindow, event_name: &str, payload: serde_json::Value) {
    emit_json_event(window, event_name, payload);
}
