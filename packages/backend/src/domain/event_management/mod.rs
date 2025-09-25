//! Event Management Domain Module
//!
//! This module provides a comprehensive event-driven architecture for the POE2 Overlord application.
//! It implements a publish-subscribe pattern using Tokio's broadcast channels, allowing different
//! parts of the system to communicate asynchronously through typed events.
//!
//! Key components:
//! - **Event Types**: Defines the different types of events that can be published/subscribed to
//! - **Event Channels**: Manages broadcast channels for each event type with configurable capacity
//! - **Subscriptions**: Tracks active subscriptions and manages subscriber lifecycle
//! - **Sessions**: Monitors event management sessions with statistics and metrics
//! - **Repositories**: Provides persistence layer for subscriptions, sessions, and statistics

pub mod events;
pub mod models;
pub mod service;
pub mod traits;

// Re-export core event management types for easy access
pub use events::EventManagementEvent;
pub use models::{
    EventChannel, EventChannelConfig, EventManagementSession, EventManagementStats, EventPayload,
    EventSubscription, EventType,
};
pub use service::{EventManagementServiceImpl, SimpleEventChannelManager};
pub use traits::{
    ChannelStats, EventChannelManager, EventManagementService, EventManagementSessionRepository,
    EventManagementStatsRepository, EventSubscriptionRepository,
};
