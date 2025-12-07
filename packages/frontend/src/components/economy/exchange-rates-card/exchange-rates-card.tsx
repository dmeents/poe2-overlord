import { ArrowPathIcon } from '@heroicons/react/24/outline';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';

export function ExchangeRatesCard() {
  const { currencyData, isLoading } = useEconomy();

  return (
    <Card title='Exchange Rates' icon={<ArrowPathIcon />} className='mb-6'>
      {isLoading ? (
        <LoadingSpinner />
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
  );
}
