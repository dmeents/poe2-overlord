//! Zone Configuration Domain Module
//!
//! This module provides zone configuration functionality for mapping zone names
//! to their corresponding acts and town status. It replaces the unreliable
//! log-based act detection with a configuration-driven approach.
//!
//! ## Architecture
//! - **Models**: Core data structures for zone configuration
//! - **Traits**: Service contracts and repository interfaces
//! - **Service**: Main business logic for zone configuration
//! - **Repository**: Data persistence and configuration loading

pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export main types and implementations for easy access
pub use models::*;
pub use repository::ZoneConfigurationRepositoryImpl;
pub use service::ZoneConfigurationServiceImpl;
pub use traits::*;
