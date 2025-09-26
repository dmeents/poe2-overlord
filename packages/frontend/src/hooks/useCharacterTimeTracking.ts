import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import type { Character, TimeTrackingData } from '../types';

interface UseCharacterTimeTrackingProps {
  activeCharacter: Character | null;
}

export function useCharacterTimeTracking({
  activeCharacter,
}: UseCharacterTimeTrackingProps) {
  const [timeTrackingData, setTimeTrackingData] =
    useState<TimeTrackingData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

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

  useEffect(() => {
    fetchTimeTrackingData();
  }, [fetchTimeTrackingData]);

  return {
    timeTrackingData,
    activeSessions: timeTrackingData?.active_sessions ?? [],
    completedSessions: timeTrackingData?.completed_sessions ?? [],
    allStats: timeTrackingData?.all_location_stats ?? [],
    summary: timeTrackingData?.summary ?? null,
    isLoading,
    error,
    refetch: fetchTimeTrackingData,
  };
}
