import { act, renderHook } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { createMockCharacter, createMockSummary } from '../test/mock-data';
import { useCharacterList } from './useCharacterList';

describe('useCharacterList', () => {
  const mockCharacters = [
    createMockCharacter({
      id: '1',
      name: 'Alpha',
      class: 'Warrior',
      ascendency: 'Titan',
      level: 50,
      league: 'Standard',
      hardcore: false,
      solo_self_found: false,
      created_at: '2024-01-01T00:00:00Z',
      last_played: '2024-01-10T00:00:00Z',
      summary: createMockSummary({ total_play_time: 3600 }),
    }),
    createMockCharacter({
      id: '2',
      name: 'Beta',
      class: 'Sorceress',
      ascendency: 'Stormweaver',
      level: 75,
      league: 'Rise of the Abyssal',
      hardcore: true,
      solo_self_found: false,
      created_at: '2024-01-05T00:00:00Z',
      last_played: '2024-01-15T00:00:00Z',
      summary: createMockSummary({ total_play_time: 7200 }),
    }),
    createMockCharacter({
      id: '3',
      name: 'Gamma',
      class: 'Warrior',
      ascendency: 'Warbringer',
      level: 60,
      league: 'Standard',
      hardcore: false,
      solo_self_found: true,
      created_at: '2024-01-03T00:00:00Z',
      last_played: '2024-01-12T00:00:00Z',
      summary: createMockSummary({ total_play_time: 5400 }),
    }),
  ];

  describe('initial state', () => {
    it('shows all characters initially', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      expect(result.current.filteredCharacters).toHaveLength(3);
      expect(result.current.characterCount).toBe(3);
      expect(result.current.totalCount).toBe(3);
    });

    it('has no active filters initially', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      expect(result.current.hasActiveFilters).toBe(false);
    });

    it('sorts by last_played descending initially', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      expect(result.current.filteredCharacters[0].name).toBe('Beta');
      expect(result.current.filteredCharacters[1].name).toBe('Gamma');
      expect(result.current.filteredCharacters[2].name).toBe('Alpha');
    });
  });

  describe('league filter', () => {
    it('filters by Standard league', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('league', 'Standard');
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(result.current.filteredCharacters.every(c => c.league === 'Standard')).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by Rise of the Abyssal league', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('league', 'Rise of the Abyssal');
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].name).toBe('Beta');
    });
  });

  describe('hardcore filter', () => {
    it('filters hardcore only', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('hardcore', true);
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].hardcore).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters non-hardcore only', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('hardcore', false);
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(result.current.filteredCharacters.every(c => !c.hardcore)).toBe(true);
    });
  });

  describe('solo self found filter', () => {
    it('filters SSF only', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('soloSelfFound', true);
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].solo_self_found).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters non-SSF only', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('soloSelfFound', false);
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(result.current.filteredCharacters.every(c => !c.solo_self_found)).toBe(true);
    });
  });

  describe('class filter', () => {
    it('filters by single class', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('classes', ['Warrior']);
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(result.current.filteredCharacters.every(c => c.class === 'Warrior')).toBe(true);
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by multiple classes', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('classes', ['Warrior', 'Sorceress']);
      });

      expect(result.current.filteredCharacters).toHaveLength(3);
    });
  });

  describe('ascendency filter', () => {
    it('filters by single ascendency', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('ascendencies', ['Titan']);
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].ascendency).toBe('Titan');
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters by multiple ascendencies', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('ascendencies', ['Titan', 'Warbringer']);
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(result.current.filteredCharacters.every(c => c.class === 'Warrior')).toBe(true);
    });
  });

  describe('name search filter', () => {
    it('filters by partial name match', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('nameSearch', 'alp');
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].name).toBe('Alpha');
      expect(result.current.hasActiveFilters).toBe(true);
    });

    it('filters case-insensitively', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('nameSearch', 'BETA');
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].name).toBe('Beta');
    });

    it('trims whitespace', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('nameSearch', '  gamma  ');
      });

      expect(result.current.filteredCharacters).toHaveLength(1);
      expect(result.current.filteredCharacters[0].name).toBe('Gamma');
    });
  });

  describe('combined filters', () => {
    it('applies multiple filters together', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('league', 'Standard');
        result.current.updateFilter('classes', ['Warrior']);
      });

      expect(result.current.filteredCharacters).toHaveLength(2);
      expect(
        result.current.filteredCharacters.every(
          c => c.league === 'Standard' && c.class === 'Warrior',
        ),
      ).toBe(true);
    });
  });

  describe('sorting', () => {
    it('sorts by level ascending', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('level', 'asc');
      });

      expect(result.current.filteredCharacters[0].level).toBe(50);
      expect(result.current.filteredCharacters[1].level).toBe(60);
      expect(result.current.filteredCharacters[2].level).toBe(75);
    });

    it('sorts by level descending', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('level', 'desc');
      });

      expect(result.current.filteredCharacters[0].level).toBe(75);
      expect(result.current.filteredCharacters[2].level).toBe(50);
    });

    it('sorts by name alphabetically', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('name', 'asc');
      });

      expect(result.current.filteredCharacters[0].name).toBe('Alpha');
      expect(result.current.filteredCharacters[1].name).toBe('Beta');
      expect(result.current.filteredCharacters[2].name).toBe('Gamma');
    });

    it('sorts by created_at', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('created_at', 'asc');
      });

      expect(result.current.filteredCharacters[0].name).toBe('Alpha');
      expect(result.current.filteredCharacters[1].name).toBe('Gamma');
      expect(result.current.filteredCharacters[2].name).toBe('Beta');
    });

    it('sorts by play_time', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('play_time', 'asc');
      });

      expect(result.current.filteredCharacters[0].name).toBe('Alpha');
      expect(result.current.filteredCharacters[1].name).toBe('Gamma');
      expect(result.current.filteredCharacters[2].name).toBe('Beta');
    });

    it('toggles direction on same field', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('level');
      });

      expect(result.current.sort.direction).toBe('desc');

      act(() => {
        result.current.updateSort('level');
      });

      expect(result.current.sort.direction).toBe('asc');
    });
  });

  describe('clearFilters', () => {
    it('resets all filters to defaults', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateFilter('league', 'Standard');
        result.current.updateFilter('hardcore', true);
        result.current.updateFilter('nameSearch', 'test');
      });

      expect(result.current.hasActiveFilters).toBe(true);

      act(() => {
        result.current.clearFilters();
      });

      expect(result.current.hasActiveFilters).toBe(false);
      expect(result.current.filteredCharacters).toHaveLength(3);
    });
  });

  describe('resetSort', () => {
    it('resets sort to default', () => {
      const { result } = renderHook(() => useCharacterList(mockCharacters));

      act(() => {
        result.current.updateSort('level', 'asc');
      });

      act(() => {
        result.current.resetSort();
      });

      expect(result.current.sort).toEqual({ field: 'last_played', direction: 'desc' });
    });
  });

  describe('empty list', () => {
    it('handles empty character list', () => {
      const { result } = renderHook(() => useCharacterList([]));

      expect(result.current.filteredCharacters).toEqual([]);
      expect(result.current.characterCount).toBe(0);
      expect(result.current.totalCount).toBe(0);
    });
  });
});
