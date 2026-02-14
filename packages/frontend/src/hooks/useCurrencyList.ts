import { useCallback, useMemo, useState } from 'react';
import type { CurrencyExchangeRate } from '../types/economy';

export interface CurrencySortOption {
  field: 'name' | 'primary_value' | 'volume' | 'change_percent';
  direction: 'asc' | 'desc';
}

const defaultSort: CurrencySortOption = {
  field: 'primary_value',
  direction: 'desc',
};

export function useCurrencyList(currencies: CurrencyExchangeRate[]) {
  const [sort, setSort] = useState<CurrencySortOption>(defaultSort);

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

  const resetSort = useCallback(() => {
    setSort(defaultSort);
  }, []);

  const { sortedCurrencies, currencyCount, totalCount } = useMemo(() => {
    const sorted = [...currencies];

    sorted.sort((a, b) => {
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

    return {
      sortedCurrencies: sorted,
      currencyCount: sorted.length,
      totalCount: currencies.length,
    };
  }, [currencies, sort]);

  return {
    sort,
    updateSort,
    resetSort,
    sortedCurrencies,
    currencyCount,
    totalCount,
  };
}
