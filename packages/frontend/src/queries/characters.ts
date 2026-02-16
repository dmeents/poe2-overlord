import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { useErrorHandler } from '@/hooks/useErrorHandler';
import { parseError } from '@/utils/error-handling';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import type { CharacterData } from '../types/character';

const characterQueryKeys = {
  all: ['characters'] as const,
  lists: () => [...characterQueryKeys.all, 'list'] as const,
  list: (filters: string) => [...characterQueryKeys.lists(), { filters }] as const,
  details: () => [...characterQueryKeys.all, 'detail'] as const,
  detail: (id: string) => [...characterQueryKeys.details(), id] as const,
  active: () => [...characterQueryKeys.all, 'active'] as const,
};

export function useCharacters() {
  return useQuery({
    queryKey: characterQueryKeys.lists(),
    queryFn: async (): Promise<CharacterData[]> => {
      return await invoke<CharacterData[]>('get_all_characters');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

export function useActiveCharacter() {
  return useQuery({
    queryKey: characterQueryKeys.active(),
    queryFn: async (): Promise<CharacterData | null> => {
      return await invoke<CharacterData | null>('get_active_character');
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

export function useCreateCharacter() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

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
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.all });
    },
    onError: err => {
      const error = parseError(err);
      handleError(error);
    },
  });
}

export function useUpdateCharacter() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async ({
      characterId,
      data,
      currentLevel,
    }: {
      characterId: string;
      data: CharacterFormData;
      currentLevel: number;
    }): Promise<CharacterData> => {
      return await invoke<CharacterData>('update_character', {
        characterId,
        updateParams: {
          name: data.name,
          class: data.class,
          ascendency: data.ascendency,
          league: data.league,
          hardcore: data.hardcore,
          solo_self_found: data.solo_self_found,
          level: currentLevel,
        },
      });
    },
    onSuccess: updatedCharacter => {
      // Update the specific character in cache
      queryClient.setQueryData(characterQueryKeys.detail(updatedCharacter.id), updatedCharacter);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
    onError: err => {
      const error = parseError(err);
      handleError(error);
    },
  });
}

export function useDeleteCharacter() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

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
    onError: err => {
      const error = parseError(err);
      handleError(error);
    },
  });
}

export function useSetActiveCharacter() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async (characterId: string): Promise<void> => {
      return await invoke('set_active_character', { characterId });
    },
    onSuccess: () => {
      // Invalidate active character query to refetch
      queryClient.invalidateQueries({ queryKey: characterQueryKeys.active() });
    },
    onError: err => {
      const error = parseError(err);
      handleError(error);
    },
  });
}
