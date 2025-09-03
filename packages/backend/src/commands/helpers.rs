use crate::commands::CommandResult;
use crate::errors::AppError;
use log::error;

/// Macro to handle service method calls with consistent error handling
#[macro_export]
macro_rules! handle_service_call {
    ($service:expr, $method:ident($($args:expr),*)) => {
        $service.$method($($args),*).map_err(|e| {
            error!("Service call failed: {}", e);
            AppError::Internal(format!("Service operation failed: {}", e))
        })
    };
}

/// Macro to handle async service method calls with consistent error handling
#[macro_export]
macro_rules! handle_async_service_call {
    ($service:expr, $method:ident($($args:expr),*)) => {
        $service.$method($($args),*).await.map_err(|e| {
            error!("Async service call failed: {}", e);
            AppError::Internal(format!("Service operation failed: {}", e))
        })
    };
}

/// Macro for Tauri commands that handles service calls and converts to CommandResult
#[macro_export]
macro_rules! command_service_call {
    ($service:expr, $method:ident($($args:expr),*)) => {
        to_command_result($service.$method($($args),*).map_err(|e| {
            error!("Service call failed: {}", e);
            AppError::Internal(format!("Service operation failed: {}", e))
        }))
    };
}

/// Macro for async Tauri commands that handles service calls and converts to CommandResult
#[macro_export]
macro_rules! async_command_service_call {
    ($service:expr, $method:ident($($args:expr),*)) => {
        to_command_result($service.$method($($args),*).await.map_err(|e| {
            error!("Async service call failed: {}", e);
            AppError::Internal(format!("Service operation failed: {}", e))
        }))
    };
}

/// Helper function to convert any error to AppError with context
pub fn to_app_error<E: std::fmt::Display>(error: E, context: &str) -> AppError {
    AppError::Internal(format!("{}: {}", context, error))
}

/// Helper function to log and convert errors
pub fn log_and_convert_error<E: std::fmt::Display>(error: E, operation: &str) -> AppError {
    error!("{} failed: {}", operation, error);
    AppError::Internal(format!("{} failed: {}", operation, error))
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
) -> CommandResult<()>
where
    E: std::fmt::Display,
{
    result.map_err(|e| {
        error!("{} failed: {}", operation, e);
        format!("{} failed: {}", operation, e)
    })
}
