import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type {
  GameDataVersion,
  ItemCategory,
  ItemData,
  ItemSearchParams,
  ItemSearchResult,
} from '@/types/item-data';

export const itemDataQueryKeys = {
  all: ['item-data'] as const,
  byId: (id: string) => [...itemDataQueryKeys.all, 'by-id', id] as const,
  byName: (name: string) => [...itemDataQueryKeys.all, 'by-name', name] as const,
  search: (params: ItemSearchParams) => [...itemDataQueryKeys.all, 'search', params] as const,
  categories: () => [...itemDataQueryKeys.all, 'categories'] as const,
  favorites: () => [...itemDataQueryKeys.all, 'favorites'] as const,
  version: () => [...itemDataQueryKeys.all, 'version'] as const,
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
 * Fetches a single item by its internal id (e.g. "base/Metadata/Items/...").
 * Set id to null to disable.
 */
export function useItem(id: string | null) {
  return useQuery({
    queryKey: itemDataQueryKeys.byId(id ?? ''),
    queryFn: () => invoke<ItemData | null>('get_item', { id }),
    enabled: !!id,
    staleTime: Infinity,
    gcTime: Infinity,
  });
}

/**
 * Searches items with optional filters and pagination.
 * staleTime is Infinity because game data is static between patch imports.
 */
export function useSearchItems(params: ItemSearchParams) {
  return useQuery({
    queryKey: itemDataQueryKeys.search(params),
    queryFn: () => invoke<ItemSearchResult>('search_items', { params }),
    staleTime: Infinity,
    gcTime: Infinity,
  });
}

/**
 * Fetches the full list of item categories for populating filter dropdowns.
 * staleTime is Infinity because categories only change on a patch import.
 */
export function useItemCategories() {
  return useQuery({
    queryKey: itemDataQueryKeys.categories(),
    queryFn: () => invoke<ItemCategory[]>('get_item_categories'),
    staleTime: Infinity,
    gcTime: Infinity,
  });
}

/**
 * Fetches all favorited items, ordered by most recently favorited first.
 */
export function useFavoriteItems() {
  return useQuery({
    queryKey: itemDataQueryKeys.favorites(),
    queryFn: () => invoke<ItemData[]>('get_favorite_items'),
    staleTime: 0,
  });
}

/**
 * Returns the currently imported game data version (patch version, extraction
 * timestamp, import timestamp). Null when no data has been imported yet.
 */
export function useGameDataVersion() {
  return useQuery({
    queryKey: itemDataQueryKeys.version(),
    queryFn: () => invoke<GameDataVersion | null>('get_game_data_version'),
    staleTime: Infinity,
    gcTime: Infinity,
  });
}

/**
 * Toggles the favorite state for an item. Returns true if the item is now
 * favorited, false if it was removed.
 *
 * Invalidates the favorites list on success so it stays in sync.
 */
export function useToggleFavorite() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (itemId: string) => invoke<boolean>('toggle_item_favorite', { itemId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: itemDataQueryKeys.favorites() });
    },
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
