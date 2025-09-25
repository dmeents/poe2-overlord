pub use crate::errors::AppError;
pub use crate::errors::AppResult;

pub type CommandResult<T> = Result<T, String>;

pub fn to_command_result<T>(result: AppResult<T>) -> CommandResult<T> {
    result.map_err(|e| e.to_string())
}

pub fn error_to_command_result<T, E: std::fmt::Display>(
    error: E,
    context: &str,
) -> CommandResult<T> {
    Err(format!("{}: {}", context, error))
}

use log::{debug, error};

#[derive(Debug, Clone)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: &str) -> CommandResult<T> {
        Err(message.to_string())
    }

    pub fn success_no_data() -> CommandResult<()> {
        Ok(())
    }
}

#[macro_export]
macro_rules! command_service_call {
    ($service:expr, $method:ident($($args:expr),*), $operation:expr) => {
        to_command_result($service.$method($($args),*).map_err(|e| {
            error!("Service call failed: {}", e);
            AppError::internal_error($operation, &e.to_string())
        }))
    };
}

#[macro_export]
macro_rules! async_command_service_call {
    ($service:expr, $method:ident($($args:expr),*), $operation:expr) => {
        to_command_result($service.$method($($args),*).await.map_err(|e| {
            error!("Async service call failed: {}", e);
            AppError::internal_error($operation, &e.to_string())
        }))
    };
}

#[macro_export]
macro_rules! command_log {
    ($level:ident, $command:expr, $message:expr) => {
        log::$level!("[{}] {}", $command, $message);
    };

    ($level:ident, $command:expr, $message:expr, $($args:tt)*) => {
        log::$level!("[{}] {}", $command, format!($message, $($args)*));
    };
}

#[macro_export]
macro_rules! command_entry {
    ($command:expr) => {
        debug!("[{}] Command invoked", $command);
    };
}

#[macro_export]
macro_rules! command_exit {
    ($command:expr, $result:expr) => {
        match $result {
            Ok(_) => debug!("[{}] Command completed successfully", $command),
            Err(ref e) => error!("[{}] Command failed: {}", $command, e),
        }
    };
}

pub fn to_app_error<E: std::fmt::Display>(error: E, operation: &str) -> AppError {
    AppError::internal_error(operation, &error.to_string())
}

pub fn log_and_convert_error<E: std::fmt::Display>(error: E, operation: &str) -> AppError {
    error!("{} failed: {}", operation, error);
    AppError::internal_error(operation, &error.to_string())
}

pub fn handle_command_error<E: std::fmt::Display>(error: E, operation: &str) -> CommandResult<()> {
    error!("{} failed: {}", operation, error);
    Err(format!("{} failed: {}", operation, error))
}

pub async fn handle_async_command_error<E: std::fmt::Display>(
    result: Result<(), E>,
    operation: &str,
) -> CommandResult<()> {
    result.map_err(|e| {
        error!("{} failed: {}", operation, e);
        format!("{} failed: {}", operation, e)
    })
}

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

pub fn validate_string_param(param: &str, param_name: &str) -> CommandResult<()> {
    if param.trim().is_empty() {
        return Err(format!("{} cannot be empty", param_name));
    }
    Ok(())
}

pub fn validate_positive_number<T: PartialOrd + std::fmt::Display + From<i32>>(
    value: T,
    param_name: &str,
) -> CommandResult<()> {
    if value <= T::from(0) {
        return Err(format!("{} must be positive, got {}", param_name, value));
    }
    Ok(())
}
