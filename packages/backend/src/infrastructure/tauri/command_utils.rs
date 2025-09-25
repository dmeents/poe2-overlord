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

use log::{debug, error};

/// Command response wrapper for consistent formatting
#[derive(Debug, Clone)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> CommandResponse<T> {
    /// Create a successful response
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(message: &str) -> CommandResult<T> {
        Err(message.to_string())
    }

    /// Create a success response with no data
    pub fn success_no_data() -> CommandResult<()> {
        Ok(())
    }
}

/// Macro for Tauri commands that handles service calls and converts to CommandResult
#[macro_export]
macro_rules! command_service_call {
    ($service:expr, $method:ident($($args:expr),*), $operation:expr) => {
        to_command_result($service.$method($($args),*).map_err(|e| {
            error!("Service call failed: {}", e);
            AppError::internal_error($operation, &e.to_string())
        }))
    };
}

/// Macro for async Tauri commands that handles service calls and converts to CommandResult
#[macro_export]
macro_rules! async_command_service_call {
    ($service:expr, $method:ident($($args:expr),*), $operation:expr) => {
        to_command_result($service.$method($($args),*).await.map_err(|e| {
            error!("Async service call failed: {}", e);
            AppError::internal_error($operation, &e.to_string())
        }))
    };
}

/// Macro for consistent command logging
#[macro_export]
macro_rules! command_log {
    ($level:ident, $command:expr, $message:expr) => {
        log::$level!("[{}] {}", $command, $message);
    };

    ($level:ident, $command:expr, $message:expr, $($args:tt)*) => {
        log::$level!("[{}] {}", $command, format!($message, $($args)*));
    };
}

/// Macro for command entry logging
#[macro_export]
macro_rules! command_entry {
    ($command:expr) => {
        debug!("[{}] Command invoked", $command);
    };
}

/// Macro for command exit logging
#[macro_export]
macro_rules! command_exit {
    ($command:expr, $result:expr) => {
        match $result {
            Ok(_) => debug!("[{}] Command completed successfully", $command),
            Err(ref e) => error!("[{}] Command failed: {}", $command, e),
        }
    };
}

/// Helper function to convert any error to AppError with context
pub fn to_app_error<E: std::fmt::Display>(error: E, operation: &str) -> AppError {
    AppError::internal_error(operation, &error.to_string())
}

/// Helper function to log and convert errors
pub fn log_and_convert_error<E: std::fmt::Display>(error: E, operation: &str) -> AppError {
    error!("{} failed: {}", operation, error);
    AppError::internal_error(operation, &error.to_string())
}

/// Helper function for Tauri commands to handle errors consistently
pub fn handle_command_error<E: std::fmt::Display>(error: E, operation: &str) -> CommandResult<()> {
    error!("{} failed: {}", operation, error);
    Err(format!("{} failed: {}", operation, error))
}

/// Helper function for Tauri commands to handle async errors consistently
pub async fn handle_async_command_error<E: std::fmt::Display>(
    result: Result<(), E>,
    operation: &str,
) -> CommandResult<()> {
    result.map_err(|e| {
        error!("{} failed: {}", operation, e);
        format!("{} failed: {}", operation, e)
    })
}

/// Helper function to log command execution time
pub async fn log_command_execution<F, Fut, T>(command_name: &str, f: F) -> CommandResult<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = CommandResult<T>>,
{
    let start = std::time::Instant::now();
    command_entry!(command_name);

    let result = f().await;

    let duration = start.elapsed();
    debug!("[{}] Command executed in {:?}", command_name, duration);

    command_exit!(command_name, result);
    result
}

/// Helper function to validate command parameters
pub fn validate_string_param(param: &str, param_name: &str) -> CommandResult<()> {
    if param.trim().is_empty() {
        return Err(format!("{} cannot be empty", param_name));
    }
    Ok(())
}

/// Helper function to validate numeric parameters
pub fn validate_positive_number<T: PartialOrd + std::fmt::Display + From<i32>>(
    value: T,
    param_name: &str,
) -> CommandResult<()> {
    if value <= T::from(0) {
        return Err(format!("{} must be positive, got {}", param_name, value));
    }
    Ok(())
}
