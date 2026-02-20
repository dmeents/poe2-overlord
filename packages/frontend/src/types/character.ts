import type { WalkthroughProgress } from './walkthrough';

export const CHARACTER_CLASSES = [
  'Warrior',
  'Sorceress',
  'Ranger',
  'Huntress',
  'Monk',
  'Mercenary',
  'Witch',
  'Druid',
] as const;

export const LEAGUES = ['Standard', 'Rise of the Abyssal', 'The Fate of the Vaal'] as const;

const ALL_ASCENDENCIES = [
  'Titan',
  'Warbringer',
  'Smith of Katava',
  'Stormweaver',
  'Chronomancer',
  'Disciple of Varashta',
  'Deadeye',
  'Pathfinder',
  'Ritualist',
  'Amazon',
  'Invoker',
  'Acolyte of Chayula',
  'Gemling Legionnaire',
  'Tactitian',
  'Witchhunter',
  'Blood Mage',
  'Infernalist',
  'Lich',
  'Shaman',
  'Oracle',
] as const;

export type CharacterClass = (typeof CHARACTER_CLASSES)[number];
export type League = (typeof LEAGUES)[number];
export type Ascendency = (typeof ALL_ASCENDENCIES)[number];

const ASCENDENCIES_BY_CLASS: Record<CharacterClass, readonly Ascendency[]> = {
  Warrior: ['Titan', 'Warbringer', 'Smith of Katava'],
  Sorceress: ['Stormweaver', 'Chronomancer', 'Disciple of Varashta'],
  Ranger: ['Deadeye', 'Pathfinder'],
  Huntress: ['Ritualist', 'Amazon'],
  Monk: ['Invoker', 'Acolyte of Chayula'],
  Mercenary: ['Gemling Legionnaire', 'Tactitian', 'Witchhunter'],
  Witch: ['Blood Mage', 'Infernalist', 'Lich'],
  Druid: ['Shaman', 'Oracle'],
};

export function getAscendenciesForClass(characterClass: CharacterClass): readonly Ascendency[] {
  return ASCENDENCIES_BY_CLASS[characterClass] || [];
}

type LocationType = 'Zone' | 'Hideout' | 'Town';

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

/**
 * Zone statistics enriched with wiki metadata.
 * Maps to backend's EnrichedZoneStats (not the basic ZoneStats).
 * Includes both tracking data (duration, deaths, visits) and metadata (bosses, npcs, etc).
 */
export interface ZoneStats {
  // Tracking data (from backend ZoneStats)
  zone_name: string;
  duration: number;
  deaths: number;
  visits: number;
  first_visited: string;
  last_visited: string;
  is_active: boolean;
  entry_timestamp?: string;
  act?: number;
  is_town: boolean;

  // Enrichment data (from wiki metadata)
  area_id?: string;
  area_level?: number;
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
  total_town_time: number;
  total_zones_visited: number;
  total_deaths: number;
  play_time_act1: number;
  play_time_act2: number;
  play_time_act3: number;
  play_time_act4: number;
  play_time_act5: number;
  play_time_interlude: number;
  play_time_endgame: number;
}

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
  current_location?: EnrichedLocationState;
  summary: CharacterSummary;
  zones: ZoneStats[];
  walkthrough_progress: WalkthroughProgress;
  last_updated: string;
}
