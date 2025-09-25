use crate::domain::time_tracking::{
    events::{
        SessionEnded, SessionStarted, TimeTrackingDataCleared, TimeTrackingDataLoaded,
        TimeTrackingDataSaved, TimeTrackingEvent,
    },
    models::{LocationSession, LocationStats, LocationType},
    repository::TimeTrackingRepositoryImpl,
    traits::{TimeTrackingEventPublisher, TimeTrackingRepository, TimeTrackingService},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::{debug, warn};
use serde_json;
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::broadcast;

/// Size of the event broadcast channel
const EVENT_CHANNEL_SIZE: usize = 100;

/// Main implementation of the time tracking service
/// Handles business logic, event publishing, and frontend communication
#[derive(Clone)]
pub struct TimeTrackingServiceImpl {
    /// Repository for data persistence and retrieval
    repository: Arc<dyn TimeTrackingRepository>,
    /// Event broadcaster for publishing time tracking events
    event_sender: broadcast::Sender<TimeTrackingEvent>,
    /// Tracks when the PoE process started for time calculations
    poe_process_start_time: Arc<tokio::sync::RwLock<Option<DateTime<Utc>>>>,
}

impl Default for TimeTrackingServiceImpl {
    /// Creates a default service instance, panicking on failure
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::error!("Failed to create TimeTrackingServiceImpl: {}", e);
            panic!("Failed to initialize time tracking service");
        })
    }
}

impl TimeTrackingServiceImpl {
    /// Creates a new time tracking service with default repository
    pub fn new() -> AppResult<Self> {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);
        let repository = Arc::new(TimeTrackingRepositoryImpl::new()?);

        Ok(Self {
            repository,
            event_sender,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        })
    }

    /// Creates a new service with a custom repository (useful for testing)
    pub fn with_repository(repository: Arc<dyn TimeTrackingRepository>) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);

        Self {
            repository,
            event_sender,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Returns a receiver for subscribing to time tracking events
    pub fn subscribe(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }

    /// Sets the PoE process start time for time calculations
    pub async fn set_poe_process_start_time(&self, start_time: DateTime<Utc>) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = Some(start_time);
    }

    /// Gets the PoE process start time
    pub async fn get_poe_process_start_time(&self) -> Option<DateTime<Utc>> {
        let poe_start_time = self.poe_process_start_time.read().await;
        *poe_start_time
    }


    /// Emits time tracking events to the frontend via Tauri
    fn emit_time_tracking_event(window: &WebviewWindow, event: &TimeTrackingEvent) {
        match event {
            TimeTrackingEvent::SessionStarted(session_event) => {
                Self::emit_json_event(
                    window,
                    "time-tracking-session-started",
                    serde_json::json!({
                        "location_id": session_event.session.location_id,
                        "location_name": session_event.session.location_name,
                        "location_type": session_event.session.location_type,
                        "entry_timestamp": session_event.session.entry_timestamp
                    }),
                );
            }
            TimeTrackingEvent::SessionEnded(session_event) => {
                Self::emit_json_event(
                    window,
                    "time-tracking-session-ended",
                    serde_json::json!({
                        "location_id": session_event.session.location_id,
                        "location_name": session_event.session.location_name,
                        "location_type": session_event.session.location_type,
                        "duration_seconds": session_event.session.duration_seconds,
                        "entry_timestamp": session_event.session.entry_timestamp,
                        "exit_timestamp": session_event.session.exit_timestamp
                    }),
                );
            }
            TimeTrackingEvent::StatsUpdated(stats_event) => {
                Self::emit_json_event(
                    window,
                    "time-tracking-stats-updated",
                    serde_json::json!({
                        "location_id": stats_event.stats.location_id,
                        "location_name": stats_event.stats.location_name,
                        "location_type": stats_event.stats.location_type,
                        "total_visits": stats_event.stats.total_visits,
                        "total_time_seconds": stats_event.stats.total_time_seconds,
                        "average_session_seconds": stats_event.stats.average_session_seconds,
                        "last_visited": stats_event.stats.last_visited
                    }),
                );
            }
            _ => {
                debug!("Unhandled time tracking event type: {:?}", event);
            }
        }
    }

    /// Helper function to emit JSON events to the frontend
    fn emit_json_event(window: &WebviewWindow, event_name: &str, payload: serde_json::Value) {
        if let Err(e) = window.emit(event_name, &payload) {
            warn!("Failed to emit JSON event '{}': {}", event_name, e);
        }
    }

    /// Generates a unique location ID from name and type
    fn generate_location_id(location_name: &str, location_type: &LocationType) -> String {
        format!(
            "{}_{}",
            location_type,
            location_name.to_lowercase().replace(' ', "_")
        )
    }
}

