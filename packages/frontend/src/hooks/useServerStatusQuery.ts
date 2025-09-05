import { useQuery, useQueryClient } from '@tanstack/react-query';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect } from 'react';

export interface ServerStatus {
  ip_address: string;
  port: number;
  is_online: boolean;
  latency_ms: number | null;
  timestamp: string;
}

const SERVER_STATUS_QUERY_KEY = ['server-status'] as const;

export function useServerStatusQuery() {
  const queryClient = useQueryClient();

  // Set up the query with initial null data
  const {
    data: serverStatus,
    isLoading,
    error,
  } = useQuery({
    queryKey: SERVER_STATUS_QUERY_KEY,
    queryFn: () => null, // We don't fetch data, we only listen to events
    initialData: null,
    staleTime: Infinity, // Never consider data stale since it's event-driven
    gcTime: Infinity, // Never garbage collect since we want to persist across routes
  });

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

            // Update the query cache with the new server status
            queryClient.setQueryData(SERVER_STATUS_QUERY_KEY, status);
          }
        );
        unlistenFns.push(unlistenServerStatusUpdated);
      } catch (err) {
        console.error('Failed to set up server status event listeners:', err);
      }
    };

    setupListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, [queryClient]);

  // Refresh server status (for manual refresh if needed)
  const refreshStatus = useCallback(async () => {
    // Since we're event-driven, we just invalidate the query
    // The actual data will come from events
    await queryClient.invalidateQueries({ queryKey: SERVER_STATUS_QUERY_KEY });
  }, [queryClient]);

  return {
    serverStatus,
    isLoading,
    error: error?.message || null,
    refreshStatus,
  };
}
