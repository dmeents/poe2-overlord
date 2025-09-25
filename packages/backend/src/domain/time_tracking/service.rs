use crate::domain::character::traits::CharacterService as CharacterServiceTrait;
use crate::domain::time_tracking::{
    events::{
        SessionEnded, SessionStarted, StatsUpdated, TimeTrackingDataCleared,
        TimeTrackingDataLoaded, TimeTrackingDataSaved, TimeTrackingEvent,
    },
    models::{CharacterTimeTrackingData, LocationSession, LocationStats, LocationType},
    traits::{TimeTrackingEventPublisher, TimeTrackingService},
};
use crate::errors::{AppError, AppResult};
use crate::utils::time_calculations::{
    validate_no_session_overlap, validate_session_data, ValidationResult,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::{debug, error, warn};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

/// Character-aware session tracking constants
const EVENT_CHANNEL_SIZE: usize = 100;
const TEMP_FILE_EXTENSION: &str = "tmp";

/// Time tracking service implementation that handles business logic for time tracking
#[derive(Clone)]
pub struct TimeTrackingServiceImpl {
    /// Character-scoped data: character_id -> data
    active_sessions: Arc<RwLock<HashMap<String, HashMap<String, LocationSession>>>>, // character_id -> location_id -> session
    completed_sessions: Arc<RwLock<HashMap<String, Vec<LocationSession>>>>, // character_id -> sessions
    stats_cache: Arc<RwLock<HashMap<String, HashMap<String, LocationStats>>>>, // character_id -> location_id -> stats
    event_sender: broadcast::Sender<TimeTrackingEvent>,
    data_directory: PathBuf, // Base directory for character-specific data files
    poe_process_start_time: Arc<RwLock<Option<DateTime<Utc>>>>,
    character_service: Option<Arc<dyn CharacterServiceTrait>>, // Optional character service for active character lookup
}

impl Default for TimeTrackingServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeTrackingServiceImpl {
    /// Create a new time tracking service
    pub fn new() -> Self {
        Self::with_character_service(None)
    }

    /// Create a new time tracking service with character service
    pub fn with_character_service(
        character_service: Option<Arc<dyn CharacterServiceTrait>>,
    ) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);

        // Use system config directory
        let data_directory = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord");

        Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            completed_sessions: Arc::new(RwLock::new(HashMap::new())),
            stats_cache: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            data_directory,
            poe_process_start_time: Arc::new(RwLock::new(None)),
            character_service,
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

    /// Validate session data using the utility functions
    fn validate_session_data_internal(&self, session: &LocationSession) -> AppResult<()> {
        match validate_session_data(
            session.entry_timestamp,
            session.exit_timestamp,
            session.duration_seconds,
        ) {
            ValidationResult::Valid => Ok(()),
            ValidationResult::Error(reason) => {
                Err(AppError::validation_error("session_data", &reason))
            }
            ValidationResult::Warning(_) => Ok(()), // Warnings are logged but don't fail validation
        }
    }

    /// Validate no session overlap
    fn validate_no_session_overlap_internal(
        &self,
        _character_id: &str,
        _location_id: &str,
        active_sessions: &[LocationSession],
    ) -> AppResult<()> {
        // Convert active sessions to the format expected by the validation function
        let session_timestamps: Vec<(DateTime<Utc>, Option<DateTime<Utc>>)> = active_sessions
            .iter()
            .map(|session| (session.entry_timestamp, session.exit_timestamp))
            .collect();

        match validate_no_session_overlap(Utc::now(), None, &session_timestamps) {
            ValidationResult::Valid => Ok(()),
            ValidationResult::Error(reason) => {
                Err(AppError::validation_error("session_overlap", &reason))
            }
            ValidationResult::Warning(_) => Ok(()), // Warnings are logged but don't fail validation
        }
    }

    /// Update location statistics with a new session
    fn update_location_stats_internal(&self, stats: &mut LocationStats, session: &LocationSession) {
        if let Some(duration) = session.duration_seconds {
            stats.total_visits += 1;
            stats.total_time_seconds += duration;
            stats.average_session_seconds =
                stats.total_time_seconds as f64 / stats.total_visits as f64;
            stats.last_visited = Some(session.exit_timestamp.unwrap_or(Utc::now()));
        }
    }

    /// Calculate total play time from completed sessions
    fn calculate_total_play_time_internal(&self, completed_sessions: &[LocationSession]) -> u64 {
        completed_sessions
            .iter()
            .filter_map(|session| session.duration_seconds)
            .sum()
    }

    /// Calculate total hideout time from completed sessions
    fn calculate_total_hideout_time_internal(&self, completed_sessions: &[LocationSession]) -> u64 {
        completed_sessions
            .iter()
            .filter(|session| session.location_type == LocationType::Hideout)
            .filter_map(|session| session.duration_seconds)
            .sum()
    }

    /// Calculate total play time since process start
    async fn calculate_total_play_time_since_process_start_internal(
        &self,
        character_id: &str,
    ) -> u64 {
        if let Some(process_start_time) = self.get_poe_process_start_time().await {
            let completed_sessions = self.get_completed_sessions(character_id).await;
            let active_sessions = self.get_active_sessions(character_id).await;

            let mut total_time = 0u64;

            // Add time from completed sessions that started after process start
            for session in completed_sessions {
                if session.entry_timestamp >= process_start_time {
                    if let Some(duration) = session.duration_seconds {
                        total_time += duration;
                    }
                }
            }

            // Add current time from active sessions that started after process start
            for session in active_sessions {
                if session.entry_timestamp >= process_start_time {
                    total_time +=
                        (Utc::now() - session.entry_timestamp).num_seconds().max(0) as u64;
                }
            }

            total_time
        } else {
            0
        }
    }

    /// Get character data file path
    fn get_character_data_file_path(&self, character_id: &str) -> PathBuf {
        self.data_directory
            .join(format!("time_tracking_{}.json", character_id))
    }

    /// Load character time tracking data from file
    async fn load_character_data_from_file(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTimeTrackingData>> {
        let file_path = self.get_character_data_file_path(character_id);

        if !file_path.exists() {
            return Ok(None);
        }

        match fs::read_to_string(&file_path) {
            Ok(content) => match serde_json::from_str::<CharacterTimeTrackingData>(&content) {
                Ok(data) => {
                    debug!("Loaded time tracking data for character: {}", character_id);
                    Ok(Some(data))
                }
                Err(e) => {
                    error!(
                        "Failed to parse time tracking data for character {}: {}",
                        character_id, e
                    );
                    Err(AppError::serialization_error(
                        "time_tracking_data",
                        &e.to_string(),
                    ))
                }
            },
            Err(e) => {
                error!(
                    "Failed to read time tracking data file for character {}: {}",
                    character_id, e
                );
                Err(AppError::file_system_error("read", &e.to_string()))
            }
        }
    }

    /// Save character time tracking data to file
    async fn save_character_data_to_file(&self, data: &CharacterTimeTrackingData) -> AppResult<()> {
        // Ensure data directory exists
        if !self.data_directory.exists() {
            if let Err(e) = fs::create_dir_all(&self.data_directory) {
                error!("Failed to create data directory: {}", e);
                return Err(AppError::file_system_error(
                    "create_directory",
                    &e.to_string(),
                ));
            }
        }

        let file_path = self.get_character_data_file_path(&data.character_id);
        let temp_file_path = file_path.with_extension(TEMP_FILE_EXTENSION);

        // Write to temporary file first
        let json_content = match serde_json::to_string_pretty(data) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to serialize time tracking data for character {}: {}",
                    data.character_id, e
                );
                return Err(AppError::serialization_error(
                    "time_tracking_data",
                    &e.to_string(),
                ));
            }
        };

        if let Err(e) = fs::write(&temp_file_path, json_content) {
            error!(
                "Failed to write time tracking data to temp file for character {}: {}",
                data.character_id, e
            );
            return Err(AppError::file_system_error("write_temp", &e.to_string()));
        }

        // Atomically move temp file to final location
        if let Err(e) = fs::rename(&temp_file_path, &file_path) {
            error!(
                "Failed to move temp file to final location for character {}: {}",
                data.character_id, e
            );
            // Clean up temp file
            let _ = fs::remove_file(&temp_file_path);
            return Err(AppError::file_system_error("rename", &e.to_string()));
        }

        debug!(
            "Saved time tracking data for character: {}",
            data.character_id
        );
        Ok(())
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

        // Get current active sessions for validation
        let active_sessions = self.get_active_sessions(character_id).await;

        // Validate no overlap
        self.validate_no_session_overlap_internal(character_id, &location_id, &active_sessions)?;

        // Create new session
        let session = LocationSession::new(
            character_id.to_string(),
            location_id.clone(),
            location_name,
            location_type,
        );

        // Validate session data
        self.validate_session_data_internal(&session)?;

        // Store active session
        {
            let mut active_sessions_map = self.active_sessions.write().await;
            let character_sessions = active_sessions_map
                .entry(character_id.to_string())
                .or_insert_with(HashMap::new);
            character_sessions.insert(location_id.clone(), session.clone());
        }

        // Publish event
        let event = TimeTrackingEvent::SessionStarted(SessionStarted::new(session.clone()));
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to publish session started event: {}", e);
        }

        debug!(
            "Started session for character {} at location {}",
            character_id, location_id
        );
        Ok(())
    }

    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        // Get and remove active session
        let session = {
            let mut active_sessions_map = self.active_sessions.write().await;
            if let Some(character_sessions) = active_sessions_map.get_mut(character_id) {
                character_sessions.remove(location_id)
            } else {
                return Err(AppError::time_tracking_error(
                    "active_session",
                    &format!(
                        "Session not found for character {} at location {}",
                        character_id, location_id
                    ),
                ));
            }
        };

        if let Some(mut session) = session {
            // End the session
            session.end_session();

            // Validate session data
            self.validate_session_data_internal(&session)?;

            // Add to completed sessions
            {
                let mut completed_sessions_map = self.completed_sessions.write().await;
                let character_sessions = completed_sessions_map
                    .entry(character_id.to_string())
                    .or_insert_with(Vec::new);
                character_sessions.push(session.clone());
            }

            // Update stats
            {
                let mut stats_cache = self.stats_cache.write().await;
                let character_stats = stats_cache
                    .entry(character_id.to_string())
                    .or_insert_with(HashMap::new);

                let stats = character_stats
                    .entry(location_id.to_string())
                    .or_insert_with(|| {
                        LocationStats::new(
                            character_id.to_string(),
                            location_id.to_string(),
                            session.location_name.clone(),
                            session.location_type.clone(),
                        )
                    });

                self.update_location_stats_internal(stats, &session);

                // Publish stats updated event
                let event = TimeTrackingEvent::StatsUpdated(StatsUpdated::new(stats.clone()));
                if let Err(e) = self.event_sender.send(event) {
                    warn!("Failed to publish stats updated event: {}", e);
                }
            }

            // Publish session ended event
            let event = TimeTrackingEvent::SessionEnded(SessionEnded::new(session.clone()));
            if let Err(e) = self.event_sender.send(event) {
                warn!("Failed to publish session ended event: {}", e);
            }

            debug!(
                "Ended session for character {} at location {}",
                character_id, location_id
            );
            Ok(())
        } else {
            Err(AppError::time_tracking_error(
                "active_session",
                &format!(
                    "Session not found for character {} at location {}",
                    character_id, location_id
                ),
            ))
        }
    }

    async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let active_sessions_map = self.active_sessions.read().await;
        if let Some(character_sessions) = active_sessions_map.get(character_id) {
            character_sessions.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let completed_sessions_map = self.completed_sessions.read().await;
        if let Some(character_sessions) = completed_sessions_map.get(character_id) {
            character_sessions.clone()
        } else {
            Vec::new()
        }
    }

    async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats> {
        let stats_cache = self.stats_cache.read().await;
        if let Some(character_stats) = stats_cache.get(character_id) {
            character_stats.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    async fn get_total_play_time(&self, character_id: &str) -> u64 {
        let completed_sessions = self.get_completed_sessions(character_id).await;
        self.calculate_total_play_time_internal(&completed_sessions)
    }

    async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64 {
        self.calculate_total_play_time_since_process_start_internal(character_id)
            .await
    }

    async fn get_total_hideout_time(&self, character_id: &str) -> u64 {
        let completed_sessions = self.get_completed_sessions(character_id).await;
        self.calculate_total_hideout_time_internal(&completed_sessions)
    }

    async fn get_last_known_location(&self, character_id: &str) -> Option<LocationSession> {
        // Get all completed sessions and find the most recent one
        let mut sessions = self.get_completed_sessions(character_id).await;

        // Also check for active sessions (current location)
        let active_sessions = self.get_active_sessions(character_id).await;
        sessions.extend(active_sessions);

        // Sort by entry timestamp (most recent first)
        sessions.sort_by(|a, b| b.entry_timestamp.cmp(&a.entry_timestamp));

        sessions.first().cloned()
    }

    async fn clear_character_data(&self, character_id: &str) -> AppResult<()> {
        // Clear active sessions
        {
            let mut active_sessions_map = self.active_sessions.write().await;
            active_sessions_map.remove(character_id);
        }

        // Clear completed sessions
        {
            let mut completed_sessions_map = self.completed_sessions.write().await;
            completed_sessions_map.remove(character_id);
        }

        // Clear stats cache
        {
            let mut stats_cache = self.stats_cache.write().await;
            stats_cache.remove(character_id);
        }

        // Delete data file
        let file_path = self.get_character_data_file_path(character_id);
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                warn!(
                    "Failed to delete time tracking data file for character {}: {}",
                    character_id, e
                );
            }
        }

        // Publish event
        let event = TimeTrackingEvent::TimeTrackingDataCleared(TimeTrackingDataCleared::new(
            character_id.to_string(),
        ));
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to publish data cleared event: {}", e);
        }

        debug!(
            "Cleared all time tracking data for character: {}",
            character_id
        );
        Ok(())
    }

    async fn load_all_character_data(&self) -> AppResult<()> {
        if let Some(ref character_service) = self.character_service {
            let characters = character_service.get_all_characters().await;
            let character_count = characters.len();

            for character in &characters {
                if let Err(e) = self.load_character_data(&character.id).await {
                    warn!(
                        "Failed to load time tracking data for character {}: {}",
                        character.name, e
                    );
                }
            }

            debug!(
                "Loaded time tracking data for {} characters",
                character_count
            );
        } else {
            warn!("No character service available, cannot load character time tracking data");
        }

        Ok(())
    }

    async fn save_all_character_data(&self) -> AppResult<()> {
        let completed_sessions_map = self.completed_sessions.read().await;
        let stats_cache = self.stats_cache.read().await;

        for character_id in completed_sessions_map.keys() {
            let completed_sessions = completed_sessions_map
                .get(character_id)
                .cloned()
                .unwrap_or_default();
            let stats = stats_cache
                .get(character_id)
                .map(|character_stats| character_stats.values().cloned().collect::<Vec<_>>())
                .unwrap_or_default();

            let data = CharacterTimeTrackingData {
                character_id: character_id.clone(),
                completed_sessions,
                stats: stats.clone(),
            };

            if let Err(e) = self.save_character_data_to_file(&data).await {
                warn!(
                    "Failed to save time tracking data for character {}: {}",
                    character_id, e
                );
            }
        }

        debug!("Saved all character time tracking data");
        Ok(())
    }

    async fn load_character_data(&self, character_id: &str) -> AppResult<()> {
        match self.load_character_data_from_file(character_id).await? {
            Some(data) => {
                // Load completed sessions
                {
                    let mut completed_sessions_map = self.completed_sessions.write().await;
                    completed_sessions_map
                        .insert(character_id.to_string(), data.completed_sessions.clone());
                }

                // Load stats
                let stats_count = data.stats.len();
                {
                    let mut stats_cache = self.stats_cache.write().await;
                    let character_stats: HashMap<String, LocationStats> = data
                        .stats
                        .into_iter()
                        .map(|stat| (stat.location_id.clone(), stat))
                        .collect();
                    stats_cache.insert(character_id.to_string(), character_stats);
                }

                // Publish event
                let event = TimeTrackingEvent::TimeTrackingDataLoaded(TimeTrackingDataLoaded::new(
                    character_id.to_string(),
                    data.completed_sessions.len(),
                    stats_count,
                ));
                if let Err(e) = self.event_sender.send(event) {
                    warn!("Failed to publish data loaded event: {}", e);
                }

                debug!("Loaded time tracking data for character: {}", character_id);
                Ok(())
            }
            None => {
                debug!(
                    "No time tracking data found for character: {}",
                    character_id
                );
                Ok(())
            }
        }
    }

    async fn save_character_data(&self, character_id: &str) -> AppResult<()> {
        let completed_sessions = self.get_completed_sessions(character_id).await;
        let stats = self.get_all_stats(character_id).await;

        let data = CharacterTimeTrackingData {
            character_id: character_id.to_string(),
            completed_sessions: completed_sessions.clone(),
            stats: stats.clone(),
        };

        self.save_character_data_to_file(&data).await?;

        // Publish event
        let event = TimeTrackingEvent::TimeTrackingDataSaved(TimeTrackingDataSaved::new(
            character_id.to_string(),
            completed_sessions.len(),
            stats.len(),
        ));
        if let Err(e) = self.event_sender.send(event) {
            warn!("Failed to publish data saved event: {}", e);
        }

        debug!("Saved time tracking data for character: {}", character_id);
        Ok(())
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }

    async fn set_poe_process_start_time(&self, start_time: chrono::DateTime<chrono::Utc>) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = Some(start_time);
    }

    async fn clear_poe_process_start_time(&self) {
        let mut poe_start_time = self.poe_process_start_time.write().await;
        *poe_start_time = None;
    }

    async fn end_all_active_sessions_global(&self) -> AppResult<()> {
        let active_sessions_map = self.active_sessions.read().await;
        let mut character_ids = Vec::new();

        // Collect all character IDs with active sessions
        for (character_id, sessions) in active_sessions_map.iter() {
            if !sessions.is_empty() {
                character_ids.push(character_id.clone());
            }
        }
        drop(active_sessions_map);

        // End all active sessions for each character
        for character_id in character_ids {
            let active_sessions = self.get_active_sessions(&character_id).await;
            for session in active_sessions {
                if let Err(e) = self.end_session(&character_id, &session.location_id).await {
                    warn!(
                        "Failed to end session for character {} at location {}: {}",
                        character_id, session.location_id, e
                    );
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl TimeTrackingEventPublisher for TimeTrackingServiceImpl {
    async fn publish_event(&self, event: TimeTrackingEvent) -> AppResult<()> {
        if let Err(e) = self.event_sender.send(event) {
            return Err(AppError::event_emission_error(&e.to_string()));
        }
        Ok(())
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }
}
