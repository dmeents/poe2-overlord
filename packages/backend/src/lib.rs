// Module declarations
pub mod commands;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

// Re-export commonly used items
pub use commands::*;
pub use errors::*;
pub use handlers::*;
pub use models::*;
pub use services::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            check_poe2_process,
            test_config_service,
            get_config,
            update_config,
            get_poe_client_log_path,
            set_poe_client_log_path,
            get_log_level,
            set_log_level,
            reset_config_to_defaults,
            get_default_poe_client_log_path,
            reset_poe_client_log_path_to_default,
            start_log_monitoring,
            stop_log_monitoring,
            is_log_monitoring_active,
            get_log_file_size,
            read_last_log_lines,
            subscribe_to_log_events
        ])
        .setup(|app| setup_app(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
