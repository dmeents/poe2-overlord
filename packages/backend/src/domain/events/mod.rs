//! Unified Event System
//!
//! This module provides a single, unified event system for the entire application.
//! It replaces all domain-specific event systems with a centralized, consistent
//! approach to event publishing and subscribing.
//!
//! Key components:
//! - **AppEvent**: Single enum containing all possible events
//! - **EventBus**: Central event bus for publishing and subscribing
//! - **EventType**: Categorization of event types for channel management
//! - **ChannelManager**: Manages broadcast channels for each event type
//! - **Publisher/Subscriber**: Simple interfaces for event operations

pub mod channel_manager;
pub mod event_bus;
pub mod event_types;
pub mod publisher;
pub mod subscriber;
pub mod traits;

// Re-export core types for easy access
pub use channel_manager::ChannelManager;
pub use event_bus::EventBus;
pub use event_types::{AppEvent, ChannelConfig, EventType};
pub use publisher::EventPublisher;
pub use subscriber::EventSubscriber;
pub use traits::{EventPublisherTrait, EventSubscriberTrait};
