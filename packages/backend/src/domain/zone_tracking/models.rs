use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// String used to identify hideout zones
pub const HIDEOUT_KEYWORD: &str = "hideout";

/// Act number assigned to hideout zones to separate them from story act playtimes
pub const HIDEOUT_ACT: u32 = 10;

/// Checks if a zone name represents a hideout zone (case-insensitive)
///
/// This is the centralized hideout detection logic to avoid duplication.
/// Use this function wherever you need to check if a zone name is a hideout.
pub fn is_hideout_zone(zone_name: &str) -> bool {
    zone_name.to_lowercase().contains(HIDEOUT_KEYWORD)
}

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
    #[serde(default)]
    pub act: Option<u32>,
    #[serde(default)]
    pub is_town: bool,
}

impl ZoneStats {
    pub fn new(zone_name: String, act: Option<u32>, is_town: bool) -> Self {
        let now = Utc::now();
        Self {
            zone_name,
            duration: 0,
            deaths: 0,
            visits: 0, // Initialize to 0; activate() will set it to 1
            first_visited: now,
            last_visited: now,
            is_active: false,
            entry_timestamp: None,
            act,
            is_town,
        }
    }

    pub fn is_hideout(&self) -> bool {
        is_hideout_zone(&self.zone_name)
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
    pub total_town_time: u64,
    pub total_zones_visited: u32,
    pub total_deaths: u32,
    pub play_time_act1: u64,
    pub play_time_act2: u64,
    pub play_time_act3: u64,
    pub play_time_act4: u64,
    #[serde(default)]
    pub play_time_act5: u64,
    pub play_time_interlude: u64,
    pub play_time_endgame: u64,
}

impl TrackingSummary {
    pub fn new(character_id: String) -> Self {
        Self {
            character_id,
            total_play_time: 0,
            total_hideout_time: 0,
            total_town_time: 0,
            total_zones_visited: 0,
            total_deaths: 0,
            play_time_act1: 0,
            play_time_act2: 0,
            play_time_act3: 0,
            play_time_act4: 0,
            play_time_act5: 0,
            play_time_interlude: 0,
            play_time_endgame: 0,
        }
    }

    pub fn from_zones(character_id: &str, zones: &[ZoneStats]) -> Self {
        let mut summary = Self::new(character_id.to_string());

        for zone in zones {
            // Use get_current_time_spent() to include active timer
            let zone_time = zone.get_current_time_spent();

            summary.total_play_time += zone_time;
            summary.total_deaths += zone.deaths;

            // Calculate hideout time
            if zone.is_hideout() {
                summary.total_hideout_time += zone_time;
            }

            // Calculate town time (excluding hideouts)
            if zone.is_town && !zone.is_hideout() {
                summary.total_town_time += zone_time;
            }

            // Calculate act-specific time (only for zones with known act)
            if let Some(act) = zone.act {
                match act {
                    1 => summary.play_time_act1 += zone_time,
                    2 => summary.play_time_act2 += zone_time,
                    3 => summary.play_time_act3 += zone_time,
                    4 => summary.play_time_act4 += zone_time,
                    5 => summary.play_time_act5 += zone_time,
                    6 => summary.play_time_interlude += zone_time,
                    10 => summary.play_time_endgame += zone_time,
                    _ => {
                        // Unknown act (0, 7-9, 11+)
                        // Only counts in total_play_time, not act-specific
                    }
                }
            }
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
            5 => self.play_time_act5,
            _ => 0,
        }
    }

    pub fn get_total_story_time(&self) -> u64 {
        self.play_time_act1
            + self.play_time_act2
            + self.play_time_act3
            + self.play_time_act4
            + self.play_time_act5
            + self.play_time_interlude
    }

    pub fn get_act_breakdown(&self) -> Vec<(String, u64)> {
        vec![
            ("Act 1".to_string(), self.play_time_act1),
            ("Act 2".to_string(), self.play_time_act2),
            ("Act 3".to_string(), self.play_time_act3),
            ("Act 4".to_string(), self.play_time_act4),
            ("Act 5".to_string(), self.play_time_act5),
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
