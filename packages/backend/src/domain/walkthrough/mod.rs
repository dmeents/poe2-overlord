//! Walkthrough domain module
//!
//! This module provides functionality for managing walkthrough guides and character progress
//! through the Path of Exile 2 campaign. It includes models, services, repositories, and
//! Tauri command handlers for walkthrough functionality.

pub mod commands;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export commonly used types for convenience
pub use commands::{
    advance_character_walkthrough_step, get_character_walkthrough_progress, get_walkthrough_guide,
    get_walkthrough_step, handle_walkthrough_scene_change, mark_character_campaign_completed,
    move_character_to_walkthrough_step,
};
pub use models::{
    CharacterWalkthroughProgress, Objective, WalkthroughAct, WalkthroughGuide, WalkthroughProgress,
    WalkthroughStep, WalkthroughStepResult,
};
pub use repository::WalkthroughRepositoryImpl;
pub use service::WalkthroughServiceImpl;
pub use traits::{WalkthroughRepository, WalkthroughService};
