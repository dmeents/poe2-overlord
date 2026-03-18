use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::error;
use std::sync::Arc;

use crate::domain::zone_tracking::is_hideout_zone;
use crate::errors::AppResult;
use crate::infrastructure::events::{AppEvent, EventBus};

use super::experience;
use super::models::{ActiveZoneInfo, LevelChartEvent, LevelEvent, LevelEventResponse, LevelingStats};
use super::traits::{LevelingRepository, LevelingService};

pub struct LevelingServiceImpl {
    repository: Arc<dyn LevelingRepository>,
    event_bus: Arc<EventBus>,
}

impl LevelingServiceImpl {
    pub fn new(
        repository: Arc<dyn LevelingRepository>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }

    /// Accumulates the time spent in the current active grinding zone into the DB counter.
    /// Skips towns and hideouts. No-ops if there is no active zone.
    async fn accumulate_active_zone_time(&self, character_id: &str) -> AppResult<()> {
        let Some(zone_info) = self.repository.get_active_zone_info(character_id).await? else {
            return Ok(());
        };

        if zone_info.is_town || is_hideout_zone(&zone_info.zone_name) {
            return Ok(());
        }

        let last_level_reached_at =
            self.repository.get_last_level_reached_at(character_id).await?;

        let elapsed = Self::grinding_elapsed_secs(&zone_info, last_level_reached_at);
        if elapsed > 0 {
            self.repository
                .increment_active_seconds_at_level(character_id, elapsed)
                .await?;
        }

        Ok(())
    }

    /// Computes the live grinding seconds for `zone_info` without writing to the DB.
    /// Uses `max(entry_timestamp, last_level_reached_at)` as the effective start to
    /// avoid counting pre-level-up time toward the new level.
    fn compute_dynamic_zone_seconds(
        zone_info: &ActiveZoneInfo,
        last_level_reached_at: Option<DateTime<Utc>>,
    ) -> u64 {
        if zone_info.is_town || is_hideout_zone(&zone_info.zone_name) {
            return 0;
        }
        Self::grinding_elapsed_secs(zone_info, last_level_reached_at)
    }

    /// Shared elapsed-seconds calculation used by both helpers above.
    fn grinding_elapsed_secs(
        zone_info: &ActiveZoneInfo,
        last_level_reached_at: Option<DateTime<Utc>>,
    ) -> u64 {
        let effective_start = match last_level_reached_at {
            Some(level_ts) => level_ts.max(zone_info.entry_timestamp),
            None => zone_info.entry_timestamp,
        };
        Utc::now()
            .signed_duration_since(effective_start)
            .num_seconds()
            .max(0) as u64
    }

    async fn compute_stats(&self, character_id: &str) -> AppResult<LevelingStats> {
        let current_level = self.repository.get_character_level(character_id).await?;
        let deaths_at_current_level =
            self.repository.get_deaths_at_current_level(character_id).await?;
        let recent_events_raw =
            self.repository.get_recent_level_events(character_id, 200).await?;
        let levels_gained_last_hour =
            self.repository.count_levels_in_last_minutes(character_id, 60).await?;

        let xp_to_next_level = if current_level < 100 {
            experience::xp_for_level(current_level)
        } else {
            0
        };

        let (xp_per_hour, last_level_reached_at) = compute_xp_per_hour(&recent_events_raw);

        // Persisted counter + live contribution from the current active zone
        let stored_active_seconds =
            self.repository.get_active_seconds_at_level(character_id).await?;
        let zone_info = self.repository.get_active_zone_info(character_id).await?;
        let is_actively_grinding = zone_info
            .as_ref()
            .map(|z| !z.is_town && !is_hideout_zone(&z.zone_name))
            .unwrap_or(false);
        let dynamic_zone_seconds = zone_info
            .as_ref()
            .map(|z| Self::compute_dynamic_zone_seconds(z, last_level_reached_at))
            .unwrap_or(0);
        let active_seconds_at_level = stored_active_seconds + dynamic_zone_seconds;

        let estimated_seconds_to_next_level = estimated_seconds(
            xp_per_hour,
            active_seconds_at_level,
            xp_to_next_level,
            current_level,
        );

        let recent_events = build_event_responses(&recent_events_raw);
        let chart_events = build_chart_events(&recent_events_raw);

        Ok(LevelingStats {
            character_id: character_id.to_string(),
            current_level,
            xp_per_hour,
            estimated_seconds_to_next_level,
            last_level_reached_at,
            levels_gained_last_hour,
            deaths_at_current_level,
            xp_to_next_level,
            recent_events,
            active_seconds_at_level,
            is_actively_grinding,
            chart_events,
        })
    }
}

