use crate::domain::time_tracking::models::{
    CharacterTimeTrackingData, LocationSession, LocationStats, LocationType,
};
use crate::domain::time_tracking::traits::TimeTrackingRepository;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::persistence::{
    ScopedPersistenceRepository, ScopedPersistenceRepositoryImpl,
};
use crate::infrastructure::time::calculations::validate_no_session_overlap;
use async_trait::async_trait;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// File prefix for time tracking data files
const TIME_TRACKING_FILE_PREFIX: &str = "time_tracking_";
/// File suffix for time tracking data files
const TIME_TRACKING_FILE_SUFFIX: &str = ".json";

/// Implementation of time tracking repository with in-memory caching and persistent storage
#[derive(Clone)]
pub struct TimeTrackingRepositoryImpl {
    /// In-memory cache of active sessions: character_id -> location_id -> session
    active_sessions: Arc<RwLock<HashMap<String, HashMap<String, LocationSession>>>>,
    /// In-memory cache of completed sessions: character_id -> sessions
    completed_sessions: Arc<RwLock<HashMap<String, Vec<LocationSession>>>>,
    /// In-memory cache of location statistics: character_id -> location_id -> stats
    stats_cache: Arc<RwLock<HashMap<String, HashMap<String, LocationStats>>>>,
    /// Persistent storage for character time tracking data
    persistence: ScopedPersistenceRepositoryImpl<CharacterTimeTrackingData, String>,
}

impl TimeTrackingRepositoryImpl {
    /// Creates a new time tracking repository with persistent storage
    pub fn new() -> AppResult<Self> {
        let persistence =
            ScopedPersistenceRepositoryImpl::<CharacterTimeTrackingData, String>::new_in_data_dir(
                TIME_TRACKING_FILE_PREFIX,
                TIME_TRACKING_FILE_SUFFIX,
            )?;

        // Initialize repository with empty in-memory caches
        let repository = Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            completed_sessions: Arc::new(RwLock::new(HashMap::new())),
            stats_cache: Arc::new(RwLock::new(HashMap::new())),
            persistence,
        };

        Ok(repository)
    }
}

#[async_trait]
impl TimeTrackingRepository for TimeTrackingRepositoryImpl {
    /// Saves character time tracking data to persistent storage
    async fn save_character_data(&self, data: &CharacterTimeTrackingData) -> AppResult<()> {
        self.persistence.save_scoped(&data.character_id, data).await
    }

    /// Loads character time tracking data from persistent storage
    async fn load_character_data(
        &self,
        character_id: &str,
    ) -> AppResult<Option<CharacterTimeTrackingData>> {
        self.persistence
            .load_scoped(&character_id.to_string())
            .await
    }

    /// Deletes all time tracking data for a character from both storage and memory
    async fn delete_character_data(&self, character_id: &str) -> AppResult<()> {
        // Delete from persistent storage
        self.persistence
            .delete_scoped(&character_id.to_string())
            .await?;

        // Clear from in-memory caches
        {
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.remove(character_id);
        }
        {
            let mut completed_sessions = self.completed_sessions.write().await;
            completed_sessions.remove(character_id);
        }
        {
            let mut stats_cache = self.stats_cache.write().await;
            stats_cache.remove(character_id);
        }

        debug!("Time tracking data deleted for character: {}", character_id);
        Ok(())
    }

    /// Checks if time tracking data exists for a character in persistent storage
    async fn character_data_exists(&self, character_id: &str) -> AppResult<bool> {
        self.persistence
            .exists_scoped(&character_id.to_string())
            .await
    }

