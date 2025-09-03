import type { TimeTrackingData } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';

export function useTimeTracking() {
  const [timeTrackingData, setTimeTrackingData] =
    useState<TimeTrackingData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notification, setNotification] = useState<{
    type: 'info' | 'success' | 'warning';
    message: string;
  } | null>(null);

  // Fetch all time tracking data in a single call
  const fetchTimeTrackingData = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const data = await invoke<TimeTrackingData>('get_time_tracking_data');
      setTimeTrackingData(data);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to fetch time tracking data'
      );
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Refresh all data
  const refreshData = useCallback(async () => {
    await fetchTimeTrackingData();
  }, [fetchTimeTrackingData]);

  // Clear all time tracking data
  const clearAllData = useCallback(async () => {
    try {
      setError(null);
      await invoke('clear_all_time_tracking_data');
      setNotification({
        type: 'success',
        message: 'All time tracking data cleared successfully',
      });
      // Refresh data after clearing
      await fetchTimeTrackingData();
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to clear time tracking data'
      );
    }
  }, [fetchTimeTrackingData]);

  // Start a time tracking session
  const startSession = useCallback(
    async (locationName: string, locationType: 'Zone' | 'Act' | 'Hideout') => {
      try {
        setError(null);
        await invoke('start_time_tracking_session', {
          locationName,
          locationType,
        });
        setNotification({
          type: 'success',
          message: `Started tracking session for ${locationName}`,
        });
        // Refresh data after starting session
        await fetchTimeTrackingData();
      } catch (err) {
        setError(
          err instanceof Error
            ? err.message
            : 'Failed to start time tracking session'
        );
      }
    },
    [fetchTimeTrackingData]
  );

  // End a time tracking session
  const endSession = useCallback(
    async (locationId: string) => {
      try {
        setError(null);
        await invoke('end_time_tracking_session', { locationId });
        setNotification({
          type: 'success',
          message: 'Time tracking session ended successfully',
        });
        // Refresh data after ending session
        await fetchTimeTrackingData();
      } catch (err) {
        setError(
          err instanceof Error
            ? err.message
            : 'Failed to end time tracking session'
        );
      }
    },
    [fetchTimeTrackingData]
  );

  // End all active sessions
  const endAllActiveSessions = useCallback(async () => {
    try {
      setError(null);
      await invoke('end_all_active_sessions');
      setNotification({
        type: 'success',
        message: 'All active sessions ended successfully',
      });
      // Refresh data after ending sessions
      await fetchTimeTrackingData();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to end all active sessions'
      );
    }
  }, [fetchTimeTrackingData]);

  // Set up real-time event listeners for time tracking updates
  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    const setupListeners = async () => {
      try {
        // Listen for time tracking events
        const unlistenSessionStarted = await listen(
          'time-tracking-session-started',
          event => {
            console.log('Time tracking session started event received:', event);
            fetchTimeTrackingData();
          }
        );
        unlistenFns.push(unlistenSessionStarted);

        const unlistenSessionEnded = await listen(
          'time-tracking-session-ended',
          event => {
            console.log('Time tracking session ended event received:', event);
            fetchTimeTrackingData();
          }
        );
        unlistenFns.push(unlistenSessionEnded);

        const unlistenStatsUpdated = await listen(
          'time-tracking-stats-updated',
          event => {
            console.log('Time tracking stats updated event received:', event);
            fetchTimeTrackingData();
          }
        );
        unlistenFns.push(unlistenStatsUpdated);
      } catch (err) {
        console.error('Failed to set up time tracking event listeners:', err);
      }
    };

    setupListeners();

    // Cleanup listeners
    return () => {
      unlistenFns.forEach(unlisten => unlisten());
    };
  }, [fetchTimeTrackingData]);

  // Initial load
  useEffect(() => {
    fetchTimeTrackingData();
  }, [fetchTimeTrackingData]);

  // Clear notification after 5 seconds
  useEffect(() => {
    if (notification) {
      const timer = setTimeout(() => {
        setNotification(null);
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [notification]);

  return {
    // Data from unified endpoint
    timeTrackingData,
    activeSessions: timeTrackingData?.active_sessions ?? [],
    completedSessions: timeTrackingData?.completed_sessions ?? [],
    allStats: timeTrackingData?.all_location_stats ?? [],
    summary: timeTrackingData?.summary ?? null,

    // State
    isLoading,
    error,
    notification,

    // Actions
    fetchTimeTrackingData,
    refreshData,
    clearAllData,
    startSession,
    endSession,
    endAllActiveSessions,
    clearNotification: () => setNotification(null),
  };
}
