//! Character Tracking Domain Module
//!
//! This module provides comprehensive character tracking functionality for Path of Exile characters.
//! It combines location tracking and time tracking into a unified service that handles scene detection,
//! current location state management, zone statistics, and playtime tracking.
//!
//! ## Architecture
//! - **Models**: Core data structures for character tracking, location state, and zone statistics
//! - **Traits**: Service contracts and repository interfaces
//! - **Service**: Main business logic and event handling
//! - **Repository**: Data persistence and in-memory caching
//! - **Events**: Event system for real-time updates
//! - **Commands**: Tauri command handlers for frontend integration

pub mod commands;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export main types and implementations for easy access
pub use commands::*;
pub use models::*;
pub use repository::CharacterTrackingRepositoryImpl;
pub use service::{CharacterTrackingServiceImpl, ZoneBasedSceneTypeDetector};
pub use traits::*;
