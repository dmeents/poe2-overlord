use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {operation} - {message}")]
    Config { operation: String, message: String },

    #[error("Log monitoring error: {operation} - {message}")]
    LogMonitor { operation: String, message: String },

    #[error("Process monitoring error: {operation} - {message}")]
    ProcessMonitor { operation: String, message: String },

    #[error("File system error: {operation} - {message}")]
    FileSystem { operation: String, message: String },

    #[error("Serialization error: {operation} - {message}")]
    Serialization { operation: String, message: String },

    #[error("Character management error: {operation} - {message}")]
    CharacterManagement { operation: String, message: String },

    #[error("Time tracking error: {operation} - {message}")]
    TimeTracking { operation: String, message: String },

    #[error("Validation error: {operation} - {message}")]
    Validation { operation: String, message: String },

    #[error("Event emission error: {operation} - {message}")]
    EventEmission { operation: String, message: String },

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
            operation: "notify_operation".to_string(),
            message: err.to_string(),
        }
    }
}

impl AppError {
    pub fn file_system_error(operation: &str, message: &str) -> Self {
        Self::FileSystem {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn serialization_error(operation: &str, message: &str) -> Self {
        Self::Serialization {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn character_management_error(operation: &str, message: &str) -> Self {
        Self::CharacterManagement {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn time_tracking_error(operation: &str, message: &str) -> Self {
        Self::TimeTracking {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn validation_error(operation: &str, message: &str) -> Self {
        Self::Validation {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn internal_error(operation: &str, message: &str) -> Self {
        Self::Internal {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn config_error(operation: &str, message: &str) -> Self {
        Self::Config {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn log_monitor_error(operation: &str, message: &str) -> Self {
        Self::LogMonitor {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn process_monitor_error(operation: &str, message: &str) -> Self {
        Self::ProcessMonitor {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }

    pub fn event_emission_error(operation: &str, message: &str) -> Self {
        Self::EventEmission {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

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

#[macro_export]
macro_rules! handle_service_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::internal_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_file_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::file_system_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_serialization_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::serialization_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_character_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::character_management_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_time_tracking_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::time_tracking_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_event_emission_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::event_emission_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_config_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::config_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_log_monitor_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::log_monitor_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_process_monitor_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::process_monitor_error($operation, &e.to_string()))
    };
}

#[macro_export]
macro_rules! handle_validation_error {
    ($result:expr, $operation:expr) => {
        $result.map_err(|e| AppError::validation_error($operation, &e.to_string()))
    };
}
