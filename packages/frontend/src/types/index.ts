import React from 'react';

export interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

export interface ZoneChangeEvent {
  zone_name: string;
  timestamp: string;
}

export interface ActChangeEvent {
  act_name: string;
  timestamp: string;
}

export interface LocationSession {
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act';
  entry_timestamp: string;
  exit_timestamp?: string;
  duration_seconds?: number;
}

export interface LocationStats {
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act';
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
}

// Updated to match the new backend SceneChangeEvent structure
export type SceneChangeEvent =
  | { type: 'Zone'; Zone: ZoneChangeEvent }
  | { type: 'Act'; Act: ActChangeEvent };

// Helper functions to extract data from SceneChangeEvent
export const getSceneEventName = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.Zone.zone_name;
    case 'Act':
      return event.Act.act_name;
  }
};

export const getSceneEventTimestamp = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.Zone.timestamp;
    case 'Act':
      return event.Act.timestamp;
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
