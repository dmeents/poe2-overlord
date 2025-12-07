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

  const leftColumn = (
    <>
      <CharacterStatusCard />
      <Card
        title={ECONOMY_TYPE_LABELS[selectedEconomyType]}
        subtitle={
          isSoloSelfFound
            ? `${league} - Trading not available in SSF`
            : currencyData
              ? `${isHardcore ? 'HC ' : ''}${league} • Updated ${formatTimeAgo(currencyData.fetched_at)}`
              : `${isHardcore ? 'HC ' : ''}${league}`
        }
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
        ) : (
          <EconomyList currencies={currencyData.currencies} />
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
