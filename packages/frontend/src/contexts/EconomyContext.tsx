/* eslint-disable react-refresh/only-export-components */

import { useQueryClient } from '@tanstack/react-query';
import { createContext, useContext, useEffect, useMemo, useState } from 'react';
import {
  economyQueryKeys,
  useAggregatedTopCurrencies,
  useCurrencyExchange,
} from '@/queries/economy';
import type { CurrencyExchangeData, EconomyType, TopCurrencyItem } from '@/types/economy';
import { useCharacter } from './CharacterContext';

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

const EconomyContext = createContext<EconomyContextValue | undefined>(undefined);

export function EconomyProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter } = useCharacter();
  const queryClient = useQueryClient();

  const [selectedEconomyType, setSelectedEconomyType] = useState<EconomyType>('Currency');

  const league = activeCharacter?.league || 'Rise of the Abyssal';
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

  useEffect(() => {
    if (currencyData) {
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
