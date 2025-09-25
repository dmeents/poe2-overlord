//! Time Tracking Domain Module
//!
//! This module provides comprehensive time tracking functionality for Path of Exile characters.
//! It tracks location sessions, calculates statistics, and manages persistent storage.
//!
//! ## Architecture
//! - **Models**: Core data structures for sessions, statistics, and summaries
//! - **Traits**: Service contracts and repository interfaces
//! - **Service**: Main business logic and event handling
//! - **Repository**: Data persistence and in-memory caching
//! - **Events**: Event system for real-time updates
//! - **Commands**: Tauri command handlers for frontend integration

pub mod commands;
pub mod events;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export main types and implementations for easy access
pub use commands::*;
pub use events::*;
pub use models::*;
pub use repository::TimeTrackingRepositoryImpl;
pub use service::TimeTrackingServiceImpl;
pub use traits::*;
