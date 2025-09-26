//! Configuration Domain Module
//!
//! This module provides a complete configuration management system for the POE2 Overlord application.
//! It follows Domain-Driven Design (DDD) principles with clean separation between different layers.
//!
//! ## Architecture
//!
//! - **Models**: Core data structures and business logic for configuration
//! - **Traits**: Interfaces defining service and repository contracts
//! - **Service**: High-level business logic and coordination
//! - **Repository**: Data persistence and storage operations
//! - **Commands**: Tauri command handlers for frontend integration
//! - **Events**: Event-driven architecture for configuration change notifications
//!
//! ## Key Features
//!
//! - Atomic configuration file operations
//! - Real-time configuration change events
//! - Validation and error handling
//! - POE client log path management
//! - Application logging level control

pub mod commands;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;


// Re-export public API for convenient access
pub use commands::*;
pub use models::*;
pub use repository::ConfigurationRepositoryImpl;
pub use service::*;
pub use traits::*;
