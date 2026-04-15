//! POE2 Overlord Backend Library
//!
//! This library provides the core backend functionality for the POE2 Overlord application,
//! including domain services, infrastructure components, and Tauri command handlers.

// Core application modules
pub mod application; // Application setup, service orchestration, and service registry
pub mod domain; // Business logic, models, services, and domain-specific functionality
pub mod errors; // Centralized error handling and custom error types
pub mod infrastructure; // External integrations, events, monitoring, parsing, and system utilities

// Application setup and initialization
pub use application::setup_app;

// Tauri command handlers - exposed to the frontend
pub use domain::character::commands::*; // Character CRUD operations and tracking
pub use domain::configuration::commands::*; // Configuration management
pub use domain::economy::commands::*; // Economy and currency exchange data
pub use domain::game_monitoring::commands::*; // Game process monitoring
pub use domain::leveling::commands::*; // Leveling stats and XP tracking
pub use domain::server_monitoring::commands::*; // Server status monitoring
pub use domain::walkthrough::commands::*; // Walkthrough guide and progress tracking
pub use domain::zone_configuration::commands::*; // Zone metadata lookup
pub use domain::notes::commands::*; // Notes CRUD and pin management

// Core error handling
pub use errors::*;

// Tauri command utilities
pub type CommandResult<T> = Result<T, SerializableError>;

pub fn to_command_result<T>(result: AppResult<T>) -> CommandResult<T> {
    result.map_err(SerializableError::from)
}

// Log analysis event models for tracking game state changes
pub use domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ServerConnectionEvent, ZoneChangeEvent,
};

// Character domain models and data structures
pub use domain::character::{
    Ascendency, CharacterClass, CharacterData, CharacterSummaryResponse, CharacterUpdateParams,
    EnrichedLocationState, League, LocationState, LocationType,
};

// Zone tracking domain models
pub use domain::zone_tracking::{TrackingSummary, ZoneStats};

// Walkthrough domain models and data structures
pub use domain::walkthrough::{
    CharacterWalkthroughProgress, Objective, WalkthroughAct, WalkthroughGuide, WalkthroughProgress,
    WalkthroughStep, WalkthroughStepResult,
};

// Character tracking functionality is now part of the character domain

// Configuration domain models and services
pub use domain::configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationService, ConfigurationServiceImpl,
};

// Unified event system
pub use infrastructure::events::{AppEvent, ChannelConfig, ChannelManager, EventBus, EventType};

// Game monitoring domain services and models
pub use domain::game_monitoring::{
    GameMonitoringService, GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
    ProcessDetectorImpl,
};

// Infrastructure services and utilities
// Note: Server monitoring is now handled by the domain service
pub use domain::log_analysis::traits::LogAnalysisService; // Log parsing and analysis

pub use infrastructure::events::TauriEventBridge; // Tauri event system integration
                                                  // Infrastructure utilities
pub use infrastructure::time::{
    // Time calculation and validation utilities
    calculate_active_session_duration_seconds,
    calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds,
    validate_duration,
    validate_no_session_overlap,
    validate_session_data,
    validate_timestamp_order,
    ValidationResult,
};

/// Main application entry point that configures and runs the Tauri application.
///
/// This function sets up the Tauri application with:
/// - Required plugins for shell and process management
/// - All command handlers exposed to the frontend
/// - Application initialization via `setup_app`
/// - Error handling for application startup failures
pub fn run() {
    tauri::Builder::default()
        // Initialize Tauri plugins for system integration
        .plugin(tauri_plugin_shell::init())      // Shell operations (file dialogs, etc.)
        .plugin(tauri_plugin_process::init())    // Process management and monitoring

        // Register all command handlers that can be called from the frontend
        .invoke_handler(tauri::generate_handler![
            // Configuration management commands
            get_config,
            get_default_config,
            update_config,
            reset_config_to_defaults,
            get_zone_refresh_interval,
            set_zone_refresh_interval,
            get_zone_refresh_interval_options,


            // Character management commands
            create_character,
            get_character,
            get_all_characters,
            get_all_characters_summary,
            get_character_zones,
            update_character,
            delete_character,
            set_active_character,
            get_active_character,

            // Economy commands
            get_currency_exchange_data,
            refresh_all_economy_data,
            get_all_currencies,
            search_currencies,
            toggle_currency_star,
            get_starred_currencies,

            // Game process monitoring commands
            get_game_process_status,

            // Server monitoring commands
            get_server_status,

            // Leveling stats commands
            get_leveling_stats,

            // Walkthrough guide commands
            get_walkthrough_guide,
            get_character_walkthrough_progress,
            update_character_walkthrough_progress,

            // Zone configuration commands
            get_zone_metadata_by_name,

            // Notes commands
            create_note,
            get_note,
            get_all_notes,
            get_pinned_notes,
            update_note,
            delete_note,
            toggle_note_pin,
        ])
        // Initialize application services and start background tasks
        .setup(|app| setup_app(app))
        // Run the application with generated Tauri context
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
