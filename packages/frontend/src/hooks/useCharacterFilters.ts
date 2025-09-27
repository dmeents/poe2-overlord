import { useCallback, useState } from 'react';
import type { Ascendency, CharacterClass, League } from '../types';

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

export function useCharacterFilters() {
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

  const hasActiveFilters = useCallback(() => {
    return (
      filters.league !== 'All' ||
      filters.hardcore !== null ||
      filters.soloSelfFound !== null ||
      filters.classes.length > 0 ||
      filters.ascendencies.length > 0 ||
      filters.nameSearch.trim() !== ''
    );
  }, [filters]);

  return {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters: hasActiveFilters(),
  };
}
