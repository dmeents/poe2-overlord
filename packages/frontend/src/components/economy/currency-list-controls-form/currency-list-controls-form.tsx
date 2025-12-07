import { memo } from 'react';
import { Input } from '../../forms/form-input/form-input';
import { SortSelect } from '../../forms/form-sort-select/form-sort-select';
import { Button } from '../../ui/button/button';
import { useEconomy } from '@/contexts/EconomyContext';
import {
  ECONOMY_TYPE_ICONS,
  ECONOMY_TYPE_LABELS,
  ECONOMY_TYPES,
} from '@/utils/economy-icons';
import type {
  CurrencyFilters as CurrencyFiltersType,
  CurrencySortOption,
} from '@/hooks/useCurrencyList';
import {
  controlsContainerClasses,
  searchInputContainerClasses,
  sortSelectContainerClasses,
  countDisplayClasses,
  clearButtonClasses,
} from './currency-list-controls-form.styles';

interface CurrencyListControlsFormProps {
  filters: CurrencyFiltersType;
  onFilterChange: <K extends keyof CurrencyFiltersType>(
    key: K,
    value: CurrencyFiltersType[K]
  ) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: CurrencySortOption;
  onSortChange: (
    field: CurrencySortOption['field'],
    direction?: CurrencySortOption['direction']
  ) => void;
  onResetSort: () => void;
  currencyCount: number;
  totalCount: number;
}

const SORT_OPTIONS = [
  { value: 'primary_value', label: 'Value' },
  { value: 'name', label: 'Name' },
  { value: 'volume', label: 'Volume' },
  { value: 'change_percent', label: 'Change %' },
];

export const CurrencyListControlsForm = memo(function CurrencyListControlsForm({
  filters,
  onFilterChange,
  onClearFilters,
  hasActiveFilters,
  sort,
  onSortChange,
  onResetSort,
  currencyCount,
  totalCount,
}: CurrencyListControlsFormProps) {
  const { selectedEconomyType, setSelectedEconomyType } = useEconomy();
  return (
    <div className='space-y-4 p-4'>
      <div className='flex flex-wrap gap-2'>
        {ECONOMY_TYPES.map(type => (
          <Button
            key={type}
            variant={selectedEconomyType === type ? 'primary' : 'outline'}
            size='sm'
            onClick={() => setSelectedEconomyType(type)}
          >
            <img
              src={ECONOMY_TYPE_ICONS[type]}
              alt={ECONOMY_TYPE_LABELS[type]}
              className='w-4 h-4 mr-2'
              onError={e => {
                e.currentTarget.style.display = 'none';
              }}
            />
            {ECONOMY_TYPE_LABELS[type]}
          </Button>
        ))}
      </div>
      <div className={controlsContainerClasses}>
        <div className={searchInputContainerClasses}>
          <Input
            id='currency-search'
            type='text'
            placeholder='Search currencies...'
            value={filters.search}
            onChange={value => onFilterChange('search', String(value || ''))}
          />
        </div>
        <div className={sortSelectContainerClasses}>
          <SortSelect
            id='currency-sort'
            value={sort.field}
            direction={sort.direction}
            options={SORT_OPTIONS}
            onChange={(field, direction) =>
              onSortChange(field as CurrencySortOption['field'], direction)
            }
            onReset={onResetSort}
          />
        </div>
      </div>
      <div className='flex items-center'>
        {hasActiveFilters && (
          <button
            onClick={onClearFilters}
            className={clearButtonClasses}
            type='button'
          >
            Clear search
          </button>
        )}

        <div className={countDisplayClasses}>
          {currencyCount === totalCount ? (
            <span>Showing all {totalCount} items</span>
          ) : (
            <span>
              Showing {currencyCount} of {totalCount} items
            </span>
          )}
        </div>
      </div>
    </div>
  );
});
