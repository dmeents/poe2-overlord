import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';

export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  latency_ms: number | null;
  timestamp: string;
}

export function useServerStatus() {
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Set up real-time event listeners for server status updates
  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupListeners = async () => {
      try {
        // Listen for server status updates
        const unlistenServerStatusUpdated = await listen(
          'server-status-updated',
          event => {
            console.log('Server status updated event received:', event);
            const status = event.payload as ServerStatus;
            setServerStatus(status);
            setIsLoading(false);
          }
        );
        unlistenFns.push(unlistenServerStatusUpdated);
      } catch (err) {
        console.error('Failed to set up server status event listeners:', err);
        setError('Failed to set up server status monitoring');
      }
    };

    setupListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, []);

  // Refresh server status (for manual refresh if needed)
  const refreshStatus = useCallback(async () => {
    // Since we're event-driven, we just set loading state
    // The actual data will come from events
    setIsLoading(true);
  }, []);

  return {
    serverStatus,
    isLoading,
    error,
    refreshStatus,
  };
}
