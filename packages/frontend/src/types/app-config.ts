export type BackgroundImage = 'None' | 'VolcanicRuins';

export interface BackgroundImageOption {
  value: string;
  label: string;
  filename: string | null;
}

export type ZoneRefreshInterval =
  | 'FiveMinutes'
  | 'OneHour'
  | 'TwelveHours'
  | 'TwentyFourHours'
  | 'ThreeDays'
  | 'SevenDays';

export interface ZoneRefreshIntervalOption {
  value: string;
  label: string;
  /** Interval duration in seconds. Maps to Rust i64, safe for time intervals. */
  seconds: number;
}

export interface AppConfig {
  /** Configuration schema version for migration compatibility */
  config_version: number;
  poe_client_log_path: string;
  log_level: string;
  zone_refresh_interval: ZoneRefreshInterval;
  hide_optional_objectives: boolean;
  hide_league_start_objectives: boolean;
  hide_flavor_text: boolean;
  hide_objective_descriptions: boolean;
  ui_zoom_level: number;
  background_image: BackgroundImage;
}

/** Event emitted when configuration changes (from backend). */
export interface ConfigurationChangedEvent {
  new_config: AppConfig;
  previous_config: AppConfig;
  /** ISO 8601 timestamp from backend chrono::DateTime<Utc> */
  timestamp: string;
}
