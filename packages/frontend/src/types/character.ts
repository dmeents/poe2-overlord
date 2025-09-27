// Character-related types - Updated to match unified backend CharacterData model
import type {
  LocationState,
  TrackingSummary,
  ZoneStats,
} from './character-tracking';

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
  summary: TrackingSummary;
  zones: ZoneStats[];
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

// Legacy Character type for backward compatibility during transition
// This will be removed once all components are updated
export interface Character {
  id: string;
  name: string;
  class: CharacterClass;
  ascendency: Ascendency;
  league: League;
  hardcore: boolean;
  solo_self_found: boolean;
  created_at: string;
  last_played?: string;
  is_active: boolean;
  level: number;
  trackingData?: {
    character_id: string;
    current_location?: LocationState;
    summary: TrackingSummary;
    zones: ZoneStats[];
    last_updated: string;
  };
}
