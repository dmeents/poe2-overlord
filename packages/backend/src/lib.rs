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
    ActChangeEvent, AppConfig, HideoutChangeEvent, LocationSession, LocationStats, LocationType,
    OverlayState, ProcessInfo, SceneChangeEvent, ServerConnectionEvent, TimeTrackingEvent,
    ZoneChangeEvent,
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
            // Time tracking commands
            get_time_tracking_data,
            start_time_tracking_session,
            end_time_tracking_session,
            end_all_active_sessions,
            clear_all_time_tracking_data,
        ])
        .setup(|app| setup_app(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
