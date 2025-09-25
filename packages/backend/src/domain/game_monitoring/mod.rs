pub mod commands;
pub mod events;
pub mod models;
pub mod service;
pub mod traits;

// Re-export main types for easy access
pub use commands::{
    CheckGameProcessStatus, GameMonitoringCommandHandler, GameMonitoringCommandResult,
    StartGameMonitoring, StopGameMonitoring, UpdateGameMonitoringConfig,
};
pub use events::{
    GameMonitoringEvent, GameProcessStarted, GameProcessStatusUpdated, GameProcessStopped,
};
pub use models::{GameMonitoringConfig, GameProcessStatus};
pub use service::GameMonitoringServiceImpl;
pub use traits::{GameMonitoringEventPublisher, GameMonitoringService, ProcessDetector};