#[async_trait]
impl LevelingService for LevelingServiceImpl {
    async fn record_level_up(
        &self,
        character_id: &str,
        _old_level: u32,
        new_level: u32,
    ) -> AppResult<()> {
        // Capture active zone time before resetting (final time in old-level zone)
        if let Err(e) = self.accumulate_active_zone_time(character_id).await {
            error!(
                "LEVELING: Failed to accumulate active zone time before level-up: {}",
                e
            );
        }

        // Capture death counter before reset
        let deaths = self.repository.get_deaths_at_current_level(character_id).await?;

        self.repository
            .insert_level_event(character_id, new_level, Utc::now(), deaths)
            .await?;

        self.repository.reset_deaths_at_current_level(character_id).await?;
        self.repository.reset_active_seconds_at_level(character_id).await?;

        let stats = self.compute_stats(character_id).await?;
        if let Err(e) = self
            .event_bus
            .publish(AppEvent::leveling_stats_updated(
                character_id.to_string(),
                stats,
            ))
            .await
        {
            error!("Failed to publish leveling stats event: {}", e);
        }

        Ok(())
    }

    async fn record_death(&self, character_id: &str) -> AppResult<()> {
        self.repository.increment_deaths_at_current_level(character_id).await?;

        let stats = self.compute_stats(character_id).await?;
        if let Err(e) = self
            .event_bus
            .publish(AppEvent::leveling_stats_updated(
                character_id.to_string(),
                stats,
            ))
            .await
        {
            error!("Failed to publish leveling stats event after death: {}", e);
        }

        Ok(())
    }

    async fn get_leveling_stats(&self, character_id: &str) -> AppResult<LevelingStats> {
        self.compute_stats(character_id).await
    }

    async fn record_active_zone_exit(&self, character_id: &str) -> AppResult<()> {
        self.accumulate_active_zone_time(character_id).await
    }

    async fn finalize_active_zone_times(&self) -> AppResult<()> {
        let character_ids = self.repository.get_all_active_zone_character_ids().await?;
        for character_id in character_ids {
            if let Err(e) = self.accumulate_active_zone_time(&character_id).await {
                error!(
                    "LEVELING: Failed to accumulate active zone time for {}: {}",
                    character_id, e
                );
            }
        }
        Ok(())
    }

    async fn emit_stats_update(&self, character_id: &str) -> AppResult<()> {
        let stats = self.compute_stats(character_id).await?;
        if let Err(e) = self
            .event_bus
            .publish(AppEvent::leveling_stats_updated(
                character_id.to_string(),
                stats,
            ))
            .await
        {
            error!("LEVELING: Failed to publish stats update after zone transition: {}", e);
        }
        Ok(())
    }
}

