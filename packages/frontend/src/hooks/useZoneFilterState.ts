import { useFilterState, createFilterStateConfig } from './useFilterState';
import type { LocationType } from '../types';

export interface ZoneFilters extends Record<string, unknown> {
  search: string;
  locationType: LocationType | 'All';
  act: string | 'All';
  isTown: boolean | null; // null = all, true = towns only, false = non-towns only
  isActive: boolean | null; // null = all, true = active only, false = inactive only
  minVisits: number | null;
  maxVisits: number | null;
  minDeaths: number | null;
  maxDeaths: number | null;
}

export interface ZoneSortOption {
  field:
    | 'last_visited'
    | 'duration'
    | 'visits'
    | 'deaths'
    | 'zone_level'
    | 'location_name'
    | 'first_visited';
  direction: 'asc' | 'desc';
}

const defaultFilters: ZoneFilters = {
  search: '',
  locationType: 'All',
  act: 'All',
  isTown: null,
  isActive: null,
  minVisits: null,
  maxVisits: null,
  minDeaths: null,
  maxDeaths: null,
};

const defaultSort: ZoneSortOption = {
  field: 'last_visited',
  direction: 'desc',
};

const hasActiveFiltersFn = (filters: ZoneFilters): boolean => {
  return (
    filters.search.trim() !== '' ||
    filters.locationType !== 'All' ||
    filters.act !== 'All' ||
    filters.isTown !== null ||
    filters.isActive !== null ||
    filters.minVisits !== null ||
    filters.maxVisits !== null ||
    filters.minDeaths !== null ||
    filters.maxDeaths !== null
  );
};

const config = createFilterStateConfig(
  defaultFilters,
  defaultSort,
  hasActiveFiltersFn
);

/**
 * Hook for managing zone filter and sort state
 * 
 * This hook provides the same interface as the original useZoneFilters
 * but uses the generic useFilterState implementation internally.
 * 
 * @returns Object containing filter state and management functions
 */
export function useZoneFilters() {
  return useFilterState<ZoneFilters, ZoneSortOption>(config);
}
