use crate::domain::time_tracking::models::{LocationSession, LocationStats};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStarted {
    pub session: LocationSession,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEnded {
    pub session: LocationSession,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsUpdated {
    pub stats: LocationStats,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataCleared {
    pub character_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataLoaded {
    pub character_id: String,
    pub completed_sessions_count: usize,
    pub location_stats_count: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrackingDataSaved {
    pub character_id: String,
    pub completed_sessions_count: usize,
    pub location_stats_count: usize,
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

    pub fn is_session_state_change(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::SessionStarted(_) | TimeTrackingEvent::SessionEnded(_)
        )
    }

    pub fn is_persistence_event(&self) -> bool {
        matches!(
            self,
            TimeTrackingEvent::TimeTrackingDataLoaded(_) | TimeTrackingEvent::TimeTrackingDataSaved(_)
        )
    }
}
