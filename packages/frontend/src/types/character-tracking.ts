// Character tracking types that match the backend models
// These types are now embedded within CharacterData in the unified model

export type LocationType = 'Zone' | 'Act' | 'Hideout';

export interface LocationState {
  scene?: string;
  act?: string;
  is_town: boolean;
  location_type: LocationType;
  last_updated: string;
}

export interface ZoneStats {
  location_id: string;
  location_name: string;
  location_type: LocationType;
  act?: string;
  is_town: boolean;
  duration: number;
  deaths: number;
  visits: number;
  first_visited: string;
  last_visited: string;
  is_active: boolean;
  entry_timestamp?: string;
  zone_level?: number;
}

export interface TrackingSummary {
  character_id: string;
  total_play_time: number;
  total_hideout_time: number;
  total_zones_visited: number;
  total_deaths: number;
}

// Legacy CharacterTrackingData interface for backward compatibility
// This is now embedded within CharacterData in the unified model
export interface CharacterTrackingData {
  character_id: string;
  current_location?: LocationState;
  summary: TrackingSummary;
  zones: ZoneStats[];
  last_updated: string;
}

// Event types for character tracking data updates
// Updated to work with the new unified CharacterData model
export interface CharacterTrackingDataUpdatedEvent {
  character_id: string;
  data: CharacterTrackingData;
  timestamp: string;
}
