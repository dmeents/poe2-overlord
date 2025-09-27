//! Game Monitoring Domain
//!
//! This module provides functionality for monitoring Path of Exile 2 game processes.
//! It includes process detection, status tracking, event publishing, and integration
//! with time tracking services.
//!
//! ## Key Components
//!
//! - **Models**: Data structures for process status and configuration
//! - **Events**: Event system for notifying about process status changes
//! - **Traits**: Abstract interfaces for process detection and monitoring services
//! - **Service**: Main implementation that orchestrates monitoring operations
//!
//! ## Integration
//!
//! The game monitoring service integrates with:
//! - Time tracking services (start/stop sessions when game starts/stops)
//! - Event management system (publishes status change events)
//! - Process detection infrastructure (platform-specific process finding)

pub mod commands;
pub mod models;
pub mod service;
pub mod traits;

// Re-export the main types and traits for easy access
pub use models::{GameMonitoringConfig, GameProcessStatus};
pub use service::GameMonitoringServiceImpl;
pub use traits::{GameMonitoringService, ProcessDetector};
