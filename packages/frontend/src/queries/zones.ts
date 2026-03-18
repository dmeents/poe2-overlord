import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export interface ZoneMetadata {
  zone_name: string;
  area_id?: string;
  act: number;
  area_level?: number;
  is_town: boolean;
  has_waypoint: boolean;
  bosses: string[];
  monsters: string[];
  npcs: string[];
  connected_zones: string[];
  description?: string;
  points_of_interest: string[];
  image_url?: string;
  first_discovered: string;
  last_updated: string;
  wiki_url?: string;
}

export const zoneQueryKeys = {
  metadata: (zoneName: string) => ['zones', 'metadata', zoneName] as const,
};

export function useZoneMetadata(zoneName: string | undefined) {
  return useQuery({
    queryKey: zoneQueryKeys.metadata(zoneName ?? ''),
    queryFn: async (): Promise<ZoneMetadata | null> => {
      if (!zoneName) return null;
      return await invoke<ZoneMetadata | null>('get_zone_metadata_by_name', { zoneName });
    },
    enabled: !!zoneName,
    staleTime: 5 * 60 * 1000, // 5 minutes — wiki data changes infrequently
  });
}
