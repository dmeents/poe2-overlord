import { useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

const MONITORING_STATUS_QUERY_KEY = ['monitoring-status'] as const;

export function useMonitoringQuery() {
  const queryClient = useQueryClient();

  // Set up the query for monitoring status - fetch initial data, then cache it
  const {
    data: isMonitoring,
    isLoading,
    error,
  } = useQuery({
    queryKey: MONITORING_STATUS_QUERY_KEY,
    queryFn: async () => {
      const status = await invoke<boolean>('is_log_monitoring_active');
      return status;
    },
    initialData: false,
    staleTime: 10 * 1000, // 10 seconds - monitoring status can change frequently
    gcTime: 2 * 60 * 1000, // 2 minutes - keep in cache briefly
  });

  // Refresh monitoring status (for manual refresh if needed)
  const refreshMonitoringStatus = useCallback(async () => {
    await queryClient.invalidateQueries({
      queryKey: MONITORING_STATUS_QUERY_KEY,
    });
  }, [queryClient]);

  return {
    isMonitoring: isMonitoring || false,
    isLoading,
    error: error?.message || null,
    refreshMonitoringStatus,
  };
}
