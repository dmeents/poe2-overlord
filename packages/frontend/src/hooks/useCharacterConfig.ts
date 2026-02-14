import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import {
  type Ascendency,
  CHARACTER_CLASSES,
  type CharacterClass,
  type CharacterData,
  getAscendenciesForClass,
  LEAGUES,
  type League,
} from '../types/character';

// Option types for form components
interface CharacterClassOption {
  value: CharacterClass;
  label: string;
}

interface LeagueOption {
  value: League;
  label: string;
}

interface AscendencyOption {
  value: Ascendency;
  label: string;
}

// Pre-compute static options at module level
const characterClassesOptions: CharacterClassOption[] = CHARACTER_CLASSES.map(cls => ({
  value: cls,
  label: cls,
}));

const leagueOptions: LeagueOption[] = LEAGUES.map(league => ({
  value: league,
  label: league,
}));

// Pre-compute all ascendencies by class
const ascendenciesOptions: Record<CharacterClass, AscendencyOption[]> = (() => {
  const result: Record<CharacterClass, AscendencyOption[]> = {} as Record<
    CharacterClass,
    AscendencyOption[]
  >;

  for (const characterClass of CHARACTER_CLASSES) {
    result[characterClass] = getAscendenciesForClass(characterClass).map(ascendency => ({
      value: ascendency,
      label: ascendency,
    }));
  }

  return result;
})();

// Helper to get ascendencies for a specific class
function getAscendenciesForClassOptions(characterClass: CharacterClass): AscendencyOption[] {
  return ascendenciesOptions[characterClass] || [];
}

// Helper to get default form data for character creation/editing
function getDefaultFormData(character?: CharacterData): CharacterFormData {
  return {
    name: character?.name || '',
    class: character?.class || 'Warrior',
    ascendency: character?.ascendency || 'Titan',
    league: character?.league || 'Standard',
    hardcore: character?.hardcore || false,
    solo_self_found: character?.solo_self_found || false,
  };
}

/**
 * Hook for managing character configuration data (classes, leagues, ascendencies).
 * Uses static frontend constants computed at module level.
 */
export function useCharacterConfig() {
  return {
    // Static data (always available, no loading needed)
    characterClasses: characterClassesOptions,
    leagues: leagueOptions,
    ascendencies: ascendenciesOptions,

    // Helpers
    getAscendenciesForClass: getAscendenciesForClassOptions,
    getDefaultFormData,
  };
}
