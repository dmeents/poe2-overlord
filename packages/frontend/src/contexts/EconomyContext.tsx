/* eslint-disable react-refresh/only-export-components */

import { useQueryClient } from '@tanstack/react-query';
import { createContext, useContext, useEffect, useMemo, useRef, useState } from 'react';
import {
  economyQueryKeys,
  useAggregatedTopCurrencies,
  useCurrencyExchange,
} from '@/queries/economy';
import type { CurrencyExchangeData, EconomyType, TopCurrencyItem } from '@/types/economy';
import type { AppError } from '@/types/error';
import { DEFAULT_LEAGUE } from '@/types/character';
import { parseError } from '@/utils/error-handling';
import { useCharacter } from './CharacterContext';

interface EconomyContextValue {
  currencyData: CurrencyExchangeData | undefined;
  isLoading: boolean;
  isError: boolean;
  error: AppError | null;
  selectedEconomyType: EconomyType;
  setSelectedEconomyType: (type: EconomyType) => void;
  league: string;
  isHardcore: boolean;
  isSoloSelfFound: boolean;
  aggregatedTopCurrencies: TopCurrencyItem[];
  isLoadingAggregated: boolean;
}

const EconomyContext = createContext<EconomyContextValue | undefined>(undefined);

export function EconomyProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter } = useCharacter();
  const queryClient = useQueryClient();

  const [selectedEconomyType, setSelectedEconomyType] = useState<EconomyType>('Currency');

  const league = activeCharacter?.league || DEFAULT_LEAGUE;
  const isHardcore = activeCharacter?.hardcore || false;
  const isSoloSelfFound = activeCharacter?.solo_self_found || false;

  const {
    data: currencyData,
    isLoading,
    isError,
    error,
  } = useCurrencyExchange(league, isHardcore, selectedEconomyType);

  const { data: aggregatedTopCurrencies = [], isLoading: isLoadingAggregated } =
    useAggregatedTopCurrencies(league, isHardcore);

  const lastFetchedAtRef = useRef<string | null>(null);
  useEffect(() => {
    if (currencyData && currencyData.fetched_at !== lastFetchedAtRef.current) {
      lastFetchedAtRef.current = currencyData.fetched_at;
      queryClient.invalidateQueries({
        queryKey: economyQueryKeys.aggregatedTop(league, isHardcore),
      });
    }
  }, [currencyData, league, isHardcore, queryClient]);

  const contextValue = useMemo(
    () => ({
      currencyData,
      isLoading,
      isError,
      error: error ? parseError(error) : null,
      selectedEconomyType,
      setSelectedEconomyType,
      league,
      isHardcore,
      isSoloSelfFound,
      aggregatedTopCurrencies,
      isLoadingAggregated,
    }),
    [
      currencyData,
      isLoading,
      isError,
      error,
      selectedEconomyType,
      league,
      isHardcore,
      isSoloSelfFound,
      aggregatedTopCurrencies,
      isLoadingAggregated,
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
