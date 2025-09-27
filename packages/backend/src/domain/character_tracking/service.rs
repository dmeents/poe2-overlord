use crate::domain::character_tracking::models::{
    CharacterTrackingData, LocationState, LocationType, SceneTypeConfig, ZoneStats,
};
use crate::domain::character_tracking::traits::{
    CharacterTrackingEventPublisher, CharacterTrackingRepository, CharacterTrackingService,
    SceneTypeDetector,
};
use crate::domain::events::{AppEvent, EventBus, EventType};
use crate::domain::log_analysis::models::{
    ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::debug;
use std::sync::Arc;
use tauri::WebviewWindow;
use tokio::sync::broadcast;

/// Main implementation of the character tracking service
/// Combines location tracking and time tracking functionality
#[derive(Clone)]
pub struct CharacterTrackingServiceImpl {
    /// Repository for data persistence and retrieval
    repository: Arc<dyn CharacterTrackingRepository>,
    /// Event bus for publishing character tracking events
    event_bus: Arc<EventBus>,
    /// Scene type detection logic
    scene_type_detector: Arc<tokio::sync::RwLock<dyn SceneTypeDetector + Send + Sync>>,
    /// Tracks when the PoE process started for time calculations
    poe_process_start_time: Arc<tokio::sync::RwLock<Option<DateTime<Utc>>>>,
}

impl Default for CharacterTrackingServiceImpl {
    /// Creates a default service instance, panicking on failure
    fn default() -> Self {
        let event_bus = Arc::new(crate::domain::events::EventBus::new());
        Self::new(event_bus).unwrap_or_else(|e| {
            log::error!("Failed to create CharacterTrackingServiceImpl: {}", e);
            panic!("Failed to initialize character tracking service");
        })
    }
}

impl CharacterTrackingServiceImpl {
    /// Creates a new character tracking service with default repository
    pub fn new(event_bus: Arc<EventBus>) -> AppResult<Self> {
        let repository = Arc::new(
            crate::domain::character_tracking::repository::CharacterTrackingRepositoryImpl::new()?,
        );
        let scene_type_detector = Arc::new(tokio::sync::RwLock::new(
            crate::domain::character_tracking::service::SimpleSceneTypeDetector::new(
                SceneTypeConfig::default(),
            ),
        ));

        Ok(Self {
            repository,
            event_bus,
            scene_type_detector,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        })
    }

    /// Creates a new service with a custom repository (useful for testing)
    pub fn with_repository(
        repository: Arc<dyn CharacterTrackingRepository>,
        event_bus: Arc<EventBus>,
        scene_type_detector: Arc<tokio::sync::RwLock<dyn SceneTypeDetector + Send + Sync>>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            scene_type_detector,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Returns a receiver for subscribing to character tracking events
    pub async fn subscribe(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(EventType::System).await
    }

    /// Helper function to publish character tracking events
    async fn publish_character_tracking_event(
        &self,
        event: crate::domain::character_tracking::traits::CharacterTrackingEvent,
    ) -> AppResult<()> {
        // Convert CharacterTrackingEvent to AppEvent
        let app_event = match event {
            crate::domain::character_tracking::traits::CharacterTrackingEvent::ZoneEntered(event) => AppEvent::SystemError {
                error_message: format!(
                    "Zone entered: {} ({})",
                    event.zone.location_name, event.zone.location_type
                ),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            crate::domain::character_tracking::traits::CharacterTrackingEvent::ZoneLeft(event) => AppEvent::SystemError {
                error_message: format!(
                    "Zone left: {} ({})",
                    event.zone.location_name, event.zone.location_type
                ),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            crate::domain::character_tracking::traits::CharacterTrackingEvent::StatsUpdated(event) => AppEvent::SystemError {
                error_message: format!(
                    "Stats updated: {} ({})",
                    event.zone.location_name, event.zone.location_type
                ),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            crate::domain::character_tracking::traits::CharacterTrackingEvent::CharacterTrackingDataCleared(_) => AppEvent::SystemError {
                error_message: "Character tracking data cleared".to_string(),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            crate::domain::character_tracking::traits::CharacterTrackingEvent::CharacterTrackingDataLoaded(event) => AppEvent::SystemError {
                error_message: format!("Character tracking data loaded: {} zones", event.zones_count),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            crate::domain::character_tracking::traits::CharacterTrackingEvent::CharacterTrackingDataSaved(event) => AppEvent::SystemError {
                error_message: format!("Character tracking data saved: {} zones", event.zones_count),
                error_type: "CharacterTracking".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        };

        self.event_bus.publish(app_event).await
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

    /// Generates a unique location ID from name and type
    fn generate_location_id(location_name: &str, location_type: &LocationType) -> String {
        format!(
            "{}_{}",
            location_type,
            location_name.to_lowercase().replace(' ', "_")
        )
    }

    /// Converts a scene change to a zone event
    async fn convert_to_zone_event(
        &self,
        character_id: &str,
        location_type: LocationType,
        location_name: String,
        act: Option<String>,
    ) -> AppResult<()> {
        let location_id = Self::generate_location_id(&location_name, &location_type);

        // Enter the zone
        self.enter_zone(character_id, location_id, location_name, location_type, act)
            .await?;

        debug!(
            "Converted scene change to zone event for character {}",
            character_id
        );
        Ok(())
    }

    /// Creates a scene change event from content and detected scene type
    fn create_scene_change_event(
        &self,
        content: &str,
        location_type: LocationType,
    ) -> SceneChangeEvent {
        let timestamp = chrono::Utc::now().to_rfc3339();

        match location_type {
            LocationType::Hideout => SceneChangeEvent::Hideout(HideoutChangeEvent {
                hideout_name: content.to_string(),
                timestamp,
            }),
            LocationType::Act => SceneChangeEvent::Act(ActChangeEvent {
                act_name: content.to_string(),
                timestamp,
            }),
            LocationType::Zone => SceneChangeEvent::Zone(ZoneChangeEvent {
                zone_name: content.to_string(),
                timestamp,
            }),
        }
    }

    /// Validates if a scene change event represents an actual change
    /// Returns Some(event) if it's a real change, None if it's the same location
    async fn validate_scene_change_event(
        &self,
        event: &SceneChangeEvent,
        character_id: &str,
    ) -> AppResult<Option<SceneChangeEvent>> {
        match event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Validating hideout change: {}", hideout_event.hideout_name);
                let data = self.repository.load_character_data(character_id).await?;
                let result = data
                    .as_ref()
                    .and_then(|d| d.current_location.as_ref())
                    .and_then(|l| l.get_current_scene())
                    .map(|s| s != &hideout_event.hideout_name)
                    .unwrap_or(true); // If no state exists, it's a change
                debug!("Hideout change validation result: {}", result);
                if result {
                    Ok(Some(event.clone()))
                } else {
                    Ok(None)
                }
            }
            SceneChangeEvent::Zone(zone_event) => {
                debug!("Validating zone change: {}", zone_event.zone_name);
                let data = self.repository.load_character_data(character_id).await?;
                let result = data
                    .as_ref()
                    .and_then(|d| d.current_location.as_ref())
                    .and_then(|l| l.get_current_scene())
                    .map(|s| s != &zone_event.zone_name)
                    .unwrap_or(true); // If no state exists, it's a change
                debug!("Zone change validation result: {}", result);
                if result {
                    Ok(Some(event.clone()))
                } else {
                    Ok(None)
                }
            }
            SceneChangeEvent::Act(act_event) => {
                debug!("Processing act event: {}", act_event.act_name);
                debug!("Act event always returned for session continuity");
                Ok(Some(event.clone()))
            }
        }
    }
}

#[async_trait]
impl CharacterTrackingService for CharacterTrackingServiceImpl {
    async fn process_scene_content(
        &self,
        content: &str,
        character_id: &str,
    ) -> AppResult<Option<SceneChangeEvent>> {
        // Detect scene type from content
        let location_type = {
            let detector = self.scene_type_detector.read().await;
            detector.detect_scene_type(content)
        };

        let event = self.create_scene_change_event(content, location_type.clone());

        // Validate if this is an actual scene change
        let result = self
            .validate_scene_change_event(&event, character_id)
            .await?;

        match &result {
            Some(validated_event) => {
                debug!(
                    "Scene change validated as actual change: {:?}",
                    validated_event
                );

                // Update current location state for character
                let mut data = self
                    .repository
                    .load_character_data(character_id)
                    .await?
                    .unwrap_or_else(|| CharacterTrackingData::new(character_id.to_string()));

                let location_state = match validated_event {
                    SceneChangeEvent::Hideout(hideout_event) => LocationState::new_for_location(
                        Some(hideout_event.hideout_name.clone()),
                        None,
                        LocationType::Hideout,
                    ),
                    SceneChangeEvent::Zone(zone_event) => LocationState::new_for_location(
                        Some(zone_event.zone_name.clone()),
                        None,
                        LocationType::Zone,
                    ),
                    SceneChangeEvent::Act(act_event) => LocationState::new_for_location(
                        None,
                        Some(act_event.act_name.clone()),
                        LocationType::Act,
                    ),
                };

                data.current_location = Some(location_state);
                self.repository.save_character_data(&data).await?;

                // Convert to zone event
                let scene_name = match validated_event {
                    SceneChangeEvent::Hideout(hideout_event) => hideout_event.hideout_name.clone(),
                    SceneChangeEvent::Zone(zone_event) => zone_event.zone_name.clone(),
                    SceneChangeEvent::Act(act_event) => act_event.act_name.clone(),
                };

                let act = match validated_event {
                    SceneChangeEvent::Act(act_event) => Some(act_event.act_name.clone()),
                    _ => None,
                };

                self.convert_to_zone_event(character_id, location_type.clone(), scene_name, act)
                    .await?;
            }
            None => {
                debug!("Scene change content was not an actual change, skipping broadcast");
            }
        }

        Ok(result)
    }

    async fn get_current_location(&self, character_id: &str) -> AppResult<Option<LocationState>> {
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            Ok(data.current_location)
        } else {
            Ok(None)
        }
    }

    async fn reset_tracking(&self, character_id: &str) -> AppResult<()> {
        let data = CharacterTrackingData::new(character_id.to_string());
        self.repository.save_character_data(&data).await?;

        debug!(
            "Character tracking state reset for character {}",
            character_id
        );
        Ok(())
    }

    async fn get_current_scene(&self, character_id: &str) -> AppResult<Option<String>> {
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            Ok(data
                .current_location
                .and_then(|l| l.get_current_scene().cloned()))
        } else {
            Ok(None)
        }
    }

    async fn get_current_act(&self, character_id: &str) -> AppResult<Option<String>> {
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            Ok(data
                .current_location
                .and_then(|l| l.get_current_act().cloned()))
        } else {
            Ok(None)
        }
    }

    async fn get_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTrackingData>> {
        self.repository.load_character_data(character_id).await
    }

    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
    ) -> AppResult<()> {
        // Deactivate any currently active zone for this character
        if let Some(active_zone) = self.repository.get_active_zone(character_id).await? {
            let mut deactivated_zone = active_zone;
            deactivated_zone.deactivate();
            self.repository
                .upsert_zone(character_id, deactivated_zone)
                .await?;
        }

        // Create or update the new zone
        let mut zone = self
            .repository
            .find_zone(character_id, &location_id)
            .await?
            .unwrap_or_else(|| {
                ZoneStats::new(
                    location_id.clone(),
                    location_name.clone(),
                    location_type.clone(),
                    act,
                )
            });

        zone.activate();
        self.repository.upsert_zone(character_id, zone).await?;

        debug!(
            "Character {} entered zone {} ({})",
            character_id, location_name, location_type
        );
        Ok(())
    }

    async fn leave_zone(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        if let Some(mut zone) = self.repository.find_zone(character_id, location_id).await? {
            zone.deactivate();
            self.repository.upsert_zone(character_id, zone).await?;
        }

        debug!("Character {} left zone {}", character_id, location_id);
        Ok(())
    }

    async fn record_death(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        self.repository
            .record_death(character_id, location_id)
            .await?;

        debug!(
            "Recorded death for character {} in zone {}",
            character_id, location_id
        );
        Ok(())
    }

    async fn add_zone_time(
        &self,
        character_id: &str,
        location_id: &str,
        seconds: u64,
    ) -> AppResult<()> {
        self.repository
            .add_zone_time(character_id, location_id, seconds)
            .await?;
        Ok(())
    }

    async fn get_active_zone(&self, character_id: &str) -> AppResult<Option<ZoneStats>> {
        self.repository.get_active_zone(character_id).await
    }

    async fn get_all_zones(&self, character_id: &str) -> AppResult<Vec<ZoneStats>> {
        self.repository.get_all_zones(character_id).await
    }

    async fn get_zones_by_time(&self, character_id: &str) -> AppResult<Vec<ZoneStats>> {
        self.repository.get_zones_by_time(character_id).await
    }

    async fn get_total_play_time(&self, character_id: &str) -> AppResult<u64> {
        self.repository.get_total_play_time(character_id).await
    }

    async fn get_total_hideout_time(&self, character_id: &str) -> AppResult<u64> {
        self.repository.get_total_hideout_time(character_id).await
    }

    async fn get_total_deaths(&self, character_id: &str) -> AppResult<u32> {
        self.repository.get_total_deaths(character_id).await
    }

    async fn clear_character_data(&self, character_id: &str) -> AppResult<()> {
        // Delete from repository (both storage and memory)
        self.repository.delete_character_data(character_id).await?;

        // Publish data cleared event
        let event = crate::domain::character_tracking::traits::CharacterTrackingEvent::CharacterTrackingDataCleared(
            crate::domain::character_tracking::traits::CharacterTrackingDataCleared::new(character_id.to_string())
        );

        if let Err(e) = self.publish_character_tracking_event(event).await {
            log::warn!("Failed to publish data cleared event: {}", e);
        }

        debug!(
            "Cleared character tracking data for character {}",
            character_id
        );
        Ok(())
    }

    async fn update_scene_type_config(&self, config: SceneTypeConfig) -> AppResult<()> {
        let mut detector = self.scene_type_detector.write().await;
        detector.update_scene_type_config(config);
        Ok(())
    }

    async fn get_scene_type_config(&self) -> AppResult<SceneTypeConfig> {
        let detector = self.scene_type_detector.read().await;
        Ok(detector.get_scene_type_config().clone())
    }

    async fn subscribe_to_events(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.subscribe().await
    }

    async fn start_frontend_event_emission(&self, _window: WebviewWindow) {
        let mut event_receiver = self.subscribe().await.unwrap_or_else(|_| {
            // Create a dummy receiver if subscription fails
            let (_, receiver) = broadcast::channel(1);
            receiver
        });

        // Spawn background task to handle event emission
        tokio::spawn(async move {
            debug!("Character tracking frontend event emission started");

            // Listen for events and emit them to frontend
            while let Ok(app_event) = event_receiver.recv().await {
                // Convert AppEvent back to CharacterTrackingEvent for frontend emission
                if let AppEvent::SystemError {
                    error_message,
                    error_type,
                    ..
                } = app_event
                {
                    if error_type == "CharacterTracking" {
                        // For now, just log the character tracking events
                        debug!("Character tracking event: {}", error_message);
                    }
                }
            }

            debug!("Character tracking frontend event emission stopped");
        });
    }
}

#[async_trait]
impl CharacterTrackingEventPublisher for CharacterTrackingServiceImpl {
    /// Publishes a character tracking event to all subscribers
    async fn publish_event(
        &self,
        event: crate::domain::character_tracking::traits::CharacterTrackingEvent,
    ) -> AppResult<()> {
        self.publish_character_tracking_event(event).await
    }

    /// Returns a receiver for subscribing to character tracking events
    async fn subscribe_to_events(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.subscribe().await
    }
}

/// Simple implementation of scene type detection using keyword matching
/// Provides basic scene categorization based on configured keywords
pub struct SimpleSceneTypeDetector {
    config: SceneTypeConfig,
}

impl SimpleSceneTypeDetector {
    /// Creates a new scene type detector with the provided configuration
    pub fn new(config: SceneTypeConfig) -> Self {
        Self { config }
    }
}

impl SceneTypeDetector for SimpleSceneTypeDetector {
    /// Detects scene type by checking keywords in order of specificity
    /// Hideout -> Act -> Zone (default)
    fn detect_scene_type(&self, content: &str) -> LocationType {
        let lower_content = content.to_lowercase();

        if self.is_hideout_content(&lower_content) {
            return LocationType::Hideout;
        }

        if self.is_act_content(&lower_content) {
            return LocationType::Act;
        }

        LocationType::Zone
    }

    /// Checks if content contains hideout keywords
    fn is_hideout_content(&self, lower_content: &str) -> bool {
        self.config
            .hideout_keywords
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    /// Checks if content contains act keywords
    fn is_act_content(&self, lower_content: &str) -> bool {
        self.config
            .act_keywords
            .iter()
            .any(|keyword| lower_content.contains(keyword))
    }

    /// All content is considered zone content by default
    fn is_zone_content(&self, _lower_content: &str) -> bool {
        true
    }

    /// Gets the current scene type configuration
    fn get_scene_type_config(&self) -> &SceneTypeConfig {
        &self.config
    }

    /// Updates the scene type configuration
    fn update_scene_type_config(&mut self, config: SceneTypeConfig) {
        self.config = config;
    }
}
