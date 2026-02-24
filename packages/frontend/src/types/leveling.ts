/** Per-level data point for the leveling history chart. */
export interface LevelChartEvent {
  level: number;
  /** XP/hr for this level transition. Null for the first tracked event. */
  xp_per_hour: number | null;
  /** Deaths that occurred while grinding to this level. */
  deaths_at_level: number;
}

/** Per-level-up event data for display in the history list. */
export interface LevelEventResponse {
  level: number;
  /** ISO 8601 timestamp of when this level was reached. */
  reached_at: string;
  /** Deaths that occurred while grinding to this level. */
  deaths_at_level: number;
  /** Elapsed seconds between the previous level-up and this one. */
  time_from_previous_level_seconds: number | null;
  /** Effective XP earned (base + death re-grinds) to complete this level. */
  effective_xp_earned: number | null;
  /** XP per hour implied by this single level transition. */
  xp_per_hour: number | null;
}

/** Computed leveling statistics for a character. */
export interface LevelingStats {
  character_id: string;
  current_level: number;
  /** Rolling XP/hr over recent level transitions. Null if < 2 data points. */
  xp_per_hour: number | null;
  /** Estimated seconds until the next level-up. Null if no XP/hr. */
  estimated_seconds_to_next_level: number | null;
  /** ISO 8601 timestamp of when the current level was reached. */
  last_level_reached_at: string | null;
  /** Number of levels gained in the last 60 minutes. */
  levels_gained_last_hour: number;
  /** Deaths since the current level was reached. */
  deaths_at_current_level: number;
  /** Raw XP needed to go from current level to next level. */
  xp_to_next_level: number;
  /** Last 5 level events, most recent first. */
  recent_events: LevelEventResponse[];
  /** Accumulated active grinding seconds at the current level (persisted + live segment). */
  active_seconds_at_level: number;
  /** True when the player is in a non-town, non-hideout zone (timer should tick live). */
  is_actively_grinding: boolean;
  /** All level events sorted ascending by level, for the chart. */
  chart_events: LevelChartEvent[];
}
