import type {
  LocationSession,
  LocationStats,
  TimeTrackingSummary,
} from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';

export function useTimeTracking() {
  const [activeSessions, setActiveSessions] = useState<LocationSession[]>([]);
  const [completedSessions, setCompletedSessions] = useState<LocationSession[]>(
    []
  );
  const [allStats, setAllStats] = useState<LocationStats[]>([]);
  const [summary, setSummary] = useState<TimeTrackingSummary | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notification, setNotification] = useState<{
    type: 'info' | 'success' | 'warning';
    message: string;
  } | null>(null);

  // Fetch active sessions
  const fetchActiveSessions = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const sessions = await invoke<LocationSession[]>('get_active_sessions');
      setActiveSessions(sessions);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to fetch active sessions'
      );
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Fetch completed sessions
  const fetchCompletedSessions = useCallback(async () => {
    try {
      setError(null);
      const sessions = await invoke<LocationSession[]>(
        'get_completed_sessions'
      );
      setCompletedSessions(sessions);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to fetch completed sessions'
      );
    }
  }, []);

  // Fetch all location statistics
  const fetchAllStats = useCallback(async () => {
    try {
      setError(null);
      const stats = await invoke<LocationStats[]>('get_all_location_stats');
      setAllStats(stats);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to fetch location statistics'
      );
    }
  }, []);

  // Fetch time tracking summary
  const fetchSummary = useCallback(async () => {
    try {
      setError(null);
      const summaryData = await invoke<TimeTrackingSummary>(
        'get_time_tracking_summary'
      );
      setSummary(summaryData);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to fetch time tracking summary'
      );
    }
  }, []);

  // Refresh all data
  const refreshData = useCallback(async () => {
    await Promise.all([
      fetchActiveSessions(),
      fetchCompletedSessions(),
      fetchAllStats(),
      fetchSummary(),
    ]);
  }, [
    fetchActiveSessions,
    fetchCompletedSessions,
    fetchAllStats,
    fetchSummary,
  ]);

  // Clear all time tracking data
  const clearAllData = useCallback(async () => {
    try {
      setError(null);
      await invoke('clear_all_time_tracking_data');
      await refreshData();
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to clear time tracking data'
      );
    }
  }, [refreshData]);

  // Start a time tracking session
  const startSession = useCallback(
    async (locationName: string, locationType: 'Zone' | 'Act') => {
      try {
        setError(null);
        await invoke('start_time_tracking_session', {
          locationName,
          locationType: locationType.toLowerCase(),
        });
        await fetchActiveSessions();
      } catch (err) {
        setError(
          err instanceof Error
            ? err.message
            : 'Failed to start time tracking session'
        );
      }
    },
    [fetchActiveSessions]
  );

  // End a time tracking session
  const endSession = useCallback(
    async (locationId: string) => {
      try {
        setError(null);
        await invoke('end_time_tracking_session', { locationId });
        await refreshData();
      } catch (err) {
        setError(
          err instanceof Error
            ? err.message
            : 'Failed to end time tracking session'
        );
      }
    },
    [refreshData]
  );

  // End all active time tracking sessions
  const endAllActiveSessions = useCallback(async () => {
    try {
      setError(null);
      await invoke('end_all_active_sessions');
      await refreshData();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to end all active sessions'
      );
    }
  }, [refreshData]);

  // Clear notification
  const clearNotification = useCallback(() => {
    setNotification(null);
  }, []);

  // Set up real-time event listeners
  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupListeners = async () => {
      try {
        // Listen for session started events
        const unlistenSessionStarted = await listen(
          'time-tracking-session-started',
          () => {
            fetchActiveSessions();
          }
        );
        unlistenFns.push(unlistenSessionStarted);

        // Listen for session ended events
        const unlistenSessionEnded = await listen(
          'time-tracking-session-ended',
          () => {
            refreshData();
          }
        );
        unlistenFns.push(unlistenSessionEnded);

        // Listen for stats updated events
        const unlistenStatsUpdated = await listen(
          'time-tracking-stats-updated',
          () => {
            fetchAllStats();
            fetchSummary();
          }
        );
        unlistenFns.push(unlistenStatsUpdated);

        // Listen for POE2 process status changes
        const unlistenProcessStatus = await listen(
          'poe2-process-status',
          event => {
            const processInfo = event.payload as { running: boolean };

            // If POE2 process stopped and we had active sessions, show notification
            if (!processInfo.running && activeSessions.length > 0) {
              setNotification({
                type: 'info',
                message: `POE2 process exited. All active time tracking sessions have been automatically ended.`,
              });

              // Clear notification after 5 seconds
              setTimeout(() => setNotification(null), 5000);
            }
          }
        );
        unlistenFns.push(unlistenProcessStatus);
      } catch (err) {
        console.error('Failed to set up time tracking event listeners:', err);
      }
    };

    setupListeners();

    // Cleanup function
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, [fetchActiveSessions, fetchAllStats, fetchSummary, refreshData]);

  // Initial data fetch
  useEffect(() => {
    refreshData();
  }, [refreshData]);

  return {
    // Data
    activeSessions,
    completedSessions,
    allStats,
    summary,

    // State
    isLoading,
    error,
    notification,

    // Actions
    refreshData,
    clearAllData,
    startSession,
    endSession,
    endAllActiveSessions,
    clearNotification,

    // Individual fetch functions
    fetchActiveSessions,
    fetchCompletedSessions,
    fetchAllStats,
    fetchSummary,
  };
}
