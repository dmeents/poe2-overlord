//! # Application Layer
//!
//! The application layer serves as the orchestration and coordination layer for the POE2 Overlord application.
//! It implements the dependency injection pattern and manages the lifecycle of domain services and infrastructure components.
//!
//! ## Architecture Overview
//!
//! This layer follows clean architecture principles and is responsible for:
//! - **Service Registration**: Dependency injection and service lifecycle management
//! - **Application Bootstrap**: Initial setup, configuration, and service orchestration  
//! - **Background Task Management**: Coordination of long-running monitoring and processing tasks
//!
//! ## Key Components
//!
//! ### Service Registry (`service_registry.rs`)
//! - Implements dependency injection container
//! - Manages service initialization and registration with Tauri's app state
//! - Provides centralized access to all application services
//!
//! ### Application Setup (`app_setup.rs`)
//! - Handles application bootstrap and configuration
//! - Sets up logging, loads persisted data, and starts background services
//! - Coordinates the initialization sequence of all application components
//!
//! ### Service Orchestrator (`service_orchestrator.rs`)
//! - Manages background task lifecycle and coordination
//! - Handles service startup, monitoring, and event emission
//! - Provides clean separation between service initialization and runtime execution
//!
//! ## Service Dependencies
//!
//! The application layer coordinates the following key services:
//! - **Character Management**: Character data persistence and operations
//! - **Time Tracking**: Play time monitoring and analytics
//! - **Game Monitoring**: Process detection and game state tracking
//! - **Log Analysis**: Real-time log parsing and event extraction
//! - **Server Monitoring**: Network connectivity and server status tracking
//! - **Configuration Management**: Application settings and preferences
//!

pub mod app_setup;
pub mod service_orchestrator;
pub mod service_registry;

// Re-export the main application setup function for use in main.rs
pub use app_setup::setup_app;

// Re-export service orchestration functions for background task management
pub use service_orchestrator::{
    start_game_process_monitoring, start_log_monitoring, start_ping_event_emission,
    start_time_tracking_emission,
};

// Re-export service registry components for dependency injection
pub use service_registry::{ServiceInitializer, ServiceInstances};