/// Computes a rolling XP/hr using up to 5 consecutive level-event pairs.
/// Returns (xp_per_hour, last_level_reached_at).
fn compute_xp_per_hour(
    events: &[LevelEvent],
) -> (Option<f64>, Option<chrono::DateTime<Utc>>) {
    let last_reached = events.first().map(|e| e.reached_at);

    if events.len() < 2 {
        return (None, last_reached);
    }

    let mut total_xp: u64 = 0;
    let mut total_seconds: i64 = 0;
    let pairs_to_use = (events.len() - 1).min(5);

    for i in 0..pairs_to_use {
        let newer = &events[i];
        let older = &events[i + 1];

        let time_diff = newer
            .reached_at
            .signed_duration_since(older.reached_at)
            .num_seconds();

        if time_diff <= 0 {
            continue;
        }

        let eff_xp = experience::effective_xp_earned(older.level, newer.deaths_at_level);
        total_xp += eff_xp;
        total_seconds += time_diff;
    }

    if total_seconds <= 0 {
        return (None, last_reached);
    }

    let xp_per_hour = total_xp as f64 / (total_seconds as f64 / 3600.0);
    (Some(xp_per_hour), last_reached)
}

/// Computes estimated seconds until next level-up using active grinding time instead of
/// wall-clock time. This avoids over-estimating progress earned in towns/hideouts.
fn estimated_seconds(
    xp_per_hour: Option<f64>,
    active_seconds_at_level: u64,
    xp_to_next_level: u64,
    current_level: u32,
) -> Option<u64> {
    if current_level >= 100 {
        return None;
    }
    let xp_hr = xp_per_hour?;

    if xp_hr <= 0.0 {
        return None;
    }

    let active_hours = active_seconds_at_level as f64 / 3600.0;
    let estimated_xp_earned = active_hours * xp_hr;
    let remaining_xp = (xp_to_next_level as f64 - estimated_xp_earned).max(0.0);
    let seconds_remaining = (remaining_xp / xp_hr * 3600.0) as u64;

    Some(seconds_remaining)
}

/// Builds the last 5 level event responses with per-transition stats.
fn build_event_responses(events: &[LevelEvent]) -> Vec<LevelEventResponse> {
    events
        .iter()
        .take(5)
        .enumerate()
        .map(|(i, event)| {
            let (time_from_previous_level_seconds, effective_xp, xp_per_hour) =
                if let Some(older) = events.get(i + 1) {
                    let time_diff = event
                        .reached_at
                        .signed_duration_since(older.reached_at)
                        .num_seconds();
                    if time_diff > 0 {
                        let eff_xp =
                            experience::effective_xp_earned(older.level, event.deaths_at_level);
                        let xp_hr = eff_xp as f64 / (time_diff as f64 / 3600.0);
                        (Some(time_diff as u64), Some(eff_xp), Some(xp_hr))
                    } else {
                        (None, None, None)
                    }
                } else {
                    (None, None, None)
                };

            LevelEventResponse {
                level: event.level,
                reached_at: event.reached_at,
                deaths_at_level: event.deaths_at_level,
                time_from_previous_level_seconds,
                effective_xp_earned: effective_xp,
                xp_per_hour,
            }
        })
        .collect()
}

