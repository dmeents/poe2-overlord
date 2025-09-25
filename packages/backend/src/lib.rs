// Module declarations
pub mod commands;
pub mod domain;
pub mod errors;
pub mod handlers;
pub mod infrastructure;
pub mod models;
pub mod parsers;
pub mod services;
pub mod utils;

// Re-export commonly used items
pub use commands::*;
pub use domain::character::commands::*;
pub use domain::time_tracking::commands::*;
pub use errors::*;
pub use handlers::*;
// Import specific models to avoid naming conflicts with services
pub use models::{
    ActChangeEvent, HideoutChangeEvent, OverlayState, ProcessInfo, SceneChangeEvent,
    ServerConnectionEvent, ZoneChangeEvent,
};
// Import character models from domain
pub use domain::character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterUpdateParams, League,
};
// Import time tracking from domain
pub use domain::time_tracking::{
    LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingEvent,
    TimeTrackingService, TimeTrackingServiceImpl, TimeTrackingSummary,
};
// Import configuration from domain
pub use domain::configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
};
// Import game monitoring from domain
pub use domain::game_monitoring::{
    GameMonitoringEvent, GameMonitoringEventPublisher, GameMonitoringService,
    GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
};
// Export infrastructure components
pub use infrastructure::monitoring::ServerMonitor;
pub use infrastructure::parsing::{LocationTracker, LogAnalyzer, SceneTypeConfig};
pub use infrastructure::tauri::{EventDispatcher, EventService};
// Export service registry
pub use services::registry::ServiceRegistryImpl;
pub use services::traits::ServiceRegistry;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            // Configuration commands
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
            // Log commands
            is_log_monitoring_active,
            get_log_file_size,
            read_last_log_lines,
            // Note: Old time tracking commands removed in favor of character-aware commands
            // Character commands
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
            // Character time tracking commands
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
        .setup(|app| setup_app(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
