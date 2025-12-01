/* eslint-disable react-refresh/only-export-components */
import type { GameProcessStatus } from '@/types/process';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { createContext, useContext, useState } from 'react';

interface GameProcessContextValue {
  processInfo: GameProcessStatus | null;
  gameRunning: boolean;
  isListening: boolean;
}

const GameProcessContext = createContext<GameProcessContextValue | undefined>(
  undefined
);

export function GameProcessProvider({ children }: React.PropsWithChildren) {
  const [processInfo, setProcessInfo] = useState<GameProcessStatus | null>(
    null
  );

  const { isListening } = useAppEventListener(
    'GameProcessStatusChanged',
    payload => {
      setProcessInfo(payload.new_status);
    }
  );

  return (
    <GameProcessContext.Provider
      value={{
        processInfo,
        gameRunning: processInfo?.running || false,
        isListening,
      }}
    >
      {children}
    </GameProcessContext.Provider>
  );
}

export function useGameProcess() {
  const context = useContext(GameProcessContext);

  if (context === undefined) {
    throw new Error('useGameProcess must be used within GameProcessProvider');
  }

  return context;
}
