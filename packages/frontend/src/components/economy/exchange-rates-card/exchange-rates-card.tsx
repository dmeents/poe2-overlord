import { ArrowPathIcon } from '@heroicons/react/24/outline';
import { Card } from '@/components/ui/card/card';
import { ItemTooltip } from '@/components/ui/item-tooltip/item-tooltip';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';

export function ExchangeRatesCard() {
  const { currencyData, isLoading } = useEconomy();

  return (
    <Card title="Exchange Rates" accentColor="molten" icon={<ArrowPathIcon />}>
      {isLoading ? (
        <LoadingSpinner />
      ) : currencyData ? (
        <div className="flex items-center justify-center gap-6 py-6">
          <div className="flex flex-col items-center gap-2">
            <ItemTooltip
              itemName={currencyData.primary_currency.name}
              fallbackImageUrl={currencyData.primary_currency.image_url}>
              <img
                src={currencyData.primary_currency.image_url}
                alt={currencyData.primary_currency.name}
                className="w-8 h-8"
              />
            </ItemTooltip>
            <span className="text-stone-50 font-semibold text-lg">1</span>
            <span className="text-xs text-stone-500 -mt-1">
              {currencyData.primary_currency.name}
            </span>
          </div>

          <span className="text-stone-500 text-xl">↔</span>

          <div className="flex flex-col items-center gap-2">
            <ItemTooltip
              itemName={currencyData.secondary_currency.name}
              fallbackImageUrl={currencyData.secondary_currency.image_url}>
              <img
                src={currencyData.secondary_currency.image_url}
                alt={currencyData.secondary_currency.name}
                className="w-8 h-8"
              />
            </ItemTooltip>
            <span className="text-stone-50 font-semibold text-lg">
              {currencyData.secondary_rate.toFixed(2)}
            </span>
            <span className="text-xs text-stone-500 -mt-1">
              {currencyData.secondary_currency.name}
            </span>
          </div>

          {currencyData.tertiary_currency && currencyData.tertiary_rate && (
            <>
              <span className="text-stone-500 text-xl">↔</span>

              <div className="flex flex-col items-center gap-2">
                <ItemTooltip
                  itemName={currencyData.tertiary_currency.name}
                  fallbackImageUrl={currencyData.tertiary_currency.image_url}>
                  <img
                    src={currencyData.tertiary_currency.image_url}
                    alt={currencyData.tertiary_currency.name}
                    className="w-8 h-8"
                  />
                </ItemTooltip>
                <span className="text-stone-50 font-semibold text-lg">
                  {currencyData.tertiary_rate.toFixed(0)}
                </span>
                <span className="text-xs text-stone-500 -mt-1">
                  {currencyData.tertiary_currency.name}
                </span>
              </div>
            </>
          )}
        </div>
      ) : (
        <div className="text-stone-400 text-sm text-center py-8">
          No exchange rate data available
        </div>
      )}
    </Card>
  );
}
