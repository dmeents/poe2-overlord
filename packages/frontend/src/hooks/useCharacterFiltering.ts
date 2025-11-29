import { useMemo } from 'react';
import type { CharacterData } from '../types/character';
import type { CharacterFilters, SortOption } from './useCharacterFilters';

export function useCharacterFiltering(
  characters: CharacterData[],
  filters: CharacterFilters,
  sort: SortOption
) {
  const filteredAndSortedCharacters = useMemo(() => {
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

    return filtered;
  }, [characters, filters, sort]);

  const characterCount = filteredAndSortedCharacters.length;
  const totalCount = characters.length;

  return {
    filteredCharacters: filteredAndSortedCharacters,
    characterCount,
    totalCount,
  };
}
