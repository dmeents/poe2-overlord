// Core application modules
pub mod application;  // Application setup, service orchestration, and service registry
pub mod domain;       // Business logic, models, services, and domain-specific functionality
pub mod errors;       // Centralized error handling and custom error types
pub mod infrastructure; // External integrations, monitoring, parsing, and system utilities

// Application setup and initialization
pub use application::setup_app;

// Tauri command handlers - exposed to the frontend
pub use domain::character::commands::*;        // Character CRUD operations
pub use domain::configuration::commands::*;    // Configuration management
pub use domain::log_analysis::commands::*;     // Log file analysis and monitoring
pub use domain::time_tracking::commands::*;    // Time tracking and session management

// Core error handling
pub use errors::*;

// Game monitoring models and state
pub use domain::game_monitoring::models::{OverlayState, ProcessInfo};

// Log analysis event models for tracking game state changes
pub use domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ServerConnectionEvent, ZoneChangeEvent,
};

// Character domain models and data structures
pub use domain::character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterUpdateParams, League,
};

// Time tracking domain models and services
pub use domain::time_tracking::{
    LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingEvent,
    TimeTrackingService, TimeTrackingServiceImpl, TimeTrackingSummary,
};

// Configuration domain models and services
pub use domain::configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
};

// Game monitoring domain services and models
pub use domain::game_monitoring::{
    GameMonitoringEvent, GameMonitoringEventPublisher, GameMonitoringService,
    GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
};

// Location tracking domain services and models
pub use domain::location_tracking::{
    LocationTrackingEvent, LocationTrackingService, LocationTrackingServiceImpl,
    LocationTrackingSession, LocationTrackingStats, LocationState, SceneTypeConfig,
    SimpleSceneTypeDetector,
};
pub use domain::location_tracking::models::SceneType;

// Infrastructure services and utilities
pub use infrastructure::monitoring::ServerMonitor;  // Server connectivity monitoring
pub use domain::log_analysis::traits::LogAnalysisService;  // Log parsing and analysis
pub use infrastructure::system::{detect_os, get_os_name, OperatingSystem, PoeClientLogPaths};  // OS detection and paths
pub use infrastructure::tauri::{EventDispatcher, EventService};  // Tauri event system integration
pub use infrastructure::time::{  // Time calculation and validation utilities
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
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
            
            // Log monitoring commands
            is_log_monitoring_active,
            get_log_file_size,
            read_last_log_lines,
            
            // Character management commands
            create_character,
            update_character,
            get_all_characters,
            get_character,
            get_active_character,
            set_active_character,
            delete_character,
            get_available_character_classes,
            get_available_leagues,
            get_available_ascendencies_for_class,
            clear_all_character_data,
            update_character_level,
            increment_character_deaths,
            
            // Time tracking commands
            get_character_time_tracking_data,
            clear_character_time_tracking_data,
            get_character_active_sessions,
            get_character_completed_sessions,
            get_character_last_known_location,
            get_character_location_stats,
            get_character_total_play_time,
            get_character_total_play_time_since_process_start,
            get_character_total_hideout_time,
        ])
        // Initialize application services and start background tasks
        .setup(|app| setup_app(app))
        // Run the application with generated Tauri context
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
