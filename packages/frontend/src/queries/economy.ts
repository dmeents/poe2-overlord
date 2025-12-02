import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type {
  CurrencyExchangeData,
  EconomyType,
  TopCurrencyItem,
} from '@/types/economy';

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
  aggregatedTop: (league: string) =>
    [...economyQueryKeys.all, 'aggregated-top', league] as const,
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
    staleTime: 15 * 60 * 1000, // 5 minutes - economy data doesn't change too frequently
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}

// Hook to get aggregated top currencies across all economy types
export function useAggregatedTopCurrencies(
  league: string = 'Rise of the Abyssal'
) {
  return useQuery({
    queryKey: economyQueryKeys.aggregatedTop(league),
    queryFn: async (): Promise<TopCurrencyItem[]> => {
      return await invoke<TopCurrencyItem[]>('get_aggregated_top_currencies', {
        league,
      });
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}
