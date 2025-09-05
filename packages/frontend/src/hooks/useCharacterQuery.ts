import type { Character } from '@/types';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

const ACTIVE_CHARACTER_QUERY_KEY = ['active-character'] as const;

export function useCharacterQuery() {
  const queryClient = useQueryClient();

  // Set up the query for active character - fetch initial data, then cache it
  const {
    data: activeCharacter,
    isLoading,
    error,
  } = useQuery({
    queryKey: ACTIVE_CHARACTER_QUERY_KEY,
    queryFn: async () => {
      const character = await invoke<Character | null>('get_active_character');
      return character;
    },
    initialData: null,
    staleTime: 30 * 1000, // 30 seconds - character can change
    gcTime: 5 * 60 * 1000, // 5 minutes - keep in cache
  });

  // Refresh active character (for manual refresh if needed)
  const refreshActiveCharacter = useCallback(async () => {
    await queryClient.invalidateQueries({
      queryKey: ACTIVE_CHARACTER_QUERY_KEY,
    });
  }, [queryClient]);

  return {
    activeCharacter,
    isLoading,
    error: error?.message || null,
    refreshActiveCharacter,
  };
}
