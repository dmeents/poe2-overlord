//! Configuration Event System
//!
//! This module implements a comprehensive event-driven architecture for configuration
//! change notifications. It provides real-time broadcasting of configuration changes
//! throughout the application, allowing components to react immediately to configuration
//! updates without polling.
//!
//! # Architecture
//!
//! The event system is built around three main components:
//!
//! - **ConfigurationEventHandler**: Core event broadcasting functionality
//! - **ConfigurationEventListener**: Individual event subscription and listening
//! - **ConfigurationEventManager**: High-level coordination and management
//!
//! # Event Flow
//!
//! 1. Configuration changes trigger event creation
//! 2. Events are broadcast through a Tokio broadcast channel
//! 3. Multiple subscribers can receive events simultaneously
//! 4. Events include both old and new configuration states
//! 5. Timestamps enable event ordering and debugging
//!
//! # Use Cases
//!
//! - Log level changes updating runtime logging configuration
//! - POE client path changes updating file watchers
//! - Configuration validation results for UI updates
//! - Audit logging of configuration changes
//!
//! # Error Handling
//!
//! The event system is designed to be resilient:
//! - Missing receivers don't prevent configuration changes
//! - Lagged receivers automatically catch up or skip events
//! - Closed channels are handled gracefully with appropriate logging

use crate::domain::configuration::models::{AppConfig, ConfigurationChangedEvent};
use crate::errors::AppResult;
use log::{debug, info, warn};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Core configuration event broadcasting handler
/// 
/// This struct manages the broadcasting of configuration change events to multiple
/// subscribers using Tokio's broadcast channel. It provides thread-safe event
/// emission with automatic receiver management.
/// 
/// # Capacity Management
/// 
/// The broadcast channel has a limited capacity. When the channel is full,
/// the oldest messages are dropped. Receivers that can't keep up will receive
/// `RecvError::Lagged` errors indicating how many events were missed.
/// 
/// # Thread Safety
/// 
/// All operations are thread-safe and can be called concurrently from multiple
/// async tasks without additional synchronization.
pub struct ConfigurationEventHandler {
    /// Broadcast sender for distributing configuration change events
    event_sender: broadcast::Sender<ConfigurationChangedEvent>,
}

impl ConfigurationEventHandler {
    /// Create a new event handler with default capacity (16 events)
    /// 
    /// The default capacity is suitable for most use cases where configuration
    /// changes are infrequent. For applications with high-frequency configuration
    /// updates, consider using `with_capacity()` with a larger buffer.
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(16);
        Self { event_sender }
    }

    /// Create a new event handler with a specific channel capacity
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Maximum number of events to buffer in the channel
    /// 
    /// # Capacity Guidelines
    /// 
    /// - Small applications: 8-16 events
    /// - Medium applications: 32-64 events  
    /// - High-frequency updates: 128+ events
    pub fn with_capacity(capacity: usize) -> Self {
        let (event_sender, _) = broadcast::channel(capacity);
        Self { event_sender }
    }

    /// Broadcast a configuration change event to all subscribers
    /// 
    /// Creates and sends a configuration change event containing both the new
    /// and previous configuration states. All active subscribers will receive
    /// this event asynchronously.
    /// 
    /// # Arguments
    /// 
    /// * `new_config` - The configuration state after the change
    /// * `previous_config` - The configuration state before the change
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the event was successfully broadcast
    /// * `Err(AppError)` if no receivers are available
    /// 
    /// # Behavior
    /// 
    /// - Events are timestamped automatically
    /// - The number of receivers that received the event is logged
    /// - If no receivers exist, a warning is logged and an error is returned
    pub fn broadcast_config_change(
        &self,
        new_config: AppConfig,
        previous_config: AppConfig,
    ) -> AppResult<()> {
        let event = ConfigurationChangedEvent::new(new_config, previous_config);

        match self.event_sender.send(event) {
            Ok(receiver_count) => {
                debug!(
                    "Configuration change event broadcasted to {} receivers",
                    receiver_count
                );
                Ok(())
            }
            Err(broadcast::error::SendError(_event)) => {
                warn!("Failed to broadcast configuration change event: no receivers");
                Err(crate::errors::AppError::event_emission_error(
                    "emit_configuration_change",
                    "No receivers for configuration change event",
                ))
            }
        }
    }

    /// Create a new subscriber to configuration change events
    /// 
    /// Returns a new receiver that will receive all future configuration
    /// change events. Each receiver operates independently and maintains
    /// its own position in the event stream.
    /// 
    /// # Returns
    /// 
    /// A new `broadcast::Receiver` for configuration change events
    pub fn subscribe(&self) -> broadcast::Receiver<ConfigurationChangedEvent> {
        self.event_sender.subscribe()
    }

    /// Get the current number of active receivers
    /// 
    /// This count includes all active receivers that have been created
    /// but not yet dropped. It's useful for monitoring and debugging
    /// the event system.
    /// 
    /// # Returns
    /// 
    /// The number of currently active receivers
    pub fn receiver_count(&self) -> usize {
        self.event_sender.receiver_count()
    }
}

