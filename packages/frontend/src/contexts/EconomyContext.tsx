/* eslint-disable react-refresh/only-export-components */
import { createContext, useContext, useMemo, useState } from 'react';
import { useCurrencyExchange } from '@/queries/economy';
import { useCharacter } from './CharacterContext';
import type { CurrencyExchangeData, EconomyType } from '@/types/economy';

interface EconomyContextValue {
  currencyData: CurrencyExchangeData | undefined;
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
  selectedEconomyType: EconomyType;
  setSelectedEconomyType: (type: EconomyType) => void;
  league: string;
}

const EconomyContext = createContext<EconomyContextValue | undefined>(
  undefined
);

export function EconomyProvider({ children }: React.PropsWithChildren) {
  // Get active character to determine league
  const { activeCharacter } = useCharacter();

  // State for selected economy type (defaults to Currency)
  const [selectedEconomyType, setSelectedEconomyType] =
    useState<EconomyType>('Currency');

  // Determine league from active character or default to standard league
  const league = activeCharacter?.league || 'Rise of the Abyssal';

  // Fetch currency exchange data from poe.ninja via backend
  const {
    data: currencyData,
    isLoading,
    isError,
    error,
  } = useCurrencyExchange(league, selectedEconomyType);

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
    }),
    [currencyData, isLoading, isError, error, selectedEconomyType, league]
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
