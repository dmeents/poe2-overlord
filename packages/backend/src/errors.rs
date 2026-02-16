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

    /// Network-related errors (HTTP requests, timeouts, etc.)
    #[error("Network error: {message}")]
    Network { message: String },

    /// Serialization/deserialization errors
    #[error("Serialization error: {message}")]
    Serialization { message: String },

    /// Security violations (path traversal, unauthorized access, etc.)
    #[error("Security error: {message}")]
    Security { message: String },
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
        AppError::Serialization {
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

    /// Convenience constructor for network errors
    pub fn network_error(operation: &str, message: &str) -> Self {
        Self::Network {
            message: format!("{}: {}", operation, message),
        }
    }

    /// Convenience constructor for serialization errors
    pub fn serialization_error(operation: &str, message: &str) -> Self {
        Self::Serialization {
            message: format!("{}: {}", operation, message),
        }
    }

    /// Convenience constructor for security errors
    pub fn security_error(operation: &str, message: &str) -> Self {
        Self::Security {
            message: format!("{}: {}", operation, message),
        }
    }
}

/// Type alias for results that use AppError as the error type
pub type AppResult<T> = Result<T, AppError>;

/// Serializable error for IPC communication between backend and frontend.
///
/// This struct provides a structured error format that can be serialized to JSON
/// and sent across the Tauri IPC boundary. It contains an error code (variant name)
/// and a human-readable message with operation context.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializableError {
    /// Error category code (e.g., "filesystem", "validation", "internal", "network", "serialization", "security")
    pub code: String,
    /// Human-readable error message with operation context
    pub message: String,
}

impl From<AppError> for SerializableError {
    fn from(error: AppError) -> Self {
        let code = match error {
            AppError::FileSystem { .. } => "filesystem",
            AppError::Validation { .. } => "validation",
            AppError::Internal { .. } => "internal",
            AppError::Network { .. } => "network",
            AppError::Serialization { .. } => "serialization",
            AppError::Security { .. } => "security",
        };

        Self {
            code: code.to_string(),
            message: error.to_string(),
        }
    }
}

impl std::fmt::Display for SerializableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializable_error_from_filesystem() {
        let app_error = AppError::file_system_error("read_file", "File not found");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "filesystem");
        assert!(serializable.message.contains("read_file"));
        assert!(serializable.message.contains("File not found"));
    }

    #[test]
    fn test_serializable_error_from_validation() {
        let app_error = AppError::validation_error("validate_input", "Invalid data");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "validation");
        assert!(serializable.message.contains("validate_input"));
        assert!(serializable.message.contains("Invalid data"));
    }

    #[test]
    fn test_serializable_error_from_internal() {
        let app_error = AppError::internal_error("process_data", "Unexpected failure");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "internal");
        assert!(serializable.message.contains("process_data"));
        assert!(serializable.message.contains("Unexpected failure"));
    }

    #[test]
    fn test_serializable_error_from_network() {
        let app_error = AppError::network_error("fetch_data", "Connection refused");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "network");
        assert!(serializable.message.contains("fetch_data"));
        assert!(serializable.message.contains("Connection refused"));
    }

    #[test]
    fn test_serializable_error_from_serialization() {
        let app_error = AppError::serialization_error("parse_json", "Invalid JSON");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "serialization");
        assert!(serializable.message.contains("parse_json"));
        assert!(serializable.message.contains("Invalid JSON"));
    }

    #[test]
    fn test_serializable_error_from_security() {
        let app_error = AppError::security_error("check_path", "Path traversal detected");
        let serializable = SerializableError::from(app_error);

        assert_eq!(serializable.code, "security");
        assert!(serializable.message.contains("check_path"));
        assert!(serializable.message.contains("Path traversal detected"));
    }

    #[test]
    fn test_serializable_error_json_serialization() {
        let app_error = AppError::validation_error("test_op", "test message");
        let serializable = SerializableError::from(app_error);

        let json = serde_json::to_string(&serializable).expect("Should serialize to JSON");
        assert!(json.contains("\"code\":\"validation\""));
        assert!(json.contains("\"message\":\"Validation error: test_op: test message\""));
    }

    #[test]
    fn test_serializable_error_json_deserialization() {
        let json = r#"{"code":"network","message":"Network error: fetch: timeout"}"#;
        let serializable: SerializableError =
            serde_json::from_str(json).expect("Should deserialize from JSON");

        assert_eq!(serializable.code, "network");
        assert_eq!(serializable.message, "Network error: fetch: timeout");
    }

    #[test]
    fn test_serializable_error_display() {
        let serializable = SerializableError {
            code: "test".to_string(),
            message: "Test error message".to_string(),
        };

        assert_eq!(format!("{}", serializable), "Test error message");
    }
}