impl Default for ConfigurationEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual event listener for configuration changes
/// 
/// This struct provides a convenient interface for subscribing to and receiving
/// configuration change events. Each listener maintains its own position in the
/// event stream and can be used independently of other listeners.
/// 
/// # Usage Patterns
/// 
/// - **Async Listening**: Use `listen_for_change()` to wait for the next event
/// - **Non-blocking Polling**: Use `try_receive()` to check for events without blocking
/// - **Event Processing**: Handle both successful events and error conditions
/// 
/// # Error Handling
/// 
/// The listener automatically handles common broadcast channel errors:
/// - **Lagged**: Automatically recovers from missed events
/// - **Closed**: Returns appropriate errors when the channel is closed
/// - **Empty**: Non-blocking operations return `None` when no events are available
pub struct ConfigurationEventListener {
    /// The underlying broadcast receiver for configuration events
    receiver: broadcast::Receiver<ConfigurationChangedEvent>,
}

impl ConfigurationEventListener {
    /// Create a new event listener from an event handler
    /// 
    /// # Arguments
    /// 
    /// * `event_handler` - The event handler to subscribe to
    pub fn new(event_handler: &ConfigurationEventHandler) -> Self {
        Self {
            receiver: event_handler.subscribe(),
        }
    }

    /// Wait for the next configuration change event (blocking)
    /// 
    /// This method will asynchronously wait for the next configuration change
    /// event to arrive. It handles lag recovery automatically by recursively
    /// calling itself when events are missed.
    /// 
    /// # Returns
    /// 
    /// * `Ok(ConfigurationChangedEvent)` when an event is received
    /// * `Err(AppError)` if the event channel is closed
    /// 
    /// # Behavior
    /// 
    /// - Blocks until an event is available
    /// - Automatically recovers from lagged events
    /// - Logs received events and lag recovery attempts
    /// - Returns an error only if the channel is permanently closed
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut listener = ConfigurationEventListener::new(&event_handler);
    /// match listener.listen_for_change().await {
    ///     Ok(event) => println!("Config changed at {}", event.timestamp),
    ///     Err(e) => eprintln!("Event channel closed: {}", e),
    /// }
    /// ```
    pub async fn listen_for_change(&mut self) -> AppResult<ConfigurationChangedEvent> {
        match self.receiver.recv().await {
            Ok(event) => {
                info!("Received configuration change event at {}", event.timestamp);
                Ok(event)
            }
            Err(broadcast::error::RecvError::Closed) => {
                Err(crate::errors::AppError::event_emission_error(
                    "emit_configuration_change",
                    "Configuration event channel closed",
                ))
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                warn!(
                    "Configuration event listener lagged, skipped {} events",
                    skipped
                );
                Box::pin(self.listen_for_change()).await
            }
        }
    }

