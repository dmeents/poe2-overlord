use thiserror::Error;

/// Central error type for the POE2 Overlord application.
///
/// This enum provides structured error handling across all application domains,
/// with each variant containing contextual information about the operation that failed
/// and a descriptive error message. The `thiserror` derive macro automatically
/// implements `std::error::Error` and provides human-readable error formatting.
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration-related errors (file loading, validation, etc.)
    #[error("Configuration error: {operation} - {message}")]
    Config { operation: String, message: String },

    /// Log file monitoring and analysis errors
    #[error("Log monitoring error: {operation} - {message}")]
    LogMonitor { operation: String, message: String },

    /// Game process detection and monitoring errors
    #[error("Process monitoring error: {operation} - {message}")]
    ProcessMonitor { operation: String, message: String },

    /// File system operations (read, write, path resolution, etc.)
    #[error("File system error: {operation} - {message}")]
    FileSystem { operation: String, message: String },

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {operation} - {message}")]
    Serialization { operation: String, message: String },

    /// Character data management and persistence errors
    #[error("Character management error: {operation} - {message}")]
    CharacterManagement { operation: String, message: String },

    /// Time tracking and session management errors
    #[error("Time tracking error: {operation} - {message}")]
    TimeTracking { operation: String, message: String },

    /// Data validation and business rule enforcement errors
    #[error("Validation error: {operation} - {message}")]
    Validation { operation: String, message: String },

    /// Event system and message broadcasting errors
    #[error("Event emission error: {operation} - {message}")]
    EventEmission { operation: String, message: String },

    /// Generic internal errors for unexpected failures
    #[error("Internal error: {operation} - {message}")]
    Internal { operation: String, message: String },
}

// Standard library error conversions
impl From<std::io::Error> for AppError {
    /// Converts I/O errors to file system errors for consistent error handling
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem {
            operation: "io_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    /// Converts JSON serialization errors to application serialization errors
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization {
            operation: "json_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    /// Converts generic boxed errors to internal errors as a fallback
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Internal {
            operation: "unknown_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<notify::Error> for AppError {
    /// Converts file system notification errors to log monitoring errors
    fn from(err: notify::Error) -> Self {
        AppError::LogMonitor {
            operation: "notify_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl AppError {
    /// Convenience constructor for file system errors
    pub fn file_system_error(operation: &str, message: &str) -> Self {
        Self::FileSystem {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for serialization errors
    pub fn serialization_error(operation: &str, message: &str) -> Self {
        Self::Serialization {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for character management errors
    pub fn character_management_error(operation: &str, message: &str) -> Self {
        Self::CharacterManagement {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for time tracking errors
    pub fn time_tracking_error(operation: &str, message: &str) -> Self {
        Self::TimeTracking {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for validation errors
    pub fn validation_error(operation: &str, message: &str) -> Self {
        Self::Validation {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for internal errors
    pub fn internal_error(operation: &str, message: &str) -> Self {
        Self::Internal {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for configuration errors
    pub fn config_error(operation: &str, message: &str) -> Self {
        Self::Config {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for log monitoring errors
    pub fn log_monitor_error(operation: &str, message: &str) -> Self {
        Self::LogMonitor {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for process monitoring errors
    pub fn process_monitor_error(operation: &str, message: &str) -> Self {
        Self::ProcessMonitor {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Convenience constructor for event emission errors
    pub fn event_emission_error(operation: &str, message: &str) -> Self {
        Self::EventEmission {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }
}

/// Type alias for results that use AppError as the error type
pub type AppResult<T> = Result<T, AppError>;

/// Trait for converting any Result to AppResult with custom error mapping
pub trait ToAppError<T> {
    /// Maps any error to an AppError using a custom message generator
    fn map_app_error<F>(self, operation: &str, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ToAppError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    /// Converts any Result to AppResult by mapping the error to an internal AppError
    fn map_app_error<F>(self, operation: &str, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|_e| AppError::internal_error(operation, &f()))
    }
}

// Error handling macros for consistent error conversion across the application

/// Maps any error to an internal AppError with the specified operation context
#[macro_export]
macro_rules! handle_service_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::internal_error($operation, &e.to_string()))
    };
}

/// Maps any error to a file system AppError with the specified operation context
#[macro_export]
macro_rules! handle_file_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::file_system_error($operation, &e.to_string()))
    };
}

/// Maps any error to a serialization AppError with the specified operation context
#[macro_export]
macro_rules! handle_serialization_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::serialization_error($operation, &e.to_string()))
    };
}

/// Maps any error to a character management AppError with the specified operation context
#[macro_export]
macro_rules! handle_character_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::character_management_error($operation, &e.to_string()))
    };
}

/// Maps any error to a time tracking AppError with the specified operation context
#[macro_export]
macro_rules! handle_time_tracking_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::time_tracking_error($operation, &e.to_string()))
    };
}

/// Maps any error to an event emission AppError with the specified operation context
#[macro_export]
macro_rules! handle_event_emission_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::event_emission_error($operation, &e.to_string()))
    };
}

/// Maps any error to a configuration AppError with the specified operation context
#[macro_export]
macro_rules! handle_config_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::config_error($operation, &e.to_string()))
    };
}

/// Maps any error to a log monitoring AppError with the specified operation context
#[macro_export]
macro_rules! handle_log_monitor_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::log_monitor_error($operation, &e.to_string()))
    };
}

/// Maps any error to a process monitoring AppError with the specified operation context
#[macro_export]
macro_rules! handle_process_monitor_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::process_monitor_error($operation, &e.to_string()))
    };
}

/// Maps any error to a validation AppError with the specified operation context
#[macro_export]
macro_rules! handle_validation_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::validation_error($operation, &e.to_string()))
    };
}
