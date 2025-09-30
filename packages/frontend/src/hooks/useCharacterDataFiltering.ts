import { useCallback } from 'react';
import type { CharacterData } from '../types';
import type { CharacterFilters, SortOption } from './useCharacterFilterState';
import { useDataFiltering, FilterHelpers, SortHelpers } from './useDataFiltering';

/**
 * Character-specific filter function that combines multiple filter criteria
 */
function createCharacterFilterFunction(): (item: CharacterData, filters: CharacterFilters) => boolean {
  return (item, filters) => {
    // League filter
    if (filters.league !== 'All' && item.league !== filters.league) {
      return false;
    }

    // Hardcore filter
    if (filters.hardcore !== null && item.hardcore !== filters.hardcore) {
      return false;
    }

    // Solo Self Found filter
    if (filters.soloSelfFound !== null && item.solo_self_found !== filters.soloSelfFound) {
      return false;
    }

    // Class filter
    if (filters.classes.length > 0 && !filters.classes.includes(item.class)) {
      return false;
    }

    // Ascendency filter
    if (filters.ascendencies.length > 0 && !filters.ascendencies.includes(item.ascendency)) {
      return false;
    }

    // Name search filter
    if (filters.nameSearch.trim() !== '') {
      const searchTerm = filters.nameSearch.toLowerCase().trim();
      if (!item.name.toLowerCase().includes(searchTerm)) {
        return false;
      }
    }

    return true;
  };
}

/**
 * Character-specific sort function
 */
function createCharacterSortFunction(): (a: CharacterData, b: CharacterData, sort: SortOption) => number {
  return (a, b, sort) => {
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
        comparison = new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
        break;
      }
      case 'last_played': {
        const aLastPlayed = a.last_played ? new Date(a.last_played).getTime() : 0;
        const bLastPlayed = b.last_played ? new Date(b.last_played).getTime() : 0;
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
  };
}

/**
 * Hook for character data filtering and sorting using the generic useDataFiltering
 * 
 * This hook replaces the old useCharacterFiltering with a more generic approach
 * that leverages the useDataFiltering hook for better maintainability.
 * 
 * @param characters - Array of character data to filter and sort
 * @param filters - Character filter criteria
 * @param sort - Sort configuration
 * @returns Filtered and sorted characters with statistics
 */
export function useCharacterDataFiltering(
  characters: CharacterData[],
  filters: CharacterFilters,
  sort: SortOption
) {
  // Create memoized filter and sort functions
  const filterFunction = useCallback(createCharacterFilterFunction(), []);
  const sortFunction = useCallback(createCharacterSortFunction(), []);

  // Use the generic data filtering hook
  const { filteredData, count, totalCount } = useDataFiltering({
    data: characters,
    filters,
    sort,
    filterFunction,
    sortFunction,
  });

  return {
    filteredCharacters: filteredData,
    characterCount: count,
    totalCount,
  };
}
