//! Tauri integration infrastructure for frontend communication
//! 
//! Provides utilities for event dispatching, command handling, and communication
//! between the Rust backend and the frontend application. Handles real-time
//! event broadcasting and Tauri-specific operations.

pub mod command_utils;
pub mod event_dispatcher;
pub mod event_publisher;
pub mod event_utils;

pub use command_utils::*;
pub use event_dispatcher::{EventDispatcher, EventService};
pub use event_publisher::{EventPublisher, TauriGameMonitoringEventPublisher};
pub use event_utils::{
    emit_event, emit_json_event, emit_scene_change_event, emit_time_tracking_event, EventEmitter,
};
