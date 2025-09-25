pub mod application;
pub mod domain;
pub mod errors;
pub mod infrastructure;

pub use application::setup_app;
pub use domain::character::commands::*;
pub use domain::configuration::commands::*;
pub use domain::log_analysis::commands::*;
pub use domain::time_tracking::commands::*;
pub use errors::*;
pub use domain::game_monitoring::models::{OverlayState, ProcessInfo};
pub use domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ServerConnectionEvent, ZoneChangeEvent,
};
pub use domain::character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterUpdateParams, League,
};
pub use domain::time_tracking::{
    LocationSession, LocationStats, LocationType, TimeTrackingData, TimeTrackingEvent,
    TimeTrackingService, TimeTrackingServiceImpl, TimeTrackingSummary,
};
pub use domain::configuration::{
    AppConfig, ConfigurationChangedEvent, ConfigurationFileInfo, ConfigurationService,
    ConfigurationServiceImpl, ConfigurationValidationResult,
};
pub use domain::game_monitoring::{
    GameMonitoringEvent, GameMonitoringEventPublisher, GameMonitoringService,
    GameMonitoringServiceImpl, GameProcessStatus, ProcessDetector,
};
pub use infrastructure::monitoring::ServerMonitor;
pub use infrastructure::parsing::{LocationTracker, LogAnalyzer, SceneTypeConfig};
pub use infrastructure::system::{detect_os, get_os_name, OperatingSystem, PoeClientLogPaths};
pub use infrastructure::tauri::{EventDispatcher, EventService};
pub use infrastructure::time::{
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
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
            is_log_monitoring_active,
            get_log_file_size,
            read_last_log_lines,
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
