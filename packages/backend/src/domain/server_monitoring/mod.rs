//! # Server Monitoring Domain
//!
//! This module provides comprehensive server connectivity monitoring capabilities for the POE2 Overlord application.
//! It tracks server status, manages monitoring sessions, collects statistics, and provides real-time connectivity
//! information through an event-driven architecture.
//!
//! ## Key Components
//!
//! - **Models**: Core data structures for server status, configuration, sessions, and statistics
//! - **Traits**: Service contracts and repository interfaces defining the domain boundaries
//! - **Repository**: Persistence layer implementations with in-memory caching and file-based storage
//! - **Service**: Business logic orchestration for server monitoring operations
//! - **Events**: Event-driven communication for real-time status updates and monitoring notifications
//!
//! ## Architecture
//!
//! The domain follows a clean architecture pattern with clear separation of concerns:
//! - Domain models are pure data structures with business logic methods
//! - Repository traits define persistence contracts
//! - Service traits define business operation contracts
//! - Events enable loose coupling between components
//!

pub mod models;
pub mod repository;
pub mod service;
pub mod traits;


pub use models::{
    ServerInfo, ServerMonitoringConfig, ServerMonitoringSession, ServerMonitoringStats,
    ServerStatus,
};
pub use repository::{
    ServerInfoRepositoryImpl, ServerMonitoringSessionRepositoryImpl,
    ServerMonitoringStatsRepositoryImpl, ServerStatusRepositoryImpl,
};
pub use service::{NetworkConnectivityImpl, ServerMonitoringServiceImpl};
pub use traits::{
    NetworkConfig, NetworkConnectivity, ServerInfoRepository, ServerMonitoringService,
    ServerMonitoringSessionRepository, ServerMonitoringStatsRepository, ServerStatusRepository,
};
