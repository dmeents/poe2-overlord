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

// Updated to match the actual backend SceneChangeEvent serialization format
export type SceneChangeEvent =
  | { type: 'Zone'; zone_name: string; timestamp: string }
  | { type: 'Act'; act_name: string; timestamp: string }
  | { type: 'Hideout'; hideout_name: string; timestamp: string };

export interface LocationSession {
  character_id: string;
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act' | 'Hideout';
  entry_timestamp: string;
  exit_timestamp?: string;
  duration_seconds?: number;
}

export interface LocationStats {
  character_id: string;
  location_id: string;
  location_name: string;
  location_type: 'Zone' | 'Act' | 'Hideout';
  total_visits: number;
  total_time_seconds: number;
  average_session_seconds: number;
  last_visited?: string;
}

export interface TimeTrackingSummary {
  character_id: string;
  active_sessions: LocationSession[];
  top_locations: LocationStats[];
  total_locations_tracked: number;
  total_active_sessions: number;
  total_play_time_seconds: number;
  total_play_time_since_process_start_seconds: number;
  total_hideout_time_seconds: number;
}

export interface TimeTrackingData {
  character_id: string;
  active_sessions: LocationSession[];
  completed_sessions: LocationSession[];
  all_location_stats: LocationStats[];
  summary: TimeTrackingSummary;
}

// Helper functions to extract data from SceneChangeEvent
export const getSceneEventName = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.zone_name;
    case 'Act':
      return event.act_name;
    case 'Hideout':
      return event.hideout_name;
  }
};

export const getSceneEventTimestamp = (event: SceneChangeEvent): string => {
  switch (event.type) {
    case 'Zone':
      return event.timestamp;
    case 'Act':
      return event.timestamp;
    case 'Hideout':
      return event.timestamp;
  }
};

export const isZoneEvent = (
  event: SceneChangeEvent
): event is { type: 'Zone'; zone_name: string; timestamp: string } => {
  return event.type === 'Zone';
};

export const isActEvent = (
  event: SceneChangeEvent
): event is { type: 'Act'; act_name: string; timestamp: string } => {
  return event.type === 'Act';
};

export const isHideoutEvent = (
  event: SceneChangeEvent
): event is { type: 'Hideout'; hideout_name: string; timestamp: string } => {
  return event.type === 'Hideout';
};
