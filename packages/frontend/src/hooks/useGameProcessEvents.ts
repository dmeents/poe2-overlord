import type { GameProcessStatusChangedEvent, ProcessInfo } from '@/types/process';
import { GAME_CONFIG } from '@/utils/constants';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

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
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupEventListeners = async () => {
      try {
        // Listen for game process status updates from Rust backend
        const unlistenProcess = await listen<GameProcessStatusChangedEvent>(
          GAME_CONFIG.EVENT_NAME,
          event => {
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
          }
        );

        unlistenFns.push(unlistenProcess);
        setIsListening(true);

        // Request initial status from backend to handle timing issues
        try {
          const { invoke } = await import('@tauri-apps/api/core');
          const initialStatus = await invoke<ProcessInfo>(
            'get_game_process_status'
          );
          setProcessInfo(initialStatus);
        } catch {
          // No initial status available, rely on events only
        }
      } catch {
        // Failed to set up event listeners
      }
    };

    setupEventListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
      setIsListening(false);
    };
  }, []);

  return {
    processInfo,
    gameRunning: processInfo?.running || false,
    isListening,
  };
}
