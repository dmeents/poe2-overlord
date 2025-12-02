import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { Card } from '@/components/ui/card/card';
import { Button } from '@/components/ui/button/button';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
import {
  ECONOMY_TYPE_ICONS,
  ECONOMY_TYPE_LABELS,
  ECONOMY_TYPES,
} from '@/utils/economy-icons';
import { EconomyList } from '@/components/economy/economy-list/economy-list';
import { CurrencyDollarIcon } from '@heroicons/react/24/outline';
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
    setSelectedEconomyType,
    league,
  } = useEconomy();

  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <LoadingSpinner />
          </div>
        </div>
      </div>
    );
  }

  if (isError) {
    console.error('Economy data error:', error);
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <div className='text-red-400 max-w-2xl'>
              <p className='font-bold mb-2'>Error loading economy data:</p>
              <p className='text-sm'>
                {error?.message || String(error) || 'Unknown error'}
              </p>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!currencyData) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <div className='text-zinc-400'>No economy data available</div>
          </div>
        </div>
      </div>
    );
  }

  const sortedCurrencies = [...currencyData.currencies].sort(
    (a, b) => (b.raw_divine_value || 0) - (a.raw_divine_value || 0)
  );

  const leftColumn = (
    <>
      <CharacterStatusCard />
      <Card title='Economy Type' subtitle={league} className='mt-6'>
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
      </Card>
      <Card
        title={`${ECONOMY_TYPE_LABELS[selectedEconomyType]} (${currencyData.currencies.length})`}
        subtitle={`Updated ${formatTimeAgo(currencyData.fetched_at)}`}
        className='mt-6'
      >
        <EconomyList currencies={sortedCurrencies} />
      </Card>
    </>
  );

  const rightColumn = (
    <>
      <Card
        title='Currency Exchange Rates'
        icon={<CurrencyDollarIcon />}
        className='mb-6'
      >
        <div className='flex items-center justify-center gap-6 py-4'>
          <div className='flex flex-col items-center gap-2'>
            <img
              src={currencyData.primary_currency.image_url}
              alt={currencyData.primary_currency.name}
              className='w-8 h-8'
            />
            <span className='text-white font-semibold text-lg'>1</span>
            <span className='text-xs text-zinc-500 -mt-1'>
              {currencyData.primary_currency.name}
            </span>
          </div>

          <span className='text-zinc-500 text-xl'>↔</span>

          <div className='flex flex-col items-center gap-2'>
            <img
              src={currencyData.secondary_currency.image_url}
              alt={currencyData.secondary_currency.name}
              className='w-8 h-8'
            />
            <span className='text-white font-semibold text-lg'>
              {currencyData.secondary_rate.toFixed(2)}
            </span>
            <span className='text-xs text-zinc-500 -mt-1'>
              {currencyData.secondary_currency.name}
            </span>
          </div>

          <span className='text-zinc-500 text-xl'>↔</span>

          <div className='flex flex-col items-center gap-2'>
            <img
              src={currencyData.tertiary_currency.image_url}
              alt={currencyData.tertiary_currency.name}
              className='w-8 h-8'
            />
            <span className='text-white font-semibold text-lg'>
              {currencyData.tertiary_rate.toFixed(0)}
            </span>
            <span className='text-xs text-zinc-500 -mt-1'>
              {currencyData.tertiary_currency.name}
            </span>
          </div>
        </div>
      </Card>

      <Card
        title='Top Currencies'
        icon={<CurrencyDollarIcon />}
        className='mb-6'
      >
        <div className='space-y-2'>
          {currencyData.currencies
            .slice()
            .sort(
              (a, b) => (b.raw_divine_value || 0) - (a.raw_divine_value || 0)
            )
            .slice(0, 10)
            .map(currency => (
              <div
                key={currency.id}
                className='flex items-center justify-between text-sm p-2 rounded hover:bg-zinc-700/30'
              >
                <div className='flex items-center gap-2'>
                  <img
                    src={currency.image_url}
                    alt={currency.name}
                    className='w-6 h-6'
                  />
                  <span className='text-white'>{currency.name}</span>
                </div>
                <div className='text-right'>
                  <div className='text-zinc-300 flex items-center gap-1'>
                    <span>{currency.display_value.value.toFixed(2)}</span>
                    <img
                      src={currency.display_value.currency_image_url}
                      alt={currency.display_value.currency_name}
                      className='w-4 h-4'
                      title={currency.display_value.currency_name}
                    />
                  </div>
                  {currency.raw_divine_value && (
                    <div className='text-xs text-zinc-500'>
                      {currency.raw_divine_value.toFixed(6)} divine
                    </div>
                  )}
                </div>
              </div>
            ))}
        </div>
      </Card>

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
