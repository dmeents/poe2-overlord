import type { CurrencyExchangeRate } from '@/types/economy';
import { useCurrencyList } from '@/hooks/useCurrencyList';
import { EconomyRow } from '../economy-row/economy-row';
import { CurrencyListControlsForm } from '../currency-list-controls-form/currency-list-controls-form';

interface EconomyListProps {
  currencies: CurrencyExchangeRate[];
  onCurrencyClick?: (currency: CurrencyExchangeRate) => void;
}

export function EconomyList({ currencies, onCurrencyClick }: EconomyListProps) {
  // Currency list with filtering and sorting
  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters,
    filteredCurrencies,
    currencyCount,
    totalCount,
  } = useCurrencyList(currencies);

  return (
    <div>
      {/* Controls */}
      <div className='mb-4'>
        <CurrencyListControlsForm
          filters={filters}
          onFilterChange={updateFilter}
          onClearFilters={clearFilters}
          hasActiveFilters={hasActiveFilters}
          sort={sort}
          onSortChange={updateSort}
          onResetSort={resetSort}
          currencyCount={currencyCount}
          totalCount={totalCount}
        />
      </div>

      {/* Currency List */}
      {filteredCurrencies.length === 0 ? (
        <div className='text-center py-8 text-zinc-400'>
          {hasActiveFilters
            ? 'No currencies match your search'
            : 'No currency data available'}
        </div>
      ) : (
        <div>
          {filteredCurrencies.map((currency, index) => (
            <div
              key={currency.id}
              className={index === 0 ? 'border-t border-zinc-700/50' : ''}
            >
              <EconomyRow currency={currency} onClick={onCurrencyClick} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
