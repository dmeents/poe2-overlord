import type { ServerStatus, ServerStatusChangedEvent } from '@/types';
import { useCallback, useState } from 'react';
import { useTauriEventListener } from './useTauriEventListener';

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

  // Handler for server status events
  const handleServerStatusChanged = useCallback((event: ServerStatusChangedEvent) => {
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
  }, []);

  // Get initial server status
  const getInitialServerStatus = useCallback(async (): Promise<ServerStatus | null> => {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<ServerStatus>('get_server_status');
    } catch {
      // No initial status available, rely on events only
      return null;
    }
  }, []);

  // Use the generic Tauri event listener
  const { isListening, error } = useTauriEventListener({
    eventName: 'server-status-changed',
    handler: handleServerStatusChanged,
    getInitialData: getInitialServerStatus,
  });

  return {
    serverStatus,
    isListening,
    error,
  };
}
