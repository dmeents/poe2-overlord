use crate::errors::{AppError, AppResult};
use crate::models::{
    events::SceneChangeEvent, LocationSession, LocationStats, LocationType, TimeTrackingEvent,
};
use chrono::{DateTime, Utc};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
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
const MIN_SESSION_DURATION_SECONDS: i64 = 1;

/// Character-aware session tracker for tracking time spent in different game locations
/// All operations are scoped to a specific character
pub struct CharacterSessionTracker {
    // Character-scoped data: character_id -> data
    active_sessions: Arc<RwLock<HashMap<String, HashMap<String, LocationSession>>>>, // character_id -> location_id -> session
    completed_sessions: Arc<RwLock<HashMap<String, Vec<LocationSession>>>>, // character_id -> sessions
    stats_cache: Arc<RwLock<HashMap<String, HashMap<String, LocationStats>>>>, // character_id -> location_id -> stats
    event_sender: broadcast::Sender<TimeTrackingEvent>,
    data_directory: PathBuf, // Base directory for character-specific data files
    poe_process_start_time: Arc<RwLock<Option<DateTime<Utc>>>>,
    character_manager: Option<Arc<crate::services::character_manager::CharacterManager>>, // Optional character manager for active character lookup
}

impl CharacterSessionTracker {
    /// Create a new character-aware session tracker
    pub fn new() -> Self {
        Self::with_data_directory_and_character_manager(None, None)
    }

    /// Create a new character-aware session tracker with character manager
    pub fn with_character_manager(
        character_manager: Arc<crate::services::character_manager::CharacterManager>,
    ) -> Self {
        Self::with_data_directory_and_character_manager(None, Some(character_manager))
    }

    /// Create a new character-aware session tracker with a custom data directory (mainly for testing)
    pub fn with_data_directory(custom_dir: Option<PathBuf>) -> Self {
        Self::with_data_directory_and_character_manager(custom_dir, None)
    }

