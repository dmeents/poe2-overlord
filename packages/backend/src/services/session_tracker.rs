use crate::errors::{AppError, AppResult};
use crate::models::{events::SceneChangeEvent, LocationSession, LocationStats, LocationType, TimeTrackingEvent};
use chrono::{DateTime, Utc};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::broadcast;

/// Time tracking constants
const EVENT_CHANNEL_SIZE: usize = 100;
const DATA_FILE_NAME: &str = "time_tracking.json";
const TEMP_FILE_EXTENSION: &str = "tmp";
const MIN_SESSION_DURATION_SECONDS: i64 = 1;

/// Session tracker for tracking time spent in different game locations (zones and acts)
pub struct SessionTracker {
    active_sessions: Arc<RwLock<HashMap<String, LocationSession>>>,
    completed_sessions: Arc<RwLock<Vec<LocationSession>>>,
    stats_cache: Arc<RwLock<HashMap<String, LocationStats>>>,
    event_sender: broadcast::Sender<TimeTrackingEvent>,
    data_file_path: PathBuf,
    poe_process_start_time: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl SessionTracker {
    /// Create a new session tracker
    pub fn new() -> Self {
        Self::with_data_directory(None)
    }

    /// Create a new session tracker with a custom data directory (mainly for testing)
    pub fn with_data_directory(custom_dir: Option<PathBuf>) -> Self {
        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_SIZE);

        // Use custom directory if provided, otherwise use system config directory
        let config_dir = custom_dir.unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("poe2-overlord")
        });

        let data_file_path = config_dir.join(DATA_FILE_NAME);

        let service = Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            completed_sessions: Arc::new(RwLock::new(Vec::new())),
            stats_cache: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            data_file_path,
            poe_process_start_time: Arc::new(RwLock::new(None)),
        };

        // Load existing data
        if let Err(e) = service.load_data() {
            warn!("Failed to load time tracking data, starting fresh: {}", e);
        }

        service
    }

    /// Get the event receiver for subscribing to time tracking events
    pub fn subscribe(&self) -> broadcast::Receiver<TimeTrackingEvent> {
        self.event_sender.subscribe()
    }

    /// Handle a scene change event for time tracking
    pub async fn handle_scene_change(&self, event: &SceneChangeEvent) {
        match event {
            SceneChangeEvent::Hideout(hideout_event) => {
                debug!("Hideout change detected: {}", hideout_event.hideout_name);

                // End any active act session when entering a hideout
                if let Err(e) = self.end_sessions_by_type(&LocationType::Act).await {
                    error!("Failed to end act sessions when entering hideout: {}", e);
                }

                // Start time tracking for the hideout
                if let Err(e) = self
                    .start_session(hideout_event.hideout_name.clone(), LocationType::Hideout)
                    .await
                {
                    error!("Failed to start hideout time tracking: {}", e);
                }
            }
            SceneChangeEvent::Zone(zone_event) => {
                debug!("Zone change detected: {}", zone_event.zone_name);

                // Start time tracking for the new zone
                if let Err(e) = self
                    .start_session(zone_event.zone_name.clone(), LocationType::Zone)
                    .await
                {
                    error!("Failed to start zone time tracking: {}", e);
                }
            }
            SceneChangeEvent::Act(act_event) => {
                debug!("Act change detected: {}", act_event.act_name);

                // Start time tracking for the new act
                if let Err(e) = self
                    .start_session(act_event.act_name.clone(), LocationType::Act)
                    .await
                {
                    error!("Failed to start act time tracking: {}", e);
                }
            }
        }
    }

    /// Start a new session for a location
    pub async fn start_session(
        &self,
        location_name: String,
        location_type: LocationType,
    ) -> AppResult<()> {
        let location_id = self.generate_location_id(&location_name, &location_type);

        // End any existing session for this location type
        self.end_sessions_by_type(&location_type).await?;

        // Special case: hideout sessions should also end act and zone sessions
        if location_type == LocationType::Hideout {
            self.end_sessions_by_type(&LocationType::Act).await?;
            self.end_sessions_by_type(&LocationType::Zone).await?;
        }

        // Special case: zone and act sessions should end hideout sessions
        if location_type == LocationType::Zone || location_type == LocationType::Act {
            self.end_sessions_by_type(&LocationType::Hideout).await?;
        }

        let session = LocationSession {
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
            active_sessions.insert(location_id.clone(), session.clone());
        }

        debug!(
            "Started time tracking session for {}: {}",
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
        self.update_stats_for_location(&location_id).await?;

        Ok(())
    }

    /// End a session for a specific location
    pub async fn end_session(&self, location_id: &str) -> AppResult<()> {
        let session = {
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.remove(location_id)
        };

        if let Some(mut session) = session {
            let exit_time = Utc::now();
            let duration = (exit_time - session.entry_timestamp).num_seconds().max(MIN_SESSION_DURATION_SECONDS) as u64;

            session.exit_timestamp = Some(exit_time);
            session.duration_seconds = Some(duration);

            // Move to completed sessions
            {
                let mut completed = self.completed_sessions.write().await;
                completed.push(session.clone());
            }

            debug!(
                "Ended time tracking session for {}: {} (duration: {}s)",
                session.location_type.to_string(),
                session.location_name,
                duration
            );

            // Broadcast session ended event
            if let Err(e) = self
                .event_sender
                .send(TimeTrackingEvent::SessionEnded(session.clone()))
            {
                debug!("Failed to send session ended event: {}", e);
            }

            // Update stats
            self.update_stats_for_location(location_id).await?;

            // Save data
            self.save_data()?;

            Ok(())
        } else {
            Err(AppError::LogMonitor(format!(
                "No active session found for location: {}",
                location_id
            )))
        }
    }

    /// End all sessions of a specific type (useful when changing acts or entering hideouts)
    pub async fn end_sessions_by_type(&self, location_type: &LocationType) -> AppResult<()> {
        let sessions_to_end: Vec<String> = {
            let active_sessions = self.active_sessions.read().await;
            active_sessions
                .iter()
                .filter(|(_, session)| session.location_type == *location_type)
                .map(|(id, _)| id.clone())
                .collect()
        };

        for location_id in sessions_to_end {
            self.end_session(&location_id).await?;
        }

        Ok(())
    }

    /// End all active sessions (useful when game process exits)
    pub async fn end_all_active_sessions(&self) -> AppResult<()> {
        let sessions_to_end: Vec<String> = {
            let active_sessions = self.active_sessions.read().await;
            active_sessions.keys().cloned().collect()
        };

        if sessions_to_end.is_empty() {
            debug!("No active sessions to end");
            return Ok(());
        }

        debug!(
            "Ending {} active sessions due to game exit",
            sessions_to_end.len()
        );

        for location_id in sessions_to_end {
            if let Err(e) = self.end_session(&location_id).await {
                warn!(
                    "Failed to end session during cleanup for {}: {}",
                    location_id, e
                );
            }
        }

        // Save data after ending all sessions
        self.save_data()?;

        Ok(())
    }

    /// Get current active sessions
    pub async fn get_active_sessions(&self) -> Vec<LocationSession> {
        let active_sessions = self.active_sessions.read().await;
        active_sessions.values().cloned().collect()
    }

    /// Get completed sessions
    pub async fn get_completed_sessions(&self) -> Vec<LocationSession> {
        let completed_sessions = self.completed_sessions.read().await;
        completed_sessions.clone()
    }

    /// Get statistics for all locations
    pub async fn get_all_stats(&self) -> Vec<LocationStats> {
        let stats_cache = self.stats_cache.read().await;
        stats_cache.values().cloned().collect()
    }

    /// Get statistics for a specific location
    pub async fn get_location_stats(&self, location_id: &str) -> Option<LocationStats> {
        let stats_cache = self.stats_cache.read().await;
        stats_cache.get(location_id).cloned()
    }

    /// Update statistics for a specific location
    async fn update_stats_for_location(&self, location_id: &str) -> AppResult<()> {
        let mut stats_cache = self.stats_cache.write().await;

        // Calculate stats from completed sessions
        let completed_sessions = self.completed_sessions.read().await;
        let location_sessions: Vec<&LocationSession> = completed_sessions
            .iter()
            .filter(|session| session.location_id == location_id)
            .collect();

        if let Some(first_session) = location_sessions.first() {
            let total_visits = location_sessions.len() as u32;
            let total_time: u64 = location_sessions
                .iter()
                .filter_map(|session| session.duration_seconds)
                .sum();

            let average_session = if total_visits > 0 {
                total_time as f64 / total_visits as f64
            } else {
                0.0
            };

            let last_visited = location_sessions
                .iter()
                .map(|session| session.exit_timestamp.unwrap_or(session.entry_timestamp))
                .max();

            let stats = LocationStats {
                location_id: location_id.to_string(),
                location_name: first_session.location_name.clone(),
                location_type: first_session.location_type.clone(),
                total_visits,
                total_time_seconds: total_time,
                average_session_seconds: average_session,
                last_visited,
            };

            stats_cache.insert(location_id.to_string(), stats.clone());

            // Broadcast stats updated event
            if let Err(e) = self
                .event_sender
                .send(TimeTrackingEvent::StatsUpdated(stats))
            {
                debug!("Failed to send stats updated event: {}", e);
            }
        }

        Ok(())
    }

    /// Generate a unique ID for a location
    fn generate_location_id(&self, name: &str, location_type: &LocationType) -> String {
        let type_prefix = match location_type {
            LocationType::Zone => "zone",
            LocationType::Act => "act",
            LocationType::Hideout => "hideout",
        };
        format!("{}:{}", type_prefix, name.to_lowercase().replace(" ", "_"))
    }

    /// Save time tracking data to file
    fn save_data(&self) -> AppResult<()> {
        let data = TimeTrackingData {
            completed_sessions: self.get_completed_sessions_sync(),
            stats: self.get_all_stats_sync(),
        };

        let content = serde_json::to_string_pretty(&data).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize time tracking data: {}", e))
        })?;

        // Ensure directory exists
        if let Some(parent) = self.data_file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    AppError::FileSystem(format!("Failed to create directory: {}", e))
                })?;
            }
        }

        // Write to a temporary file first, then rename to ensure atomic write
        let temp_path = self.data_file_path.with_extension(TEMP_FILE_EXTENSION);
        fs::write(&temp_path, content)
            .map_err(|e| AppError::FileSystem(format!("Failed to write temp file: {}", e)))?;

        fs::rename(&temp_path, &self.data_file_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to rename temp file: {}", e)))?;

        debug!("Time tracking data saved successfully");
        Ok(())
    }

    /// Calculate total play time from all completed and active sessions
    pub async fn get_total_play_time(&self) -> u64 {
        let completed = self.completed_sessions.read().await;
        let active = self.active_sessions.read().await;

        // Sum completed session durations
        let completed_time: u64 = completed
            .iter()
            .filter_map(|session| session.duration_seconds)
            .sum();

        // Sum active session durations (calculated from entry time to now)
        let active_time: u64 = active
            .values()
            .map(|session| {
                let now = Utc::now();
                let duration = now.signed_duration_since(session.entry_timestamp);
                // Use milliseconds for more precision, then convert to seconds
                (duration.num_milliseconds().max(0) / 1000) as u64
            })
            .sum();

        completed_time + active_time
    }

    /// Set the POE process start time
    pub async fn set_poe_process_start_time(&self) {
        let mut start_time = self.poe_process_start_time.write().await;
        *start_time = Some(Utc::now());
        debug!("POE process start time set to: {:?}", start_time);
    }

    /// Clear the POE process start time (when process stops)
    pub async fn clear_poe_process_start_time(&self) {
        let mut start_time = self.poe_process_start_time.write().await;
        *start_time = None;
        debug!("POE process start time cleared");
    }

    /// Get the POE process start time
    pub async fn get_poe_process_start_time(&self) -> Option<DateTime<Utc>> {
        let start_time = self.poe_process_start_time.read().await;
        *start_time
    }

    /// Calculate total play time since POE process started
    pub async fn get_total_play_time_since_process_start(&self) -> u64 {
        let poe_start_time = self.get_poe_process_start_time().await;

        // If no POE process start time is set, return 0
        if poe_start_time.is_none() {
            return 0;
        }

        let poe_start = poe_start_time.unwrap();
        let completed = self.completed_sessions.read().await;
        let active = self.active_sessions.read().await;

        // Sum completed session durations that started after POE process start
        let completed_time: u64 = completed
            .iter()
            .filter(|session| session.entry_timestamp >= poe_start)
            .filter_map(|session| session.duration_seconds)
            .sum();

        // Sum active session durations that started after POE process start
        let active_time: u64 = active
            .values()
            .filter(|session| session.entry_timestamp >= poe_start)
            .map(|session| {
                let now = Utc::now();
                let duration = now.signed_duration_since(session.entry_timestamp);
                // Use milliseconds for more precision, then convert to seconds
                (duration.num_milliseconds().max(0) / 1000) as u64
            })
            .sum();

        completed_time + active_time
    }

    /// Calculate total time spent in hideout
    pub async fn get_total_hideout_time(&self) -> u64 {
        let completed = self.completed_sessions.read().await;
        let active = self.active_sessions.read().await;

        // Sum completed hideout session durations
        let completed_time: u64 = completed
            .iter()
            .filter(|session| session.location_type == LocationType::Hideout)
            .filter_map(|session| session.duration_seconds)
            .sum();

        // Add active hideout session time if any
        let active_time: u64 = active
            .values()
            .filter(|session| session.location_type == LocationType::Hideout)
            .map(|session| {
                let now = Utc::now();
                let duration = now.signed_duration_since(session.entry_timestamp);
                // Use milliseconds for more precision, then convert to seconds
                (duration.num_milliseconds().max(0) / 1000) as u64
            })
            .sum();

        completed_time + active_time
    }

    /// Load time tracking data from file
    fn load_data(&self) -> AppResult<()> {
        if !self.data_file_path.exists() {
            debug!("No time tracking data file found, starting fresh");
            return Ok(());
        }

        let content = fs::read_to_string(&self.data_file_path).map_err(|e| {
            AppError::FileSystem(format!("Failed to read time tracking data file: {}", e))
        })?;

        let data: TimeTrackingData = serde_json::from_str(&content).map_err(|e| {
            AppError::Serialization(format!("Failed to parse time tracking data: {}", e))
        })?;

        // Restore completed sessions
        {
            let mut completed = self.completed_sessions.blocking_write();
            *completed = data.completed_sessions;
        }

        // Restore stats cache
        {
            let mut stats_cache = self.stats_cache.blocking_write();
            for stats in data.stats {
                stats_cache.insert(stats.location_id.clone(), stats);
            }
        }

        debug!("Time tracking data loaded successfully");
        Ok(())
    }

    /// Clear all time tracking data
    pub async fn clear_all_data(&self) -> AppResult<()> {
        {
            let mut active = self.active_sessions.write().await;
            active.clear();
        }

        {
            let mut completed = self.completed_sessions.write().await;
            completed.clear();
        }

        {
            let mut stats = self.stats_cache.write().await;
            stats.clear();
        }

        // Remove data file
        if self.data_file_path.exists() {
            fs::remove_file(&self.data_file_path)
                .map_err(|e| AppError::FileSystem(format!("Failed to remove data file: {}", e)))?;
        }

        debug!("All time tracking data cleared");
        Ok(())
    }

    /// Handle application shutdown by ending all active sessions
    pub async fn shutdown(&self) -> AppResult<()> {
        debug!("Shutting down time tracking service, ending all active sessions");

        let active_sessions: Vec<String> = {
            let sessions = self.active_sessions.read().await;
            sessions.keys().cloned().collect()
        };

        for location_id in active_sessions {
            if let Err(e) = self.end_session(&location_id).await {
                warn!(
                    "Failed to end session during shutdown for {}: {}",
                    location_id, e
                );
            }
        }

        Ok(())
    }

    // Helper methods for internal use

    /// Get completed sessions synchronously (for internal use)
    fn get_completed_sessions_sync(&self) -> Vec<LocationSession> {
        self.completed_sessions.blocking_read().clone()
    }

    /// Get all stats synchronously (for internal use)
    fn get_all_stats_sync(&self) -> Vec<LocationStats> {
        self.stats_cache.blocking_read().values().cloned().collect()
    }
}

/// Internal data structure for persistence
#[derive(Debug, Serialize, Deserialize)]
struct TimeTrackingData {
    completed_sessions: Vec<LocationSession>,
    stats: Vec<LocationStats>,
}

impl std::fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationType::Zone => write!(f, "Zone"),
            LocationType::Act => write!(f, "Act"),
            LocationType::Hideout => write!(f, "Hideout"),
        }
    }
}
