import type {
  Ascendency,
  CharacterClass,
  CharacterSummaryData,
  League,
} from '../../types/character';

export interface CharacterFilters {
  league: League | 'All';
  hardcore: boolean | null;
  soloSelfFound: boolean | null;
  classes: CharacterClass[];
  ascendencies: Ascendency[];
  nameSearch: string;
}

export type CharacterSortField = 'level' | 'last_played' | 'created_at' | 'name' | 'play_time';

export const characterListConfig = {
  defaultFilters: {
    league: 'All' as League | 'All',
    hardcore: null,
    soloSelfFound: null,
    classes: [] as CharacterClass[],
    ascendencies: [] as Ascendency[],
    nameSearch: '',
  },

  defaultSort: {
    field: 'last_played' as CharacterSortField,
    direction: 'desc' as const,
  },

  filterFn: (character: CharacterSummaryData, filters: CharacterFilters): boolean => {
    // League filter
    if (filters.league !== 'All' && character.league !== filters.league) {
      return false;
    }

    // Hardcore filter
    if (filters.hardcore !== null && character.hardcore !== filters.hardcore) {
      return false;
    }

    // Solo Self Found filter
    if (filters.soloSelfFound !== null && character.solo_self_found !== filters.soloSelfFound) {
      return false;
    }

    // Class filter
    if (filters.classes.length > 0 && !filters.classes.includes(character.class)) {
      return false;
    }

    // Ascendency filter
    if (filters.ascendencies.length > 0 && !filters.ascendencies.includes(character.ascendency)) {
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
  },

  sortFn: (
    a: CharacterSummaryData,
    b: CharacterSummaryData,
    sort: { field: CharacterSortField; direction: 'asc' | 'desc' },
  ): number => {
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
  },

  chipConfigs: [
    {
      key: 'league' as const,
      label: (value: League | 'All') => `League: ${value}`,
      isActive: (value: League | 'All') => value !== 'All',
    },
    {
      key: 'hardcore' as const,
      label: (value: boolean | null) => (value ? 'Hardcore' : 'Non-Hardcore'),
      isActive: (value: boolean | null) => value !== null,
    },
    {
      key: 'soloSelfFound' as const,
      label: (value: boolean | null) => (value ? 'SSF' : 'Non-SSF'),
      isActive: (value: boolean | null) => value !== null,
    },
    {
      key: 'classes' as const,
      label: (value: CharacterClass[]) => `Class: ${value[0]}`,
      isActive: (value: CharacterClass[]) => value.length > 0,
    },
    {
      key: 'ascendencies' as const,
      label: (value: Ascendency[]) => `Ascendency: ${value[0]}`,
      isActive: (value: Ascendency[]) => value.length > 0,
    },
    {
      key: 'nameSearch' as const,
      label: (value: string) => `Name: ${value}`,
      isActive: (value: string) => value.trim() !== '',
    },
  ],
};