    /// Create a new character-aware session tracker with a custom data directory and character manager
    pub fn with_data_directory_and_character_manager(
        custom_dir: Option<PathBuf>,
        character_manager: Option<Arc<crate::services::character_manager::CharacterManager>>,
    ) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);

        // Use custom directory if provided, otherwise use system config directory
        let data_directory = custom_dir.unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("poe2-overlord")
        });

        let service = Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            completed_sessions: Arc::new(RwLock::new(HashMap::new())),
            stats_cache: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            data_directory,
            poe_process_start_time: Arc::new(RwLock::new(None)),
            character_manager,
        };

        service
    }

    /// Get the event receiver for subscribing to time tracking events
    pub fn subscribe(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }

    /// Get the data file path for a specific character
    fn get_character_data_path(&self, character_id: &str) -> PathBuf {
        self.data_directory
            .join(format!("time_tracking_{}.json", character_id))
    }

    /// Load character-specific time tracking data from file
    async fn load_character_data(&self, character_id: &str) -> AppResult<()> {
        let data_file_path = self.get_character_data_path(character_id);

        if !data_file_path.exists() {
            debug!(
                "No time tracking data file found for character {}, starting fresh",
                character_id
            );
            return Ok(());
        }

        let content = fs::read_to_string(&data_file_path).map_err(|e| {
            AppError::FileSystem(format!("Failed to read time tracking data file: {}", e))
        })?;

        let data: CharacterTimeTrackingData = serde_json::from_str(&content).map_err(|e| {
            AppError::Serialization(format!("Failed to parse time tracking data: {}", e))
        })?;

        // Restore completed sessions
        {
            let mut completed = self.completed_sessions.write().await;
            completed.insert(character_id.to_string(), data.completed_sessions);
        }

        // Restore stats cache
        {
            let mut stats_cache = self.stats_cache.write().await;
            let mut character_stats = HashMap::new();
            for stats in data.stats {
                character_stats.insert(stats.location_id.clone(), stats);
            }
            stats_cache.insert(character_id.to_string(), character_stats);
        }

        debug!(
            "Time tracking data loaded successfully for character {}",
            character_id
        );
        Ok(())
    }

    /// Save character-specific time tracking data to file
    async fn save_character_data(&self, character_id: &str) -> AppResult<()> {
        let data_file_path = self.get_character_data_path(character_id);

        let completed_sessions = {
            let completed = self.completed_sessions.read().await;
            completed.get(character_id).cloned().unwrap_or_default()
        };

        let stats = {
            let stats_cache = self.stats_cache.read().await;
            stats_cache
                .get(character_id)
                .map(|character_stats| character_stats.values().cloned().collect())
                .unwrap_or_default()
        };

        let data = CharacterTimeTrackingData {
            character_id: character_id.to_string(),
            completed_sessions,
            stats,
        };

        let content = serde_json::to_string_pretty(&data).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize time tracking data: {}", e))
        })?;

        // Write to a temporary file first, then rename to ensure atomic write
        let temp_path = data_file_path.with_extension(TEMP_FILE_EXTENSION);
        fs::write(&temp_path, content)
            .map_err(|e| AppError::FileSystem(format!("Failed to write temp file: {}", e)))?;

        fs::rename(&temp_path, &data_file_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to rename temp file: {}", e)))?;

        debug!(
            "Time tracking data saved successfully for character {}",
            character_id
        );
        Ok(())
    }

    /// Handle a scene change event for the active character (if any)
    /// If no character is active, this method does nothing
    pub async fn handle_scene_change_for_active_character(&self, event: &SceneChangeEvent) {
        if let Some(character_manager) = &self.character_manager {
            if let Some(active_character) = character_manager.get_active_character().await {
                debug!(
                    "Handling scene change for active character: {}",
                    active_character.name
                );
                self.handle_scene_change(&active_character.id, event).await;
            } else {
                debug!("Scene change event received, but no active character - ignoring");
            }
        } else {
            debug!("Scene change event received, but no character manager available - ignoring");
        }
    }

    /// Handle a scene change event for a specific character
    pub async fn handle_scene_change(&self, character_id: &str, event: &SceneChangeEvent) {
        // Load character data if not already loaded
        if let Err(e) = self.load_character_data(character_id).await {
            warn!("Failed to load character data for {}: {}", character_id, e);
        }

        match event {
            SceneChangeEvent::Hideout(hideout_event) => {
                if let Err(e) = self
                    .start_session(
                        character_id,
                        hideout_event.hideout_name.clone(),
                        LocationType::Hideout,
                    )
                    .await
                {
                    error!("Failed to start hideout session: {}", e);
                }
            }
            SceneChangeEvent::Act(act_event) => {
                if let Err(e) = self
                    .start_session(character_id, act_event.act_name.clone(), LocationType::Act)
                    .await
                {
                    error!("Failed to start act session: {}", e);
                }
            }
            SceneChangeEvent::Zone(zone_event) => {
                if let Err(e) = self
                    .start_session(
                        character_id,
                        zone_event.zone_name.clone(),
                        LocationType::Zone,
                    )
                    .await
                {
                    error!("Failed to start zone session: {}", e);
                }
            }
        }
    }

    /// Start a new session for a character and location
    pub async fn start_session(
        &self,
        character_id: &str,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()> {
        let location_id = self.generate_location_id(&location_name, &location_type);

        // End any existing session for this location type for this character
        self.end_sessions_by_type(character_id, &location_type)
            .await?;

        // Special case: hideout sessions should also end act and zone sessions
        if location_type == LocationType::Hideout {
            self.end_sessions_by_type(character_id, &LocationType::Act)
                .await?;
            self.end_sessions_by_type(character_id, &LocationType::Zone)
                .await?;
        }

        // Special case: zone and act sessions should end hideout sessions
        if location_type == LocationType::Zone || location_type == LocationType::Act {
            self.end_sessions_by_type(character_id, &LocationType::Hideout)
                .await?;
        }

        let session = LocationSession {
            character_id: character_id.to_string(),
            location_id: location_id.clone(),
            location_name: location_name.clone(),
            location_type: location_type.clone(),
            entry_timestamp: Utc::now(),
            exit_timestamp: None,
            duration_seconds: None,
        };

        // Store the active session
        {
            let mut active_sessions = self.active_sessions.write().await;
            let character_sessions = active_sessions
                .entry(character_id.to_string())
                .or_insert_with(HashMap::new);
            character_sessions.insert(location_id.clone(), session.clone());
        }

        debug!(
            "Started time tracking session for character {} in {}: {}",
            character_id,
            location_type.to_string(),
            location_name
        );

        // Broadcast session started event
        if let Err(e) = self
            .event_sender
            .send(TimeTrackingEvent::SessionStarted(session))
        {
            debug!("Failed to send session started event: {}", e);
        }

        // Update stats
        self.update_stats_for_location(character_id, &location_id)
            .await?;

        Ok(())
    }

    /// End a session for a specific character and location
    pub async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        let session = {
            let mut active_sessions = self.active_sessions.write().await;
            if let Some(character_sessions) = active_sessions.get_mut(character_id) {
                character_sessions.remove(location_id)
            } else {
                None
            }
        };

        if let Some(mut session) = session {
            let now = Utc::now();
            session.exit_timestamp = Some(now);
            session.duration_seconds = Some(
                (now.timestamp() - session.entry_timestamp.timestamp())
                    .max(MIN_SESSION_DURATION_SECONDS) as u64,
            );

            // Add to completed sessions
            {
                let mut completed_sessions = self.completed_sessions.write().await;
                let character_sessions = completed_sessions
                    .entry(character_id.to_string())
                    .or_insert_with(Vec::new);
                character_sessions.push(session.clone());
            }

            debug!(
                "Ended time tracking session for character {} in {}: {}",
                character_id,
                session.location_type.to_string(),
                session.location_name
            );

            // Broadcast session ended event
            if let Err(e) = self
                .event_sender
                .send(TimeTrackingEvent::SessionEnded(session))
            {
                debug!("Failed to send session ended event: {}", e);
            }

            // Update stats
            self.update_stats_for_location(character_id, location_id)
                .await?;

            // Save data
            self.save_character_data(character_id).await?;
        }

        Ok(())
    }

    /// End all active sessions for a character
    pub async fn end_all_active_sessions(&self, character_id: &str) -> AppResult<()> {
        let active_sessions = {
            let active_sessions = self.active_sessions.read().await;
            active_sessions
                .get(character_id)
                .map(|sessions| sessions.keys().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        };

        for location_id in active_sessions {
            self.end_session(character_id, &location_id).await?;
        }

        debug!("Ended all active sessions for character {}", character_id);
        Ok(())
    }

    /// End sessions by type for a specific character
    async fn end_sessions_by_type(
        &self,
        character_id: &str,
        location_type: &LocationType,
    ) -> AppResult<()> {
        let sessions_to_end = {
            let active_sessions = self.active_sessions.read().await;
            if let Some(character_sessions) = active_sessions.get(character_id) {
                character_sessions
                    .iter()
                    .filter(|(_, session)| session.location_type == *location_type)
                    .map(|(location_id, _)| location_id.clone())
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };

        for location_id in sessions_to_end {
            self.end_session(character_id, &location_id).await?;
        }

        Ok(())
    }

    /// Generate a unique location ID
    fn generate_location_id(&self, location_name: &str, location_type: &LocationType) -> String {
        format!(
            "{}_{}",
            location_type.to_string().to_lowercase(),
            location_name
        )
    }

    /// Update stats for a specific location and character
    async fn update_stats_for_location(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<()> {
        // Get all completed sessions for this character and location
        let sessions = {
            let completed_sessions = self.completed_sessions.read().await;
            completed_sessions
                .get(character_id)
                .map(|character_sessions| {
                    character_sessions
                        .iter()
                        .filter(|session| session.location_id == *location_id)
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };

        if sessions.is_empty() {
            return Ok(());
        }

        let first_session = &sessions[0];
        let total_visits = sessions.len() as u32;
        let total_time_seconds: u64 = sessions
            .iter()
            .map(|s| s.duration_seconds.unwrap_or(0))
            .sum();
        let average_session_seconds = if total_visits > 0 {
            total_time_seconds as f64 / total_visits as f64
        } else {
            0.0
        };
        let last_visited = sessions
            .iter()
            .map(|s| s.exit_timestamp.unwrap_or(s.entry_timestamp))
            .max();

        let stats = LocationStats {
            character_id: character_id.to_string(),
            location_id: location_id.to_string(),
            location_name: first_session.location_name.clone(),
            location_type: first_session.location_type.clone(),
            total_visits,
            total_time_seconds,
            average_session_seconds,
            last_visited,
        };

        // Update stats cache
        {
            let mut stats_cache = self.stats_cache.write().await;
            let character_stats = stats_cache
                .entry(character_id.to_string())
                .or_insert_with(HashMap::new);
            character_stats.insert(location_id.to_string(), stats.clone());
        }

        // Broadcast stats updated event
        if let Err(e) = self
            .event_sender
            .send(TimeTrackingEvent::StatsUpdated(stats))
        {
            debug!("Failed to send stats updated event: {}", e);
        }

        Ok(())
    }

    /// Get active sessions for a specific character
    pub async fn get_active_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let active_sessions = self.active_sessions.read().await;
        active_sessions
            .get(character_id)
            .map(|sessions| sessions.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Get completed sessions for a specific character
    pub async fn get_completed_sessions(&self, character_id: &str) -> Vec<LocationSession> {
        let completed_sessions = self.completed_sessions.read().await;
        completed_sessions
            .get(character_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all stats for a specific character
    pub async fn get_all_stats(&self, character_id: &str) -> Vec<LocationStats> {
        let stats_cache = self.stats_cache.read().await;
        stats_cache
            .get(character_id)
            .map(|character_stats| character_stats.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Get total play time for a specific character
    pub async fn get_total_play_time(&self, character_id: &str) -> u64 {
        let stats = self.get_all_stats(character_id).await;
        stats.iter().map(|s| s.total_time_seconds).sum()
    }

    /// Get total play time since process start for a specific character
    pub async fn get_total_play_time_since_process_start(&self, character_id: &str) -> u64 {
        let process_start = {
            let start_time = self.poe_process_start_time.read().await;
            *start_time
        };

        if let Some(start_time) = process_start {
            let completed_sessions = self.get_completed_sessions(character_id).await;
            let active_sessions = self.get_active_sessions(character_id).await;

            let completed_time: u64 = completed_sessions
                .iter()
                .filter(|s| s.entry_timestamp >= start_time)
                .map(|s| s.duration_seconds.unwrap_or(0))
                .sum();

            let active_time: u64 = active_sessions
                .iter()
                .filter(|s| s.entry_timestamp >= start_time)
                .map(|s| {
                    let now = Utc::now();
                    (now.timestamp() - s.entry_timestamp.timestamp()).max(0) as u64
                })
                .sum();

            completed_time + active_time
        } else {
            0
        }
    }

    /// Get total hideout time for a specific character
    pub async fn get_total_hideout_time(&self, character_id: &str) -> u64 {
        let stats = self.get_all_stats(character_id).await;
        stats
            .iter()
            .filter(|s| s.location_type == LocationType::Hideout)
            .map(|s| s.total_time_seconds)
            .sum()
    }

    /// Clear all time tracking data for a specific character
    pub async fn clear_character_data(&self, character_id: &str) -> AppResult<()> {
        {
            let mut active = self.active_sessions.write().await;
            active.remove(character_id);
        }

        {
            let mut completed = self.completed_sessions.write().await;
            completed.remove(character_id);
        }

        {
            let mut stats = self.stats_cache.write().await;
            stats.remove(character_id);
        }

        // Remove data file
        let data_file_path = self.get_character_data_path(character_id);
        if data_file_path.exists() {
            fs::remove_file(&data_file_path)
                .map_err(|e| AppError::FileSystem(format!("Failed to remove data file: {}", e)))?;
        }

        debug!(
            "All time tracking data cleared for character {}",
            character_id
        );
        Ok(())
    }

    /// Set the POE process start time (global operation)
    pub async fn set_poe_process_start_time(&self) {
        let mut start_time = self.poe_process_start_time.write().await;
        *start_time = Some(Utc::now());
        debug!("Set POE process start time");
    }

    /// Clear the POE process start time (global operation)
    pub async fn clear_poe_process_start_time(&self) {
        let mut start_time = self.poe_process_start_time.write().await;
        *start_time = None;
        debug!("Cleared POE process start time");
    }

    /// End all active sessions for all characters (global operation)
    pub async fn end_all_active_sessions_global(&self) -> AppResult<()> {
        debug!("Ending all active sessions for all characters");

        let character_ids = {
            let active_sessions = self.active_sessions.read().await;
            active_sessions.keys().cloned().collect::<Vec<_>>()
        };

        for character_id in character_ids {
            if let Err(e) = self.end_all_active_sessions(&character_id).await {
                error!(
                    "Failed to end active sessions for character {}: {}",
                    character_id, e
                );
            }
        }

        debug!("Ended all active sessions for all characters");
        Ok(())
    }

    /// Handle application shutdown by ending all active sessions for all characters
    pub async fn shutdown(&self) -> AppResult<()> {
        debug!("Shutting down character session tracker, ending all active sessions");

        let character_ids = {
            let active_sessions = self.active_sessions.read().await;
            active_sessions.keys().cloned().collect::<Vec<_>>()
        };

        for character_id in character_ids {
            if let Err(e) = self.end_all_active_sessions(&character_id).await {
                error!(
                    "Failed to end active sessions for character {}: {}",
                    character_id, e
                );
            }
        }

        debug!("Character session tracker shutdown completed");
        Ok(())
    }
}

/// Character-specific time tracking data structure for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CharacterTimeTrackingData {
    character_id: String,
    completed_sessions: Vec<LocationSession>,
    stats: Vec<LocationStats>,
}
