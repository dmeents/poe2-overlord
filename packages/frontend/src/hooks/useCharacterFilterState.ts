import type { Ascendency, CharacterClass, League } from '../types';
import { useFilterState, type FilterStateConfig } from './useFilterState';

export interface CharacterFilters extends Record<string, unknown> {
  league: League | 'All';
  hardcore: boolean | null; // null = all, true = hardcore only, false = non-hardcore only
  soloSelfFound: boolean | null; // null = all, true = SSF only, false = non-SSF only
  classes: CharacterClass[];
  ascendencies: Ascendency[];
  nameSearch: string;
}

export interface SortOption extends Record<string, unknown> {
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

const hasActiveFiltersFn = (filters: CharacterFilters): boolean => {
  return (
    filters.league !== 'All' ||
    filters.hardcore !== null ||
    filters.soloSelfFound !== null ||
    filters.classes.length > 0 ||
    filters.ascendencies.length > 0 ||
    filters.nameSearch.trim() !== ''
  );
};

const config: FilterStateConfig<CharacterFilters, SortOption> = {
  defaultFilters,
  defaultSort,
  hasActiveFiltersFn,
};

/**
 * Hook for managing character filter and sort state
 *
 * This hook provides the same interface as the original useCharacterFilters
 * but uses the generic useFilterState implementation internally.
 *
 * @returns Object containing filter state and management functions
 */
export function useCharacterFilters() {
  return useFilterState<CharacterFilters, SortOption>(config);
}
