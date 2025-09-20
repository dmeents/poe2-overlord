// Module declarations
pub mod commands;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod parsers;
pub mod services;
pub mod utils;

// Re-export commonly used items
pub use commands::*;
pub use errors::*;
pub use handlers::*;
// Import specific models to avoid naming conflicts with services
pub use models::{
    ActChangeEvent, AppConfig, Ascendency, Character, CharacterClass, CharacterData,
    CharacterUpdateParams, HideoutChangeEvent, League, LocationSession, LocationStats,
    LocationType, OverlayState, ProcessInfo, SceneChangeEvent, ServerConnectionEvent,
    TimeTrackingEvent, ZoneChangeEvent,
};
pub use services::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            check_game_process,
            get_config,
            get_default_config,
            update_config,
            reset_config_to_defaults,
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
            remove_character,
            get_characters_by_last_played,
            get_characters_by_class,
            get_characters_by_league,
            is_character_name_available,
            get_available_character_classes,
            get_available_leagues,
            get_available_ascendencies_for_class,
            update_character_last_played,
            get_character_count,
            has_characters,
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
