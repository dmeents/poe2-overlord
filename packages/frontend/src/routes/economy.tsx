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
import {
  ArrowPathIcon,
  ArrowTrendingUpIcon,
} from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { formatTimeAgo } from '@/utils/format-time-ago';
import type { TopCurrencyItem } from '@/types/economy';

// Calculate items sold per hour (volume / primary_value)
const calculateItemsSoldPerHour = (
  volume: number,
  primaryValue: number
): string => {
  const itemsSold = volume / primaryValue;
  return itemsSold.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
};

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
    aggregatedTopCurrencies,
    isLoadingAggregated,
  } = useEconomy();

  const sortedCurrencies = currencyData
    ? [...currencyData.currencies].sort(
        (a, b) => b.primary_value - a.primary_value
      )
    : [];

  const leftColumn = (
    <>
      <CharacterStatusCard />
      <Card
        title='Category'
        subtitle={
          isSoloSelfFound
            ? `${league} - Trading not available in SSF`
            : `${isHardcore ? 'HC ' : ''}${league}`
        }
        className='mt-6'
      >
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
        title={`${ECONOMY_TYPE_LABELS[selectedEconomyType]}${currencyData ? ` (${currencyData.currencies.length})` : ''}`}
        subtitle={
          currencyData
            ? `Updated ${formatTimeAgo(currencyData.fetched_at)}`
            : undefined
        }
        className='mt-6'
      >
        {isLoading ? (
          <div className='flex items-center justify-center py-12'>
            <LoadingSpinner />
          </div>
        ) : isError ? (
          <div className='text-red-400 text-center py-8'>
            <p className='font-bold mb-2'>Error loading economy data:</p>
            <p className='text-sm'>
              {error?.message || String(error) || 'Unknown error'}
            </p>
          </div>
        ) : !currencyData ? (
          <div className='text-zinc-400 text-center py-8'>
            No economy data available
          </div>
        ) : (
          <EconomyList currencies={sortedCurrencies} />
        )}
      </Card>
    </>
  );

  const rightColumn = (
    <>
      <Card title='Exchange Rates' icon={<ArrowPathIcon />} className='mb-6'>
        {isLoading ? (
          <div className='flex items-center justify-center py-8'>
            <LoadingSpinner />
          </div>
        ) : currencyData ? (
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

            {currencyData.tertiary_currency && currencyData.tertiary_rate && (
              <>
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
              </>
            )}
          </div>
        ) : (
          <div className='text-zinc-400 text-sm text-center py-8'>
            No exchange rate data available
          </div>
        )}
      </Card>

      <Card title='Top Items' icon={<ArrowTrendingUpIcon />} className='mb-6'>
        {isLoadingAggregated ? (
          <div className='flex items-center justify-center py-8'>
            <LoadingSpinner />
          </div>
        ) : aggregatedTopCurrencies.length > 0 ? (
          <div className='space-y-2'>
            {aggregatedTopCurrencies.map((currency: TopCurrencyItem) => (
              <div
                key={`${currency.economy_type}-${currency.id}`}
                className='flex items-start justify-between text-sm p-2 hover:bg-zinc-700/30'
              >
                <div className='flex items-center gap-2 flex-1 min-w-0'>
                  <img
                    src={currency.image_url}
                    alt={currency.name}
                    className='w-6 h-6 flex-shrink-0'
                  />
                  <div className='flex-1 min-w-0'>
                    <div className='text-white truncate'>{currency.name}</div>
                    <div className='flex items-center gap-3 text-xs text-zinc-400 mt-0.5'>
                      {currency.volume !== null && (
                        <span title='Number of items sold per hour'>
                          {calculateItemsSoldPerHour(
                            currency.volume,
                            currency.primary_value
                          )}{' '}
                          / hr
                        </span>
                      )}
                      {currency.change_percent !== null && (
                        <span
                          className={`font-semibold ${
                            currency.change_percent >= 0
                              ? 'text-emerald-400'
                              : 'text-red-400'
                          }`}
                        >
                          {currency.change_percent >= 0 ? '+' : ''}
                          {currency.change_percent.toFixed(0)}%
                        </span>
                      )}
                    </div>
                  </div>
                </div>
                <div className='text-right'>
                  <div className='text-zinc-300 font-semibold flex items-center justify-end gap-1'>
                    {currency.primary_value.toLocaleString('en-US', {
                      maximumFractionDigits: 0,
                    })}
                    <img
                      src={currency.primary_currency_image_url}
                      alt={currency.primary_currency_name}
                      className='w-4 h-4'
                      title={currency.primary_currency_name}
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className='text-zinc-400 text-sm text-center py-8'>
            No aggregated data yet. Browse different economy types to populate
            this list.
          </div>
        )}
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
