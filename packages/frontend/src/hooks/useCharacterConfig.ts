import { useMemo } from 'react';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import {
  CHARACTER_CLASSES,
  LEAGUES,
  getAscendenciesForClass,
  type CharacterClass,
  type CharacterData,
  type Ascendency,
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

/**
 * Hook for managing character configuration data (classes, leagues, ascendencies).
 * Now uses static frontend constants instead of backend commands.
 */
export function useCharacterConfig() {
  // Convert static constants to option format for form components
  const characterClasses = useMemo<CharacterClassOption[]>(
    () =>
      CHARACTER_CLASSES.map(cls => ({
        value: cls,
        label: cls,
      })),
    []
  );

  const leagues = useMemo<LeagueOption[]>(
    () =>
      LEAGUES.map(league => ({
        value: league,
        label: league,
      })),
    []
  );

  // Pre-compute all ascendencies by class
  const ascendencies = useMemo<
    Record<CharacterClass, AscendencyOption[]>
  >(() => {
    const result: Record<CharacterClass, AscendencyOption[]> = {} as Record<
      CharacterClass,
      AscendencyOption[]
    >;

    for (const characterClass of CHARACTER_CLASSES) {
      result[characterClass] = getAscendenciesForClass(characterClass).map(
        ascendency => ({
          value: ascendency,
          label: ascendency,
        })
      );
    }

    return result;
  }, []);

  // Helper to get ascendencies for a specific class
  const getAscendenciesForClassOptions = useMemo(
    () =>
      (characterClass: CharacterClass): AscendencyOption[] => {
        return ascendencies[characterClass] || [];
      },
    [ascendencies]
  );

  // Helper to get default form data for character creation/editing
  const getDefaultFormData = useMemo(
    () =>
      (character?: CharacterData): CharacterFormData => ({
        name: character?.name || '',
        class: character?.class || 'Warrior',
        ascendency: character?.ascendency || 'Titan',
        league: character?.league || 'Standard',
        hardcore: character?.hardcore || false,
        solo_self_found: character?.solo_self_found || false,
      }),
    []
  );

  return {
    // Static data (always available, no loading needed)
    characterClasses,
    leagues,
    ascendencies,

    // Helpers
    getAscendenciesForClass: getAscendenciesForClassOptions,
    getDefaultFormData,
  };
}
