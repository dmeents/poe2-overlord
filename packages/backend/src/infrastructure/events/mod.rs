//! Unified event system for pub/sub communication and Tauri frontend integration.

pub mod bridge;
pub mod bus;
pub mod channels;
pub mod types;

pub use bridge::TauriEventBridge;
pub use bus::EventBus;
pub use channels::ChannelManager;
pub use types::{AppEvent, ChannelConfig, EventType};
