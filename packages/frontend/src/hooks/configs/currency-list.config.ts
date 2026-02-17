import type { CurrencyExchangeRate } from '../../types/economy';

export type CurrencySortField = 'name' | 'primary_value' | 'volume' | 'change_percent';

// Sort-only config for economy (no filters)
export const currencyListConfig = {
  defaultSort: {
    field: 'primary_value' as CurrencySortField,
    direction: 'desc' as const,
  },

  sortFn: (
    a: CurrencyExchangeRate,
    b: CurrencyExchangeRate,
    sort: { field: CurrencySortField; direction: 'asc' | 'desc' },
  ): number => {
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
  },
};
