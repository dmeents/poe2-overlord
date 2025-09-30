import { useCallback, useMemo } from 'react';
import type { ZoneStats } from '../types';
import type { ZoneFilters, ZoneSortOption } from './useZoneFilterState';
import { useDataFiltering, FilterHelpers, SortHelpers } from './useDataFiltering';

/**
 * Zone-specific filter function that combines multiple filter criteria
 */
function createZoneFilterFunction(): (item: ZoneStats, filters: ZoneFilters) => boolean {
  return (item, filters) => {
    // Search filter
    if (filters.search.trim() !== '') {
      const searchTerm = filters.search.toLowerCase().trim();
      const searchableText = [
        item.location_name,
        item.act || '',
        item.location_type,
        item.zone_level ? `level ${item.zone_level}` : '',
      ]
        .join(' ')
        .toLowerCase();

      if (!searchableText.includes(searchTerm)) {
        return false;
      }
    }

    // Location type filter
    if (filters.locationType !== 'All' && item.location_type !== filters.locationType) {
      return false;
    }

    // Act filter
    if (filters.act !== 'All' && item.act !== filters.act) {
      return false;
    }

    // Town filter
    if (filters.isTown !== null && item.is_town !== filters.isTown) {
      return false;
    }

    // Active filter
    if (filters.isActive !== null && item.is_active !== filters.isActive) {
      return false;
    }

    // Visit count filters
    if (filters.minVisits !== null && item.visits < filters.minVisits) {
      return false;
    }
    if (filters.maxVisits !== null && item.visits > filters.maxVisits) {
      return false;
    }

    // Death count filters
    if (filters.minDeaths !== null && item.deaths < filters.minDeaths) {
      return false;
    }
    if (filters.maxDeaths !== null && item.deaths > filters.maxDeaths) {
      return false;
    }

    return true;
  };
}

/**
 * Zone-specific sort function
 */
function createZoneSortFunction(): (a: ZoneStats, b: ZoneStats, sort: ZoneSortOption) => number {
  return (a, b, sort) => {
    let comparison = 0;

    switch (sort.field) {
      case 'last_visited': {
        const aLastVisited = a.last_visited ? new Date(a.last_visited).getTime() : 0;
        const bLastVisited = b.last_visited ? new Date(b.last_visited).getTime() : 0;
        comparison = aLastVisited - bLastVisited;
        break;
      }
      case 'first_visited': {
        const aFirstVisited = a.first_visited ? new Date(a.first_visited).getTime() : 0;
        const bFirstVisited = b.first_visited ? new Date(b.first_visited).getTime() : 0;
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
      case 'zone_level': {
        const aLevel = a.zone_level || 0;
        const bLevel = b.zone_level || 0;
        comparison = aLevel - bLevel;
        break;
      }
      case 'location_name': {
        comparison = a.location_name.localeCompare(b.location_name);
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
 * Zone summary statistics calculation function
 */
function createZoneSummaryFunction(): (data: ZoneStats[], filteredData: ZoneStats[]) => any {
  return (data, filteredData) => {
    if (data.length === 0) {
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

    const totalTime = data.reduce((sum, zone) => sum + zone.duration, 0);
    const totalVisits = data.reduce((sum, zone) => sum + zone.visits, 0);
    const totalDeaths = data.reduce((sum, zone) => sum + zone.deaths, 0);

    const mostVisitedZone = data.reduce(
      (max, zone) => (zone.visits > max.visits ? zone : max),
      data[0]
    );

    const longestTimeZone = data.reduce(
      (max, zone) => (zone.duration > max.duration ? zone : max),
      data[0]
    );

    return {
      totalZones: data.length,
      totalTime,
      totalVisits,
      totalDeaths,
      averageTime: totalTime / data.length,
      mostVisitedZone,
      longestTimeZone,
    };
  };
}

/**
 * Hook for zone data filtering and sorting using the generic useDataFiltering
 * 
 * This hook replaces the old useZoneFiltering with a more generic approach
 * that leverages the useDataFiltering hook for better maintainability.
 * 
 * @param zones - Array of zone data to filter and sort
 * @param filters - Zone filter criteria
 * @param sort - Sort configuration
 * @returns Filtered and sorted zones with statistics
 */
export function useZoneDataFiltering(
  zones: ZoneStats[],
  filters: ZoneFilters,
  sort: ZoneSortOption
) {
  // Create memoized filter, sort, and summary functions
  const filterFunction = useCallback(createZoneFilterFunction(), []);
  const sortFunction = useCallback(createZoneSortFunction(), []);
  const summaryFunction = useCallback(createZoneSummaryFunction(), []);

  // Use the generic data filtering hook
  const { filteredData, count, totalCount, summary } = useDataFiltering({
    data: zones,
    filters,
    sort,
    filterFunction,
    sortFunction,
    summaryFunction,
  });

  return {
    filteredZones: filteredData,
    zoneCount: count,
    totalCount,
    summary,
  };
}
