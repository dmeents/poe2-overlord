use crate::domain::time_tracking::models::{LocationSession, LocationStats};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Event fired when a new location session is started
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStarted {
    /// The session that was started
    pub session: LocationSession,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl SessionStarted {
    /// Creates a new session started event with current timestamp
    pub fn new(session: LocationSession) -> Self {
        Self {
            session,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Event fired when a location session is ended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEnded {
    /// The session that was ended
    pub session: LocationSession,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl SessionEnded {
    /// Creates a new session ended event with current timestamp
    pub fn new(session: LocationSession) -> Self {
        Self {
            session,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Event fired when location statistics are updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsUpdated {
    /// The updated location statistics
    pub stats: LocationStats,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl StatsUpdated {
    /// Creates a new stats updated event with current timestamp
    pub fn new(stats: LocationStats) -> Self {
        Self {
            stats,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Event fired when all time tracking data is cleared for a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataCleared {
    /// Character whose data was cleared
    pub character_id: String,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataCleared {
    /// Creates a new data cleared event with current timestamp
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Event fired when time tracking data is loaded from storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataLoaded {
    /// Character whose data was loaded
    pub character_id: String,
    /// Number of completed sessions loaded
    pub completed_sessions_count: usize,
    /// Number of location stats loaded
    pub location_stats_count: usize,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataLoaded {
    /// Creates a new data loaded event with current timestamp
    pub fn new(
        character_id: String,
        completed_sessions_count: usize,
        location_stats_count: usize,
    ) -> Self {
        Self {
            character_id,
            completed_sessions_count,
            location_stats_count,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Event fired when time tracking data is saved to storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataSaved {
    /// Character whose data was saved
    pub character_id: String,
    /// Number of completed sessions saved
    pub completed_sessions_count: usize,
    /// Number of location stats saved
    pub location_stats_count: usize,
    /// When this event occurred
    pub occurred_at: SystemTime,
}

impl TimeTrackingDataSaved {
    /// Creates a new data saved event with current timestamp
    pub fn new(
        character_id: String,
        completed_sessions_count: usize,
        location_stats_count: usize,
    ) -> Self {
        Self {
            character_id,
            completed_sessions_count,
            location_stats_count,
            occurred_at: SystemTime::now(),
        }
    }
}

/// Enumeration of all possible time tracking events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTrackingEvent {
    /// A new location session was started
    SessionStarted(SessionStarted),
    /// A location session was ended
    SessionEnded(SessionEnded),
    /// Location statistics were updated
    StatsUpdated(StatsUpdated),
    /// All time tracking data was cleared for a character
    TimeTrackingDataCleared(TimeTrackingDataCleared),
    /// Time tracking data was loaded from storage
    TimeTrackingDataLoaded(TimeTrackingDataLoaded),
    /// Time tracking data was saved to storage
    TimeTrackingDataSaved(TimeTrackingDataSaved),
}

impl TimeTrackingEvent {
    /// Extracts the character ID from any time tracking event
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

    /// Gets the timestamp when the event occurred
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

    /// Returns true if this event represents a session state change (start/end)
    pub fn is_session_state_change(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::SessionStarted(_) | TimeTrackingEvent::SessionEnded(_)
        )
    }

    /// Returns true if this event represents a persistence operation (load/save)
    pub fn is_persistence_event(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::TimeTrackingDataLoaded(_)
                | TimeTrackingEvent::TimeTrackingDataSaved(_)
        )
    }
}
