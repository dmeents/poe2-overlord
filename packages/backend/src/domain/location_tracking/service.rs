use crate::domain::location_tracking::models::{
    LocationHistoryEntry, LocationState, LocationTrackingConfig, LocationTrackingSession,
    LocationTrackingStats, SceneTypeConfig,
};
use crate::domain::location_tracking::traits::{
    LocationHistoryRepository, LocationStateRepository, LocationTrackingService,
    LocationTrackingSessionRepository, LocationTrackingStatsRepository, SceneTypeDetector,
};
use crate::errors::AppResult;
use crate::domain::log_analysis::models::{ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent};
use crate::domain::location_tracking::models::SceneType;
use async_trait::async_trait;
use log::{debug, info};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main implementation of the location tracking service
/// Orchestrates scene change detection, state management, and data persistence
pub struct LocationTrackingServiceImpl {
    /// Configuration for tracking behavior
    config: Arc<RwLock<LocationTrackingConfig>>,
    /// Repository for persisting location state
    state_repository: Arc<dyn LocationStateRepository>,
    /// Repository for managing tracking sessions
    session_repository: Arc<dyn LocationTrackingSessionRepository>,
    /// Repository for storing aggregated statistics
    stats_repository: Arc<dyn LocationTrackingStatsRepository>,
    /// Repository for storing location history
    history_repository: Arc<dyn LocationHistoryRepository>,
    /// Scene type detection logic
    scene_type_detector: Arc<RwLock<dyn SceneTypeDetector + Send + Sync>>,
    /// Current location state
    current_state: Arc<RwLock<LocationState>>,
    /// Current active session
    current_session: Arc<RwLock<Option<LocationTrackingSession>>>,
}

impl LocationTrackingServiceImpl {
    /// Creates a new location tracking service with the provided dependencies
    pub fn new(
        config: LocationTrackingConfig,
        state_repository: Arc<dyn LocationStateRepository>,
        session_repository: Arc<dyn LocationTrackingSessionRepository>,
        stats_repository: Arc<dyn LocationTrackingStatsRepository>,
        history_repository: Arc<dyn LocationHistoryRepository>,
        scene_type_detector: Arc<RwLock<dyn SceneTypeDetector + Send + Sync>>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let current_state = Arc::new(RwLock::new(LocationState::new()));

        Self {
            config,
            state_repository,
            session_repository,
            stats_repository,
            history_repository,
            scene_type_detector,
            current_state,
            current_session: Arc::new(RwLock::new(None)),
        }
    }

    /// Starts a new tracking session and saves it to storage
    async fn start_tracking_session(&self) -> AppResult<()> {
        let session = LocationTrackingSession::new();
        self.session_repository.save_session(&session).await?;
        
        let mut current_session = self.current_session.write().await;
        *current_session = Some(session);
        
        info!("Started new location tracking session");
        Ok(())
    }

    /// Ends the current tracking session and updates statistics
    async fn end_tracking_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;
            
            // Update aggregated statistics
            let mut stats = self.stats_repository.load_stats().await?;
            stats.total_sessions += 1;
            stats.total_scene_changes += session.total_scene_changes;
            stats.total_act_changes += session.total_act_changes;
            stats.total_zone_changes += session.total_zone_changes;
            stats.total_hideout_changes += session.total_hideout_changes;
            
            // Update average session duration
            if let Some(duration) = session.get_session_duration() {
                let duration_seconds = duration.num_seconds() as f64;
                if stats.total_sessions == 1 {
                    stats.average_session_duration_seconds = duration_seconds;
                } else {
                    stats.average_session_duration_seconds = 
                        (stats.average_session_duration_seconds * (stats.total_sessions - 1) as f64 + duration_seconds) 
                        / stats.total_sessions as f64;
                }
            }
            
            self.stats_repository.update_stats(&stats).await?;
            
            let mut current_session = self.current_session.write().await;
            *current_session = None;
            
            info!("Ended location tracking session");
        }
        Ok(())
    }

    /// Creates a scene change event from content and detected scene type
    fn create_scene_change_event(&self, content: &str, scene_type: SceneType) -> SceneChangeEvent {
        let timestamp = chrono::Utc::now().to_rfc3339();

        match scene_type {
            SceneType::Hideout => SceneChangeEvent::Hideout(HideoutChangeEvent {
                hideout_name: content.to_string(),
                timestamp,
            }),
            SceneType::Act => SceneChangeEvent::Act(ActChangeEvent {
                act_name: content.to_string(),
                timestamp,
            }),
            SceneType::Zone => SceneChangeEvent::Zone(ZoneChangeEvent {
                zone_name: content.to_string(),
                timestamp,
            }),
        }
    }
}

