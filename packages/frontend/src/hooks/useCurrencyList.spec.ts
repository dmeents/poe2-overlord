import { act, renderHook } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { createMockCurrency } from '../test/mock-data';
import { useCurrencyList } from './useCurrencyList';

describe('useCurrencyList', () => {
  const mockCurrencies = [
    createMockCurrency({ id: '1', name: 'Alpha', primary_value: 100, volume: 500, change_percent: 5 }),
    createMockCurrency({ id: '2', name: 'Beta', primary_value: 50, volume: 1000, change_percent: -3 }),
    createMockCurrency({ id: '3', name: 'Gamma', primary_value: 200, volume: 250, change_percent: 10 }),
  ];

  describe('initial state', () => {
    it('defaults to primary_value descending sort', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      expect(result.current.sort).toEqual({
        field: 'primary_value',
        direction: 'desc',
      });
    });

    it('sorts by primary_value descending initially', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      expect(result.current.sortedCurrencies[0].name).toBe('Gamma');
      expect(result.current.sortedCurrencies[1].name).toBe('Alpha');
      expect(result.current.sortedCurrencies[2].name).toBe('Beta');
    });
  });

  describe('updateSort', () => {
    it('sorts by name ascending', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('name', 'asc');
      });

      expect(result.current.sortedCurrencies[0].name).toBe('Alpha');
      expect(result.current.sortedCurrencies[1].name).toBe('Beta');
      expect(result.current.sortedCurrencies[2].name).toBe('Gamma');
    });

    it('sorts by name descending', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('name', 'desc');
      });

      expect(result.current.sortedCurrencies[0].name).toBe('Gamma');
      expect(result.current.sortedCurrencies[1].name).toBe('Beta');
      expect(result.current.sortedCurrencies[2].name).toBe('Alpha');
    });

    it('sorts by primary_value ascending', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('primary_value', 'asc');
      });

      expect(result.current.sortedCurrencies[0].primary_value).toBe(50);
      expect(result.current.sortedCurrencies[1].primary_value).toBe(100);
      expect(result.current.sortedCurrencies[2].primary_value).toBe(200);
    });

    it('sorts by volume ascending', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('volume', 'asc');
      });

      expect(result.current.sortedCurrencies[0].volume).toBe(250);
      expect(result.current.sortedCurrencies[1].volume).toBe(500);
      expect(result.current.sortedCurrencies[2].volume).toBe(1000);
    });

    it('sorts by change_percent ascending', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('change_percent', 'asc');
      });

      expect(result.current.sortedCurrencies[0].change_percent).toBe(-3);
      expect(result.current.sortedCurrencies[1].change_percent).toBe(5);
      expect(result.current.sortedCurrencies[2].change_percent).toBe(10);
    });

    it('toggles direction when clicking same field', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('name');
      });

      expect(result.current.sort.direction).toBe('desc');

      act(() => {
        result.current.updateSort('name');
      });

      expect(result.current.sort.direction).toBe('asc');
    });

    it('handles null volume values', () => {
      const currenciesWithNulls = [
        createMockCurrency({ name: 'A', volume: 100 }),
        createMockCurrency({ name: 'B', volume: null }),
        createMockCurrency({ name: 'C', volume: 50 }),
      ];

      const { result } = renderHook(() => useCurrencyList(currenciesWithNulls));

      act(() => {
        result.current.updateSort('volume', 'asc');
      });

      expect(result.current.sortedCurrencies[0].name).toBe('B'); // null treated as 0
      expect(result.current.sortedCurrencies[1].name).toBe('C');
      expect(result.current.sortedCurrencies[2].name).toBe('A');
    });

    it('handles null change_percent values', () => {
      const currenciesWithNulls = [
        createMockCurrency({ name: 'A', change_percent: 5 }),
        createMockCurrency({ name: 'B', change_percent: null }),
        createMockCurrency({ name: 'C', change_percent: -5 }),
      ];

      const { result } = renderHook(() => useCurrencyList(currenciesWithNulls));

      act(() => {
        result.current.updateSort('change_percent', 'asc');
      });

      expect(result.current.sortedCurrencies[0].name).toBe('C');
      expect(result.current.sortedCurrencies[1].name).toBe('B'); // null treated as 0
      expect(result.current.sortedCurrencies[2].name).toBe('A');
    });
  });

  describe('resetSort', () => {
    it('resets to default sort', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      act(() => {
        result.current.updateSort('name', 'asc');
      });

      expect(result.current.sort).toEqual({ field: 'name', direction: 'asc' });

      act(() => {
        result.current.resetSort();
      });

      expect(result.current.sort).toEqual({ field: 'primary_value', direction: 'desc' });
    });
  });

  describe('currency counts', () => {
    it('returns correct currency count', () => {
      const { result } = renderHook(() => useCurrencyList(mockCurrencies));

      expect(result.current.currencyCount).toBe(3);
      expect(result.current.totalCount).toBe(3);
    });

    it('handles empty currency list', () => {
      const { result } = renderHook(() => useCurrencyList([]));

      expect(result.current.currencyCount).toBe(0);
      expect(result.current.totalCount).toBe(0);
      expect(result.current.sortedCurrencies).toEqual([]);
    });
  });
});
