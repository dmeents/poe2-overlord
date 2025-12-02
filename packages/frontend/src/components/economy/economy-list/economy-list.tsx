import type { CurrencyExchangeRate } from '@/types/economy';
import { EconomyRow } from '../economy-row/economy-row';

interface EconomyListProps {
  currencies: CurrencyExchangeRate[];
  onCurrencyClick?: (currency: CurrencyExchangeRate) => void;
}

export function EconomyList({ currencies, onCurrencyClick }: EconomyListProps) {
  if (currencies.length === 0) {
    return (
      <div className='text-center py-8 text-zinc-400'>
        No currency data available
      </div>
    );
  }

  return (
    <div className='-mx-6'>
      {currencies.map(currency => (
        <EconomyRow
          key={currency.id}
          currency={currency}
          onClick={onCurrencyClick}
        />
      ))}
    </div>
  );
}
