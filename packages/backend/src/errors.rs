use thiserror::Error;

/// Central error type for the POE2 Overlord application.
///
/// Simplified error handling with three main categories covering all application needs.
#[derive(Error, Debug)]
pub enum AppError {
    /// File system operations (read, write, path resolution, etc.)
    #[error("File system error: {message}")]
    FileSystem { message: String },

    /// Data validation and business rule enforcement errors
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// Generic internal errors for unexpected failures
    #[error("Internal error: {message}")]
    Internal { message: String },
}

// Standard library error conversions
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal {
            message: format!("JSON error: {}", err),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Internal {
            message: err.to_string(),
        }
    }
}

impl From<notify::Error> for AppError {
    fn from(err: notify::Error) -> Self {
        AppError::FileSystem {
            message: format!("File notification error: {}", err),
        }
    }
}

impl AppError {
    /// Convenience constructor for file system errors
    pub fn file_system_error(operation: &str, message: &str) -> Self {
        Self::FileSystem {
            message: format!("{}: {}", operation, message),
        }
    }

    /// Convenience constructor for validation errors
    pub fn validation_error(operation: &str, message: &str) -> Self {
        Self::Validation {
            message: format!("{}: {}", operation, message),
        }
    }

    /// Convenience constructor for internal errors
    pub fn internal_error(operation: &str, message: &str) -> Self {
        Self::Internal {
            message: format!("{}: {}", operation, message),
        }
    }
}

/// Type alias for results that use AppError as the error type
pub type AppResult<T> = Result<T, AppError>;
