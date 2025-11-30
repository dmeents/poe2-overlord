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
  seconds: number;
}

export interface AppConfig {
  poe_client_log_path: string;
  log_level: string;
  zone_refresh_interval: ZoneRefreshInterval;
}
