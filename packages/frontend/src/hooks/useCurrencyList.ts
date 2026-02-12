import { useCallback, useMemo, useState } from 'react';
import type { CurrencyExchangeRate } from '../types/economy';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type CurrencyFilters = {};

export interface CurrencySortOption {
  field: 'name' | 'primary_value' | 'volume' | 'change_percent';
  direction: 'asc' | 'desc';
}

const defaultFilters: CurrencyFilters = {};

const defaultSort: CurrencySortOption = {
  field: 'primary_value',
  direction: 'desc',
};

export function useCurrencyList(currencies: CurrencyExchangeRate[]) {
  const [filters, setFilters] = useState<CurrencyFilters>(defaultFilters);
  const [sort, setSort] = useState<CurrencySortOption>(defaultSort);

  const updateFilter = useCallback(
    <K extends keyof CurrencyFilters>(key: K, value: CurrencyFilters[K]) => {
      setFilters(prev => ({ ...prev, [key]: value }));
    },
    [],
  );

  const updateSort = useCallback(
    (field: CurrencySortOption['field'], direction?: CurrencySortOption['direction']) => {
      setSort(prev => ({
        field,
        direction:
          direction ?? (prev.field === field && prev.direction === 'desc' ? 'asc' : 'desc'),
      }));
    },
    [],
  );

  const clearFilters = useCallback(() => {
    setFilters(defaultFilters);
  }, []);

  const resetSort = useCallback(() => {
    setSort(defaultSort);
  }, []);

  const { filteredCurrencies, currencyCount, totalCount, hasActiveFilters } = useMemo(() => {
    // Apply filters (currently none - just use all currencies)
    const filtered = [...currencies];

    // Sort the filtered currencies
    filtered.sort((a, b) => {
      let comparison = 0;

      switch (sort.field) {
        case 'name': {
          comparison = a.name.localeCompare(b.name);
          break;
        }
        case 'primary_value': {
          comparison = a.primary_value - b.primary_value;
          break;
        }
        case 'volume': {
          const aVolume = a.volume ?? 0;
          const bVolume = b.volume ?? 0;
          comparison = aVolume - bVolume;
          break;
        }
        case 'change_percent': {
          const aChange = a.change_percent ?? 0;
          const bChange = b.change_percent ?? 0;
          comparison = aChange - bChange;
          break;
        }
        default: {
          comparison = 0;
        }
      }

      return sort.direction === 'asc' ? comparison : -comparison;
    });

    // Check if any filters are active
    const hasActive = false;

    return {
      filteredCurrencies: filtered,
      currencyCount: filtered.length,
      totalCount: currencies.length,
      hasActiveFilters: hasActive,
    };
  }, [currencies, sort]);

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
    filteredCurrencies,
    currencyCount,
    totalCount,
    hasActiveFilters,
  };
}
