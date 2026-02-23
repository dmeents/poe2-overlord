import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { DEFAULT_LEAGUE } from '@/types/character';
import type {
  CurrencyExchangeData,
  CurrencySearchResult,
  EconomyType,
  TopCurrencyItem,
} from '@/types/economy';

export const economyQueryKeys = {
  all: ['economy'] as const,
  currencyExchange: (league: string, isHardcore: boolean, economyType: EconomyType) =>
    [...economyQueryKeys.all, 'currency-exchange', league, isHardcore, economyType] as const,
  aggregatedTop: (league: string, isHardcore: boolean) =>
    [...economyQueryKeys.all, 'aggregated-top', league, isHardcore] as const,
  search: (league: string, isHardcore: boolean, query: string) =>
    [...economyQueryKeys.all, 'search', league, isHardcore, query] as const,
};

export function useCurrencyExchange(
  league: string = DEFAULT_LEAGUE,
  isHardcore: boolean = false,
  economyType: EconomyType = 'Currency',
) {
  return useQuery({
    queryKey: economyQueryKeys.currencyExchange(league, isHardcore, economyType),
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

export function useSearchCurrencies(
  league: string = DEFAULT_LEAGUE,
  isHardcore: boolean = false,
  query: string = '',
  enabled: boolean = true,
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

export function useAggregatedTopCurrencies(
  league: string = DEFAULT_LEAGUE,
  isHardcore: boolean = false,
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