#[async_trait]
impl LocationTrackingService for LocationTrackingServiceImpl {
    async fn process_scene_content(&self, content: &str) -> AppResult<Option<SceneChangeEvent>> {
        // Detect scene type from content
        let scene_type = {
            let detector = self.scene_type_detector.read().await;
            detector.detect_scene_type(content)
        };

        let event = self.create_scene_change_event(content, scene_type.clone());

        // Validate if this is an actual scene change
        let result = self.validate_scene_change_event(event).await?;

        match &result {
            Some(validated_event) => {
                debug!("Scene change validated as actual change: {:?}", validated_event);
                
                // Update current location state
                let mut state = self.current_state.write().await;
                match validated_event {
                    SceneChangeEvent::Hideout(hideout_event) => {
                        state.update_scene(hideout_event.hideout_name.clone());
                    }
                    SceneChangeEvent::Zone(zone_event) => {
                        state.update_scene(zone_event.zone_name.clone());
                    }
                    SceneChangeEvent::Act(act_event) => {
                        state.update_act(act_event.act_name.clone());
                    }
                }
                
                self.state_repository.save_state(&state).await?;
                
                // Update current session if active
                if let Some(mut session) = self.current_session.read().await.clone() {
                    let scene_name = match validated_event {
                        SceneChangeEvent::Hideout(hideout_event) => hideout_event.hideout_name.clone(),
                        SceneChangeEvent::Zone(zone_event) => zone_event.zone_name.clone(),
                        SceneChangeEvent::Act(act_event) => act_event.act_name.clone(),
                    };
                    
                    session.record_scene_change(scene_type.clone(), scene_name);
                    self.session_repository.update_session(&session).await?;
                    
                    let mut current_session = self.current_session.write().await;
                    *current_session = Some(session);
                }
                
                // Update statistics
                self.stats_repository.increment_scene_change_count(scene_type.clone()).await?;
                
                // Add to history if enabled
                let config = self.config.read().await;
                if config.enable_history_tracking {
                    let history_entry = LocationHistoryEntry {
                        scene_type: scene_type.clone(),
                        scene_name: match validated_event {
                            SceneChangeEvent::Hideout(hideout_event) => hideout_event.hideout_name.clone(),
                            SceneChangeEvent::Zone(zone_event) => zone_event.zone_name.clone(),
                            SceneChangeEvent::Act(act_event) => act_event.act_name.clone(),
                        },
                        timestamp: chrono::Utc::now(),
                    };
                    
                    self.history_repository.add_history_entry(history_entry).await?;
                }
            }
            None => {
                debug!("Scene change content was not an actual change, skipping broadcast");
            }
        }

        Ok(result)
    }

    async fn get_current_location_state(&self) -> AppResult<LocationState> {
        let state = self.current_state.read().await;
        Ok(state.clone())
    }

    async fn reset_tracking(&self) -> AppResult<()> {
        let mut state = self.current_state.write().await;
        state.reset();
        self.state_repository.save_state(&state).await?;
        
        debug!("Location tracking state reset");
        Ok(())
    }

    async fn get_current_scene(&self) -> AppResult<Option<String>> {
        let state = self.current_state.read().await;
        Ok(state.get_current_scene().cloned())
    }

    async fn get_current_act(&self) -> AppResult<Option<String>> {
        let state = self.current_state.read().await;
        Ok(state.get_current_act().cloned())
    }

    async fn start_session(&self) -> AppResult<()> {
        self.start_tracking_session().await
    }

    async fn end_session(&self) -> AppResult<()> {
        self.end_tracking_session().await
    }

    async fn is_session_active(&self) -> bool {
        self.current_session.read().await.is_some()
    }

    async fn get_current_session(&self) -> AppResult<Option<LocationTrackingSession>> {
        Ok(self.current_session.read().await.clone())
    }

    async fn get_stats(&self) -> AppResult<LocationTrackingStats> {
        self.stats_repository.load_stats().await
    }

    async fn get_location_history(&self) -> AppResult<Vec<LocationHistoryEntry>> {
        self.history_repository.load_history().await
    }

    async fn clear_history(&self) -> AppResult<()> {
        self.history_repository.clear_history().await
    }

    async fn update_scene_type_config(&self, config: SceneTypeConfig) -> AppResult<()> {
        let mut detector = self.scene_type_detector.write().await;
        detector.update_scene_type_config(config);
        Ok(())
    }

    async fn get_config(&self) -> AppResult<LocationTrackingConfig> {
        Ok(self.config.read().await.clone())
    }

    async fn update_config(&self, new_config: LocationTrackingConfig) -> AppResult<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
}

impl LocationTrackingServiceImpl {
    /// Validates if a scene change event represents an actual change
    /// Returns Some(event) if it's a real change, None if it's the same location
    async fn validate_scene_change_event(
        &self,
        event: SceneChangeEvent,
    ) -> AppResult<Option<SceneChangeEvent>> {
        match &event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Validating hideout change: {}", hideout_event.hideout_name);
                let state = self.current_state.read().await;
                let result = state.get_current_scene() != Some(&hideout_event.hideout_name);
                debug!("Hideout change validation result: {}", result);
                if result {
                    Ok(Some(event))
                } else {
                    Ok(None)
                }
            }
            SceneChangeEvent::Zone(zone_event) => {
                debug!("Validating zone change: {}", zone_event.zone_name);
                let state = self.current_state.read().await;
                let result = state.get_current_scene() != Some(&zone_event.zone_name);
                debug!("Zone change validation result: {}", result);
                if result {
                    Ok(Some(event))
                } else {
                    Ok(None)
                }
            }
            SceneChangeEvent::Act(act_event) => {
                debug!("Processing act event: {}", act_event.act_name);
                debug!("Act event always returned for session continuity");
                Ok(Some(event))
            }
        }
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
    fn detect_scene_type(&self, content: &str) -> SceneType {
        let lower_content = content.to_lowercase();

        if self.is_hideout_content(&lower_content) {
            return SceneType::Hideout;
        }

        if self.is_act_content(&lower_content) {
            return SceneType::Act;
        }

        SceneType::Zone
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
