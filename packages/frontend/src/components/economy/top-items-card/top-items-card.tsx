import { ArrowTrendingUpIcon } from '@heroicons/react/24/outline';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
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

export function TopItemsCard() {
  const { aggregatedTopCurrencies, isLoadingAggregated } = useEconomy();

  return (
    <Card title="Top Items" icon={<ArrowTrendingUpIcon />} className="mb-6">
      {isLoadingAggregated ? (
        <LoadingSpinner />
      ) : aggregatedTopCurrencies.length > 0 ? (
        <div className="space-y-2">
          {aggregatedTopCurrencies.map((currency: TopCurrencyItem) => (
            <div
              key={`${currency.economy_type}-${currency.id}`}
              className="flex items-start justify-between text-sm p-2 hover:bg-zinc-700/30"
            >
              <div className="flex items-center gap-2 flex-1 min-w-0">
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className="w-6 h-6 flex-shrink-0"
                />
                <div className="flex-1 min-w-0">
                  <div className="text-white truncate">{currency.name}</div>
                  <div className="flex items-center gap-3 text-xs text-zinc-400 mt-0.5">
                    {currency.volume !== null && (
                      <span title="Number of items sold per hour">
                        {calculateItemsSoldPerHour(
                          currency.volume,
                          currency.primary_value
                        )}{' '}
                        / hr
                      </span>
                    )}
                  </div>
                </div>
              </div>
              <div className="text-right">
                <div className="text-zinc-300 font-semibold flex items-center justify-end gap-1">
                  {currency.primary_value.toLocaleString('en-US', {
                    maximumFractionDigits: 0,
                  })}
                  <img
                    src={currency.primary_currency_image_url}
                    alt={currency.primary_currency_name}
                    className="w-4 h-4"
                    title={currency.primary_currency_name}
                  />
                </div>
                {currency.change_percent !== null && (
                  <div className="text-xs text-zinc-400 mt-1 flex justify-end">
                    <span
                      className={`font-semibold opacity-60 ${
                        currency.change_percent >= 0
                          ? 'text-emerald-400'
                          : 'text-red-400'
                      }`}
                    >
                      {currency.change_percent >= 0 ? '+' : ''}
                      {currency.change_percent.toFixed(2)}%
                    </span>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-zinc-400 text-sm text-center py-8">
          No aggregated data yet. Browse different economy types to populate
          this list.
        </div>
      )}
    </Card>
  );
}
