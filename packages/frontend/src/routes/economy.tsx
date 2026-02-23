import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { EconomyRow } from '@/components/economy/economy-row/economy-row';
import { EconomyTypeBar } from '@/components/economy/economy-type-bar/economy-type-bar';
import { ExchangeRatesCard } from '@/components/economy/exchange-rates-card/exchange-rates-card';
import { TopItemsCard } from '@/components/economy/top-items-card/top-items-card';
import { ListControlBar } from '@/components/forms/list-control-bar/list-control-bar';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { Card } from '@/components/ui/card/card';
import { ErrorState } from '@/components/ui/error-state/error-state';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
import { type CurrencySortField, currencyListConfig } from '@/hooks/configs/currency-list.config';
import { useListControls } from '@/hooks/useListControls';
import { useSearchCurrencies } from '@/queries/economy';
import { searchResultToExchangeRate } from '@/types/economy';
import { ECONOMY_TYPE_ICONS, ECONOMY_TYPE_LABELS, ECONOMY_TYPES } from '@/utils/economy-icons';
import { formatTimeAgo } from '@/utils/format-time-ago';
import { hideOnError } from '@/utils/image-utils';

export const Route = createFileRoute('/economy')({
  component: EconomyPage,
});

function EconomyPage() {
  const {
    currencyData,
    isLoading,
    isError,
    error,
    selectedEconomyType,
    setSelectedEconomyType,
    league,
    isHardcore,
    isSoloSelfFound,
  } = useEconomy();

  // Check if error is due to no data available for this economy type
  const isNoDataError =
    isError &&
    error?.message &&
    (error.message.includes('No currency data available') || error.message.includes('not found'));

  const [searchQuery, setSearchQuery] = useState('');

  const { data: searchResults = [], isLoading: isSearching } = useSearchCurrencies(
    league,
    isHardcore,
    searchQuery,
    searchQuery.length > 0,
  );

  // Determine the subtitle based on data freshness
  const getSubtitle = () => {
    if (isSoloSelfFound) {
      return `${league} - Trading not available in SSF`;
    }
    if (!currencyData) {
      return `${isHardcore ? 'HC ' : ''}${league}`;
    }

    const leaguePrefix = `${isHardcore ? 'HC ' : ''}${league}`;

    if (currencyData.is_stale) {
      return `${leaguePrefix} • Last known data from ${formatTimeAgo(currencyData.fetched_at)}`;
    }

    return `${leaguePrefix} • Updated ${formatTimeAgo(currencyData.fetched_at)}`;
  };

  // Use currency list hook for sorting (only when we have data)
  const {
    sort,
    updateSort,
    resetSort,
    result: sortedCurrencies,
    filteredCount,
    totalCount,
  } = useListControls(currencyData?.currencies || [], currencyListConfig);

  const typeOptions = ECONOMY_TYPES.map(type => ({
    value: type,
    label: ECONOMY_TYPE_LABELS[type],
    icon: (
      <img
        src={ECONOMY_TYPE_ICONS[type]}
        alt={ECONOMY_TYPE_LABELS[type]}
        className="w-3.5 h-3.5"
        onError={hideOnError}
      />
    ),
  }));

  // Render list content based on state
  const renderListContent = () => {
    if (isLoading) {
      return <LoadingSpinner className="py-12" />;
    }

    if (isError) {
      if (isNoDataError) {
        return (
          <div className="text-center py-12">
            <div className="text-stone-400 mb-2">
              <svg
                className="mx-auto h-12 w-12 mb-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                aria-hidden="true">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
                />
              </svg>
            </div>
            <h3 className="text-lg font-medium text-stone-300 mb-2">No Data Available</h3>
            <p className="text-sm text-stone-400 max-w-md mx-auto">
              {ECONOMY_TYPE_LABELS[selectedEconomyType]} data is not available for this league yet.
              Try selecting a different economy type or check back later.
            </p>
          </div>
        );
      }
      return <ErrorState title="Error loading economy data" error={error} />;
    }

    if (!currencyData) {
      return <div className="text-stone-400 text-center py-8">No economy data available</div>;
    }

    if (searchQuery) {
      // Show search results
      return (
        <div>
          {isSearching ? (
            <LoadingSpinner className="py-12" />
          ) : searchResults.length === 0 ? (
            <div className="text-center py-8 text-stone-400">
              No currencies found matching "{searchQuery}"
            </div>
          ) : (
            <div>
              {searchResults.map((result, index) => (
                <div key={result.id} className={index === 0 ? 'border-t border-stone-700/50' : ''}>
                  <EconomyRow currency={searchResultToExchangeRate(result)} />
                </div>
              ))}
            </div>
          )}
        </div>
      );
    }

    // Show normal economy list (with filtering and sorting applied)
    return (
      <div>
        {sortedCurrencies.length === 0 ? (
          <div className="text-center py-8 text-stone-400">No currency data available</div>
        ) : (
          sortedCurrencies.map((currency, index) => (
            <div key={currency.id} className={index === 0 ? 'border-t border-stone-700/50' : ''}>
              <EconomyRow currency={currency} />
            </div>
          ))
        )}
      </div>
    );
  };

  const leftColumn = (
    <>
      <CharacterStatusCard />
      <Card
        title={ECONOMY_TYPE_LABELS[selectedEconomyType]}
        subtitle={getSubtitle()}
        className="mt-6">
        {/* Type selector bar */}
        <EconomyTypeBar
          types={typeOptions}
          selectedType={selectedEconomyType}
          onTypeChange={type => {
            setSelectedEconomyType(type as typeof selectedEconomyType);
            if (searchQuery) {
              setSearchQuery('');
            }
          }}
        />

        {/* Search and sort controls */}
        <ListControlBar
          searchValue={searchQuery}
          onSearchChange={setSearchQuery}
          searchPlaceholder="Search all currencies..."
          searchDebounceMs={300}
          isSearching={isSearching}
          sortField={sort.field}
          sortDirection={sort.direction}
          sortOptions={[
            { value: 'primary_value', label: 'Value' },
            { value: 'name', label: 'Name' },
            { value: 'volume', label: 'Volume' },
            { value: 'change_percent', label: 'Change %' },
          ]}
          onSortChange={(field, direction) => updateSort(field as CurrencySortField, direction)}
          onResetSort={resetSort}
          filteredCount={searchQuery ? searchResults.length : filteredCount}
          totalCount={totalCount}
          countLabel="items"
        />

        {/* List Content - Shows loading/error states */}
        {renderListContent()}
      </Card>
    </>
  );

  const rightColumn = (
    <>
      <ExchangeRatesCard />
      <TopItemsCard />
      <div className="text-xs text-stone-500 text-center">
        Economy data provided by{' '}
        <a
          href="https://poe.ninja"
          target="_blank"
          rel="noopener noreferrer"
          className="text-stone-400 hover:text-stone-300 underline transition-colors">
          poe.ninja
        </a>
      </div>
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
