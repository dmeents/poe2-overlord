pub mod monitoring;
pub mod network;
pub mod parsing;
pub mod persistence;
pub mod tauri;

// Re-export main types for easy access
pub use monitoring::process_monitor::ProcessMonitorImpl;
pub use tauri::game_monitoring_handler::GameMonitoringHandler;