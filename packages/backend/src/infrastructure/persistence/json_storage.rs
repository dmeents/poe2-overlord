use crate::errors::{AppError, AppResult};
use log::{debug, error};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// JSON storage utilities for serialization and deserialization
pub struct JsonStorage;

impl JsonStorage {
    /// Serialize data to JSON string
    pub fn serialize<T: Serialize>(data: &T) -> AppResult<String> {
        serde_json::to_string_pretty(data).map_err(|e| {
            AppError::serialization_error(
                "json_serialize",
                &format!("Failed to serialize data: {}", e),
            )
        })
    }

    /// Deserialize JSON string to data
    pub fn deserialize<T: DeserializeOwned>(json: &str) -> AppResult<T> {
        serde_json::from_str(json).map_err(|e| {
            AppError::serialization_error(
                "json_deserialize",
                &format!("Failed to deserialize JSON: {}", e),
            )
        })
    }

    /// Load and deserialize data from file
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

    /// Load and deserialize data from file, returning None if file doesn't exist
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

    /// Serialize and save data to file
    pub fn save_to_file<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> AppResult<()> {
        let path = path.as_ref();
        let json_content = Self::serialize(data)?;
        
        crate::infrastructure::persistence::FileOperations::write_file_content(path, &json_content)
    }

    /// Validate JSON string without deserializing
    pub fn validate_json(json: &str) -> AppResult<()> {
        serde_json::from_str::<serde_json::Value>(json).map_err(|e| {
            AppError::serialization_error(
                "validate_json",
                &format!("Invalid JSON: {}", e),
            )
        })?;
        Ok(())
    }

    /// Pretty print JSON string
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
