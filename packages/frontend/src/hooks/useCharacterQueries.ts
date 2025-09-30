import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { CharacterFormData } from '../components/character/character-modals/character-form-modal';
import type { CharacterData, CharacterUpdateParams } from '../types';
import { useCacheInvalidation } from './useCacheInvalidation';

// Query keys for consistent caching
export const characterQueryKeys = {
  all: ['characters'] as const,
  lists: () => [...characterQueryKeys.all, 'list'] as const,
  list: (filters: string) =>
    [...characterQueryKeys.lists(), { filters }] as const,
  details: () => [...characterQueryKeys.all, 'detail'] as const,
  detail: (id: string) => [...characterQueryKeys.details(), id] as const,
  active: () => [...characterQueryKeys.all, 'active'] as const,
};

// Hook to get all characters
export function useCharacters() {
  return useQuery({
    queryKey: characterQueryKeys.lists(),
    queryFn: async (): Promise<CharacterData[]> => {
      return await invoke<CharacterData[]>('get_all_characters');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to get active character
export function useActiveCharacter() {
  return useQuery({
    queryKey: characterQueryKeys.active(),
    queryFn: async (): Promise<CharacterData | null> => {
      return await invoke<CharacterData | null>('get_active_character');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to get a specific character by ID
export function useCharacter(characterId: string) {
  return useQuery({
    queryKey: characterQueryKeys.detail(characterId),
    queryFn: async (): Promise<CharacterData | null> => {
      return await invoke<CharacterData | null>('get_character', {
        characterId,
      });
    },
    enabled: !!characterId,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to create a new character
export function useCreateCharacter() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CharacterFormData): Promise<CharacterData> => {
      return await invoke<CharacterData>('create_character', {
        name: data.name,
        class: data.class,
        ascendency: data.ascendency,
        league: data.league,
        hardcore: data.hardcore,
        soloSelfFound: data.solo_self_found,
      });
    },
    onSuccess: () => {
      // Invalidate and refetch character queries
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.all });
    },
  });
}

// Hook to update a character
export function useUpdateCharacter() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      characterId,
      data,
    }: {
      characterId: string;
      data: CharacterFormData;
    }): Promise<CharacterData> => {
      const updateParams: CharacterUpdateParams = {
        name: data.name,
        class: data.class,
        ascendency: data.ascendency,
        league: data.league,
        hardcore: data.hardcore,
        solo_self_found: data.solo_self_found,
        level: 1, // Default level for now - can be updated later if needed
      };

      return await invoke<CharacterData>('update_character', {
        characterId,
        updateParams,
      });
    },
    onSuccess: updatedCharacter => {
      // Update the specific character in cache
      queryClient.setQueryData(
        characterQueryKeys.detail(updatedCharacter.id),
        updatedCharacter
      );
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
  });
}

// Hook to delete a character
export function useDeleteCharacter() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (characterId: string): Promise<void> => {
      return await invoke('delete_character', { characterId });
    },
    onSuccess: (_, characterId) => {
      // Remove the character from cache
      queryClient.removeQueries({
        queryKey: characterQueryKeys.detail(characterId),
      });
      // Invalidate lists and active character
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
  });
}

// Hook to set active character
export function useSetActiveCharacter() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (characterId: string): Promise<void> => {
      return await invoke('set_active_character', { characterId });
    },
    onSuccess: () => {
      // Invalidate active character query to refetch
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
  });
}
