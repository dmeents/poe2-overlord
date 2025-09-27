use crate::domain::character_tracking::models::{
    CharacterTrackingData, LocationState, LocationType, ZoneStats,
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
    /// Zone configuration service for act and town detection
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    /// Tracks when the PoE process started for time calculations
    poe_process_start_time: Arc<tokio::sync::RwLock<Option<DateTime<Utc>>>>,
}

impl Default for CharacterTrackingServiceImpl {
    /// Creates a default service instance, panicking on failure
    fn default() -> Self {
        let event_bus = Arc::new(crate::domain::events::EventBus::new());
        // Create a dummy zone config for default initialization
        let zone_config = Arc::new(crate::domain::zone_configuration::service::ZoneConfigurationServiceImpl::new(
            Arc::new(crate::domain::zone_configuration::repository::ZoneConfigurationRepositoryImpl::new(
                std::path::PathBuf::from("config/zones.json")
            ))
        ));
        Self::new(event_bus, zone_config).unwrap_or_else(|e| {
            log::error!("Failed to create CharacterTrackingServiceImpl: {}", e);
            panic!("Failed to initialize character tracking service");
        })
    }
}

impl CharacterTrackingServiceImpl {
    /// Creates a new character tracking service with zone configuration
    pub fn new(
        event_bus: Arc<EventBus>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> AppResult<Self> {
        let repository = Arc::new(
            crate::domain::character_tracking::repository::CharacterTrackingRepositoryImpl::new()?,
        );
        let scene_type_detector = Arc::new(tokio::sync::RwLock::new(
            ZoneBasedSceneTypeDetector::new(zone_config.clone()),
        ));

        Ok(Self {
            repository,
            event_bus,
            scene_type_detector,
            zone_config,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        })
    }

    /// Creates a new service with a custom repository (useful for testing)
    pub fn with_repository(
        repository: Arc<dyn CharacterTrackingRepository>,
        event_bus: Arc<EventBus>,
        scene_type_detector: Arc<tokio::sync::RwLock<dyn SceneTypeDetector + Send + Sync>>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Self {
        Self {
            repository,
            event_bus,
            scene_type_detector,
            zone_config,
            poe_process_start_time: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Returns a receiver for subscribing to character tracking events
    pub async fn subscribe(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(EventType::System).await
    }

    /// Helper function to emit character tracking data updated event
    async fn emit_character_tracking_data_updated(&self, character_id: &str) -> AppResult<()> {
        if let Some(data) = self.repository.load_character_data(character_id).await? {
            let app_event =
                AppEvent::character_tracking_data_updated(character_id.to_string(), data);

            if let Err(e) = self.event_bus.publish(app_event).await {
                log::error!(
                    "Failed to publish character tracking data updated event: {}",
                    e
                );
            }
        }
        Ok(())
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
    /// Format: {location_type}_{location_name} in snake_case
    /// Only supports Zone and Hideout types (acts are filtered out)
    fn generate_location_id(location_name: &str, location_type: &LocationType) -> String {
        let type_prefix = match location_type {
            LocationType::Zone => "zone",
            LocationType::Hideout => "hideout",
            LocationType::Act => "act", // Should not be used since acts are filtered out
        };

        let snake_case_name = location_name
            .to_lowercase()
            .replace(' ', "_")
            .replace('-', "_")
            .replace('\'', "")
            .replace('.', "")
            .replace(',', "");

        format!("{}_{}", type_prefix, snake_case_name)
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

        // Get town status from zone configuration
        let is_town = self.zone_config.is_town_zone(&location_name).await;

        // Enter the zone
        self.enter_zone(
            character_id,
            location_id,
            location_name,
            location_type,
            act,
            is_town,
        )
        .await?;

        debug!(
            "Converted scene change to zone event for character {}",
            character_id
        );
        Ok(())
    }

    /// Creates a scene change event from content and detected scene type
    /// Uses zone configuration to determine act and town status
    async fn create_scene_change_event(
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
            LocationType::Zone => {
                // For zones, act information is determined during location state creation
                SceneChangeEvent::Zone(ZoneChangeEvent {
                    zone_name: content.to_string(),
                    timestamp,
                })
            }
            LocationType::Act => {
                // This should rarely happen with zone-based detection
                SceneChangeEvent::Act(ActChangeEvent {
                    act_name: content.to_string(),
                    timestamp,
                })
            }
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

        // Filter out act-related content - these should not be processed as scene changes
        if matches!(location_type, LocationType::Act) {
            debug!("Filtering out act-related scene change: {}", content);
            return Ok(None);
        }

        let event = self
            .create_scene_change_event(content, location_type.clone())
            .await;

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
                        false, // Hideouts are not towns
                        LocationType::Hideout,
                    ),
                    SceneChangeEvent::Zone(zone_event) => {
                        // Get act and town status from zone configuration
                        let act = self
                            .zone_config
                            .get_act_for_zone(&zone_event.zone_name)
                            .await;
                        let is_town = self.zone_config.is_town_zone(&zone_event.zone_name).await;
                        LocationState::new_for_location(
                            Some(zone_event.zone_name.clone()),
                            act,
                            is_town,
                            LocationType::Zone,
                        )
                    }
                    SceneChangeEvent::Act(act_event) => LocationState::new_for_location(
                        None,
                        Some(act_event.act_name.clone()),
                        false, // Acts are not towns
                        LocationType::Act,
                    ),
                };

                data.current_location = Some(location_state);
                self.repository.save_character_data(&data).await?;

                // Emit character tracking data updated event
                self.emit_character_tracking_data_updated(character_id)
                    .await?;

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

    async fn process_scene_content_with_zone_level(
        &self,
        content: &str,
        character_id: &str,
        zone_level: u32,
    ) -> AppResult<Option<SceneChangeEvent>> {
        // Detect scene type from content
        let location_type = {
            let detector = self.scene_type_detector.read().await;
            detector.detect_scene_type(content)
        };

        // Filter out act-related content - these should not be processed as scene changes
        if matches!(location_type, LocationType::Act) {
            debug!("Filtering out act-related scene change: {}", content);
            return Ok(None);
        }

        let event = self
            .create_scene_change_event(content, location_type.clone())
            .await;

        // Validate if this is an actual scene change
        let result = self
            .validate_scene_change_event(&event, character_id)
            .await?;

        match &result {
            Some(validated_event) => {
                debug!(
                    "Scene change with zone level {} validated as actual change: {:?}",
                    zone_level, validated_event
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
                        false, // Hideouts are not towns
                        LocationType::Hideout,
                    ),
                    SceneChangeEvent::Zone(zone_event) => {
                        // Get act and town status from zone configuration
                        debug!(
                            "Looking up zone '{}' in configuration",
                            zone_event.zone_name
                        );
                        let act = self
                            .zone_config
                            .get_act_for_zone(&zone_event.zone_name)
                            .await;
                        let is_town = self.zone_config.is_town_zone(&zone_event.zone_name).await;
                        debug!(
                            "Zone '{}' -> Act: {:?}, Is Town: {}",
                            zone_event.zone_name, act, is_town
                        );

                        LocationState::new_for_location(
                            Some(zone_event.zone_name.clone()),
                            act,
                            is_town,
                            LocationType::Zone,
                        )
                    }
                    SceneChangeEvent::Act(act_event) => LocationState::new_for_location(
                        None,
                        Some(act_event.act_name.clone()),
                        false, // Acts are not towns
                        LocationType::Act,
                    ),
                };

                data.current_location = Some(location_state);
                data.touch();
                self.repository.save_character_data(&data).await?;

                // Emit character tracking data updated event
                self.emit_character_tracking_data_updated(character_id)
                    .await?;

                // Process the scene change through the tracking system
                match validated_event {
                    SceneChangeEvent::Hideout(hideout_event) => {
                        self.enter_zone(
                            character_id,
                            Self::generate_location_id(
                                &hideout_event.hideout_name,
                                &LocationType::Hideout,
                            ),
                            hideout_event.hideout_name.clone(),
                            LocationType::Hideout,
                            None,
                            false,
                        )
                        .await?;
                    }
                    SceneChangeEvent::Zone(zone_event) => {
                        let act = self
                            .zone_config
                            .get_act_for_zone(&zone_event.zone_name)
                            .await;
                        let is_town = self.zone_config.is_town_zone(&zone_event.zone_name).await;

                        self.enter_zone(
                            character_id,
                            Self::generate_location_id(&zone_event.zone_name, &LocationType::Zone),
                            zone_event.zone_name.clone(),
                            LocationType::Zone,
                            act,
                            is_town,
                        )
                        .await?;

                        // Update the zone with the level information
                        if let Some(mut data) =
                            self.repository.load_character_data(character_id).await?
                        {
                            let location_id = Self::generate_location_id(
                                &zone_event.zone_name,
                                &LocationType::Zone,
                            );
                            if let Some(active_zone) = data.find_zone_mut(&location_id) {
                                active_zone.update_zone_level(zone_level);
                                debug!(
                                    "Updated zone {} with level {}",
                                    zone_event.zone_name, zone_level
                                );
                            }
                            self.repository.save_character_data(&data).await?;
                        }
                    }
                    SceneChangeEvent::Act(act_event) => {
                        debug!("Act change event: {}", act_event.act_name);
                        // Act changes are handled differently - they don't enter zones
                    }
                }

                // Publish the event
                let scene_type = match validated_event {
                    SceneChangeEvent::Zone(_) => LocationType::Zone,
                    SceneChangeEvent::Act(_) => LocationType::Act,
                    SceneChangeEvent::Hideout(_) => LocationType::Hideout,
                };

                if let Err(e) = self
                    .event_bus
                    .publish(AppEvent::SceneChangeDetected {
                        scene_type,
                        scene_name: validated_event.get_name().to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    })
                    .await
                {
                    log::error!("Failed to publish scene change event: {}", e);
                }
            }
            None => {
                debug!("Scene change filtered out as no actual change");
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

    async fn get_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTrackingData>> {
        self.repository.load_character_data(character_id).await
    }

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()> {
        self.repository.delete_character_data(character_id).await
    }

    async fn enter_zone(
        &self,
        character_id: &str,
        location_id: String,
        location_name: String,
        location_type: LocationType,
        act: Option<String>,
        is_town: bool,
    ) -> AppResult<()> {
        // Deactivate any currently active zone for this character and calculate time spent
        if let Some(mut data) = self.repository.load_character_data(character_id).await? {
            if let Some(active_zone) = data.get_active_zone() {
                let mut deactivated_zone = active_zone.clone();
                let time_spent = deactivated_zone.stop_timer_and_add_time();
                deactivated_zone.deactivate();
                data.upsert_zone(deactivated_zone);
                self.repository.save_character_data(&data).await?;

                // Emit character tracking data updated event
                self.emit_character_tracking_data_updated(character_id)
                    .await?;

                debug!(
                    "Character {} spent {} seconds in previous zone",
                    character_id, time_spent
                );
            }
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
                    is_town,
                )
            });

        zone.activate();
        zone.start_timer(); // Start timer for the new zone
        self.repository.upsert_zone(character_id, zone).await?;

        // Emit character tracking data updated event
        self.emit_character_tracking_data_updated(character_id)
            .await?;

        debug!(
            "Character {} entered zone {} ({})",
            character_id, location_name, location_type
        );
        Ok(())
    }

    async fn leave_zone(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        if let Some(mut zone) = self.repository.find_zone(character_id, location_id).await? {
            let time_spent = zone.stop_timer_and_add_time();
            zone.deactivate();
            self.repository.upsert_zone(character_id, zone).await?;

            debug!(
                "Character {} left zone {} after spending {} seconds",
                character_id, location_id, time_spent
            );
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

    async fn finalize_all_active_zones(&self) -> AppResult<()> {
        // Get all character IDs that have tracking data
        let character_ids = self.repository.get_all_character_ids().await?;

        for character_id in character_ids {
            if let Some(mut data) = self.repository.load_character_data(&character_id).await? {
                let mut has_changes = false;

                // Find and finalize any active zones
                for zone in &mut data.zones {
                    if zone.is_active {
                        let time_spent = zone.stop_timer_and_add_time();
                        zone.deactivate();
                        has_changes = true;

                        debug!(
                            "Finalized zone {} for character {} with {} seconds",
                            zone.location_name, character_id, time_spent
                        );
                    }
                }

                // Update summary if there were changes
                if has_changes {
                    data.update_summary();
                    self.repository.save_character_data(&data).await?;
                }
            }
        }

        Ok(())
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

/// Zone-based scene type detector that uses zone configuration for act detection
/// Replaces keyword-based detection with reliable zone-to-act mapping
pub struct ZoneBasedSceneTypeDetector {
    #[allow(dead_code)] // Used in async methods, linter doesn't detect this
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
}

impl ZoneBasedSceneTypeDetector {
    /// Creates a new zone-based scene type detector
    pub fn new(
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) -> Self {
        Self { zone_config }
    }
}

impl SceneTypeDetector for ZoneBasedSceneTypeDetector {
    /// Detects scene type by checking for hideouts first, then filtering out act content
    /// Act detection is now handled by zone configuration lookup
    fn detect_scene_type(&self, content: &str) -> LocationType {
        let lower_content = content.to_lowercase();

        if self.is_hideout_content(&lower_content) {
            return LocationType::Hideout;
        }

        // Filter out act-related content - these should not be processed as scene changes
        if self.is_act_content(&lower_content) {
            return LocationType::Act; // This will be filtered out later
        }

        // All other content is treated as zones
        // Act information is determined by zone configuration lookup
        LocationType::Zone
    }

    /// Checks if content contains hideout keywords (preserves existing logic)
    fn is_hideout_content(&self, lower_content: &str) -> bool {
        lower_content.contains("hideout")
    }

    /// Checks if content contains act keywords that should be filtered out
    fn is_act_content(&self, lower_content: &str) -> bool {
        lower_content.contains("act")
            || lower_content.contains("endgame")
            || lower_content.contains("interlude")
    }

    /// All content is considered zone content by default
    fn is_zone_content(&self, _lower_content: &str) -> bool {
        true
    }
}
