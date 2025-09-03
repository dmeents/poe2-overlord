pub mod command_utils;
pub mod config_commands;
pub mod helpers;
pub mod log_commands;
pub mod server_status_commands;
pub mod time_tracking_commands;

pub use command_utils::*;
pub use config_commands::*;
pub use helpers::*;
pub use log_commands::*;
pub use server_status_commands::*;
pub use time_tracking_commands::*;

// Re-export common types for commands
pub use crate::errors::AppError;
pub use crate::errors::AppResult;

/// Type alias for Tauri command results
/// This wraps AppResult to provide consistent error handling for Tauri commands
pub type CommandResult<T> = Result<T, String>;

/// Convert AppResult to CommandResult for Tauri commands
pub fn to_command_result<T>(result: AppResult<T>) -> CommandResult<T> {
    result.map_err(|e| e.to_string())
}

/// Convert any error to CommandResult with context
pub fn error_to_command_result<T, E: std::fmt::Display>(
    error: E,
    context: &str,
) -> CommandResult<T> {
    Err(format!("{}: {}", context, error))
}
