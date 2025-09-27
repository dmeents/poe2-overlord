//! Data persistence infrastructure for file operations and storage
//!
//! Provides utilities for atomic file writing, JSON serialization/deserialization,
//! directory management, and repository pattern implementations for data persistence.

pub mod atomic_writer;
pub mod directory_manager;
pub mod file_operations;
pub mod json_storage;
pub mod persistence_repository;
pub mod repository_traits;

pub use atomic_writer::*;
pub use directory_manager::*;
pub use file_operations::*;
pub use json_storage::*;
pub use persistence_repository::*;
pub use repository_traits::*;
