use crate::errors::{AppError, AppResult};
use log::{debug, error};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// Provides JSON serialization and deserialization utilities for data persistence
/// 
/// Handles JSON file operations with proper error handling and validation.
/// Supports both required and optional file loading scenarios.
pub struct JsonStorage;

impl JsonStorage {
    /// Serializes data to pretty-printed JSON string
    /// 
    /// Converts any serializable type to a formatted JSON string with proper indentation.
    pub fn serialize<T: Serialize>(data: &T) -> AppResult<String> {
        serde_json::to_string_pretty(data).map_err(|e| {
            AppError::serialization_error(
                "json_serialize",
                &format!("Failed to serialize data: {}", e),
            )
        })
    }

    /// Deserializes JSON string to a typed object
    /// 
    /// Parses a JSON string and converts it to the specified type.
    /// Returns an error if the JSON is malformed or incompatible with the target type.
    pub fn deserialize<T: DeserializeOwned>(json: &str) -> AppResult<T> {
        serde_json::from_str(json).map_err(|e| {
            AppError::serialization_error(
                "json_deserialize",
                &format!("Failed to deserialize JSON: {}", e),
            )
        })
    }

    /// Loads and deserializes JSON data from a file
    /// 
    /// Reads the file content and deserializes it to the specified type.
    /// Returns an error if the file doesn't exist or contains invalid JSON.
    pub fn load_from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> AppResult<T> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(AppError::file_system_error(
                "load_from_file",
                &format!("File not found: {:?}", path),
            ));
        }

        let content = crate::infrastructure::persistence::FileOperations::read_file_content(path)?;
        Self::deserialize(&content)
    }

    /// Loads JSON data from a file, returning None if the file doesn't exist
    /// 
    /// Similar to load_from_file but returns None instead of an error when the file
    /// doesn't exist. Useful for optional configuration files or data that may not be present.
    pub fn load_from_file_optional<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> AppResult<Option<T>> {
        let path = path.as_ref();
        
        if !path.exists() {
            debug!("File does not exist, returning None: {:?}", path);
            return Ok(None);
        }

        match Self::load_from_file(path) {
            Ok(data) => Ok(Some(data)),
            Err(e) => {
                error!("Failed to load JSON from file {:?}: {}", path, e);
                Err(e)
            }
        }
    }

    /// Saves serializable data to a JSON file
    /// 
    /// Serializes the data to JSON and writes it to the specified file path
    /// using atomic file operations to prevent corruption.
    pub fn save_to_file<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> AppResult<()> {
        let path = path.as_ref();
        let json_content = Self::serialize(data)?;
        
        crate::infrastructure::persistence::FileOperations::write_file_content(path, &json_content)
    }

    /// Validates that a string contains valid JSON
    /// 
    /// Attempts to parse the string as JSON and returns an error if it's malformed.
    /// Useful for validating user input or file content before processing.
    pub fn validate_json(json: &str) -> AppResult<()> {
        serde_json::from_str::<serde_json::Value>(json).map_err(|e| {
            AppError::serialization_error(
                "validate_json",
                &format!("Invalid JSON: {}", e),
            )
        })?;
        Ok(())
    }

    /// Formats JSON string with proper indentation
    /// 
    /// Parses the JSON and reformats it with consistent indentation for readability.
    /// Useful for displaying JSON data in logs or user interfaces.
    pub fn pretty_print(json: &str) -> AppResult<String> {
        let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
            AppError::serialization_error(
                "pretty_print",
                &format!("Failed to parse JSON for pretty printing: {}", e),
            )
        })?;

        serde_json::to_string_pretty(&value).map_err(|e| {
            AppError::serialization_error(
                "pretty_print",
                &format!("Failed to pretty print JSON: {}", e),
            )
        })
    }
}
