//! Domain layer with core business logic

pub mod character;
pub mod configuration;
pub mod economy;
pub mod game_monitoring;
pub mod log_analysis;
pub mod server_monitoring;
pub mod walkthrough;
pub mod wiki_scraping;
pub mod zone_configuration;
pub mod zone_tracking;

pub use character::{
    Ascendency, CharacterClass, CharacterData, CharacterService,
    CharacterServiceImpl, CharacterUpdateParams, CharactersIndex, EnrichedLocationState, League,
    LocationState, LocationType,
};

pub use configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationService, ConfigurationServiceImpl,
    ConfigurationValidationResult,
};

pub use economy::{
    CurrencyExchangeData, CurrencyExchangeRate, CurrencyInfo, EconomyService, EconomyType,
};

pub use crate::infrastructure::events::{AppEvent, EventBus, EventType};

pub use game_monitoring::{
    GameMonitoringService, GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
    ProcessDetectorImpl,
};

pub use log_analysis::{
    LogAnalysisConfig, LogAnalysisError, LogAnalysisService, LogAnalysisServiceImpl, LogFileInfo,
    LogLineAnalysis,
};

pub use server_monitoring::{ServerMonitoringService, ServerMonitoringServiceImpl, ServerStatus};

pub use walkthrough::{
    get_character_walkthrough_progress, get_walkthrough_guide,
    update_character_walkthrough_progress, CharacterWalkthroughProgress, Objective, WalkthroughAct,
    WalkthroughGuide, WalkthroughProgress, WalkthroughRepository, WalkthroughRepositoryImpl,
    WalkthroughService, WalkthroughServiceImpl, WalkthroughStep, WalkthroughStepResult,
};

pub use zone_configuration::{
    ZoneConfiguration, ZoneConfigurationService, ZoneConfigurationServiceImpl,
};

pub use zone_tracking::{TrackingSummary, ZoneStats, ZoneTrackingService, ZoneTrackingServiceImpl};
