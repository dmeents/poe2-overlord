import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { CharacterFormData } from '../components/character-modals/character-form-modal';
import type {
  Ascendency,
  CharacterClass,
  CharacterData,
  League,
} from '../types';

// Backend response types
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

export function useCharacterConfig() {
  const [characterClasses, setCharacterClasses] = useState<
    CharacterClassOption[]
  >([]);
  const [leagues, setLeagues] = useState<LeagueOption[]>([]);
  const [ascendencies, setAscendencies] = useState<
    Record<CharacterClass, AscendencyOption[]>
  >({} as Record<CharacterClass, AscendencyOption[]>);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Load character classes
  const loadCharacterClasses = useCallback(async () => {
    try {
      const classes = await invoke<CharacterClass[]>(
        'get_available_character_classes'
      );
      const classOptions: CharacterClassOption[] = classes.map(cls => ({
        value: cls,
        label: cls, // Use the enum value as label for now
      }));
      setCharacterClasses(classOptions);
    } catch (err) {
      console.error('Failed to load character classes:', err);
      setError(
        err instanceof Error ? err.message : 'Failed to load character classes'
      );
    }
  }, []);

  // Load leagues
  const loadLeagues = useCallback(async () => {
    try {
      const leagueData = await invoke<League[]>('get_available_leagues');
      const leagueOptions: LeagueOption[] = leagueData.map(league => ({
        value: league,
        label: league, // Use the enum value as label for now
      }));
      setLeagues(leagueOptions);
    } catch (err) {
      console.error('Failed to load leagues:', err);
      setError(err instanceof Error ? err.message : 'Failed to load leagues');
    }
  }, []);

  // Load ascendencies for a specific class
  const loadAscendenciesForClass = useCallback(
    async (characterClass: CharacterClass) => {
      try {
        const ascendencyData = await invoke<Ascendency[]>(
          'get_available_ascendencies_for_class',
          {
            class: characterClass,
          }
        );
        const ascendencyOptions: AscendencyOption[] = ascendencyData.map(
          ascendency => ({
            value: ascendency,
            label: ascendency, // Use the enum value as label for now
          })
        );

        setAscendencies(prev => ({
          ...prev,
          [characterClass]: ascendencyOptions,
        }));
      } catch (err) {
        console.error(
          `Failed to load ascendencies for class ${characterClass}:`,
          err
        );
        setError(
          err instanceof Error
            ? err.message
            : `Failed to load ascendencies for ${characterClass}`
        );
      }
    },
    []
  );

  // Load data on mount
  useEffect(() => {
    const loadInitialData = async () => {
      try {
        setIsLoading(true);
        setError(null);
        await Promise.all([loadCharacterClasses(), loadLeagues()]);
      } catch (err) {
        console.error('Failed to load character configuration:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadInitialData();
  }, [loadCharacterClasses, loadLeagues]);

  // Get ascendencies for a class (load if not cached)
  const getAscendenciesForClass = useCallback(
    async (characterClass: CharacterClass): Promise<AscendencyOption[]> => {
      if (ascendencies[characterClass]) {
        return ascendencies[characterClass];
      }

      await loadAscendenciesForClass(characterClass);
      return ascendencies[characterClass] || [];
    },
    [ascendencies, loadAscendenciesForClass]
  );

  // Refresh all data
  const refreshData = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      await Promise.all([loadCharacterClasses(), loadLeagues()]);
    } catch (err) {
      console.error('Failed to refresh character configuration:', err);
    } finally {
      setIsLoading(false);
    }
  }, [loadCharacterClasses, loadLeagues]);

  return {
    characterClasses,
    leagues,
    ascendencies,
    isLoading,
    error,
    loadCharacterClasses,
    loadLeagues,
    loadAscendenciesForClass,
    getAscendenciesForClass,
    getDefaultFormData: useMemo(
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
    ),
    refreshData,
  };
}
