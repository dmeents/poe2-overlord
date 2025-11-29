import type { ServerStatus, ServerStatusChangedEvent } from '@/types/server';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

/**
 * Hook for listening to server status changes via Tauri events
 *
 * This hook provides a purely event-driven approach to server monitoring.
 * It listens for 'server-status-changed' events from the backend and
 * maintains the current server status in local state.
 *
 * @returns Object containing server status and listening status
 */
export function useServerStatusEvents() {
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupEventListeners = async () => {
      try {
        // Listen for server status updates from Rust backend
        const unlistenServerStatus = await listen<ServerStatusChangedEvent>(
          'server-status-changed',
          event => {
            // Handle the AppEvent structure - the payload is the entire AppEvent
            const eventPayload = event.payload as {
              ServerStatusChanged?: { new_status?: ServerStatus };
            };
            if (eventPayload && eventPayload.ServerStatusChanged) {
              const serverEvent = eventPayload.ServerStatusChanged;
              if (serverEvent.new_status) {
                setServerStatus(serverEvent.new_status);
              }
            }
          }
        );

        unlistenFns.push(unlistenServerStatus);
        setIsListening(true);

        // Backend will emit initial status event on startup if file exists
        // No need to invoke - rely purely on events
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
    serverStatus,
    isListening,
  };
}
