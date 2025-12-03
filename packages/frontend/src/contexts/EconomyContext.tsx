/* eslint-disable react-refresh/only-export-components */
import { createContext, useContext, useMemo, useState, useEffect } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import {
  useCurrencyExchange,
  useAggregatedTopCurrencies,
  economyQueryKeys,
} from '@/queries/economy';
import { useCharacter } from './CharacterContext';
import type {
  CurrencyExchangeData,
  EconomyType,
  TopCurrencyItem,
} from '@/types/economy';

interface EconomyContextValue {
  currencyData: CurrencyExchangeData | undefined;
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
  selectedEconomyType: EconomyType;
  setSelectedEconomyType: (type: EconomyType) => void;
  league: string;
  isHardcore: boolean;
  isSoloSelfFound: boolean;
  aggregatedTopCurrencies: TopCurrencyItem[];
  isLoadingAggregated: boolean;
}

const EconomyContext = createContext<EconomyContextValue | undefined>(
  undefined
);

export function EconomyProvider({ children }: React.PropsWithChildren) {
  // Get active character to determine league
  const { activeCharacter } = useCharacter();
  const queryClient = useQueryClient();

  // State for selected economy type (defaults to Currency)
  const [selectedEconomyType, setSelectedEconomyType] =
    useState<EconomyType>('Currency');

  // Determine league and hardcore status from active character or default to standard league
  const league = activeCharacter?.league || 'Rise of the Abyssal';
  const isHardcore = activeCharacter?.hardcore || false;
  const isSoloSelfFound = activeCharacter?.solo_self_found || false;

  // Fetch currency exchange data from poe.ninja via backend
  const {
    data: currencyData,
    isLoading,
    isError,
    error,
  } = useCurrencyExchange(league, isHardcore, selectedEconomyType);

  // Fetch aggregated top currencies across all types
  const { data: aggregatedTopCurrencies = [], isLoading: isLoadingAggregated } =
    useAggregatedTopCurrencies(league, isHardcore);

  // Invalidate aggregated query when currency data changes (cache was updated)
  useEffect(() => {
    if (currencyData) {
      queryClient.invalidateQueries({
        queryKey: economyQueryKeys.aggregatedTop(league, isHardcore),
      });
    }
  }, [currencyData, league, isHardcore, queryClient]);

  // Memoize the context value to prevent unnecessary re-renders
  const contextValue = useMemo(
    () => ({
      currencyData,
      isLoading,
      isError,
      error: error as Error | null,
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
    ]
  );

  return (
    <EconomyContext.Provider value={contextValue}>
      {children}
    </EconomyContext.Provider>
  );
}

export function useEconomy() {
  const context = useContext(EconomyContext);

  if (context === undefined) {
    throw new Error('useEconomy must be used within EconomyProvider');
  }

  return context;
}
