import { useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import type { LevelingStats } from '@/types/leveling';
import type { ExtractPayload, LevelingStatsUpdatedEvent } from '@/utils/events/registry';
import { EVENT_KEYS } from '@/utils/events/registry';

export const levelingQueryKeys = {
  all: ['leveling'] as const,
  stats: (characterId: string) => [...levelingQueryKeys.all, 'stats', characterId] as const,
};

export function useLevelingStats(characterId: string | undefined) {
  const queryClient = useQueryClient();

  useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.LevelingStatsUpdated,
        handler: (payload: unknown) => {
          const { character_id, stats } = payload as ExtractPayload<LevelingStatsUpdatedEvent>;

          queryClient.setQueryData(levelingQueryKeys.stats(character_id), stats);
        },
      },
    ],
    [queryClient],
  );

  return useQuery({
    queryKey: levelingQueryKeys.stats(characterId ?? ''),
    queryFn: async (): Promise<LevelingStats> => {
      return await invoke<LevelingStats>('get_leveling_stats', {
        characterId: characterId ?? '',
      });
    },
    enabled: !!characterId,
    staleTime: Infinity,
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: false,
  });
}
