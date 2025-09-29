import { useCallback, useState } from 'react';
import type { LocationType } from '../types';

export interface ZoneFilters {
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

export function useZoneFilters() {
  const [filters, setFilters] = useState<ZoneFilters>(defaultFilters);
  const [sort, setSort] = useState<ZoneSortOption>(defaultSort);

  const updateFilter = useCallback(
    <K extends keyof ZoneFilters>(key: K, value: ZoneFilters[K]) => {
      setFilters(prev => ({ ...prev, [key]: value }));
    },
    []
  );

  const updateSort = useCallback(
    (
      field: ZoneSortOption['field'],
      direction?: ZoneSortOption['direction']
    ) => {
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
