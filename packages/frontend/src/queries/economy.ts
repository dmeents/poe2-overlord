import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { DEFAULT_LEAGUE } from '@/types/character';
import type {
  BackendEconomyType,
  CurrencyExchangeData,
  CurrencySearchResult,
  EconomyType,
} from '@/types/economy';

export const economyQueryKeys = {
  all: ['economy'] as const,
  currencyExchange: (league: string, isHardcore: boolean, economyType: EconomyType) =>
    [...economyQueryKeys.all, 'currency-exchange', league, isHardcore, economyType] as const,
  allCurrencies: (league: string, isHardcore: boolean) =>
    [...economyQueryKeys.all, 'all-currencies', league, isHardcore] as const,
  starred: (league: string, isHardcore: boolean) =>
    [...economyQueryKeys.all, 'starred', league, isHardcore] as const,
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

export function useAllCurrencies(
  league: string = DEFAULT_LEAGUE,
  isHardcore: boolean = false,
  enabled: boolean = true,
) {
  return useQuery({
    queryKey: economyQueryKeys.allCurrencies(league, isHardcore),
    queryFn: async (): Promise<CurrencySearchResult[]> => {
      return await invoke<CurrencySearchResult[]>('get_all_currencies', {
        league,
        isHardcore,
      });
    },
    enabled,
    staleTime: 10 * 60 * 1000,
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}

export function useRefreshAllEconomyData() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ league, isHardcore }: { league: string; isHardcore: boolean }) => {
      return await invoke<void>('refresh_all_economy_data', { league, isHardcore });
    },
    onSuccess: (_data, { league, isHardcore }) => {
      queryClient.invalidateQueries({
        queryKey: economyQueryKeys.allCurrencies(league, isHardcore),
      });
      queryClient.invalidateQueries({ queryKey: economyQueryKeys.starred(league, isHardcore) });
    },
  });
}

export function useStarredCurrencies(league: string = DEFAULT_LEAGUE, isHardcore: boolean = false) {
  return useQuery({
    queryKey: economyQueryKeys.starred(league, isHardcore),
    queryFn: async (): Promise<CurrencySearchResult[]> => {
      return await invoke<CurrencySearchResult[]>('get_starred_currencies', {
        league,
        isHardcore,
      });
    },
    staleTime: 5 * 60 * 1000,
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: true,
  });
}

export function useToggleCurrencyStar() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({
      league,
      isHardcore,
      economyType,
      currencyId,
    }: {
      league: string;
      isHardcore: boolean;
      economyType: BackendEconomyType;
      currencyId: string;
    }): Promise<boolean> => {
      return await invoke<boolean>('toggle_currency_star', {
        league,
        isHardcore,
        economyType,
        currencyId,
      });
    },
    onSuccess: (_data, { league, isHardcore }) => {
      queryClient.invalidateQueries({ queryKey: economyQueryKeys.starred(league, isHardcore) });
    },
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
