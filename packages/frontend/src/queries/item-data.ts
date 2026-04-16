import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { ItemData } from '@/types/item-data';

export const itemDataQueryKeys = {
  all: ['item-data'] as const,
  byName: (name: string) => [...itemDataQueryKeys.all, 'by-name', name] as const,
};

/**
 * Fetches game data for an item by its exact name.
 *
 * Bridges the economy domain (poe.ninja slugs) and item_data domain (metadata paths)
 * using the shared `name` field. Returns the base item (is_unique = false).
 *
 * Set name to null to disable the query (used to defer loading until hover).
 * staleTime is Infinity because game data is static between patch imports.
 */
export function useItemByName(name: string | null) {
  return useQuery({
    queryKey: itemDataQueryKeys.byName(name ?? ''),
    queryFn: () => invoke<ItemData | null>('get_item_by_name', { name }),
    enabled: !!name,
    staleTime: Infinity,
    gcTime: Infinity,
  });
}
