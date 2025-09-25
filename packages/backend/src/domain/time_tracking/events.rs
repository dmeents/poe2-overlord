use crate::domain::time_tracking::models::{LocationSession, LocationStats};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Domain event indicating that a time tracking session has started
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStarted {
    /// The session that was started
    pub session: LocationSession,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl SessionStarted {
    pub fn new(session: LocationSession) -> Self {
        Self {
            session,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that a time tracking session has ended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEnded {
    /// The session that was ended
    pub session: LocationSession,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl SessionEnded {
    pub fn new(session: LocationSession) -> Self {
        Self {
            session,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that location statistics have been updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsUpdated {
    /// The updated location statistics
    pub stats: LocationStats,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl StatsUpdated {
    pub fn new(stats: LocationStats) -> Self {
        Self {
            stats,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that all time tracking data for a character has been cleared
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataCleared {
    /// The character ID whose data was cleared
    pub character_id: String,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataCleared {
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that time tracking data has been loaded for a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataLoaded {
    /// The character ID whose data was loaded
    pub character_id: String,
    /// Number of completed sessions loaded
    pub completed_sessions_count: usize,
    /// Number of location stats loaded
    pub location_stats_count: usize,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataLoaded {
    pub fn new(character_id: String, completed_sessions_count: usize, location_stats_count: usize) -> Self {
        Self {
            character_id,
            completed_sessions_count,
            location_stats_count,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Domain event indicating that time tracking data has been saved for a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataSaved {
    /// The character ID whose data was saved
    pub character_id: String,
    /// Number of completed sessions saved
    pub completed_sessions_count: usize,
    /// Number of location stats saved
    pub location_stats_count: usize,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataSaved {
    pub fn new(character_id: String, completed_sessions_count: usize, location_stats_count: usize) -> Self {
        Self {
            character_id,
            completed_sessions_count,
            location_stats_count,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Enum representing all possible time tracking domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTrackingEvent {
    SessionStarted(SessionStarted),
    SessionEnded(SessionEnded),
    StatsUpdated(StatsUpdated),
    TimeTrackingDataCleared(TimeTrackingDataCleared),
    TimeTrackingDataLoaded(TimeTrackingDataLoaded),
    TimeTrackingDataSaved(TimeTrackingDataSaved),
}

impl TimeTrackingEvent {
    /// Get the character ID from any time tracking event
    pub fn character_id(&self) -> &str {
        match self {
            TimeTrackingEvent::SessionStarted(event) => &event.session.character_id,
            TimeTrackingEvent::SessionEnded(event) => &event.session.character_id,
            TimeTrackingEvent::StatsUpdated(event) => &event.stats.character_id,
            TimeTrackingEvent::TimeTrackingDataCleared(event) => &event.character_id,
            TimeTrackingEvent::TimeTrackingDataLoaded(event) => &event.character_id,
            TimeTrackingEvent::TimeTrackingDataSaved(event) => &event.character_id,
        }
    }

    /// Get the timestamp when this event occurred
    pub fn occurred_at(&self) -> SystemTime {
        match self {
            TimeTrackingEvent::SessionStarted(event) => event.occurred_at,
            TimeTrackingEvent::SessionEnded(event) => event.occurred_at,
            TimeTrackingEvent::StatsUpdated(event) => event.occurred_at,
            TimeTrackingEvent::TimeTrackingDataCleared(event) => event.occurred_at,
            TimeTrackingEvent::TimeTrackingDataLoaded(event) => event.occurred_at,
            TimeTrackingEvent::TimeTrackingDataSaved(event) => event.occurred_at,
        }
    }

    /// Check if this event represents a session state change
    pub fn is_session_state_change(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::SessionStarted(_) | TimeTrackingEvent::SessionEnded(_)
        )
    }

    /// Check if this event represents a data persistence operation
    pub fn is_persistence_event(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::TimeTrackingDataLoaded(_) | TimeTrackingEvent::TimeTrackingDataSaved(_)
        )
    }
}