/// Builds chart data points for every recorded level transition, sorted ascending by level.
/// Events from the DB arrive DESC (most recent first), so we iterate in reverse.
fn build_chart_events(events: &[LevelEvent]) -> Vec<LevelChartEvent> {
    // events[0] = highest level (most recent), events[last] = lowest level (oldest)
    // Iterate in reverse to go from lowest → highest level.
    events
        .iter()
        .rev()
        .enumerate()
        .map(|(i, event)| {
            // The "older" event is the one that came just before this one in ascending order,
            // which is events[events.len() - 1 - i - 1] = events[events.len() - 2 - i].
            // In the reversed iteration, `i` is the index within the reversed slice, so
            // the "previous" (lower-level) event is at reversed index i+1.
            let asc_index = events.len() - 1 - i;
            let xp_per_hour = if asc_index + 1 < events.len() {
                let older = &events[asc_index + 1];
                let time_diff = event
                    .reached_at
                    .signed_duration_since(older.reached_at)
                    .num_seconds();
                if time_diff > 0 {
                    let eff_xp =
                        experience::effective_xp_earned(older.level, event.deaths_at_level);
                    Some(eff_xp as f64 / (time_diff as f64 / 3600.0))
                } else {
                    None
                }
            } else {
                None
            };

            LevelChartEvent {
                level: event.level,
                xp_per_hour,
                deaths_at_level: event.deaths_at_level,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_event(level: u32, minutes_ago: i64, deaths: u32) -> LevelEvent {
        LevelEvent {
            id: level as i64,
            character_id: "test".to_string(),
            level,
            reached_at: Utc::now() - chrono::Duration::minutes(minutes_ago),
            deaths_at_level: deaths,
        }
    }

    #[test]
    fn xp_per_hour_needs_at_least_two_events() {
        let single = vec![make_event(10, 5, 0)];
        let (xp_hr, _) = compute_xp_per_hour(&single);
        assert!(xp_hr.is_none());
    }

    #[test]
    fn xp_per_hour_with_two_events_is_positive() {
        // level 9 -> level 10, 60 minutes apart, no deaths
        let events = vec![make_event(10, 0, 0), make_event(9, 60, 0)];
        let (xp_hr, _) = compute_xp_per_hour(&events);
        assert!(xp_hr.is_some());
        assert!(xp_hr.unwrap() > 0.0);
    }

    #[test]
    fn xp_per_hour_with_deaths_at_68_is_higher() {
        // Deaths are stored on the NEWER event (deaths while grinding to that level).
        // Both took 60 minutes from level 68 to 69, but the second had 2 deaths during that time.
        let no_deaths = vec![make_event(69, 0, 0), make_event(68, 60, 0)];
        let with_deaths = vec![make_event(69, 0, 2), make_event(68, 60, 0)];

        let (xp_hr_clean, _) = compute_xp_per_hour(&no_deaths);
        let (xp_hr_deaths, _) = compute_xp_per_hour(&with_deaths);

        assert!(xp_hr_deaths.unwrap() > xp_hr_clean.unwrap());
    }

    #[test]
    fn xp_per_hour_uses_at_most_five_pairs() {
        // 10 events → 9 pairs available, should use 5
        let events: Vec<LevelEvent> = (0..10)
            .map(|i| make_event(10 - i, (i as i64) * 30, 0))
            .collect();
        let (xp_hr, _) = compute_xp_per_hour(&events);
        assert!(xp_hr.is_some());
    }

    #[test]
    fn estimated_seconds_returns_none_without_xp_rate() {
        let result = estimated_seconds(None, 3600, 1_000_000, 50);
        assert!(result.is_none());
    }

    #[test]
    fn estimated_seconds_at_level_100_returns_none() {
        let result = estimated_seconds(Some(1_000_000.0), 0, 0, 100);
        assert!(result.is_none());
    }

    #[test]
    fn estimated_seconds_is_positive() {
        // 1M XP/hr rate, 1M XP needed, 0 active seconds elapsed → full hour remaining
        let result = estimated_seconds(Some(1_000_000.0), 0, 1_000_000, 50);
        assert!(result.is_some());
        // Should be approximately 3600 seconds (1 hour)
        let secs = result.unwrap();
        assert!(secs > 3500 && secs <= 3600);
    }

    #[test]
    fn estimated_seconds_decreases_with_active_time() {
        // 1M XP/hr, 1M XP needed, 30 min (1800s) of active grinding → ~30 min remaining
        let result = estimated_seconds(Some(1_000_000.0), 1800, 1_000_000, 50);
        assert!(result.is_some());
        let secs = result.unwrap();
        assert!(secs > 1700 && secs <= 1800);
    }

    #[test]
    fn death_penalty_only_applied_at_68_plus() {
        // Level 67: no penalty
        let no_penalty = experience::effective_xp_earned(67, 5);
        assert_eq!(no_penalty, experience::xp_for_level(67));

        // Level 68: penalty applies
        let with_penalty = experience::effective_xp_earned(68, 1);
        assert!(with_penalty > experience::xp_for_level(68));
    }
}
