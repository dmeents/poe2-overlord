import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type {
  CurrencyExchangeData,
  CurrencySearchResult,
  EconomyType,
  TopCurrencyItem,
} from '@/types/economy';

// Query keys for consistent caching
export const economyQueryKeys = {
  all: ['economy'] as const,
  currencyExchange: (
    league: string,
    isHardcore: boolean,
    economyType: EconomyType
  ) =>
    [
      ...economyQueryKeys.all,
      'currency-exchange',
      league,
      isHardcore,
      economyType,
    ] as const,
  aggregatedTop: (league: string, isHardcore: boolean) =>
    [...economyQueryKeys.all, 'aggregated-top', league, isHardcore] as const,
  search: (league: string, isHardcore: boolean, query: string) =>
    [...economyQueryKeys.all, 'search', league, isHardcore, query] as const,
};

// Hook to get currency exchange data via Tauri backend
export function useCurrencyExchange(
  league: string = 'Rise of the Abyssal',
  isHardcore: boolean = false,
  economyType: EconomyType = 'Currency'
) {
  return useQuery({
    queryKey: economyQueryKeys.currencyExchange(
      league,
      isHardcore,
      economyType
    ),
    queryFn: async (): Promise<CurrencyExchangeData> => {
      return await invoke<CurrencyExchangeData>('get_currency_exchange_data', {
        league,
        isHardcore,
        economyType: economyType,
      });
    },
    staleTime: 10 * 60 * 1000, // 10 minutes - matches backend cache TTL
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}

// Hook to search currencies across all economy types
export function useSearchCurrencies(
  league: string = 'Rise of the Abyssal',
  isHardcore: boolean = false,
  query: string = '',
  enabled: boolean = true
) {
  return useQuery({
    queryKey: economyQueryKeys.search(league, isHardcore, query),
    queryFn: async (): Promise<CurrencySearchResult[]> => {
      return await invoke<CurrencySearchResult[]>('search_currencies', {
        league,
        isHardcore,
        query,
      });
    },
    enabled: enabled && query.length > 0, // Only search if query is not empty
    staleTime: 5 * 60 * 1000, // 5 minutes - use cached search results
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: false,
  });
}

// Hook to get aggregated top currencies across all economy types
export function useAggregatedTopCurrencies(
  league: string = 'Rise of the Abyssal',
  isHardcore: boolean = false
) {
  return useQuery({
    queryKey: economyQueryKeys.aggregatedTop(league, isHardcore),
    queryFn: async (): Promise<TopCurrencyItem[]> => {
      return await invoke<TopCurrencyItem[]>('get_aggregated_top_currencies', {
        league,
        isHardcore,
      });
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}
