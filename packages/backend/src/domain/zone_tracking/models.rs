use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Statistics for a specific zone visited by a character
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZoneStats {
    pub zone_name: String,
    pub duration: u64,
    pub deaths: u32,
    pub visits: u32,
    pub first_visited: DateTime<Utc>,
    pub last_visited: DateTime<Utc>,
    pub is_active: bool,
    pub entry_timestamp: Option<DateTime<Utc>>,
}

impl ZoneStats {
    pub fn new(zone_name: String) -> Self {
        let now = Utc::now();
        Self {
            zone_name,
            duration: 0,
            deaths: 0,
            visits: 1,
            first_visited: now,
            last_visited: now,
            is_active: false,
            entry_timestamp: None,
        }
    }

    pub fn add_time(&mut self, seconds: u64) {
        self.duration += seconds;
        self.last_visited = Utc::now();
    }

    pub fn record_death(&mut self) {
        self.deaths += 1;
        self.last_visited = Utc::now();
    }

    pub fn record_visit(&mut self) {
        self.visits += 1;
        self.last_visited = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.record_visit();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn start_timer(&mut self) {
        self.entry_timestamp = Some(Utc::now());
    }

    pub fn stop_timer_and_add_time(&mut self) -> u64 {
        if let Some(entry_time) = self.entry_timestamp {
            let now = Utc::now();
            let elapsed = (now - entry_time).num_seconds().max(0) as u64;
            self.add_time(elapsed);
            self.entry_timestamp = None;
            elapsed
        } else {
            0
        }
    }

    pub fn get_current_time_spent(&self) -> u64 {
        if let Some(entry_time) = self.entry_timestamp {
            let now = Utc::now();
            let elapsed = (now - entry_time).num_seconds().max(0) as u64;
            self.duration + elapsed
        } else {
            self.duration
        }
    }
}

/// Summary statistics aggregated from all zone visits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrackingSummary {
    pub character_id: String,
    pub total_play_time: u64,
    pub total_hideout_time: u64,
    pub total_zones_visited: u32,
    pub total_deaths: u32,
    pub play_time_act1: u64,
    pub play_time_act2: u64,
    pub play_time_act3: u64,
    pub play_time_act4: u64,
    pub play_time_interlude: u64,
    pub play_time_endgame: u64,
}

impl TrackingSummary {
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            total_play_time: 0,
            total_hideout_time: 0,
            total_zones_visited: 0,
            total_deaths: 0,
            play_time_act1: 0,
            play_time_act2: 0,
            play_time_act3: 0,
            play_time_act4: 0,
            play_time_interlude: 0,
            play_time_endgame: 0,
        }
    }

    pub fn from_zones(character_id: &str, zones: &[ZoneStats]) -> Self {
        let mut summary = Self::new(character_id.to_string());

        for zone in zones {
            summary.total_play_time += zone.duration;
            summary.total_deaths += zone.deaths;
        }

        summary.total_zones_visited = zones.len() as u32;
        summary
    }

    pub fn get_act_time(&self, act: u32) -> u64 {
        match act {
            1 => self.play_time_act1,
            2 => self.play_time_act2,
            3 => self.play_time_act3,
            4 => self.play_time_act4,
            _ => 0,
        }
    }

    pub fn get_total_story_time(&self) -> u64 {
        self.play_time_act1
            + self.play_time_act2
            + self.play_time_act3
            + self.play_time_act4
            + self.play_time_interlude
    }

    pub fn get_act_breakdown(&self) -> Vec<(String, u64)> {
        vec![
            ("Act 1".to_string(), self.play_time_act1),
            ("Act 2".to_string(), self.play_time_act2),
            ("Act 3".to_string(), self.play_time_act3),
            ("Act 4".to_string(), self.play_time_act4),
            ("Interlude".to_string(), self.play_time_interlude),
            ("Endgame".to_string(), self.play_time_endgame),
        ]
    }

    pub fn get_longest_act(&self) -> Option<(String, u64)> {
        self.get_act_breakdown()
            .into_iter()
            .max_by_key(|(_, time)| *time)
    }
}
