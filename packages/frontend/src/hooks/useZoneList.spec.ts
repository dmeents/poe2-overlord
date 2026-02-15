import { act, renderHook } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { createMockZone } from '../test/mock-data';
import { useZoneList } from './useZoneList';

describe('useZoneList', () => {
  const mockZones = [
    createMockZone({
      zone_name: 'The Coast',
      act: 1,
      is_town: false,
      is_active: false,
      duration: 300,
      visits: 2,
      deaths: 1,
      area_level: 2,
      has_waypoint: true,
      bosses: ['Hillock'],
      npcs: [],
      first_visited: '2024-01-01T10:00:00Z',
      last_visited: '2024-01-01T10:05:00Z',
    }),
    createMockZone({
      zone_name: 'Lioneye\'s Watch',
      act: 1,
      is_town: true,
      is_active: true,
      duration: 600,
      visits: 5,
      deaths: 0,
      area_level: undefined,
      has_waypoint: true,
      bosses: [],
      npcs: ['Tarkleigh', 'Nessa'],
      first_visited: '2024-01-01T09:00:00Z',
      last_visited: '2024-01-01T11:00:00Z',
    }),
    createMockZone({
      zone_name: 'The Mud Burrow',
      act: 1,
      is_town: false,
      is_active: false,
      duration: 450,
      visits: 3,
      deaths: 5,
      area_level: 4,
      has_waypoint: false,
      bosses: ['Hyrri'],
      npcs: [],
      first_visited: '2024-01-01T10:30:00Z',
      last_visited: '2024-01-01T11:30:00Z',
    }),
  ];

  describe('initial state', () => {
    it('shows all zones initially', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.filteredZones).toHaveLength(3);
      expect(result.current.zoneCount).toBe(3);
      expect(result.current.totalCount).toBe(3);
    });

    it('has no active filters initially', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.hasActiveFilters).toBe(false);
    });

    it('sorts by last_visited descending initially', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.filteredZones[0].zone_name).toBe('The Mud Burrow');
      expect(result.current.filteredZones[1].zone_name).toBe('Lioneye\'s Watch');
      expect(result.current.filteredZones[2].zone_name).toBe('The Coast');
    });
  });

  describe('search filter', () => {
    it('filters by zone name', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('search', 'coast');
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].zone_name).toBe('The Coast');
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by boss name', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('search', 'Hillock');
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].zone_name).toBe('The Coast');
    });

    it('filters by NPC name', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('search', 'Tarkleigh');
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].zone_name).toBe('Lioneye\'s Watch');
    });

    it('searches case-insensitively', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('search', 'COAST');
      });

      expect(result.current.filteredZones).toHaveLength(1);
    });
  });

  describe('act filter', () => {
    it('filters by act', () => {
      const zonesWithMultipleActs = [
        ...mockZones,
        createMockZone({ zone_name: 'Act 2 Zone', act: 2 }),
      ];

      const { result } = renderHook(() => useZoneList(zonesWithMultipleActs));

      act(() => {
        result.current.updateFilter('act', 'Act 2');
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].zone_name).toBe('Act 2 Zone');
      expect(result.current.hasActiveFilters).toBe(true);
    });
  });

  describe('isTown filter', () => {
    it('filters towns only', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('isTown', true);
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].is_town).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters non-towns only', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('isTown', false);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => !z.is_town)).toBe(true);
    });
  });

  describe('isActive filter', () => {
    it('filters active zones only', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('isActive', true);
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].is_active).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters inactive zones only', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('isActive', false);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => !z.is_active)).toBe(true);
    });
  });

  describe('visit count filters', () => {
    it('filters by minimum visits', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('minVisits', 3);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.visits >= 3)).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by maximum visits', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('maxVisits', 3);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.visits <= 3)).toBe(true);
    });

    it('filters by min and max visits', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('minVisits', 2);
        result.current.updateFilter('maxVisits', 3);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.visits >= 2 && z.visits <= 3)).toBe(true);
    });
  });

  describe('death count filters', () => {
    it('filters by minimum deaths', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('minDeaths', 1);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.deaths >= 1)).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by maximum deaths', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('maxDeaths', 1);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.deaths <= 1)).toBe(true);
    });
  });

  describe('hasBosses filter', () => {
    it('filters zones with bosses', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasBosses', true);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.bosses.length > 0)).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters zones without bosses', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasBosses', false);
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].bosses.length).toBe(0);
    });
  });

  describe('hasWaypoint filter', () => {
    it('filters zones with waypoints', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasWaypoint', true);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.has_waypoint)).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters zones without waypoints', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasWaypoint', false);
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].has_waypoint).toBe(false);
    });
  });

  describe('hasNpcs filter', () => {
    it('filters zones with NPCs', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasNpcs', true);
      });

      expect(result.current.filteredZones).toHaveLength(1);
      expect(result.current.filteredZones[0].npcs.length).toBeGreaterThan(0);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters zones without NPCs', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('hasNpcs', false);
      });

      expect(result.current.filteredZones).toHaveLength(2);
      expect(result.current.filteredZones.every(z => z.npcs.length === 0)).toBe(true);
    });
  });

  describe('sorting', () => {
    it('sorts by last_visited', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('last_visited', 'asc');
      });

      expect(result.current.filteredZones[0].zone_name).toBe('The Coast');
      expect(result.current.filteredZones[2].zone_name).toBe('The Mud Burrow');
    });

    it('sorts by first_visited', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('first_visited', 'asc');
      });

      expect(result.current.filteredZones[0].zone_name).toBe('Lioneye\'s Watch');
      expect(result.current.filteredZones[2].zone_name).toBe('The Mud Burrow');
    });

    it('sorts by duration', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('duration', 'asc');
      });

      expect(result.current.filteredZones[0].duration).toBe(300);
      expect(result.current.filteredZones[1].duration).toBe(450);
      expect(result.current.filteredZones[2].duration).toBe(600);
    });

    it('sorts by visits', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('visits', 'asc');
      });

      expect(result.current.filteredZones[0].visits).toBe(2);
      expect(result.current.filteredZones[1].visits).toBe(3);
      expect(result.current.filteredZones[2].visits).toBe(5);
    });

    it('sorts by deaths', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('deaths', 'asc');
      });

      expect(result.current.filteredZones[0].deaths).toBe(0);
      expect(result.current.filteredZones[1].deaths).toBe(1);
      expect(result.current.filteredZones[2].deaths).toBe(5);
    });

    it('sorts by area_level', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('area_level', 'asc');
      });

      // undefined area_level is treated as 0
      expect(result.current.filteredZones[0].zone_name).toBe('Lioneye\'s Watch');
      expect(result.current.filteredZones[1].area_level).toBe(2);
      expect(result.current.filteredZones[2].area_level).toBe(4);
    });

    it('sorts by zone_name', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('zone_name', 'asc');
      });

      expect(result.current.filteredZones[0].zone_name).toBe('Lioneye\'s Watch');
      expect(result.current.filteredZones[1].zone_name).toBe('The Coast');
      expect(result.current.filteredZones[2].zone_name).toBe('The Mud Burrow');
    });
  });

  describe('summary statistics', () => {
    it('calculates total zones correctly', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.totalZones).toBe(3);
    });

    it('calculates total time correctly', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.totalTime).toBe(1350); // 300 + 600 + 450
    });

    it('calculates total visits correctly', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.totalVisits).toBe(10); // 2 + 5 + 3
    });

    it('calculates total deaths correctly', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.totalDeaths).toBe(6); // 1 + 0 + 5
    });

    it('calculates average time correctly', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.averageTime).toBe(450); // 1350 / 3
    });

    it('identifies most visited zone', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.mostVisitedZone?.zone_name).toBe('Lioneye\'s Watch');
    });

    it('identifies longest time zone', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      expect(result.current.summary.longestTimeZone?.zone_name).toBe('Lioneye\'s Watch');
    });

    it('handles empty zone list', () => {
      const { result } = renderHook(() => useZoneList([]));

      expect(result.current.summary.totalZones).toBe(0);
      expect(result.current.summary.totalTime).toBe(0);
      expect(result.current.summary.averageTime).toBe(0);
      expect(result.current.summary.mostVisitedZone).toBeNull();
      expect(result.current.summary.longestTimeZone).toBeNull();
    });
  });

  describe('clearFilters', () => {
    it('resets all filters to defaults', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateFilter('search', 'test');
        result.current.updateFilter('isTown', true);
        result.current.updateFilter('minVisits', 2);
      });

      expect(result.current.hasActiveFilters).toBe(true);

      act(() => {
        result.current.clearFilters();
      });

      expect(result.current.hasActiveFilters).toBe(false);
      expect(result.current.filteredZones).toHaveLength(3);
    });
  });

  describe('resetSort', () => {
    it('resets sort to default', () => {
      const { result } = renderHook(() => useZoneList(mockZones));

      act(() => {
        result.current.updateSort('duration', 'asc');
      });

      act(() => {
        result.current.resetSort();
      });

      expect(result.current.sort).toEqual({ field: 'last_visited', direction: 'desc' });
    });
  });
});
