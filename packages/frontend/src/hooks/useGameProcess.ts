import type { ProcessInfo } from '@/types';
import { GAME_CONFIG, tauriUtils } from '@/utils';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export const useGameProcess = () => {
  const [processInfo, setProcessInfo] = useState<ProcessInfo | null>(null);
  const [gameRunning, setGameRunning] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Listen for game process status updates from Rust backend
    const unsubscribe = listen<ProcessInfo>(GAME_CONFIG.EVENT_NAME, event => {
      setProcessInfo(event.payload);
      setGameRunning(event.payload.running);
    });

    // Initial process check
    checkGameProcess();

    return () => {
      unsubscribe.then(fn => fn());
    };
  }, []);

  const checkGameProcess = async () => {
    try {
      setIsLoading(true);
      const info = await tauriUtils.checkGameProcess();
      setProcessInfo(info);
      setGameRunning(info.running);
    } catch (error) {
      console.error('Failed to check game process:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return {
    processInfo,
    gameRunning,
    isLoading,
    checkGameProcess,
  };
};
