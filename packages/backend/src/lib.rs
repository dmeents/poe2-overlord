// Module declarations
pub mod commands;
pub mod handlers;
pub mod models;
pub mod services;

// Re-export commonly used items
pub use commands::*;
pub use handlers::*;
pub use models::*;
pub use services::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![check_poe2_process])
        .setup(|app| setup_app(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
