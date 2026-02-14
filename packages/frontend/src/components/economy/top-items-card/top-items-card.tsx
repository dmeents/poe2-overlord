import { ArrowTrendingUpIcon } from '@heroicons/react/24/outline';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
import type { TopCurrencyItem } from '@/types/economy';
import { calculateItemsSoldPerHour } from '@/utils/economy-utils';
import { topItemsCardStyles } from './top-items-card.styles';

export function TopItemsCard() {
  const { aggregatedTopCurrencies, isLoadingAggregated } = useEconomy();

  return (
    <Card title="Top Items" icon={<ArrowTrendingUpIcon />} className="mb-6">
      {isLoadingAggregated ? (
        <LoadingSpinner />
      ) : aggregatedTopCurrencies.length > 0 ? (
        <div className={topItemsCardStyles.container}>
          {aggregatedTopCurrencies.map((currency: TopCurrencyItem) => (
            <div key={`${currency.economy_type}-${currency.id}`} className={topItemsCardStyles.row}>
              <div className={topItemsCardStyles.leftSection}>
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className={topItemsCardStyles.image}
                />
                <div className={topItemsCardStyles.nameContainer}>
                  <div className={topItemsCardStyles.name}>{currency.name}</div>
                  <div className={topItemsCardStyles.statsRow}>
                    {currency.volume !== null && (
                      <span title="Number of items sold per hour">
                        {calculateItemsSoldPerHour(currency.volume, currency.primary_value)} / hr
                      </span>
                    )}
                  </div>
                </div>
              </div>
              <div className={topItemsCardStyles.valueContainer}>
                <div className={topItemsCardStyles.valueRow}>
                  {currency.primary_value.toLocaleString('en-US', {
                    maximumFractionDigits: 0,
                  })}
                  <img
                    src={currency.primary_currency_image_url}
                    alt={currency.primary_currency_name}
                    className={topItemsCardStyles.valueIcon}
                    title={currency.primary_currency_name}
                  />
                </div>
                {currency.change_percent !== null && (
                  <div className={topItemsCardStyles.changeContainer}>
                    <span
                      className={
                        currency.change_percent >= 0
                          ? topItemsCardStyles.changePositive
                          : topItemsCardStyles.changeNegative
                      }>
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
        <div className={topItemsCardStyles.emptyState}>
          No aggregated data yet. Browse different economy types to populate this list.
        </div>
      )}
    </Card>
  );
}
