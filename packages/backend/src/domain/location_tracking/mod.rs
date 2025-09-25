//! Location tracking domain module
//!
//! This module provides functionality for tracking player location changes in Path of Exile 2.
//! It includes scene type detection, session management, statistics tracking, and history recording.

pub mod events;
pub mod models;
pub mod service;
pub mod traits;

// Re-export main types and traits for easy access
pub use events::LocationTrackingEvent;
pub use models::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingSession,
    LocationTrackingStats, SceneTypeConfig,
};
pub use service::{LocationTrackingServiceImpl, SimpleSceneTypeDetector};
pub use traits::{
    LocationHistoryRepository, LocationStateRepository, LocationTrackingService,
    LocationTrackingSessionRepository, LocationTrackingStatsRepository, SceneTypeDetector,
};
