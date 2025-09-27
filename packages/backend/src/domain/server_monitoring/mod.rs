//! # Server Monitoring Domain
//!
//! This module provides simplified server monitoring functionality.
//! Handles server status tracking, ping operations, and event publishing.

pub mod commands;
pub mod models;
pub mod repository;
pub mod service;

// Re-export core types for easy access
pub use commands::*;
pub use models::{ServerIp, ServerStatus};
pub use repository::{ServerIpRepository, ServerIpRepositoryTrait};
pub use service::{ServerMonitoringService, ServerMonitoringServiceImpl};
