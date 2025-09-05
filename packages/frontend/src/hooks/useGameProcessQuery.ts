import type { ProcessInfo } from '@/types';
import { GAME_CONFIG, tauriUtils } from '@/utils';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect } from 'react';

const GAME_PROCESS_QUERY_KEY = ['game-process'] as const;

export function useGameProcessQuery() {
  const queryClient = useQueryClient();

  // Set up the query with initial data fetch, then event-driven updates
  const {
    data: processInfo,
    isLoading,
    error,
  } = useQuery({
    queryKey: GAME_PROCESS_QUERY_KEY,
    queryFn: async () => {
      // Initial process check
      const info = await tauriUtils.checkGameProcess();
      return info;
    },
    initialData: null,
    staleTime: 30 * 1000, // 30 seconds - process status can change
    gcTime: 5 * 60 * 1000, // 5 minutes - keep in cache for a while
  });

  // Set up real-time event listeners for game process updates
  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupListeners = async () => {
      try {
        // Listen for game process status updates from Rust backend
        const unlistenProcess = await listen<ProcessInfo>(
          GAME_CONFIG.EVENT_NAME,
          event => {
            console.log('Game process status updated event received:', event);
            const processInfo = event.payload;

            // Update the query cache with the new process info
            queryClient.setQueryData(GAME_PROCESS_QUERY_KEY, processInfo);
          }
        );
        unlistenFns.push(unlistenProcess);
      } catch (err) {
        console.error('Failed to set up game process event listeners:', err);
      }
    };

    setupListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, [queryClient]);

  // Check game process (for manual refresh if needed)
  const checkGameProcess = useCallback(async () => {
    await queryClient.invalidateQueries({ queryKey: GAME_PROCESS_QUERY_KEY });
  }, [queryClient]);

  return {
    processInfo,
    gameRunning: processInfo?.running || false,
    isLoading,
    error: error?.message || null,
    checkGameProcess,
  };
}
