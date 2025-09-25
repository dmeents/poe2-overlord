use thiserror::Error;

/// Unified error type for the entire backend application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Log monitoring error: {message}")]
    LogMonitor { message: String },

    #[error("Process monitoring error: {message}")]
    ProcessMonitor { message: String },

    #[error("File system error: {operation} - {message}")]
    FileSystem { operation: String, message: String },

    #[error("Serialization error: {operation} - {message}")]
    Serialization { operation: String, message: String },

    #[error("Character management error: {operation} - {message}")]
    CharacterManagement { operation: String, message: String },

    #[error("Time tracking error: {operation} - {message}")]
    TimeTracking { operation: String, message: String },

    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("Event emission error: {message}")]
    EventEmission { message: String },

    #[error("Internal error: {operation} - {message}")]
    Internal { operation: String, message: String },
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem {
            operation: "io_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization {
            operation: "json_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Internal {
            operation: "unknown_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<notify::Error> for AppError {
    fn from(err: notify::Error) -> Self {
        AppError::LogMonitor {
            message: err.to_string(),
        }
    }
}

impl AppError {
    /// Create a file system error with context
    pub fn file_system_error(operation: &str, message: &str) -> Self {
        Self::FileSystem {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a serialization error with context
    pub fn serialization_error(operation: &str, message: &str) -> Self {
        Self::Serialization {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a character management error with context
    pub fn character_management_error(operation: &str, message: &str) -> Self {
        Self::CharacterManagement {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a time tracking error with context
    pub fn time_tracking_error(operation: &str, message: &str) -> Self {
        Self::TimeTracking {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a validation error with context
    pub fn validation_error(field: &str, message: &str) -> Self {
        Self::Validation {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Create an internal error with context
    pub fn internal_error(operation: &str, message: &str) -> Self {
        Self::Internal {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a configuration error
    pub fn config_error(message: &str) -> Self {
        Self::Config {
            message: message.to_string(),
        }
    }

    /// Create a log monitor error
    pub fn log_monitor_error(message: &str) -> Self {
        Self::LogMonitor {
            message: message.to_string(),
        }
    }

    /// Create a process monitor error
    pub fn process_monitor_error(message: &str) -> Self {
        Self::ProcessMonitor {
            message: message.to_string(),
        }
    }

    /// Create an event emission error
    pub fn event_emission_error(message: &str) -> Self {
        Self::EventEmission {
            message: message.to_string(),
        }
    }
}

/// Result type alias for backend operations
pub type AppResult<T> = Result<T, AppError>;

/// Helper trait for converting errors to AppError with context
pub trait ToAppError<T> {
    fn map_app_error<F>(self, operation: &str, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ToAppError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn map_app_error<F>(self, operation: &str, f: F) -> AppResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|_e| AppError::internal_error(operation, &f()))
    }
}

/// Macro for consistent error handling in service methods
#[macro_export]
macro_rules! handle_service_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::internal_error($operation, &e.to_string()))
    };
}

/// Macro for file system operations
#[macro_export]
macro_rules! handle_file_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::file_system_error($operation, &e.to_string()))
    };
}

/// Macro for serialization operations
#[macro_export]
macro_rules! handle_serialization_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::serialization_error($operation, &e.to_string()))
    };
}

/// Macro for character management operations
#[macro_export]
macro_rules! handle_character_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::character_management_error($operation, &e.to_string()))
    };
}

/// Macro for time tracking operations
#[macro_export]
macro_rules! handle_time_tracking_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::time_tracking_error($operation, &e.to_string()))
    };
}

/// Macro for event emission operations
#[macro_export]
macro_rules! handle_event_emission_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::event_emission_error(&format!("{}: {}", $operation, e.to_string())))
    };
}