    /// Try to receive an event without blocking
    /// 
    /// This method checks for available events without blocking the current task.
    /// It's useful for polling-based event processing or when you want to check
    /// for events as part of a larger processing loop.
    /// 
    /// # Returns
    /// 
    /// * `Some(ConfigurationChangedEvent)` if an event is available
    /// * `None` if no event is currently available or the channel is closed
    /// 
    /// # Behavior
    /// 
    /// - Non-blocking operation
    /// - Automatically handles lag recovery by recursing
    /// - Returns `None` for both empty channel and closed channel
    /// - Logs lag recovery attempts
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut listener = ConfigurationEventListener::new(&event_handler);
    /// if let Some(event) = listener.try_receive() {
    ///     println!("Immediate event available: {:?}", event);
    /// } else {
    ///     println!("No events currently available");
    /// }
    /// ```
    pub fn try_receive(&mut self) -> Option<ConfigurationChangedEvent> {
        match self.receiver.try_recv() {
            Ok(event) => {
                debug!("Received configuration change event at {}", event.timestamp);
                Some(event)
            }
            Err(broadcast::error::TryRecvError::Empty) => None,
            Err(broadcast::error::TryRecvError::Closed) => {
                warn!("Configuration event channel closed");
                None
            }
            Err(broadcast::error::TryRecvError::Lagged(skipped)) => {
                warn!(
                    "Configuration event listener lagged, skipped {} events",
                    skipped
                );
                self.try_receive()
            }
        }
    }
}

/// High-level configuration event management and coordination
/// 
/// This struct provides a convenient high-level interface for managing
/// configuration events, combining the functionality of the event handler
/// with convenient listener creation and management methods.
/// 
/// # Design Philosophy
/// 
/// The event manager serves as a facade over the lower-level event handler,
/// providing a more convenient API for typical use cases while maintaining
/// access to the underlying broadcast channel functionality.
/// 
/// # Use Cases
/// 
/// - Application-wide event coordination
/// - Centralized listener management
/// - Simplified event system integration
/// - Testing and debugging event flows
pub struct ConfigurationEventManager {
    /// Shared reference to the underlying event handler
    event_handler: Arc<ConfigurationEventHandler>,
}

impl ConfigurationEventManager {
    /// Create a new event manager with default settings
    pub fn new() -> Self {
        Self {
            event_handler: Arc::new(ConfigurationEventHandler::new()),
        }
    }

    /// Create a new event manager with a specific channel capacity
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Maximum number of events to buffer in the channel
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            event_handler: Arc::new(ConfigurationEventHandler::with_capacity(capacity)),
        }
    }

    /// Get a shared reference to the underlying event handler
    /// 
    /// This allows direct access to the event handler for advanced use cases
    /// while maintaining the shared ownership model.
    pub fn get_event_handler(&self) -> Arc<ConfigurationEventHandler> {
        self.event_handler.clone()
    }

    /// Broadcast a configuration change event
    /// 
    /// Convenience method that delegates to the underlying event handler's
    /// broadcast functionality.
    /// 
    /// # Arguments
    /// 
    /// * `new_config` - The new configuration state
    /// * `previous_config` - The previous configuration state
    pub fn broadcast_config_change(
        &self,
        new_config: AppConfig,
        previous_config: AppConfig,
    ) -> AppResult<()> {
        self.event_handler
            .broadcast_config_change(new_config, previous_config)
    }

    /// Create a new event listener
    /// 
    /// This is a convenience method for creating new listeners without
    /// needing direct access to the event handler.
    /// 
    /// # Returns
    /// 
    /// A new `ConfigurationEventListener` subscribed to events from this manager
    pub fn create_listener(&self) -> ConfigurationEventListener {
        ConfigurationEventListener::new(&self.event_handler)
    }

    /// Get the current number of active event receivers
    /// 
    /// # Returns
    /// 
    /// The number of currently active event listeners
    pub fn receiver_count(&self) -> usize {
        self.event_handler.receiver_count()
    }
}

impl Default for ConfigurationEventManager {
    fn default() -> Self {
        Self::new()
    }
}
