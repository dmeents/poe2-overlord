import type { GameProcessStatusChangedEvent, ProcessInfo } from '@/types';
import { GAME_CONFIG } from '@/utils';
import { useCallback, useState } from 'react';
import { useTauriEventListener } from './useTauriEventListener';

/**
 * Hook for listening to game process status changes via Tauri events
 *
 * This hook provides a purely event-driven approach to game process monitoring.
 * It listens for 'game-process-status-changed' events from the backend and
 * maintains the current process status in local state.
 *
 * @returns Object containing process info, game running state, and listening status
 */
export function useGameProcessEvents() {
  const [processInfo, setProcessInfo] = useState<ProcessInfo | null>(null);

  // Handler for game process status events
  const handleGameProcessStatusChanged = useCallback((event: GameProcessStatusChangedEvent) => {
    // Handle the AppEvent structure - the payload is the entire AppEvent
    const eventPayload = event.payload as {
      GameProcessStatusChanged?: { new_status?: ProcessInfo };
    };
    if (eventPayload && eventPayload.GameProcessStatusChanged) {
      const gameEvent = eventPayload.GameProcessStatusChanged;
      if (gameEvent.new_status) {
        const newProcessInfo: ProcessInfo = {
          name: gameEvent.new_status.name,
          pid: gameEvent.new_status.pid,
          running: gameEvent.new_status.running,
        };
        setProcessInfo(newProcessInfo);
      }
    }
  }, []);

  // Get initial game process status
  const getInitialGameProcessStatus = useCallback(async (): Promise<ProcessInfo | null> => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<ProcessInfo>('get_game_process_status');
    } catch {
      // No initial status available, rely on events only
      return null;
    }
  }, []);

  // Use the generic Tauri event listener
  const { isListening, error } = useTauriEventListener({
    eventName: GAME_CONFIG.EVENT_NAME,
    handler: handleGameProcessStatusChanged,
    getInitialData: getInitialGameProcessStatus,
  });

  return {
    processInfo,
    gameRunning: processInfo?.running || false,
    isListening,
    error,
  };
}
