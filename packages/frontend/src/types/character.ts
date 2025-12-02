// Character-related types - Updated to match unified backend CharacterData model

import type { WalkthroughProgress } from './walkthrough';

// ============================================================================
// Static Character Data Constants
// ============================================================================

export const CHARACTER_CLASSES = [
  'Warrior',
  'Sorceress',
  'Ranger',
  'Huntress',
  'Monk',
  'Mercenary',
  'Witch',
] as const;

export const LEAGUES = ['Standard', 'Third Edict'] as const;

export const ALL_ASCENDENCIES = [
  // Warrior ascendencies
  'Titan',
  'Warbringer',
  'Smith of Katava',
  // Sorceress ascendencies
  'Stormweaver',
  'Chronomancer',
  // Ranger ascendencies
  'Deadeye',
  'Pathfinder',
  // Huntress ascendencies
  'Ritualist',
  'Amazon',
  // Monk ascendencies
  'Invoker',
  'Acolyte of Chayula',
  // Mercenary ascendencies
  'Gemling Legionnaire',
  'Tactitian',
  'Witchhunter',
  // Witch ascendencies
  'Blood Mage',
  'Infernalist',
  'Lich',
] as const;

// ============================================================================
// Character Type Definitions (derived from constants)
// ============================================================================

export type CharacterClass = (typeof CHARACTER_CLASSES)[number];
export type League = (typeof LEAGUES)[number];
export type Ascendency = (typeof ALL_ASCENDENCIES)[number];

export const ASCENDENCIES_BY_CLASS: Record<
  CharacterClass,
  readonly Ascendency[]
> = {
  Warrior: ['Titan', 'Warbringer', 'Smith of Katava'],
  Sorceress: ['Stormweaver', 'Chronomancer'],
  Ranger: ['Deadeye', 'Pathfinder'],
  Huntress: ['Ritualist', 'Amazon'],
  Monk: ['Invoker', 'Acolyte of Chayula'],
  Mercenary: ['Gemling Legionnaire', 'Tactitian', 'Witchhunter'],
  Witch: ['Blood Mage', 'Infernalist', 'Lich'],
};

// Helper function to get valid ascendencies for a character class
export function getAscendenciesForClass(
  characterClass: CharacterClass
): readonly Ascendency[] {
  return ASCENDENCIES_BY_CLASS[characterClass] || [];
}

// Helper function to validate ascendency/class combination
export function isValidAscendencyForClass(
  ascendency: Ascendency,
  characterClass: CharacterClass
): boolean {
  return getAscendenciesForClass(characterClass).includes(ascendency);
}

// Location and tracking types
export type LocationType = 'Zone' | 'Hideout';

// Simplified LocationState - only stores zone reference
// Full zone data comes from joining with zones.json
export interface LocationState {
  zone_name: string;
  last_updated: string;
}

// EnrichedLocationState - returned by API with full zone metadata
export interface EnrichedLocationState {
  zone_name: string;
  act: number;
  is_town: boolean;
  location_type: LocationType;
  area_id?: string;
  area_level?: number;
  has_waypoint: boolean;
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
  // Note: API responses return EnrichedLocationState with full zone metadata
  current_location?: EnrichedLocationState;
  summary: CharacterSummary;
  zones: ZoneStats[];
  walkthrough_progress: WalkthroughProgress;
  last_updated: string;
}