#[async_trait]
impl TimeTrackingService for TimeTrackingServiceImpl {
    /// Starts a new location session for a character
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()> {
        let location_id = Self::generate_location_id(&location_name, &location_type);

        // Create new session with current timestamp
        let session = LocationSession {
            character_id: character_id.to_string(),
            location_id: location_id.clone(),
            location_name,
            location_type,
            entry_timestamp: Utc::now(),
            exit_timestamp: None,
            duration_seconds: Some(0),
        };

        // Save to repository (includes validation)
        self.repository
            .start_session(character_id, session.clone())
            .await?;

        // Publish session started event
        let event = TimeTrackingEvent::SessionStarted(SessionStarted {
            session: session.clone(),
            occurred_at: std::time::SystemTime::now(),
        });

        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to send session started event: {}", e);
        }

        debug!(
            "Started session for character {} at location {}",
            character_id, session.location_name
        );
        Ok(())
    }

    /// Ends an active location session for a character
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        // End session in repository (moves from active to completed)
        self.repository
            .end_session(character_id, location_id)
            .await?;

        // Publish session ended event (with minimal session data)
        let event = TimeTrackingEvent::SessionEnded(SessionEnded {
            session: LocationSession {
                character_id: character_id.to_string(),
                location_id: location_id.to_string(),
                location_name: String::new(),
                location_type: LocationType::Zone,
                entry_timestamp: Utc::now(),
                exit_timestamp: Some(Utc::now()),
                duration_seconds: Some(0),
            },
            occurred_at: std::time::SystemTime::now(),
        });

        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to send session ended event: {}", e);
        }

        debug!(
            "Ended session for character {} at location {}",
            character_id, location_id
        );
        Ok(())
    }

    /// Gets all currently active sessions for a character
    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let sessions_map = self
            .repository
            .get_active_sessions(character_id)
            .await
            .unwrap_or_default();
        sessions_map.into_values().collect()
    }

    /// Gets all completed sessions for a character
    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        self.repository
            .get_completed_sessions(character_id)
            .await
            .unwrap_or_default()
    }

    /// Gets all location statistics for a character
    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats> {
        let stats_map = self
            .repository
            .get_stats_cache(character_id)
            .await
            .unwrap_or_default();
        stats_map.into_values().collect()
    }

    /// Gets total play time for a character across all completed sessions
    async fn get_total_play_time(&self, character_id: &str) -> u64 {
        self.repository
            .calculate_total_play_time(character_id)
            .await
            .unwrap_or(0)
    }

    /// Gets total play time since process start (currently same as total play time)
    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64 {
        self.get_total_play_time(character_id).await
    }

    /// Gets total time spent in hideouts for a character
    async fn get_total_hideout_time(&self, character_id: &str) -> u64 {
        self.repository
            .calculate_total_hideout_time(character_id)
            .await
            .unwrap_or(0)
    }

    /// Gets the last known location for a character
    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession> {
        self.repository
            .get_last_known_location(character_id)
            .await
            .unwrap_or(None)
    }

    /// Loads time tracking data for a specific character
    async fn load_character_data(&self, character_id: &str) -> AppResult<()> {
        let data = self.repository.load_character_data(character_id).await?;

        // Publish data loaded event if data was found
        if data.is_some() {
            let event = TimeTrackingEvent::TimeTrackingDataLoaded(TimeTrackingDataLoaded {
                character_id: character_id.to_string(),
                completed_sessions_count: 0, // TODO: Calculate actual count
                location_stats_count: 0,     // TODO: Calculate actual count
                occurred_at: std::time::SystemTime::now(),
            });

            if let Err(e) = self.event_sender.send(event) {
                warn!("Failed to send data loaded event: {}", e);
            }
        }

        Ok(())
    }

    /// Saves time tracking data for a specific character
    async fn save_character_data(&self, character_id: &str) -> AppResult<()> {
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            // Save to persistent storage
            self.repository.save_character_data(&data).await?;

            // Publish data saved event
            let event = TimeTrackingEvent::TimeTrackingDataSaved(TimeTrackingDataSaved {
                character_id: character_id.to_string(),
                completed_sessions_count: data.completed_sessions.len(),
                location_stats_count: data.stats.len(),
                occurred_at: std::time::SystemTime::now(),
            });

            if let Err(e) = self.event_sender.send(event) {
                warn!("Failed to send data saved event: {}", e);
            }
        }

        debug!("Saved time tracking data for character {}", character_id);
        Ok(())
    }

    /// Returns a receiver for subscribing to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.subscribe()
    }

    /// Sets the PoE process start time for time calculations
    async fn set_poe_process_start_time(&self, start_time: DateTime<Utc>) {
        self.set_poe_process_start_time(start_time).await;
    }

    /// Clears the PoE process start time
    async fn clear_poe_process_start_time(&self) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = None;
    }

    /// Ends all active sessions globally (used when game exits)
    async fn end_all_active_sessions_global(&self) -> AppResult<()> {
        debug!("Ending all active sessions globally");
        Ok(())
    }

    /// Clears all time tracking data for a character
    async fn clear_character_data(&self, character_id: &str) -> AppResult<()> {
        // Delete from repository (both storage and memory)
        self.repository.delete_character_data(character_id).await?;

        // Publish data cleared event
        let event = TimeTrackingEvent::TimeTrackingDataCleared(TimeTrackingDataCleared {
            character_id: character_id.to_string(),
            occurred_at: std::time::SystemTime::now(),
        });

        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to send data cleared event: {}", e);
        }

        debug!("Cleared time tracking data for character {}", character_id);
        Ok(())
    }

    /// Loads time tracking data for all characters (placeholder implementation)
    async fn load_all_character_data(&self) -> AppResult<()> {
        debug!("Loading all character data");
        Ok(())
    }

    /// Saves time tracking data for all characters (placeholder implementation)
    async fn save_all_character_data(&self) -> AppResult<()> {
        debug!("Saving all character data");
        Ok(())
    }

    /// Starts emitting time tracking events to the frontend
    async fn start_frontend_event_emission(&self, window: WebviewWindow) {
        let mut event_receiver = self.subscribe();
        let window_clone = window.clone();
        
        // Spawn background task to handle event emission
        tokio::spawn(async move {
            debug!("Time tracking frontend event emission started");
            
            // Listen for events and emit them to frontend
            while let Ok(event) = event_receiver.recv().await {
                Self::emit_time_tracking_event(&window_clone, &event);
            }
            
            debug!("Time tracking frontend event emission stopped");
        });
    }
}

#[async_trait]
impl TimeTrackingEventPublisher for TimeTrackingServiceImpl {
    /// Publishes a time tracking event to all subscribers
    async fn publish_event(&self, event: TimeTrackingEvent) -> AppResult<()> {
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to publish event: {}", e);
            return Err(crate::errors::AppError::internal_error(
                "publish_event",
                &e.to_string(),
            ));
        }
        Ok(())
    }

    /// Returns a receiver for subscribing to time tracking events
    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.subscribe()
    }
}
