use crate::commands::CommandResult;
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
