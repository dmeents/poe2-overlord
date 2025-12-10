import { useState, useEffect, useRef } from 'react';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { Card } from '@/components/ui/card/card';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { ErrorState } from '@/components/ui/error-state/error-state';
import { useEconomy } from '@/contexts/EconomyContext';
import { ECONOMY_TYPE_LABELS } from '@/utils/economy-icons';
import { EconomyList } from '@/components/economy/economy-list/economy-list';
import { ExchangeRatesCard } from '@/components/economy/exchange-rates-card/exchange-rates-card';
import { TopItemsCard } from '@/components/economy/top-items-card/top-items-card';
import { createFileRoute } from '@tanstack/react-router';
import { formatTimeAgo } from '@/utils/format-time-ago';
import { useSearchCurrencies } from '@/queries/economy';

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

  const leftColumn = (
    <>
      <CharacterStatusCard />
      <Card
        title={ECONOMY_TYPE_LABELS[selectedEconomyType]}
        subtitle={getSubtitle()}
        className='mt-6'
      >
        {isLoading ? (
          <LoadingSpinner className='py-12' />
        ) : isError ? (
          <ErrorState title='Error loading economy data' error={error} />
        ) : !currencyData ? (
          <div className='text-zinc-400 text-center py-8'>
            No economy data available
          </div>
        ) : searchQuery ? (
          // Show search results
          <EconomyList
            currencies={searchResults.map(result => ({
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
            }))}
            searchQuery={searchQuery}
            onSearchChange={setSearchQuery}
            isSearching={isSearching}
            searchResultsCount={searchResults.length}
          />
        ) : (
          // Show normal economy list
          <EconomyList
            currencies={currencyData.currencies}
            searchQuery={searchQuery}
            onSearchChange={setSearchQuery}
            isSearching={isSearching}
            searchResultsCount={0}
          />
        )}
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
