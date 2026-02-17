import type { ZoneStats } from '../../types/character';
import { getDisplayAct } from '../../utils/zone-utils';

export interface ZoneFilters {
  search: string;
  act: string | 'All';
  isTown: boolean | null;
  isActive: boolean | null;
}

export type ZoneSortField =
  | 'last_visited'
  | 'duration'
  | 'visits'
  | 'deaths'
  | 'area_level'
  | 'zone_name'
  | 'first_visited';

export const zoneListConfig = {
  defaultFilters: {
    search: '',
    act: 'All' as string | 'All',
    isTown: null,
    isActive: null,
  },

  defaultSort: {
    field: 'last_visited' as ZoneSortField,
    direction: 'desc' as const,
  },

  filterFn: (zone: ZoneStats, filters: ZoneFilters): boolean => {
    // Search filter - includes zone name, act, town status, level, description, bosses, NPCs, POIs
    if (filters.search.trim() !== '') {
      const searchTerm = filters.search.toLowerCase().trim();
      const searchableText = [
        zone.zone_name,
        getDisplayAct(zone) || '',
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
      const zoneActString = getDisplayAct(zone) || '';
      if (zoneActString !== filters.act) return false;
    }

    // Town filter
    if (filters.isTown !== null && zone.is_town !== filters.isTown) {
      return false;
    }

    // Active filter
    if (filters.isActive !== null && zone.is_active !== filters.isActive) {
      return false;
    }

    return true;
  },

  sortFn: (
    a: ZoneStats,
    b: ZoneStats,
    sort: { field: ZoneSortField; direction: 'asc' | 'desc' },
  ): number => {
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
  },

  chipConfigs: [
    {
      key: 'act' as const,
      label: (value: string | 'All') => value,
      isActive: (value: string | 'All') => value !== 'All',
    },
    {
      key: 'isTown' as const,
      label: (value: boolean | null) => (value ? 'Towns Only' : 'Zones Only'),
      isActive: (value: boolean | null) => value !== null,
    },
    {
      key: 'isActive' as const,
      label: (value: boolean | null) => (value ? 'Active' : 'Inactive'),
      isActive: (value: boolean | null) => value !== null,
    },
  ],
};
