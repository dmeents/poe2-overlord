import { useCallback, useMemo, useState } from 'react';
import type { ZoneStats } from '../types/character';

export interface ZoneFilters {
  search: string;
  act: string | 'All';
  isTown: boolean | null; // null = all, true = towns only, false = non-towns only
  isActive: boolean | null; // null = all, true = active only, false = inactive only
  minVisits: number | null;
  maxVisits: number | null;
  minDeaths: number | null;
  maxDeaths: number | null;
  hasBosses: boolean | null; // null = all, true = has bosses, false = no bosses
  hasWaypoint: boolean | null; // null = all, true = has waypoint, false = no waypoint
  hasNpcs: boolean | null; // null = all, true = has NPCs, false = no NPCs
}

export interface ZoneSortOption {
  field:
    | 'last_visited'
    | 'duration'
    | 'visits'
    | 'deaths'
    | 'area_level'
    | 'zone_name'
    | 'first_visited';
  direction: 'asc' | 'desc';
}

const defaultFilters: ZoneFilters = {
  search: '',
  act: 'All',
  isTown: null,
  isActive: null,
  minVisits: null,
  maxVisits: null,
  minDeaths: null,
  maxDeaths: null,
  hasBosses: null,
  hasWaypoint: null,
  hasNpcs: null,
};

const defaultSort: ZoneSortOption = {
  field: 'last_visited',
  direction: 'desc',
};

export function useZoneList(zones: ZoneStats[]) {
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

  // Calculate summary statistics from original zones (not filtered)
  const summary = useMemo(() => {
    if (zones.length === 0) {
      return {
        totalZones: 0,
        totalTime: 0,
        totalVisits: 0,
        totalDeaths: 0,
        averageTime: 0,
        mostVisitedZone: null as ZoneStats | null,
        longestTimeZone: null as ZoneStats | null,
      };
    }

    const totalTime = zones.reduce((sum, zone) => sum + zone.duration, 0);
    const totalVisits = zones.reduce((sum, zone) => sum + zone.visits, 0);
    const totalDeaths = zones.reduce((sum, zone) => sum + zone.deaths, 0);

    const mostVisitedZone = zones.reduce(
      (max, zone) => (zone.visits > max.visits ? zone : max),
      zones[0]
    );

    const longestTimeZone = zones.reduce(
      (max, zone) => (zone.duration > max.duration ? zone : max),
      zones[0]
    );

    return {
      totalZones: zones.length,
      totalTime,
      totalVisits,
      totalDeaths,
      averageTime: totalTime / zones.length,
      mostVisitedZone,
      longestTimeZone,
    };
  }, [zones]);

  const { filteredZones, zoneCount, totalCount, hasActiveFilters } = useMemo(() => {
    // Apply filters
    const filtered = zones.filter(zone => {
      // Search filter - now includes bosses, NPCs, and description
      if (filters.search.trim() !== '') {
        const searchTerm = filters.search.toLowerCase().trim();
        const searchableText = [
          zone.zone_name,
          zone.act ? `Act ${zone.act}` : '',
          zone.is_town ? 'town' : 'zone',
          zone.area_level ? `level ${zone.area_level}` : '',
          zone.description || '',
          ...(zone.bosses || []),
          ...(zone.npcs || []),
          ...(zone.points_of_interest || []),
        ]
          .join(' ')
          .toLowerCase();

        if (!searchableText.includes(searchTerm)) {
          return false;
        }
      }

      // Act filter
      if (filters.act !== 'All') {
        const zoneActString = zone.act ? `Act ${zone.act}` : '';
        if (zoneActString !== filters.act) {
          return false;
        }
      }

      // Town filter
      if (filters.isTown !== null && zone.is_town !== filters.isTown) {
        return false;
      }

      // Active filter
      if (filters.isActive !== null && zone.is_active !== filters.isActive) {
        return false;
      }

      // Visit count filters
      if (filters.minVisits !== null && zone.visits < filters.minVisits) {
        return false;
      }
      if (filters.maxVisits !== null && zone.visits > filters.maxVisits) {
        return false;
      }

      // Death count filters
      if (filters.minDeaths !== null && zone.deaths < filters.minDeaths) {
        return false;
      }
      if (filters.maxDeaths !== null && zone.deaths > filters.maxDeaths) {
        return false;
      }

      // Bosses filter
      if (filters.hasBosses !== null) {
        const hasBosses = zone.bosses && zone.bosses.length > 0;
        if (hasBosses !== filters.hasBosses) {
          return false;
        }
      }

      // Waypoint filter
      if (
        filters.hasWaypoint !== null &&
        zone.has_waypoint !== filters.hasWaypoint
      ) {
        return false;
      }

      // NPCs filter
      if (filters.hasNpcs !== null) {
        const hasNpcs = zone.npcs && zone.npcs.length > 0;
        if (hasNpcs !== filters.hasNpcs) {
          return false;
        }
      }

      return true;
    });

    // Sort the filtered zones
    filtered.sort((a, b) => {
      let comparison = 0;

      switch (sort.field) {
        case 'last_visited': {
          const aLastVisited = a.last_visited
            ? new Date(a.last_visited).getTime()
            : 0;
          const bLastVisited = b.last_visited
            ? new Date(b.last_visited).getTime()
            : 0;
          comparison = aLastVisited - bLastVisited;
          break;
        }
        case 'first_visited': {
          const aFirstVisited = a.first_visited
            ? new Date(a.first_visited).getTime()
            : 0;
          const bFirstVisited = b.first_visited
            ? new Date(b.first_visited).getTime()
            : 0;
          comparison = aFirstVisited - bFirstVisited;
          break;
        }
        case 'duration': {
          comparison = a.duration - b.duration;
          break;
        }
        case 'visits': {
          comparison = a.visits - b.visits;
          break;
        }
        case 'deaths': {
          comparison = a.deaths - b.deaths;
          break;
        }
        case 'area_level': {
          const aLevel = a.area_level || 0;
          const bLevel = b.area_level || 0;
          comparison = aLevel - bLevel;
          break;
        }
        case 'zone_name': {
          comparison = a.zone_name.localeCompare(b.zone_name);
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
      filters.search.trim() !== '' ||
      filters.act !== 'All' ||
      filters.isTown !== null ||
      filters.isActive !== null ||
      filters.minVisits !== null ||
      filters.maxVisits !== null ||
      filters.minDeaths !== null ||
      filters.maxDeaths !== null ||
      filters.hasBosses !== null ||
      filters.hasWaypoint !== null ||
      filters.hasNpcs !== null;

    return {
      filteredZones: filtered,
      zoneCount: filtered.length,
      totalCount: zones.length,
      hasActiveFilters: hasActive,
    };
  }, [zones, filters, sort]);

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
    filteredZones,
    zoneCount,
    totalCount,
    hasActiveFilters,
    summary,
  };
}
