import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { CurrencyExchangeData, EconomyType } from '@/types/economy';

// Query keys for consistent caching
export const economyQueryKeys = {
  all: ['economy'] as const,
  currencyExchange: (league: string, economyType: EconomyType) =>
    [
      ...economyQueryKeys.all,
      'currency-exchange',
      league,
      economyType,
    ] as const,
};

// Hook to get currency exchange data via Tauri backend
export function useCurrencyExchange(
  league: string = 'Rise of the Abyssal',
  economyType: EconomyType = 'Currency'
) {
  return useQuery({
    queryKey: economyQueryKeys.currencyExchange(league, economyType),
    queryFn: async (): Promise<CurrencyExchangeData> => {
      return await invoke<CurrencyExchangeData>('get_currency_exchange_data', {
        league,
        economyType: economyType,
      });
    },
    staleTime: 5 * 60 * 1000, // 5 minutes - economy data doesn't change too frequently
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}
