pub mod event_dispatcher;
pub mod event_publisher;

// Re-export main types for easy access
pub use event_dispatcher::{EventDispatcher, EventService};
pub use event_publisher::{EventPublisher, TauriGameMonitoringEventPublisher};
