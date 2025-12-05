import { useCallback, useMemo, useState } from 'react';
import type { Ascendency, CharacterClass, CharacterData, League } from '../types/character';

export interface CharacterFilters {
  league: League | 'All';
  hardcore: boolean | null; // null = all, true = hardcore only, false = non-hardcore only
  soloSelfFound: boolean | null; // null = all, true = SSF only, false = non-SSF only
  classes: CharacterClass[];
  ascendencies: Ascendency[];
  nameSearch: string;
}

export interface SortOption {
  field: 'level' | 'last_played' | 'created_at' | 'name' | 'play_time';
  direction: 'asc' | 'desc';
}

const defaultFilters: CharacterFilters = {
  league: 'All',
  hardcore: null,
  soloSelfFound: null,
  classes: [],
  ascendencies: [],
  nameSearch: '',
};

const defaultSort: SortOption = {
  field: 'last_played',
  direction: 'desc',
};

export function useCharacterList(characters: CharacterData[]) {
  const [filters, setFilters] = useState<CharacterFilters>(defaultFilters);
  const [sort, setSort] = useState<SortOption>(defaultSort);

  const updateFilter = useCallback(
    <K extends keyof CharacterFilters>(key: K, value: CharacterFilters[K]) => {
      setFilters(prev => ({ ...prev, [key]: value }));
    },
    []
  );

  const updateSort = useCallback(
    (field: SortOption['field'], direction?: SortOption['direction']) => {
      setSort(prev => ({
        field,
        direction:
          direction ??
          (prev.field === field && prev.direction === 'desc' ? 'asc' : 'desc'),
      }));
    },
    []
  );

  const clearFilters = useCallback(() => {
    setFilters(defaultFilters);
  }, []);

  const resetSort = useCallback(() => {
    setSort(defaultSort);
  }, []);

  const { filteredCharacters, characterCount, totalCount, hasActiveFilters } = useMemo(() => {
    // Apply filters
    const filtered = characters.filter(character => {
      // League filter
      if (filters.league !== 'All' && character.league !== filters.league) {
        return false;
      }

      // Hardcore filter
      if (
        filters.hardcore !== null &&
        character.hardcore !== filters.hardcore
      ) {
        return false;
      }

      // Solo Self Found filter
      if (
        filters.soloSelfFound !== null &&
        character.solo_self_found !== filters.soloSelfFound
      ) {
        return false;
      }

      // Class filter
      if (
        filters.classes.length > 0 &&
        !filters.classes.includes(character.class)
      ) {
        return false;
      }

      // Ascendency filter
      if (
        filters.ascendencies.length > 0 &&
        !filters.ascendencies.includes(character.ascendency)
      ) {
        return false;
      }

      // Name search filter
      if (filters.nameSearch.trim() !== '') {
        const searchTerm = filters.nameSearch.toLowerCase().trim();
        if (!character.name.toLowerCase().includes(searchTerm)) {
          return false;
        }
      }

      return true;
    });

    // Sort the filtered characters
    filtered.sort((a, b) => {
      let comparison = 0;

      switch (sort.field) {
        case 'level': {
          comparison = a.level - b.level;
          break;
        }
        case 'name': {
          comparison = a.name.localeCompare(b.name);
          break;
        }
        case 'created_at': {
          comparison =
            new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
          break;
        }
        case 'last_played': {
          const aLastPlayed = a.last_played
            ? new Date(a.last_played).getTime()
            : 0;
          const bLastPlayed = b.last_played
            ? new Date(b.last_played).getTime()
            : 0;
          comparison = aLastPlayed - bLastPlayed;
          break;
        }
        case 'play_time': {
          const aPlayTime = a.summary?.total_play_time || 0;
          const bPlayTime = b.summary?.total_play_time || 0;
          comparison = aPlayTime - bPlayTime;
          break;
        }
        default: {
          comparison = 0;
        }
      }

      return sort.direction === 'asc' ? comparison : -comparison;
    });

    // Check if any filters are active
    const hasActive =
      filters.league !== 'All' ||
      filters.hardcore !== null ||
      filters.soloSelfFound !== null ||
      filters.classes.length > 0 ||
      filters.ascendencies.length > 0 ||
      filters.nameSearch.trim() !== '';

    return {
      filteredCharacters: filtered,
      characterCount: filtered.length,
      totalCount: characters.length,
      hasActiveFilters: hasActive,
    };
  }, [characters, filters, sort]);

  return {
    // State
    filters,
    sort,
    // Actions
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    // Computed
    filteredCharacters,
    characterCount,
    totalCount,
    hasActiveFilters,
  };
}
