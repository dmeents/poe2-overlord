import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';
import type { Character, TimeTrackingData } from '../types';

export function useCharacterTimeTracking() {
  const [activeCharacter, setActiveCharacter] = useState<Character | null>(
    null
  );
  const [timeTrackingData, setTimeTrackingData] =
    useState<TimeTrackingData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notification, setNotification] = useState<{
    type: 'info' | 'success' | 'warning';
    message: string;
  } | null>(null);

  // Load active character
  const loadActiveCharacter = useCallback(async () => {
    try {
      const character = await invoke<Character | null>('get_active_character');
      setActiveCharacter(character);
    } catch (err) {
      console.error('Failed to load active character:', err);
      setActiveCharacter(null);
    }
  }, []);

  // Fetch time tracking data for active character
  const fetchTimeTrackingData = useCallback(async () => {
    if (!activeCharacter) {
      setTimeTrackingData(null);
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      const data = await invoke<TimeTrackingData>(
        'get_character_time_tracking_data',
        {
          characterId: activeCharacter.id,
        }
      );
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
  }, [activeCharacter]);

  // Refresh all data
  const refreshData = useCallback(async () => {
    await loadActiveCharacter();
    await fetchTimeTrackingData();
  }, [loadActiveCharacter, fetchTimeTrackingData]);

  // Clear all time tracking data for active character
  const clearAllData = useCallback(async () => {
    if (!activeCharacter) return;

    try {
      setError(null);
      await invoke('clear_character_time_tracking_data', {
        characterId: activeCharacter.id,
      });
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
  }, [activeCharacter, fetchTimeTrackingData]);

  // Start a time tracking session for active character
  const startSession = useCallback(
    async (locationName: string, locationType: 'Zone' | 'Act' | 'Hideout') => {
      if (!activeCharacter) {
        setError('No active character selected');
        return;
      }

      try {
        setError(null);
        await invoke('start_character_time_tracking_session', {
          characterId: activeCharacter.id,
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
    [activeCharacter, fetchTimeTrackingData]
  );

  // End a time tracking session for active character
  const endSession = useCallback(
    async (locationId: string) => {
      if (!activeCharacter) {
        setError('No active character selected');
        return;
      }

      try {
        setError(null);
        await invoke('end_character_time_tracking_session', {
          characterId: activeCharacter.id,
          locationId,
        });
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
    [activeCharacter, fetchTimeTrackingData]
  );

  // End all active sessions for active character
  const endAllActiveSessions = useCallback(async () => {
    if (!activeCharacter) {
      setError('No active character selected');
      return;
    }

    try {
      setError(null);
      await invoke('end_all_character_active_sessions', {
        characterId: activeCharacter.id,
      });
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
  }, [activeCharacter, fetchTimeTrackingData]);

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

  // Load active character and time tracking data on mount
  useEffect(() => {
    loadActiveCharacter();
  }, [loadActiveCharacter]);

  // Fetch time tracking data when active character changes
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
    // Character info
    activeCharacter,
    hasActiveCharacter: !!activeCharacter,

    // Data from character-specific endpoint
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
    loadActiveCharacter,
    fetchTimeTrackingData,
    refreshData,
    clearAllData,
    startSession,
    endSession,
    endAllActiveSessions,
    clearNotification: () => setNotification(null),
  };
}
