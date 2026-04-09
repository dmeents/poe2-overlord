/* eslint-disable react-refresh/only-export-components */

import { useQueryClient } from '@tanstack/react-query';
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';
import {
  economyQueryKeys,
  useAllCurrencies,
  useCurrencyExchange,
  useRefreshAllEconomyData,
  useStarredCurrencies,
  useToggleCurrencyStar,
} from '@/queries/economy';
import { DEFAULT_LEAGUE } from '@/types/character';
import type {
  BackendEconomyType,
  CurrencyExchangeData,
  CurrencyExchangeRate,
  CurrencySearchResult,
  EconomyType,
} from '@/types/economy';
import { searchResultToExchangeRate } from '@/types/economy';
import type { AppError } from '@/types/error';
import { parseError } from '@/utils/error-handling';
import { useCharacter } from './CharacterContext';

interface EconomyContextValue {
  currencyData: CurrencyExchangeData | undefined;
  allCurrencies: CurrencyExchangeRate[];
  isLoading: boolean;
  isError: boolean;
  error: AppError | null;
  selectedEconomyType: EconomyType;
  setSelectedEconomyType: (type: EconomyType) => void;
  league: string;
  isHardcore: boolean;
  isSoloSelfFound: boolean;
  starredCurrencies: CurrencySearchResult[];
  starredCurrencyIds: Set<string>;
  isLoadingStarred: boolean;
  toggleStar: (currencyId: string, economyType: BackendEconomyType) => void;
  refreshAll: () => void;
  isRefreshing: boolean;
}

const EconomyContext = createContext<EconomyContextValue | undefined>(undefined);

export function EconomyProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter } = useCharacter();
  const queryClient = useQueryClient();

  const [selectedEconomyType, setSelectedEconomyType] = useState<EconomyType>('All');

  const league = activeCharacter?.league || DEFAULT_LEAGUE;
  const isHardcore = activeCharacter?.hardcore || false;
  const isSoloSelfFound = activeCharacter?.solo_self_found || false;

  const isAllTab = selectedEconomyType === 'All';

  // Per-type currency exchange data (used when not on 'All' tab)
  const {
    data: currencyData,
    isLoading: isLoadingExchange,
    isError: isErrorExchange,
    error: errorExchange,
  } = useCurrencyExchange(league, isHardcore, isAllTab ? 'Currency' : selectedEconomyType);

  // All currencies across all cached types (used for 'All' tab)
  const {
    data: allCurrenciesRaw = [],
    isLoading: isLoadingAll,
    isError: isErrorAll,
    error: errorAll,
  } = useAllCurrencies(league, isHardcore, isAllTab);

  const allCurrencies = useMemo(
    () => allCurrenciesRaw.map(searchResultToExchangeRate),
    [allCurrenciesRaw],
  );

  const isLoading = isAllTab ? isLoadingAll : isLoadingExchange;
  const isError = isAllTab ? isErrorAll : isErrorExchange;
  const error = isAllTab ? errorAll : errorExchange;

  // Starred currencies
  const { data: starredCurrencies = [], isLoading: isLoadingStarred } = useStarredCurrencies(
    league,
    isHardcore,
  );

  const starredCurrencyIds = useMemo(
    () => new Set(starredCurrencies.map(c => `${c.economy_type}:${c.id}`)),
    [starredCurrencies],
  );

  const toggleStarMutation = useToggleCurrencyStar();
  const toggleStar = useCallback(
    (currencyId: string, economyType: BackendEconomyType) => {
      toggleStarMutation.mutate({ league, isHardcore, economyType, currencyId });
    },
    [toggleStarMutation, league, isHardcore],
  );

  // Refresh all economy types
  const refreshAllMutation = useRefreshAllEconomyData();
  const [isManualRefreshing, setIsManualRefreshing] = useState(false);
  const refreshAll = useCallback(() => {
    setIsManualRefreshing(true);
    refreshAllMutation.mutate(
      { league, isHardcore },
      { onSettled: () => setIsManualRefreshing(false) },
    );
  }, [refreshAllMutation, league, isHardcore]);
  // Only spin the icon for manual refreshes, not the silent on-mount prefetch
  const isRefreshing = isManualRefreshing;

  // Fire refresh-all once on mount to silently populate the cache.
  // Intentionally only re-runs when league changes (not isHardcore) so we
  // don't double-fetch when the character context first resolves.
  const hasRefreshedOnMount = useRef(false);
  const isHardcoreRef = useRef(isHardcore);
  isHardcoreRef.current = isHardcore;
  // biome-ignore lint/correctness/useExhaustiveDependencies: intentional – only re-runs on league change; isHardcore read via ref
  useEffect(() => {
    // eslint-disable-line react-hooks/exhaustive-deps
    if (!hasRefreshedOnMount.current && league && !isSoloSelfFound) {
      hasRefreshedOnMount.current = true;
      refreshAllMutation.mutate({ league, isHardcore: isHardcoreRef.current });
    }
  }, [league]);

  // Invalidate all-currencies query when per-type data refreshes
  const lastFetchedAtRef = useRef<string | null>(null);
  useEffect(() => {
    if (currencyData && currencyData.fetched_at !== lastFetchedAtRef.current) {
      lastFetchedAtRef.current = currencyData.fetched_at;
      queryClient.invalidateQueries({
        queryKey: economyQueryKeys.allCurrencies(league, isHardcore),
      });
    }
  }, [currencyData, league, isHardcore, queryClient]);

  const contextValue = useMemo(
    () => ({
      currencyData,
      allCurrencies,
      isLoading,
      isError,
      error: error ? parseError(error) : null,
      selectedEconomyType,
      setSelectedEconomyType,
      league,
      isHardcore,
      isSoloSelfFound,
      starredCurrencies,
      starredCurrencyIds,
      isLoadingStarred,
      toggleStar,
      refreshAll,
      isRefreshing,
    }),
    [
      currencyData,
      allCurrencies,
      isLoading,
      isError,
      error,
      selectedEconomyType,
      league,
      isHardcore,
      isSoloSelfFound,
      starredCurrencies,
      starredCurrencyIds,
      isLoadingStarred,
      toggleStar,
      refreshAll,
      isRefreshing,
    ],
  );

  return <EconomyContext.Provider value={contextValue}>{children}</EconomyContext.Provider>;
}

export function useEconomy() {
  const context = useContext(EconomyContext);

  if (context === undefined) {
    throw new Error('useEconomy must be used within EconomyProvider');
  }

  return context;
}
