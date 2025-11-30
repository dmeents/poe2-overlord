//! File management infrastructure for atomic JSON file operations
//!
//! Provides a type-safe API for reading and writing JSON files with atomic
//! write guarantees and XDG-compliant directory path management.

pub mod paths;
pub mod service;
pub mod traits;

pub use paths::AppPaths;
pub use service::FileService;
pub use traits::FileOperations;
