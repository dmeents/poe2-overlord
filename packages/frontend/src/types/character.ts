// Character-related types - Updated to match unified backend CharacterData model

import type { WalkthroughProgress } from './walkthrough';

export type CharacterClass =
  | 'Warrior'
  | 'Sorceress'
  | 'Ranger'
  | 'Huntress'
  | 'Monk'
  | 'Mercenary'
  | 'Witch';

export type Ascendency =
  // Warrior ascendencies
  | 'Titan'
  | 'Warbringer'
  | 'Smith of Katava'
  // Sorceress ascendencies
  | 'Stormweaver'
  | 'Chronomancer'
  // Ranger ascendencies
  | 'Deadeye'
  | 'Pathfinder'
  // Huntress ascendencies
  | 'Ritualist'
  | 'Amazon'
  // Monk ascendencies
  | 'Invoker'
  | 'Acolyte of Chayula'
  // Mercenary ascendencies
  | 'Gemling Legionnaire'
  | 'Tactitian'
  | 'Witchhunter'
  // Witch ascendencies
  | 'Blood Mage'
  | 'Infernalist'
  | 'Lich';

export type League = 'Standard' | 'Third Edict';

// Location and tracking types
export type LocationType = 'Zone' | 'Hideout';

export interface LocationState {
  scene?: string;
  act?: string;
  is_town: boolean;
  location_type: LocationType;
  last_updated: string;
}

export interface ZoneStats {
  // Primary identifier - zone name
  zone_name: string;

  // Character tracking
  duration: number;
  deaths: number;
  visits: number;
  first_visited: string;
  last_visited: string;
  is_active: boolean;
  entry_timestamp?: string;

  // Zone metadata (enriched from backend) - all fields available for future use
  area_id?: string;
  act?: number;
  area_level?: number;
  is_town: boolean;
  has_waypoint: boolean;
  bosses: string[];
  monsters: string[];
  npcs: string[];
  connected_zones: string[];
  description?: string;
  points_of_interest: string[];
  image_url?: string;
  wiki_url?: string;
  last_updated?: string;
}

export interface CharacterSummary {
  character_id: string;
  total_play_time: number;
  total_hideout_time: number;
  total_zones_visited: number;
  total_deaths: number;

  // Per-act play time tracking (in seconds)
  play_time_act1: number;
  play_time_act2: number;
  play_time_act3: number;
  play_time_act4: number;
  play_time_interlude: number;
  play_time_endgame: number;
}

// Unified character data model that matches the backend CharacterData
export interface CharacterData {
  id: string;
  name: string;
  class: CharacterClass;
  ascendency: Ascendency;
  league: League;
  hardcore: boolean;
  solo_self_found: boolean;
  level: number;
  created_at: string;
  last_played?: string;

  // Embedded tracking data (consolidated from character_tracking domain)
  current_location?: LocationState;
  summary: CharacterSummary;
  zones: ZoneStats[];
  walkthrough_progress: WalkthroughProgress;
  last_updated: string;
}

// Characters index for managing character IDs and active character
export interface CharactersIndex {
  character_ids: string[];
  active_character_id?: string;
}

// Parameters for updating an existing character
export interface CharacterUpdateParams {
  name: string;
  class: CharacterClass;
  ascendency: Ascendency;
  league: League;
  hardcore: boolean;
  solo_self_found: boolean;
  level: number;
}

// Helper type for character creation
export interface CreateCharacterRequest {
  name: string;
  class: CharacterClass;
  ascendency: Ascendency;
  league: League;
  hardcore: boolean;
  solo_self_found: boolean;
}

// Legacy CharacterTrackingData interface for backward compatibility
// This is now embedded within CharacterData in the unified model
export interface CharacterTrackingData {
  character_id: string;
  current_location?: LocationState;
  summary: CharacterSummary;
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
