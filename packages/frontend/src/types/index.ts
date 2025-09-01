import React from 'react';

export interface ProcessInfo {
  running: boolean;
  pid?: number;
  startTime?: string;
}

export interface ZoneChangeEvent {
  zone_name: string;
  timestamp: string;
}

export interface ActChangeEvent {
  act_name: string;
  timestamp: string;
}

export interface HideoutChangeEvent {
  hideout_name: string;
  timestamp: string;
}

export interface LocationSession {
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act' | 'Hideout';
  entry_timestamp: string;
  exit_timestamp?: string;
  duration_seconds?: number;
}

export interface LocationStats {
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act' | 'Hideout';
  total_visits: number;
  total_time_seconds: number;
  average_session_seconds: number;
  last_visited?: string;
}

export interface TimeTrackingSummary {
  active_sessions: LocationSession[];
  top_locations: LocationStats[];
  total_locations_tracked: number;
  total_active_sessions: number;
  total_play_time_seconds: number;
  total_play_time_since_process_start_seconds: number;
  total_hideout_time_seconds: number;
}

// Updated to match the new backend SceneChangeEvent structure
export type SceneChangeEvent =
  | { type: 'Zone'; Zone: ZoneChangeEvent }
  | { type: 'Act'; Act: ActChangeEvent }
  | { type: 'Hideout'; Hideout: HideoutChangeEvent };

// Helper functions to extract data from SceneChangeEvent
export const getSceneEventName = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.Zone.zone_name;
    case 'Act':
      return event.Act.act_name;
    case 'Hideout':
      return event.Hideout.hideout_name;
  }
};

export const getSceneEventTimestamp = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.Zone.timestamp;
    case 'Act':
      return event.Act.timestamp;
    case 'Hideout':
      return event.Hideout.timestamp;
  }
};

export const isZoneEvent = (
  event: SceneChangeEvent
): event is { type: 'Zone'; Zone: ZoneChangeEvent } => {
  return event.type === 'Zone';
};

export const isActEvent = (
  event: SceneChangeEvent
): event is { type: 'Act'; Act: ActChangeEvent } => {
  return event.type === 'Act';
};

export const isHideoutEvent = (
  event: SceneChangeEvent
): event is { type: 'Hideout'; Hideout: HideoutChangeEvent } => {
  return event.type === 'Hideout';
};

export interface ProcessStatusProps {
  poe2Running: boolean;
  processInfo: ProcessInfo | null;
  onRefresh: () => void;
}

export interface QuickActionProps {
  icon: React.ReactNode;
  label: string;
  onClick?: () => void;
}

export interface InfoPanelProps {
  title: string;
  description: string;
  icon: React.ReactNode;
}

export interface FooterProps {
  version: string;
  technology: string;
}

export interface AppConfig {
  poe_client_log_path: string;
  log_level: string;
}

export interface ServerConnectionEvent {
  ip_address: string;
  port: number;
  timestamp: string;
}

export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  last_ping_ms: number | null;
  last_seen: string;
  last_checked: string;
}
