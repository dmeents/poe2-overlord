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
  poe_client_log_path: string;
  log_level: string;
  zone_refresh_interval: ZoneRefreshInterval;
}

/** Event emitted when configuration changes (from backend). */
export interface ConfigurationChangedEvent {
  new_config: AppConfig;
  previous_config: AppConfig;
  /** ISO 8601 timestamp from backend chrono::DateTime<Utc> */
  timestamp: string;
}
