import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { ItemData } from '@/types/item-data';

export const itemDataQueryKeys = {
  all: ['item-data'] as const,
  byName: (name: string) => [...itemDataQueryKeys.all, 'by-name', name] as const,
  image: (url: string) => [...itemDataQueryKeys.all, 'image', url] as const,
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

/**
 * Resolves a `web.poecdn.com/image/...` URL to a POE2 art data URL by
 * routing through the backend proxy. The backend fetches from
 * `cdn.poe2db.tw` (which requires a Referer header browsers can't send),
 * caches to disk, and returns a `data:image/webp;base64,...` string the
 * `<img>` tag can use directly.
 *
 * staleTime/gcTime are Infinity — per-patch item art doesn't change within
 * a session, and the on-disk cache is authoritative across sessions.
 *
 * Pass null to disable (e.g. when no URL is available yet).
 */
export function useItemImage(poecdnUrl: string | null | undefined) {
  return useQuery({
    queryKey: itemDataQueryKeys.image(poecdnUrl ?? ''),
    queryFn: () => invoke<string>('get_item_image', { url: poecdnUrl }),
    enabled: !!poecdnUrl,
    staleTime: Infinity,
    gcTime: Infinity,
  });
}
