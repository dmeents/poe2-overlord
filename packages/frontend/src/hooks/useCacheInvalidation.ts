import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { characterQueryKeys } from './useCharacterQueries';

/**
 * Hook for managing automatic cache invalidation based on events.
 * Provides standardized cache invalidation strategies for different data types.
 */
export function useCacheInvalidation() {
  const queryClient = useQueryClient();

  // Invalidate character-related queries
  const invalidateCharacterQueries = useCallback(
    (characterId?: string) => {
      if (characterId) {
        // Update specific character in cache if we have the data
        // This will be called with the updated data from the event
        queryClient.invalidateQueries({
          queryKey: characterQueryKeys.detail(characterId),
        });
      }

      // Always invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.lists() });

      // Invalidate active character query
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
    [queryClient]
  );

  // Invalidate all character queries (for major changes)
  const invalidateAllCharacterQueries = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: characterQueryKeys.all });
  }, [queryClient]);

  // Generic invalidation for any query key pattern
  const invalidateQueries = useCallback(
    (queryKey: string[]) => {
      queryClient.invalidateQueries({ queryKey });
    },
    [queryClient]
  );

  // Set query data directly (for optimistic updates)
  const setQueryData = useCallback(
    <T>(queryKey: string[], data: T) => {
      queryClient.setQueryData(queryKey, data);
    },
    [queryClient]
  );

  // Remove specific queries from cache
  const removeQueries = useCallback(
    (queryKey: string[]) => {
      queryClient.removeQueries({ queryKey });
    },
    [queryClient]
  );

  return {
    invalidateCharacterQueries,
    invalidateAllCharacterQueries,
    invalidateQueries,
    setQueryData,
    removeQueries,
  };
}
