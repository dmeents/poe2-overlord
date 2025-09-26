import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import type { Character } from '../types';

export function useCharacterTotalPlayTime(characters: Character[]) {
  const [playTimes, setPlayTimes] = useState<Map<string, number>>(new Map());
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchPlayTimeForCharacter = useCallback(
    async (characterId: string): Promise<number> => {
      try {
        const totalTime = await invoke<number>(
          'get_character_total_play_time',
          {
            characterId,
          }
        );
        return totalTime;
      } catch (err) {
        console.error(
          `Failed to fetch play time for character ${characterId}:`,
          err
        );
        return 0;
      }
    },
    []
  );

  const fetchAllPlayTimes = useCallback(async () => {
    if (characters.length === 0) {
      setPlayTimes(new Map());
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const playTimePromises = characters.map(async character => {
        const totalTime = await fetchPlayTimeForCharacter(character.id);
        return { characterId: character.id, totalPlayTimeSeconds: totalTime };
      });

      const results = await Promise.all(playTimePromises);
      const playTimeMap = new Map<string, number>();

      results.forEach(({ characterId, totalPlayTimeSeconds }) => {
        playTimeMap.set(characterId, totalPlayTimeSeconds);
      });

      setPlayTimes(playTimeMap);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to fetch character play times'
      );
    } finally {
      setIsLoading(false);
    }
  }, [characters, fetchPlayTimeForCharacter]);

  const getPlayTime = useCallback(
    (characterId: string): number => {
      return playTimes.get(characterId) || 0;
    },
    [playTimes]
  );

  const refreshPlayTimes = useCallback(() => {
    fetchAllPlayTimes();
  }, [fetchAllPlayTimes]);

  // Fetch play times when characters change
  useEffect(() => {
    fetchAllPlayTimes();
  }, [fetchAllPlayTimes]);

  return {
    playTimes,
    getPlayTime,
    isLoading,
    error,
    refreshPlayTimes,
  };
}
