import { useState, useEffect, useRef } from 'react';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { Card } from '@/components/ui/card/card';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { ErrorState } from '@/components/ui/error-state/error-state';
import { useEconomy } from '@/contexts/EconomyContext';
import { ECONOMY_TYPE_LABELS } from '@/utils/economy-icons';
import { CurrencyListControlsForm } from '@/components/economy/currency-list-controls-form/currency-list-controls-form';
import { EconomyRow } from '@/components/economy/economy-row/economy-row';
import { ExchangeRatesCard } from '@/components/economy/exchange-rates-card/exchange-rates-card';
import { TopItemsCard } from '@/components/economy/top-items-card/top-items-card';
import { createFileRoute } from '@tanstack/react-router';
import { formatTimeAgo } from '@/utils/format-time-ago';
import { useSearchCurrencies } from '@/queries/economy';
import { useCurrencyList } from '@/hooks/useCurrencyList';

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
    league,
    isHardcore,
    isSoloSelfFound,
  } = useEconomy();

  // Check if error is due to no data available for this economy type
  const isNoDataError =
    isError &&
    error?.message &&
    (error.message.includes('No currency data available') ||
      error.message.includes('not found'));

  const [searchQuery, setSearchQuery] = useState('');
  const [debouncedQuery, setDebouncedQuery] = useState('');
  const debounceTimerRef = useRef<number | null>(null);

  const { data: searchResults = [], isLoading: isSearching } =
    useSearchCurrencies(
      league,
      isHardcore,
      debouncedQuery,
      debouncedQuery.length > 0
    );

  // Debounce search query with proper cleanup
  useEffect(() => {
    if (debounceTimerRef.current) {
      clearTimeout(debounceTimerRef.current);
    }

    debounceTimerRef.current = window.setTimeout(() => {
      setDebouncedQuery(searchQuery);
    }, 300);

    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, [searchQuery]);

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

  // Use currency list hook for sorting/filtering (only when we have data)
  const {
    sort,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters,
    filteredCurrencies,
    currencyCount,
    totalCount,
  } = useCurrencyList(currencyData?.currencies || []);

  // Render list content based on state
  const renderListContent = () => {
    if (isLoading) {
      return <LoadingSpinner className='py-12' />;
    }

    if (isError) {
      if (isNoDataError) {
        return (
          <div className='text-center py-12'>
            <div className='text-zinc-400 mb-2'>
              <svg
                className='mx-auto h-12 w-12 mb-4'
                fill='none'
                viewBox='0 0 24 24'
                stroke='currentColor'
              >
                <path
                  strokeLinecap='round'
                  strokeLinejoin='round'
                  strokeWidth={1.5}
                  d='M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4'
                />
              </svg>
            </div>
            <h3 className='text-lg font-medium text-zinc-300 mb-2'>
              No Data Available
            </h3>
            <p className='text-sm text-zinc-400 max-w-md mx-auto'>
              {ECONOMY_TYPE_LABELS[selectedEconomyType]} data is not available
              for this league yet. Try selecting a different economy type or
              check back later.
            </p>
          </div>
        );
      }
      return <ErrorState title='Error loading economy data' error={error} />;
    }

    if (!currencyData) {
      return (
        <div className='text-zinc-400 text-center py-8'>
          No economy data available
        </div>
      );
    }

    if (searchQuery) {
      // Show search results
      return (
        <div>
          {isSearching ? (
            <LoadingSpinner className='py-12' />
          ) : searchResults.length === 0 ? (
            <div className='text-center py-8 text-zinc-400'>
              No currencies found matching "{searchQuery}"
            </div>
          ) : (
            <div>
              {searchResults.map((result, index) => (
                <div
                  key={result.id}
                  className={index === 0 ? 'border-t border-zinc-700/50' : ''}
                >
                  <EconomyRow
                    currency={{
                      id: result.id,
                      name: result.name,
                      image_url: result.image_url,
                      display_value: result.display_value,
                      primary_value: result.primary_value,
                      secondary_value: result.secondary_value,
                      tertiary_value: result.tertiary_value,
                      volume: result.volume,
                      change_percent: result.change_percent,
                      price_history: [],
                    }}
                  />
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
        {filteredCurrencies.length === 0 ? (
          <div className='text-center py-8 text-zinc-400'>
            {hasActiveFilters
              ? 'No currencies match your filters'
              : 'No currency data available'}
          </div>
        ) : (
          filteredCurrencies.map((currency, index) => (
            <div
              key={currency.id}
              className={index === 0 ? 'border-t border-zinc-700/50' : ''}
            >
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
        className='mt-6'
      >
        {/* Controls - Always visible */}
        <div className='mb-4'>
          <CurrencyListControlsForm
            searchQuery={searchQuery}
            onSearchChange={setSearchQuery}
            isSearching={isSearching}
            onClearFilters={clearFilters}
            hasActiveFilters={hasActiveFilters}
            sort={sort}
            onSortChange={updateSort}
            onResetSort={resetSort}
            currencyCount={searchQuery ? searchResults.length : currencyCount}
            totalCount={totalCount}
          />
        </div>

        {/* List Content - Shows loading/error states */}
        {renderListContent()}
      </Card>
    </>
  );

  const rightColumn = (
    <>
      <ExchangeRatesCard />
      <TopItemsCard />
      <div className='text-xs text-zinc-500 text-center'>
        Economy data provided by{' '}
        <a
          href='https://poe.ninja'
          target='_blank'
          rel='noopener noreferrer'
          className='text-zinc-400 hover:text-zinc-300 underline transition-colors'
        >
          poe.ninja
        </a>
      </div>
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
