import { XMarkIcon } from '@heroicons/react/20/solid';
import { memo } from 'react';
import { useEconomy } from '@/contexts/EconomyContext';
import type { CurrencySortOption } from '@/hooks/useCurrencyList';
import { ECONOMY_TYPE_ICONS, ECONOMY_TYPE_LABELS, ECONOMY_TYPES } from '@/utils/economy-icons';
import { Input } from '../../forms/form-input/form-input';
import { SortSelect } from '../../forms/form-sort-select/form-sort-select';
import { Button } from '../../ui/button/button';
import {
  clearButtonClasses,
  controlsContainerClasses,
  countDisplayClasses,
  searchInputContainerClasses,
  sortSelectContainerClasses,
} from './currency-list-controls-form.styles';

interface CurrencyListControlsFormProps {
  searchQuery: string;
  onSearchChange: (value: string) => void;
  isSearching: boolean;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
  sort: CurrencySortOption;
  onSortChange: (
    field: CurrencySortOption['field'],
    direction?: CurrencySortOption['direction'],
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
  searchQuery,
  onSearchChange,
  isSearching,
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
    <div className="space-y-4 p-4">
      <div className="flex flex-wrap gap-2">
        {ECONOMY_TYPES.map(type => (
          <Button
            key={type}
            variant={selectedEconomyType === type ? 'primary' : 'outline'}
            size="sm"
            onClick={() => {
              setSelectedEconomyType(type);
              if (searchQuery) {
                onSearchChange('');
              }
            }}>
            <img
              src={ECONOMY_TYPE_ICONS[type]}
              alt={ECONOMY_TYPE_LABELS[type]}
              className="w-4 h-4 mr-2"
              onError={e => {
                e.currentTarget.style.display = 'none';
              }}
            />
            {ECONOMY_TYPE_LABELS[type]}
          </Button>
        ))}
      </div>
      <div className={controlsContainerClasses}>
        <div className={`${searchInputContainerClasses} relative`}>
          <Input
            id="currency-search"
            type="text"
            placeholder="Search all currencies..."
            value={searchQuery}
            onChange={onSearchChange}
          />
          {searchQuery && (
            <Button
              variant="icon"
              size="sm"
              onClick={() => onSearchChange('')}
              className="absolute right-2 top-1/2 -translate-y-1/2"
              title="Clear search">
              <XMarkIcon className="w-4 h-4" />
            </Button>
          )}
        </div>
        <div className={sortSelectContainerClasses}>
          <SortSelect
            id="currency-sort"
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
      <div className="flex items-center">
        {hasActiveFilters && (
          <button onClick={onClearFilters} className={clearButtonClasses} type="button">
            Clear search
          </button>
        )}

        <div className={countDisplayClasses}>
          {isSearching ? (
            <span>Searching...</span>
          ) : searchQuery ? (
            <span>
              Found {currencyCount} result{currencyCount !== 1 ? 's' : ''} across all types
            </span>
          ) : currencyCount === totalCount ? (
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
