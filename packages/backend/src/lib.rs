// Module declarations
pub mod models;
pub mod services;
pub mod commands;
pub mod handlers;

// Re-export commonly used items
pub use models::*;
pub use services::*;
pub use commands::*;
pub use handlers::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            check_poe2_process,
            toggle_overlay_visibility,
            set_window_position,
            get_window_position,
            set_always_on_top
        ])
        .setup(|app| {
            setup_app(app)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
