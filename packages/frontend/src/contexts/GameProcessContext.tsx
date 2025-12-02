/* eslint-disable react-refresh/only-export-components */
import type {
  GameProcessStatus,
  GameProcessStatusChangedEvent,
} from '@/types/process';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { createContext, useContext, useState } from 'react';
import { EVENT_KEYS, type ExtractPayload } from '@/utils/events/registry';

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

  const { isListening } = useAppEventListener([
    {
      eventType: EVENT_KEYS.GameProcessStatusChanged,
      handler: (payload: unknown) => {
        const { new_status } =
          payload as ExtractPayload<GameProcessStatusChangedEvent>;
        setProcessInfo(new_status);
      },
    },
  ]);

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
