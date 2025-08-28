use thiserror::Error;

/// Unified error type for the entire backend application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Log monitoring error: {0}")]
    LogMonitor(String),

    #[error("Process monitoring error: {0}")]
    ProcessMonitor(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<notify::Error> for AppError {
    fn from(err: notify::Error) -> Self {
        AppError::LogMonitor(err.to_string())
    }
}

/// Result type alias for backend operations
pub type AppResult<T> = Result<T, AppError>;

/// Helper trait for converting errors to AppError
pub trait ToAppError<T> {
    fn map_app_error<F, E>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> E,
        E: std::fmt::Display;
}

impl<T, E> ToAppError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn map_app_error<F, E2>(self, f: F) -> AppResult<T>
    where
        F: FnOnce() -> E2,
        E2: std::fmt::Display,
    {
        self.map_err(|_| AppError::Internal(f().to_string()))
    }
}
