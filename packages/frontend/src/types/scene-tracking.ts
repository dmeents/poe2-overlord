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

export type LocationType = 'Zone' | 'Act' | 'Hideout';

export interface ZoneStats {
  location_id: string;
  location_name: string;
  location_type: LocationType;
  act?: string;
  duration: number;
  deaths: number;
  visits: number;
  first_visited: string;
  last_visited: string;
  is_active: boolean;
}

export interface TimeTrackingSummary {
  character_id: string;
  total_play_time: number;
  total_hideout_time: number;
  total_zones_visited: number;
  total_deaths: number;
}

export interface CharacterTimeTracking {
  character_id: string;
  summary: TimeTrackingSummary;
  zones: ZoneStats[];
  last_updated: string;
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
