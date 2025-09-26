//! Domain Layer - Core Business Logic and Entities
//!
//! This module contains the complete domain layer implementation following Domain-Driven Design (DDD) principles.
//! It provides a clean separation between business logic and infrastructure concerns, ensuring that the core
//! application functionality remains independent of external dependencies.
//!
//! ## Architecture Overview
//!
//! The domain layer is organized into distinct bounded contexts, each representing a specific area of
//! business functionality:
//!
//! - **Character Management**: Complete character lifecycle management with validation and persistence
//! - **Configuration Management**: Application settings and configuration with real-time change events
//! - **Event Management**: Publish-subscribe event system for loose coupling between components
//! - **Game Monitoring**: Process detection and game state monitoring capabilities
//! - **Location Tracking**: Scene and zone tracking with configurable monitoring rules
//! - **Log Analysis**: Real-time log parsing and analysis with pattern matching
//! - **Server Monitoring**: Network connectivity and server status monitoring
//! - **Time Tracking**: Character playtime tracking with detailed session management
//!
//! ## Design Patterns
//!
//! Each domain module follows consistent architectural patterns:
//!
//! - **Models**: Core entities and value objects with business rules
//! - **Traits**: Service and repository interfaces for dependency injection
//! - **Services**: Business logic orchestration and coordination
//! - **Repositories**: Data persistence abstraction layer
//! - **Commands**: Tauri command handlers for frontend integration
//! - **Events**: Domain events for loose coupling and real-time updates
//!
//! ## Key Benefits
//!
//! - **Testability**: Clean interfaces enable easy mocking and unit testing
//! - **Maintainability**: Clear separation of concerns and consistent patterns
//! - **Extensibility**: New features can be added without affecting existing code
//! - **Performance**: Optimized data structures and async operations
//! - **Reliability**: Comprehensive error handling and validation

// Domain module declarations - each represents a distinct bounded context
pub mod character; // Character management and lifecycle operations
pub mod configuration; // Application configuration and settings management
pub mod events; // Unified event system for all application events
pub mod game_monitoring; // Game process detection and monitoring
pub mod location_tracking; // Scene/zone tracking and location history
pub mod log_analysis; // Log parsing, analysis, and pattern matching
pub mod server_monitoring; // Network connectivity and server status monitoring
pub mod time_tracking; // Character playtime tracking and session management

// Re-export core types from character domain for convenient access
pub use character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterService, CharacterUpdateParams,
    League,
};

// Re-export configuration management types
pub use configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
};

// Re-export unified event system types
pub use events::{
    AppEvent, ChannelConfig, ChannelManager, EventBus, EventPublisher, EventPublisherTrait,
    EventSubscriber, EventSubscriberTrait, EventType,
};

// Re-export game monitoring types
pub use game_monitoring::{
    GameMonitoringService, GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
};

// Re-export location tracking types
pub use location_tracking::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingService,
    LocationTrackingServiceImpl, LocationTrackingSession, LocationTrackingStats, SceneTypeConfig,
};

// Re-export log analysis types
pub use log_analysis::{
    LogAnalysisConfig, LogAnalysisError, LogAnalysisEvent, LogAnalysisService,
    LogAnalysisServiceImpl, LogAnalysisSession, LogAnalysisStats, LogFileInfo, LogLineAnalysis,
};

// Re-export server monitoring types
pub use server_monitoring::{
    NetworkConfig, NetworkConnectivity, ServerInfo, ServerMonitoringConfig,
    ServerMonitoringService, ServerMonitoringServiceImpl, ServerMonitoringSession,
    ServerMonitoringStats, ServerStatus,
};

// Re-export time tracking types
pub use time_tracking::{
    LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingEvent,
    TimeTrackingService, TimeTrackingServiceImpl, TimeTrackingSummary,
};
