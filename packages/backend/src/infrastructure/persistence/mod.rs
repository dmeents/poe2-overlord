//! Data persistence infrastructure for atomic JSON file operations
//!
//! Provides a simple, type-safe API for reading and writing JSON files with
//! atomic write guarantees and XDG-compliant directory path management.
//!
//! # Core Components
//!
//! - [`FileService`]: Atomic JSON file I/O operations
//! - [`AppPaths`]: XDG-compliant directory path helpers
//!
//! # Examples
//!
//! ## Reading and writing JSON files
//!
//! ```ignore
//! use crate::infrastructure::persistence::{FileService, AppPaths};
//!
//! // Write JSON to file (atomic)
//! let data = MyData { /* ... */ };
//! let path = AppPaths::data_dir().join("my_data.json");
//! FileService::write_json(&path, &data).await?;
//!
//! // Read JSON from file
//! let data: MyData = FileService::read_json(&path).await?;
//!
//! // Read with optional (returns None if file doesn't exist)
//! let data: Option<MyData> = FileService::read_json_optional(&path).await?;
//! ```
//!
//! ## Working with XDG directories
//!
//! ```ignore
//! use crate::infrastructure::persistence::AppPaths;
//!
//! // Get config directory and ensure it exists
//! let config_dir = AppPaths::ensure_config_dir().await?;
//! let config_path = config_dir.join("config.json");
//!
//! // Get data directory
//! let data_dir = AppPaths::data_dir();
//! let file_path = data_dir.join("my_file.json");
//! ```

pub mod file_service;
pub mod paths;

pub use file_service::FileService;
pub use paths::AppPaths;
