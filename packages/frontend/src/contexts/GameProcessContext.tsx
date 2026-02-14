/* eslint-disable react-refresh/only-export-components */

import { invoke } from '@tauri-apps/api/core';
import { createContext, useContext, useEffect, useState } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { GameProcessStatus, GameProcessStatusChangedEvent } from '@/types/process';
import { EVENT_KEYS, type ExtractPayload } from '@/utils/events/registry';

interface GameProcessContextValue {
  processInfo: GameProcessStatus | null;
  gameRunning: boolean;
}

const GameProcessContext = createContext<GameProcessContextValue | undefined>(undefined);

// Type for the ProcessInfo returned by get_game_process_status command
// (differs slightly from GameProcessStatus - no detected_at field)
interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

export function GameProcessProvider({ children }: React.PropsWithChildren) {
  const [processInfo, setProcessInfo] = useState<GameProcessStatus | null>(null);

  // Fetch initial game process status on mount
  useEffect(() => {
    const fetchInitialStatus = async () => {
      try {
        const status = await invoke<ProcessInfo>('get_game_process_status');
        // Convert ProcessInfo to GameProcessStatus format
        setProcessInfo({
          ...status,
          detected_at: new Date().toISOString(),
        });
      } catch (error) {
        console.error('Failed to fetch initial game process status:', error);
      }
    };

    fetchInitialStatus();
  }, []);

  useAppEventListener([
    {
      eventType: EVENT_KEYS.GameProcessStatusChanged,
      handler: (payload: unknown) => {
        const { new_status } = payload as ExtractPayload<GameProcessStatusChangedEvent>;
        setProcessInfo(new_status);
      },
    },
  ]);

  return (
    <GameProcessContext.Provider
      value={{
        processInfo,
        gameRunning: processInfo?.running || false,
      }}>
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
