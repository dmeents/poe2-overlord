import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import type { CharacterFormData } from '../components/character-modals/character-form-modal';
import type { Character } from '../types';

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
    queryFn: async (): Promise<Character[]> => {
      return await invoke<Character[]>('get_all_characters');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to get active character
export function useActiveCharacter() {
  return useQuery({
    queryKey: characterQueryKeys.active(),
    queryFn: async (): Promise<Character | null> => {
      return await invoke<Character | null>('get_active_character');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to get a specific character by ID
export function useCharacter(characterId: string) {
  return useQuery({
    queryKey: characterQueryKeys.detail(characterId),
    queryFn: async (): Promise<Character | null> => {
      const characters = await invoke<Character[]>('get_all_characters');
      return characters.find(c => c.id === characterId) || null;
    },
    enabled: !!characterId,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

// Hook to create a new character
export function useCreateCharacter() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CharacterFormData): Promise<Character> => {
      return await invoke<Character>('create_character', {
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
    }): Promise<Character> => {
      return await invoke<Character>('update_character', {
        characterId,
        name: data.name,
        class: data.class,
        ascendency: data.ascendency,
        league: data.league,
        hardcore: data.hardcore,
        soloSelfFound: data.solo_self_found,
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
    mutationFn: async (characterId: string): Promise<Character> => {
      return await invoke<Character>('delete_character', { characterId });
    },
    onSuccess: deletedCharacter => {
      // Remove the character from cache
      queryClient.removeQueries({
        queryKey: characterQueryKeys.detail(deletedCharacter.id),
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
