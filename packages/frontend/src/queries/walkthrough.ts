import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { WalkthroughGuide } from '@/types/walkthrough';

const walkthroughQueryKeys = {
  all: ['walkthrough'] as const,
  guide: () => [...walkthroughQueryKeys.all, 'guide'] as const,
};

export function useWalkthroughGuide() {
  return useQuery({
    queryKey: walkthroughQueryKeys.guide(),
    queryFn: async (): Promise<WalkthroughGuide> => {
      return await invoke<WalkthroughGuide>('get_walkthrough_guide');
    },
    staleTime: Infinity, // Guide data never goes stale, only updates via events
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: false,
  });
}
