import { useCallback } from 'react';
import type { CharacterFormData } from '../components/character-modals/character-form-modal';
import {
  useActiveCharacter,
  useCharacters,
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from './useCharacterQueries';

export function useCharacterManagement() {
  // Use React Query hooks for data fetching
  const {
    data: characters = [],
    isLoading: charactersLoading,
    error: charactersError,
  } = useCharacters();

  const {
    data: activeCharacter = null,
    isLoading: activeCharacterLoading,
    error: activeCharacterError,
  } = useActiveCharacter();

  // Mutation hooks
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();

  // Derived state
  const isLoading = charactersLoading || activeCharacterLoading;
  const error =
    charactersError?.message || activeCharacterError?.message || null;

  // Wrapper functions that use mutations
  const createCharacter = useCallback(
    async (data: CharacterFormData) => {
      try {
        return await createCharacterMutation.mutateAsync(data);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to create character';
        throw new Error(errorMessage);
      }
    },
    [createCharacterMutation]
  );

  const updateCharacter = useCallback(
    async (characterId: string, data: CharacterFormData) => {
      try {
        return await updateCharacterMutation.mutateAsync({ characterId, data });
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to update character';
        throw new Error(errorMessage);
      }
    },
    [updateCharacterMutation]
  );

  const deleteCharacter = useCallback(
    async (characterId: string) => {
      try {
        await deleteCharacterMutation.mutateAsync(characterId);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to delete character';
        throw new Error(errorMessage);
      }
    },
    [deleteCharacterMutation]
  );

  const setActiveCharacterId = useCallback(
    async (characterId: string) => {
      try {
        await setActiveCharacterMutation.mutateAsync(characterId);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to set active character';
        throw new Error(errorMessage);
      }
    },
    [setActiveCharacterMutation]
  );

  // Legacy functions for backward compatibility (no-op since React Query handles caching)
  const loadCharacters = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  const loadActiveCharacter = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  return {
    characters,
    activeCharacter,
    isLoading,
    error,
    loadCharacters,
    loadActiveCharacter,
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
  };
}
