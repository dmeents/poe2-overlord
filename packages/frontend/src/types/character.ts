import type { LocationSession } from './scene-tracking';

// Character-related types
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
  last_known_location?: LocationSession;
}

export interface CharacterData {
  characters: Character[];
  active_character_id?: string;
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
