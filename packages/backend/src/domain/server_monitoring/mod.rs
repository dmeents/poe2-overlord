//! # Server Monitoring Domain
//!
//! This module provides simplified server monitoring functionality.
//! Handles server status tracking, ping operations, and event publishing.

pub mod models;
pub mod repository;
pub mod service;

// Re-export core types for easy access
pub use models::ServerStatus;
pub use repository::ServerStatusRepository;
pub use service::{ServerMonitoringService, ServerMonitoringServiceImpl};
