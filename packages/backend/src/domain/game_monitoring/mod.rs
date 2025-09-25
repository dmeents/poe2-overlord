pub mod events;
pub mod models;
pub mod service;
pub mod traits;

// Re-export main types for easy access
pub use events::{GameMonitoringEvent, GameProcessStatusUpdated};
pub use models::{GameMonitoringConfig, GameProcessStatus};
pub use service::GameMonitoringServiceImpl;
pub use traits::{GameMonitoringEventPublisher, GameMonitoringService, ProcessDetector};
