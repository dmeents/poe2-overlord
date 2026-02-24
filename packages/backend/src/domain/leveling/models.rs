use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Active zone data queried for grinding-time accumulation.
#[derive(Debug, Clone)]
pub struct ActiveZoneInfo {
    pub zone_name: String,
    pub entry_timestamp: DateTime<Utc>,
    pub is_town: bool,
}

/// Raw database row for a level-up event.
#[derive(Debug, Clone)]
pub struct LevelEvent {
    pub id: i64,
    pub character_id: String,
    pub level: u32,
    pub reached_at: DateTime<Utc>,
    /// Deaths that occurred while grinding FROM the previous level TO this level.
    pub deaths_at_level: u32,
}

/// Per-event metadata computed for frontend display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelEventResponse {
    pub level: u32,
    pub reached_at: DateTime<Utc>,
    pub deaths_at_level: u32,
    /// Seconds elapsed between the previous level-up and this one.
    pub time_from_previous_level_seconds: Option<u64>,
    /// Effective XP earned (including death re-grinds) to complete this level.
    pub effective_xp_earned: Option<u64>,
    /// XP per hour implied by this single level transition.
    pub xp_per_hour: Option<f64>,
}

/// Computed leveling statistics returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelingStats {
    pub character_id: String,
    pub current_level: u32,
    /// Rolling XP/hr over the last up to 5 level transitions. None if < 2 events.
    pub xp_per_hour: Option<f64>,
    /// Estimated seconds remaining until the next level-up. None if no XP/hr.
    pub estimated_seconds_to_next_level: Option<u64>,
    /// When the current level was reached.
    pub last_level_reached_at: Option<DateTime<Utc>>,
    /// Number of levels gained in the last 60 minutes.
    pub levels_gained_last_hour: u32,
    /// Deaths since the current level was reached (resets on level-up).
    pub deaths_at_current_level: u32,
    /// Raw XP needed to go from current level to next level.
    pub xp_to_next_level: u64,
    /// Last 5 level events, most recent first.
    pub recent_events: Vec<LevelEventResponse>,
    /// Accumulated active grinding seconds at the current level (persisted + live segment).
    pub active_seconds_at_level: u64,
    /// True when the player is currently in a non-town, non-hideout zone (timer should tick).
    pub is_actively_grinding: bool,
}
