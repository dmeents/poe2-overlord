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
use std::sync::Arc;
use tokio::sync::broadcast;

/// Character-aware session tracking constants
const EVENT_CHANNEL_SIZE: usize = 100;

/// Time tracking service implementation that handles business logic for time tracking
#[derive(Clone)]
pub struct TimeTrackingServiceImpl {
    /// Time tracking repository for data operations
    repository: Arc<dyn TimeTrackingRepository>,
    /// Event broadcaster for time tracking events
    event_sender: broadcast::Sender<TimeTrackingEvent>,
    /// POE process start time for calculations
    poe_process_start_time: Arc<tokio::sync::RwLock<Option<DateTime<Utc>>>>,
}

impl Default for TimeTrackingServiceImpl {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::error!("Failed to create TimeTrackingServiceImpl: {}", e);
            panic!("Failed to initialize time tracking service");
        })
    }
}

impl TimeTrackingServiceImpl {
    /// Create a new time tracking service
    pub fn new() -> AppResult<Self> {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);
        let repository = Arc::new(TimeTrackingRepositoryImpl::new()?);

        Ok(Self {
            repository,
            event_sender,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        })
    }

    /// Create a new time tracking service with custom repository
    pub fn with_repository(repository: Arc<dyn TimeTrackingRepository>) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);

        Self {
            repository,
            event_sender,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Get the event receiver for subscribing to time tracking events
    pub fn subscribe(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }

    /// Set the POE process start time
    pub async fn set_poe_process_start_time(&self, start_time: DateTime<Utc>) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = Some(start_time);
    }

    /// Get the POE process start time
    pub async fn get_poe_process_start_time(&self) -> Option<DateTime<Utc>> {
        let poe_start_time = self.poe_process_start_time.read().await;
        *poe_start_time
    }

    /// Generate a unique location ID from location name and type
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
    async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()> {
        let location_id = Self::generate_location_id(&location_name, &location_type);

        // Create new session
        let session = LocationSession {
            character_id: character_id.to_string(),
            location_id: location_id.clone(),
            location_name,
            location_type,
            entry_timestamp: Utc::now(),
            exit_timestamp: None,
            duration_seconds: Some(0),
        };

        // Start session via repository
        self.repository
            .start_session(character_id, session.clone())
            .await?;

        // Emit event
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

    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        // End session via repository
        self.repository
            .end_session(character_id, location_id)
            .await?;

        // Emit event
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

    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let sessions_map = self
            .repository
            .get_active_sessions(character_id)
            .await
            .unwrap_or_default();
        sessions_map.into_values().collect()
    }

    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        self.repository
            .get_completed_sessions(character_id)
            .await
            .unwrap_or_default()
    }

    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats> {
        let stats_map = self
            .repository
            .get_stats_cache(character_id)
            .await
            .unwrap_or_default();
        stats_map.into_values().collect()
    }

    async fn get_total_play_time(&self, character_id: &str) -> u64 {
        self.repository
            .calculate_total_play_time(character_id)
            .await
            .unwrap_or(0)
    }

    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64 {
        // For now, just return total play time
        // TODO: Implement proper calculation based on process start time
        self.get_total_play_time(character_id).await
    }

    async fn get_total_hideout_time(&self, character_id: &str) -> u64 {
        self.repository
            .calculate_total_hideout_time(character_id)
            .await
            .unwrap_or(0)
    }

    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession> {
        self.repository
            .get_last_known_location(character_id)
            .await
            .unwrap_or(None)
    }

    async fn load_character_data(&self, character_id: &str) -> AppResult<()> {
        let data = self.repository.load_character_data(character_id).await?;

        if data.is_some() {
            // Emit event
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

    async fn save_character_data(&self, character_id: &str) -> AppResult<()> {
        // Load current data and save it
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            self.repository.save_character_data(&data).await?;

            // Emit event
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

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.subscribe()
    }

    async fn set_poe_process_start_time(&self, start_time: DateTime<Utc>) {
        self.set_poe_process_start_time(start_time).await;
    }

    async fn clear_poe_process_start_time(&self) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = None;
    }

    async fn end_all_active_sessions_global(&self) -> AppResult<()> {
        // For now, this is a simplified implementation
        // TODO: Implement proper global session ending
        debug!("Ending all active sessions globally");
        Ok(())
    }

    async fn clear_character_data(&self, character_id: &str) -> AppResult<()> {
        self.repository.delete_character_data(character_id).await?;

        // Emit event
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

    async fn load_all_character_data(&self) -> AppResult<()> {
        // For now, this is a simplified implementation
        // TODO: Implement proper loading of all character data
        debug!("Loading all character data");
        Ok(())
    }

    async fn save_all_character_data(&self) -> AppResult<()> {
        // For now, this is a simplified implementation
        // TODO: Implement proper saving of all character data
        debug!("Saving all character data");
        Ok(())
    }
}

#[async_trait]
impl TimeTrackingEventPublisher for TimeTrackingServiceImpl {
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

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.subscribe()
    }
}
