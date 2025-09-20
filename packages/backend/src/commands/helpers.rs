use crate::commands::CommandResult;
use crate::errors::AppError;
use log::error;

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
