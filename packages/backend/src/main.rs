// Hide the console window on Windows in release builds for a cleaner user experience
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Application entry point for the POE2 Overlord desktop application.
///
/// This is the main function that starts the Tauri application. It delegates
/// all initialization and execution logic to the `app_lib::run()` function,
/// which handles the complete Tauri application setup including:
/// - Plugin initialization
/// - Command handler registration
/// - Service orchestration
/// - Event system setup
/// - Background task management
fn main() {
    app_lib::run();
}
