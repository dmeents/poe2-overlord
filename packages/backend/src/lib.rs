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
pub use domain::game_monitoring::commands::*; // Game process monitoring
pub use domain::walkthrough::commands::*; // Walkthrough guide and progress tracking

// Core error handling
pub use errors::*;

// Tauri command utilities
pub type CommandResult<T> = Result<T, String>;

pub fn to_command_result<T>(result: AppResult<T>) -> CommandResult<T> {
    result.map_err(|e| e.to_string())
}

// Game monitoring models and state
pub use domain::game_monitoring::models::{OverlayState, ProcessInfo};

// Log analysis event models for tracking game state changes
pub use domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ServerConnectionEvent, ZoneChangeEvent,
};

// Character domain models and data structures
pub use domain::character::{
    Ascendency, CharacterClass, CharacterData, CharacterUpdateParams, CharactersIndex, League,
    LocationState, LocationType, TrackingSummary, ZoneStats,
};

// Walkthrough domain models and data structures
pub use domain::walkthrough::{
    CharacterWalkthroughProgress, Objective, WalkthroughAct, WalkthroughGuide, WalkthroughProgress,
    WalkthroughStep, WalkthroughStepResult,
};

// Character tracking functionality is now part of the character domain

// Configuration domain models and services
pub use domain::configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
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
/// - Application initialization via setup_app
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
            get_poe_client_log_path,
            set_poe_client_log_path,
            get_default_poe_client_log_path,
            reset_poe_client_log_path_to_default,
            get_log_level,
            set_log_level,
            get_config_file_info,
            validate_config,


            // Character management commands
            create_character,
            update_character,
            get_all_characters,
            get_active_character,
            set_active_character,
            delete_character,
            get_available_character_classes,
            get_available_leagues,
            get_available_ascendencies_for_class,

            // Character tracking commands (location and time)
            get_character_tracking_data,
            get_character_current_location,
            enter_zone,
            leave_zone,
            record_death,
            add_zone_time,
            finalize_all_active_zones,

            // Game process monitoring commands
            get_game_process_status,

            // Walkthrough guide commands
            get_walkthrough_guide,
            get_character_walkthrough_progress,
            update_character_walkthrough_progress,
        ])
        // Initialize application services and start background tasks
        .setup(|app| setup_app(app))
        // Run the application with generated Tauri context
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