    /// Gets all active sessions for a character from in-memory cache
    async fn get_active_sessions(
        &self,
        character_id: &str,
    ) -> AppResult<HashMap<String, LocationSession>> {
        let active_sessions = self.active_sessions.read().await;
        Ok(active_sessions
            .get(character_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Gets all completed sessions for a character from in-memory cache
    async fn get_completed_sessions(&self, character_id: &str) -> AppResult<Vec<LocationSession>> {
        let completed_sessions = self.completed_sessions.read().await;
        Ok(completed_sessions
            .get(character_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Gets location statistics cache for a character from in-memory cache
    async fn get_stats_cache(
        &self,
        character_id: &str,
    ) -> AppResult<HashMap<String, LocationStats>> {
        let stats_cache = self.stats_cache.read().await;
        Ok(stats_cache.get(character_id).cloned().unwrap_or_default())
    }

    /// Finds an active session by character and location ID
    async fn find_session_by_location(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationSession>> {
        let active_sessions = self.active_sessions.read().await;
        if let Some(character_sessions) = active_sessions.get(character_id) {
            Ok(character_sessions.get(location_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Gets the most recent location for a character (from active or completed sessions)
    async fn get_last_known_location(
        &self,
        character_id: &str,
    ) -> AppResult<Option<LocationSession>> {
        // Combine completed and active sessions
        let mut sessions = self.get_completed_sessions(character_id).await?;
        let active_sessions = self.get_active_sessions(character_id).await?;
        sessions.extend(active_sessions.into_values());

        // Sort by entry timestamp (most recent first)
        sessions.sort_by(|a, b| b.entry_timestamp.cmp(&a.entry_timestamp));

        Ok(sessions.first().cloned())
    }

    /// Gets statistics for a specific location from the cache
    async fn get_location_stats(
        &self,
        character_id: &str,
        location_id: &str,
    ) -> AppResult<Option<LocationStats>> {
        let stats_cache = self.stats_cache.read().await;
        if let Some(character_stats) = stats_cache.get(character_id) {
            Ok(character_stats.get(location_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Starts a new session and adds it to the active sessions cache
    async fn start_session(&self, character_id: &str, session: LocationSession) -> AppResult<()> {
        // Validate no overlapping sessions
        self.validate_no_overlapping_sessions(character_id, &session)
            .await?;

        // Add to active sessions cache
        let location_id = session.location_id.clone();
        let mut active_sessions = self.active_sessions.write().await;
        active_sessions
            .entry(character_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(location_id.clone(), session);

        debug!(
            "Started session for character {} at location {}",
            character_id, location_id
        );
        Ok(())
    }

    /// Ends a session and moves it from active to completed sessions
    async fn end_session(&self, character_id: &str, location_id: &str) -> AppResult<()> {
        let mut active_sessions = self.active_sessions.write().await;
        let mut completed_sessions = self.completed_sessions.write().await;

        // Remove from active sessions and add to completed sessions
        if let Some(character_sessions) = active_sessions.get_mut(character_id) {
            if let Some(mut session) = character_sessions.remove(location_id) {
                // Calculate final duration
                session.end_session();
                completed_sessions
                    .entry(character_id.to_string())
                    .or_insert_with(Vec::new)
                    .push(session);

                debug!(
                    "Ended session for character {} at location {}",
                    character_id, location_id
                );
                return Ok(());
            }
        }

        Err(AppError::time_tracking_error(
            "end_session",
            &format!(
                "No active session found for character {} at location {}",
                character_id, location_id
            ),
        ))
    }

    /// Updates location statistics in the cache
    async fn update_stats(
        &self,
        character_id: &str,
        location_id: &str,
        stats: LocationStats,
    ) -> AppResult<()> {
        let mut stats_cache = self.stats_cache.write().await;
        stats_cache
            .entry(character_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(location_id.to_string(), stats);

        debug!(
            "Updated stats for character {} at location {}",
            character_id, location_id
        );
        Ok(())
    }

    /// Calculates total play time from all completed sessions
    async fn calculate_total_play_time(&self, character_id: &str) -> AppResult<u64> {
        let completed_sessions = self.get_completed_sessions(character_id).await?;
        Ok(completed_sessions
            .iter()
            .filter_map(|session| session.duration_seconds)
            .sum())
    }

    /// Calculates total time spent in hideouts from completed sessions
    async fn calculate_total_hideout_time(&self, character_id: &str) -> AppResult<u64> {
        let completed_sessions = self.get_completed_sessions(character_id).await?;
        Ok(completed_sessions
            .iter()
            .filter(|session| session.location_type == LocationType::Hideout)
            .filter_map(|session| session.duration_seconds)
            .sum())
    }

    /// Gets top locations by time spent, limited to specified count
    async fn get_top_locations(
        &self,
        character_id: &str,
        limit: usize,
    ) -> AppResult<Vec<LocationStats>> {
        let stats_cache = self.get_stats_cache(character_id).await?;
        let mut stats: Vec<LocationStats> = stats_cache.into_values().collect();

        // Sort by total time spent (descending)
        stats.sort_by(|a, b| b.total_time_seconds.cmp(&a.total_time_seconds));

        // Limit to requested count
        stats.truncate(limit);
        Ok(stats)
    }

    /// Validates that a new session doesn't overlap with existing active sessions
    async fn validate_no_overlapping_sessions(
        &self,
        character_id: &str,
        new_session: &LocationSession,
    ) -> AppResult<()> {
        let active_sessions = self.get_active_sessions(character_id).await?;
        // Convert active sessions to timestamp tuples for validation
        let existing_sessions: Vec<(
            chrono::DateTime<chrono::Utc>,
            Option<chrono::DateTime<chrono::Utc>>,
        )> = active_sessions
            .into_values()
            .map(|session| (session.entry_timestamp, session.exit_timestamp))
            .collect();

        // Use infrastructure validation function
        match validate_no_session_overlap(
            new_session.entry_timestamp,
            new_session.exit_timestamp,
            &existing_sessions,
        ) {
            crate::infrastructure::time::calculations::ValidationResult::Valid => Ok(()),
            crate::infrastructure::time::calculations::ValidationResult::Error(error) => {
                Err(AppError::validation_error(
                    "validate_session_overlap",
                    &format!("Session overlap detected: {}", error),
                ))
            }
            crate::infrastructure::time::calculations::ValidationResult::Warning(warning) => {
                debug!("Session overlap warning: {}", warning);
                Ok(())
            }
        }
    }
}
