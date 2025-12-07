import type { CurrencyExchangeRate } from '@/types/economy';
import { ArrowsRightLeftIcon } from '@heroicons/react/24/outline';
import { Tooltip } from '@/components/ui/tooltip/tooltip';
import { useEconomy } from '@/contexts/EconomyContext';

interface EconomyRowProps {
  currency: CurrencyExchangeRate;
  onClick?: (currency: CurrencyExchangeRate) => void;
}

export function EconomyRow({ currency, onClick }: EconomyRowProps) {
  const { display_value } = currency;
  const { currencyData } = useEconomy();

  const handleClick = () => {
    if (onClick) {
      onClick(currency);
    }
  };

  const formattedValue = display_value.value.toLocaleString('en-US', {
    minimumFractionDigits: display_value.value >= 10 ? 1 : 2,
    maximumFractionDigits: display_value.value >= 10 ? 1 : 2,
  });

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

  const tooltipContent = currencyData ? (
    <div className='space-y-1 text-xs'>
      <div className='font-semibold border-b border-zinc-600 pb-1 mb-2'>
        Raw Values
      </div>
      <div className='flex justify-between gap-4'>
        <span className='text-zinc-400'>
          {currencyData.primary_currency.name}:
        </span>
        <span className='text-white font-mono'>
          {currency.primary_value.toFixed(6)}
        </span>
      </div>
      <div className='flex justify-between gap-4'>
        <span className='text-zinc-400'>
          {currencyData.secondary_currency.name}:
        </span>
        <span className='text-white font-mono'>
          {currency.secondary_value.toFixed(4)}
        </span>
      </div>
      {currencyData.tertiary_currency && (
        <div className='flex justify-between gap-4'>
          <span className='text-zinc-400'>
            {currencyData.tertiary_currency.name}:
          </span>
          <span className='text-white font-mono'>
            {currency.tertiary_value.toFixed(2)}
          </span>
        </div>
      )}
    </div>
  ) : null;

  return (
    <div
      onClick={handleClick}
      className={`flex items-center justify-between py-3 px-6 border-b border-zinc-700/50 transition-colors ${
        onClick ? 'cursor-pointer hover:bg-zinc-800/30' : ''
      }`}
    >
      <div className='flex items-center gap-3 flex-1 min-w-0'>
        <img
          src={currency.image_url}
          alt={currency.name}
          className='w-8 h-8 flex-shrink-0'
          onError={e => {
            e.currentTarget.style.display = 'none';
          }}
        />
        <div className='flex-1 min-w-0'>
          <div className='text-white font-medium truncate'>{currency.name}</div>
          <div className='flex items-center gap-3 text-xs text-zinc-400 mt-1'>
            {currency.volume !== null && currencyData && (
              <span title='Number of items sold per hour'>
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

      <Tooltip content={tooltipContent}>
        <div className='text-right'>
          <div className='flex items-center gap-2 justify-end text-base font-semibold text-zinc-200'>
            {display_value.inverted ? (
              <>
                <span>{formattedValue}</span>
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className='w-5 h-5'
                  title={currency.name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
                <ArrowsRightLeftIcon className='w-4 h-4 text-zinc-500' />
                <span>1</span>
                <img
                  src={display_value.currency_image_url}
                  alt={display_value.currency_name}
                  className='w-5 h-5'
                  title={display_value.currency_name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
              </>
            ) : (
              <>
                <span>{formattedValue}</span>
                <img
                  src={display_value.currency_image_url}
                  alt={display_value.currency_name}
                  className='w-5 h-5'
                  title={display_value.currency_name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
                <ArrowsRightLeftIcon className='w-4 h-4 text-zinc-500' />
                <span>1</span>
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className='w-5 h-5'
                  title={currency.name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
              </>
            )}
          </div>
          {currency.change_percent !== null && (
            <div className='text-xs text-zinc-400 mt-1 flex justify-end'>
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
      </Tooltip>
    </div>
  );
}
