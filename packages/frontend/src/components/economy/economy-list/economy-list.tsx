import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useCurrencyList } from '@/hooks/useCurrencyList';
import type { CurrencyExchangeRate } from '@/types/economy';
import { CurrencyListControlsForm } from '../currency-list-controls-form/currency-list-controls-form';
import { EconomyRow } from '../economy-row/economy-row';
import { economyListStyles } from './economy-list.styles';

interface EconomyListProps {
  currencies: CurrencyExchangeRate[];
  onCurrencyClick?: (currency: CurrencyExchangeRate) => void;
  searchQuery: string;
  onSearchChange: (value: string) => void;
  isSearching: boolean;
  searchResultsCount: number;
}

export function EconomyList({
  currencies,
  onCurrencyClick,
  searchQuery,
  onSearchChange,
  isSearching,
  searchResultsCount,
}: EconomyListProps) {
  // Currency list with sorting
  const { sort, updateSort, resetSort, sortedCurrencies, currencyCount, totalCount } =
    useCurrencyList(currencies);

  return (
    <div>
      {/* Controls */}
      <div className={economyListStyles.controlsContainer}>
        <CurrencyListControlsForm
          searchQuery={searchQuery}
          onSearchChange={onSearchChange}
          isSearching={isSearching}
          sort={sort}
          onSortChange={updateSort}
          onResetSort={resetSort}
          currencyCount={searchQuery ? searchResultsCount : currencyCount}
          totalCount={totalCount}
        />
      </div>

      {/* Currency List */}
      {isSearching ? (
        <LoadingSpinner className="py-12" />
      ) : sortedCurrencies.length === 0 ? (
        <div className={economyListStyles.emptyState}>
          {searchQuery
            ? `No currencies found matching "${searchQuery}"`
            : 'No currency data available'}
        </div>
      ) : (
        <div>
          {sortedCurrencies.map((currency, index) => (
            <div key={currency.id} className={index === 0 ? economyListStyles.firstRow : ''}>
              <EconomyRow currency={currency} onClick={onCurrencyClick} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
