//! Runtime management infrastructure for async task orchestration
//!
//! Provides centralized management of Tokio runtime and background tasks.
//! Handles task lifecycle, spawning, and coordination across the application.

pub mod runtime_manager;
pub mod task_manager;

pub use runtime_manager::RuntimeManager;
pub use task_manager::TaskManager;
