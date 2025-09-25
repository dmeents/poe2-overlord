pub mod character;
pub mod configuration;
pub mod game_monitoring;
pub mod time_tracking;

// Re-export main types for backward compatibility
pub use character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterService, CharacterUpdateParams,
    League,
};
pub use configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
};
pub use game_monitoring::{
    GameMonitoringEvent, GameMonitoringEventPublisher, GameMonitoringService,
    GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
};
pub use time_tracking::{
    LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingEvent,
    TimeTrackingSummary, TimeTrackingService, TimeTrackingServiceImpl,
};
